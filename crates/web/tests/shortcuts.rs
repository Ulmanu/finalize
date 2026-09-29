#![cfg(feature = "ssr")]

//! Keyboard registry web-surface tests: settings reference and aria hints.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::TodoService;
use momentum_domain::AccountId;
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
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("keys@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("keys@example.com", "correct horse battery")
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

async fn request(h: &Harness, uri: &str) -> (StatusCode, String) {
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(header::COOKIE, &h.cookie)
        .body(Body::empty())
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
async fn settings_renders_the_central_shortcut_registry() {
    let h = harness().await;
    let (status, body) = request(&h, "/settings").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"shortcut-registry\""), "{body}");
    assert!(body.contains("data-shortcut-id=\"quick_find\""), "{body}");
    assert!(body.contains("data-keys=\"Cmd+K\""), "{body}");
    assert!(body.contains("data-category=\"navigation\""), "{body}");
    assert!(body.contains("data-testid=\"shortcut-conflict\""), "{body}");
    assert!(
        body.contains("Reserved by Firefox address bar; use /"),
        "{body}"
    );
    assert!(
        body.contains("data-shortcut-id=\"complete_todo\""),
        "{body}"
    );
    assert!(body.contains("data-keys=\"Enter\""), "{body}");
    assert!(body.contains("data-category=\"selection\""), "{body}");
    assert!(body.contains("data-category=\"ordering\""), "{body}");
}

#[tokio::test]
async fn row_controls_carry_registry_keyboard_hints() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Alpha", None)
        .await
        .expect("create");
    let (status, body) = request(&h, "/inbox").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("aria-keyshortcuts=\"Enter\""), "{body}");
    assert!(
        body.contains("aria-keyshortcuts=\"Shift+Delete\""),
        "{body}"
    );
}
