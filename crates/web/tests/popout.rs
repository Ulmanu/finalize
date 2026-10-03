#![cfg(feature = "ssr")]

//! Popout windows: chrome-less destinations, shared state, and window-safe commands.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::projects::ProjectService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::AccountId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
    account_id: AccountId,
    cookie: String,
}

async fn harness() -> Harness {
    let store = Arc::new(MemoryAuthStore::new());
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
    ));
    let auth = Arc::new(AuthService::new(
        store.clone(),
        store.clone(),
        store.clone(),
        Arc::new(DeterministicHasher),
        Arc::new(DeterministicTokenGenerator::new()),
        clock.clone(),
    ));
    let database = MemoryDatabase::new();
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    ));
    let projects = Arc::new(ProjectService::new(
        dispatcher,
        Arc::new(MemoryProjectReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("popout@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("popout@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        Some(projects.clone()),
    );

    Harness {
        router: server::router_with_state(state),
        todos,
        projects,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(
    h: &Harness,
    method: &str,
    uri: &str,
    body: Option<&str>,
) -> (StatusCode, Option<String>, String) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, &h.cookie);
    if body.is_some() {
        builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    }
    let request = builder
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        location,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

#[tokio::test]
async fn popout_omits_chrome_and_offers_an_exit() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Shared item", None)
        .await
        .expect("create");

    let (status, _, body) = request(&h, "GET", "/inbox?popout=1", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-window=\"popout\""), "{body}");
    assert!(body.contains("data-testid=\"popout-header\""), "{body}");
    assert!(body.contains("data-testid=\"popout-exit\""), "{body}");
    assert!(body.contains("Shared item"), "{body}");
    assert!(!body.contains("id=\"app-sidebar\""), "{body}");
    assert!(!body.contains("class=\"bottom-nav\""), "{body}");
    assert!(!body.contains("class=\"app-topbar\""), "{body}");

    let (status, _, body) = request(&h, "GET", "/inbox", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-window=\"full\""), "{body}");
    assert!(body.contains("id=\"app-sidebar\""), "{body}");
    assert!(body.contains("data-testid=\"popout-link\""), "{body}");
    assert!(body.contains("?popout=1"), "{body}");
    assert!(body.contains("target=\"_blank\""), "{body}");
}

#[tokio::test]
async fn popout_projects_and_commands_share_state() {
    let h = harness().await;
    h.projects
        .create(h.account_id, "Windowed project", None)
        .await
        .expect("project");
    let project = h
        .projects
        .list(h.account_id)
        .await
        .expect("projects")
        .into_iter()
        .next()
        .expect("project")
        .id;
    let (status, _, body) =
        request(&h, "GET", &format!("/projects/{project}?popout=1"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Windowed project"), "{body}");
    assert!(!body.contains("id=\"app-sidebar\""), "{body}");

    // Commands issued from a popout are ordinary commands; the full window sees the state.
    h.todos
        .create(h.account_id, "Popout task", None)
        .await
        .expect("create");
    let todo = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;
    let (status, location, _) = request(
        &h,
        "POST",
        &format!("/todos/{todo}/complete"),
        Some("return_to=%2Finbox%3Fpopout%3D1"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(location.is_some());
    assert!(h
        .todos
        .get(h.account_id, todo)
        .await
        .unwrap()
        .unwrap()
        .completed_at
        .is_some());
    let (_, _, body) = request(&h, "GET", "/inbox?popout=1", None).await;
    assert!(!body.contains("Popout task"), "{body}");
}
