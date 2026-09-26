#![cfg(feature = "ssr")]

//! Someday projection web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
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
    account_id: momentum_domain::AccountId,
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
        .register("someday@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("someday@example.com", "correct horse battery")
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

async fn request(h: &Harness, method: &str, uri: &str, body: Option<&str>) -> (StatusCode, String) {
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
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (status, String::from_utf8(bytes.to_vec()).expect("utf-8"))
}

#[tokio::test]
async fn someday_lists_deferred_todos_and_projects_with_reactivation() {
    let h = harness().await;
    let todo = h
        .todos
        .create(h.account_id, "Deferred todo", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account_id, todo, ScheduleInput::Someday)
        .await
        .expect("defer");
    let project = h
        .projects
        .create(h.account_id, "Deferred project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .schedule(h.account_id, project, ScheduleInput::Someday)
        .await
        .expect("defer");

    let (status, page) = request(&h, "GET", "/someday", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"someday-todos\""), "{page}");
    assert!(page.contains("Deferred todo"), "{page}");
    assert!(
        page.contains("data-testid=\"someday-project-row\""),
        "{page}"
    );
    assert!(page.contains("Deferred project"), "{page}");
    assert!(page.contains("aria-label=\"Reactivate project\""), "{page}");

    let body = "when=anytime&return_to=%2Fsomeday";
    let (status, _) = request(&h, "POST", &format!("/todos/{todo}/schedule"), Some(body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (status, _) = request(
        &h,
        "POST",
        &format!("/projects/{project}/schedule"),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (_, page) = request(&h, "GET", "/someday", None).await;
    assert!(page.contains("data-testid=\"someday-empty\""), "{page}");
    let (_, anytime) = request(&h, "GET", "/anytime", None).await;
    assert!(anytime.contains("Deferred todo"), "{anytime}");
}

#[tokio::test]
async fn someday_empty_state_without_deferred_items() {
    let h = harness().await;
    let (status, page) = request(&h, "GET", "/someday", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"someday-empty\""), "{page}");
}
