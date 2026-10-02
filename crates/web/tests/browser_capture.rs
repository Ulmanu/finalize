#![cfg(feature = "ssr")]

//! Browser/PWA capture: share target, sanitization, and the share manifest.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::{TodoScope, TodoService};
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
        .register("share@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("share@example.com", "correct horse battery")
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

async fn request(
    h: &Harness,
    uri: &str,
    with_cookie: bool,
) -> (StatusCode, Option<String>, String) {
    let mut builder = Request::builder().method("GET").uri(uri);
    if with_cookie {
        builder = builder.header(header::COOKIE, &h.cookie);
    }
    let request = builder.body(Body::empty()).expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let _ = content_type;
    (
        status,
        location,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

#[tokio::test]
async fn share_target_capture_creates_an_inbox_item_with_sanitized_data() {
    let h = harness().await;
    let (status, location, _) = request(
        &h,
        "/capture?title=Example+Page&url=https%3A%2F%2Fexample.com%2Fp%3Fq%3D1&text=Hello+world",
        true,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox"));

    let created = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("captured");
    assert_eq!(created.title, "Example Page");
    assert_eq!(created.notes, "https://example.com/p?q=1\nHello world");
}

#[tokio::test]
async fn malicious_urls_are_dropped_and_untrusted_text_is_stripped() {
    let h = harness().await;
    let (status, _, _) = request(
        &h,
        "/capture?title=Alert&url=javascript%3Aalert(1)&text=Body%07+text",
        true,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let created = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("captured");
    assert_eq!(created.title, "Alert");
    assert!(!created.notes.contains("javascript"), "{}", created.notes);
    assert_eq!(created.notes, "Body text");
}

#[tokio::test]
async fn manifest_declares_the_capture_share_target() {
    let h = harness().await;
    let request = Request::builder()
        .method("GET")
        .uri("/manifest.webmanifest")
        .body(Body::empty())
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(
        content_type.contains("application/manifest+json"),
        "{content_type}"
    );
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body = String::from_utf8(bytes.to_vec()).expect("utf-8");
    assert!(body.contains("\"share_target\""), "{body}");
    assert!(body.contains("\"action\":\"/capture\""), "{body}");
}

#[tokio::test]
async fn capture_requires_authentication() {
    let h = harness().await;
    let (status, location, _) = request(&h, "/capture?title=Anonymous", false).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/login"));
    assert!(h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .is_empty());
}
