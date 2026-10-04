#![cfg(feature = "ssr")]

//! Observability: request correlation ids and privacy-safe metrics.

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
        .register("observability@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("observability@example.com", "correct horse battery")
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
    request_id: Option<&str>,
) -> (StatusCode, axum::http::HeaderMap, String) {
    let mut builder = Request::builder()
        .method("GET")
        .uri(uri)
        .header(header::COOKIE, &h.cookie);
    if let Some(request_id) = request_id {
        builder = builder.header("x-request-id", request_id);
    }
    let request = builder.body(Body::empty()).expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        headers,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

#[tokio::test]
async fn request_ids_are_assigned_or_propagated() {
    let h = harness().await;
    let (status, headers, _) = request(&h, "/inbox", None).await;
    assert_eq!(status, StatusCode::OK);
    let generated = headers
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(!generated.is_empty());

    let (status, headers, _) = request(&h, "/inbox", Some("trace-123")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        headers
            .get("x-request-id")
            .and_then(|value| value.to_str().ok()),
        Some("trace-123")
    );
}

#[tokio::test]
async fn metrics_exposition_is_privacy_safe() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "SECRET-TITLE", None)
        .await
        .expect("create");
    let (_, _, _) = request(&h, "/inbox", None).await;

    let (status, headers, body) = request(&h, "/metrics", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .contains("text/plain"));
    assert!(body.contains("momentum_http_requests_total"), "{body}");
    assert!(
        body.contains("# TYPE momentum_http_requests_total counter"),
        "{body}"
    );
    assert!(!body.contains("SECRET-TITLE"), "{body}");
    assert!(!body.contains("observability@example.com"), "{body}");
    assert!(!body.contains('@'), "{body}");
}
