#![cfg(feature = "ssr")]

//! Schedule projection web-surface tests.

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
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("schedule@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("schedule@example.com", "correct horse battery")
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
async fn scheduling_moves_items_between_projection_pages() {
    let h = harness().await;
    let id = h
        .todos
        .create(h.account_id, "Move me", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let (_, anytime) = request(&h, "GET", "/anytime", None).await;
    assert!(
        anytime.contains("data-testid=\"anytime-group-list\""),
        "{anytime}"
    );
    assert!(anytime.contains("Move me"), "{anytime}");

    let body = format!("when=evening&return_to=%2Ftodos%2F{id}");
    let (status, _) = request(&h, "POST", &format!("/todos/{id}/schedule"), Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (_, today) = request(&h, "GET", "/today", None).await;
    assert!(today.contains("data-testid=\"today-page\""), "{today}");
    assert!(today.contains("Move me"), "{today}");
    let (_, anytime) = request(&h, "GET", "/anytime", None).await;
    assert!(
        anytime.contains("Move me"),
        "items actionable today remain in Anytime: {anytime}"
    );

    let body = format!("when=someday&return_to=%2Ftodos%2F{id}");
    let (status, _) = request(&h, "POST", &format!("/todos/{id}/schedule"), Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, someday) = request(&h, "GET", "/someday", None).await;
    assert!(someday.contains("Move me"), "{someday}");
    let (_, today) = request(&h, "GET", "/today", None).await;
    assert!(!today.contains("Move me"), "{today}");
    let (_, anytime) = request(&h, "GET", "/anytime", None).await;
    assert!(
        !anytime.contains("Move me"),
        "Someday is not actionable: {anytime}"
    );

    let body = format!("when=date&date=2026-06-05&return_to=%2Ftodos%2F{id}");
    let (status, _) = request(&h, "POST", &format!("/todos/{id}/schedule"), Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, upcoming) = request(&h, "GET", "/upcoming", None).await;
    assert!(
        upcoming.contains("data-testid=\"upcoming-day-list\""),
        "{upcoming}"
    );
    assert!(upcoming.contains("Move me"), "{upcoming}");
    let (_, anytime) = request(&h, "GET", "/anytime", None).await;
    assert!(
        !anytime.contains("Move me"),
        "future items are not actionable: {anytime}"
    );
}
