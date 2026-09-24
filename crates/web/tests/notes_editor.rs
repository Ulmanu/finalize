#![cfg(feature = "ssr")]

//! Notes editor web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
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
        Utc.with_ymd_and_hms(2026, 6, 1, 9, 0, 0).unwrap(),
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
        .register("notes@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("notes@example.com", "correct horse battery")
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

async fn todo_with_notes(h: &Harness, notes: &str) -> momentum_domain::EntityId {
    let id = h
        .todos
        .create(h.account_id, "Notes task", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let body = format!("title=Notes+task&notes={}&", urlencode(notes));
    let (status, _) = request(h, "POST", &format!("/todos/{id}"), Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    id
}

fn urlencode(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            out.push(char::from(byte));
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

#[tokio::test]
async fn links_render_safely_and_blocked_schemes_stay_inert() {
    let h = harness().await;
    let id = todo_with_notes(
        &h,
        "Read https://example.com/docs and javascript:alert(1) now",
    )
    .await;

    let (status, page) = request(&h, "GET", &format!("/todos/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"note-links\""), "{page}");
    assert!(page.contains("href=\"https://example.com/docs\""), "{page}");
    assert!(page.contains("rel=\"noopener noreferrer\""), "{page}");
    assert!(page.contains("data-testid=\"blocked-link\""), "{page}");
    assert!(
        !page.contains("href=\"javascript:alert(1)"),
        "unsafe scheme must never become an href: {page}"
    );
}

#[tokio::test]
async fn find_traverses_and_replace_persists() {
    let h = harness().await;
    let id = todo_with_notes(&h, "alpha beta alpha gamma alpha").await;

    let (_, page) = request(&h, "GET", &format!("/todos/{id}?find=alpha"), None).await;
    assert!(page.contains("data-testid=\"find-results\""), "{page}");
    assert!(page.contains("1/3"), "{page}");
    assert!(page.contains("data-testid=\"find-next\""), "{page}");
    assert!(page.contains("data-testid=\"find-previous\""), "{page}");

    let (_, page) = request(&h, "GET", &format!("/todos/{id}?find=alpha&from=11"), None).await;
    assert!(page.contains("2/3"), "{page}");

    let body = format!("find=alpha&replace=gamma&return_to=%2Ftodos%2F{id}");
    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/notes/replace"),
        Some(&body),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.notes, "gamma beta gamma gamma gamma");

    let (_, page) = request(&h, "GET", &format!("/todos/{id}?find=alpha"), None).await;
    assert!(page.contains("No matches"), "{page}");
}

#[tokio::test]
async fn project_notes_can_be_searched_and_replaced() {
    let h = harness().await;
    let id = h
        .projects
        .create(h.account_id, "Notes project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .set_notes(h.account_id, id, "draft draft final")
        .await
        .expect("notes");

    let (_, page) = request(&h, "GET", &format!("/projects/{id}?find=draft"), None).await;
    assert!(page.contains("1/2"), "{page}");
    assert!(page.contains("data-testid=\"notes-tools\""), "{page}");

    let body = format!("find=draft&replace=reviewed&return_to=%2Fprojects%2F{id}");
    let (status, _) = request(
        &h,
        "POST",
        &format!("/projects/{id}/notes/replace"),
        Some(&body),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.notes, "reviewed reviewed final");
}
