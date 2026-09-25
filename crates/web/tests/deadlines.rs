#![cfg(feature = "ssr")]

//! Deadline web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::TodoService;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    todos: Arc<TodoService>,
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
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("deadline@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("deadline@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        None,
    );

    Harness {
        router: server::router_with_state(state),
        todos,
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
async fn deadlines_render_state_and_drive_the_deadlines_page() {
    let h = harness().await;
    let id = h
        .todos
        .create(h.account_id, "Deadline task", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/deadline"),
        Some(&format!("date=2026-06-10&return_to=%2Ftodos%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (_, detail) = request(&h, "GET", &format!("/todos/{id}"), None).await;
    assert!(detail.contains("Deadline: 2026-06-10"), "{detail}");
    let (_, inbox) = request(&h, "GET", "/inbox", None).await;
    assert!(inbox.contains("data-testid=\"deadline-badge\""), "{inbox}");
    assert!(inbox.contains("Due today: 2026-06-10"), "{inbox}");

    let (status, page) = request(&h, "GET", "/deadlines", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"deadlines-list\""), "{page}");
    assert!(page.contains("Deadline task"), "{page}");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/deadline"),
        Some(&format!("date=&return_to=%2Ftodos%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, page) = request(&h, "GET", "/deadlines", None).await;
    assert!(!page.contains("Deadline task"), "{page}");
    let (_, inbox) = request(&h, "GET", "/inbox", None).await;
    assert!(!inbox.contains("data-testid=\"deadline-badge\""), "{inbox}");
}

#[tokio::test]
async fn overdue_badges_appear_in_list_views() {
    let h = harness().await;
    let id = h
        .todos
        .create(h.account_id, "Overdue task", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .set_deadline(
            h.account_id,
            id,
            Some(momentum_domain::time::LocalDate::from_ymd(2026, 6, 1).expect("date")),
        )
        .await
        .expect("deadline");

    let (_, inbox) = request(&h, "GET", "/inbox", None).await;
    assert!(inbox.contains("Overdue: 2026-06-01"), "{inbox}");
}
