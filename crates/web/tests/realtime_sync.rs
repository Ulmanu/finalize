#![cfg(feature = "ssr")]

//! Realtime invalidation stream: SSE headers, initial hint, and availability.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::realtime::HintHub;
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
    account_id: AccountId,
    cookie: String,
}

async fn harness(with_hints: bool) -> Harness {
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
        .register("realtime@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("realtime@example.com", "correct horse battery")
        .await
        .expect("login");

    let mut state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos),
        None,
        None,
    );
    if with_hints {
        state = state.with_hints(Arc::new(HintHub::new()));
    }

    Harness {
        router: server::router_with_state(state),
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

fn request(h: &Harness, uri: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .header(header::COOKIE, &h.cookie)
        .body(Body::empty())
        .expect("request")
}

#[tokio::test]
async fn invalidation_stream_sends_initial_hint() {
    let h = harness(true).await;
    let response = h
        .router
        .clone()
        .oneshot(request(&h, "/sync/events"))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(content_type.contains("text/event-stream"), "{content_type}");

    let mut body = response.into_body();
    let mut text = String::new();
    for _ in 0..5 {
        let Some(frame) = body.frame().await else {
            break;
        };
        let frame = frame.expect("frame");
        if let Ok(data) = frame.into_data() {
            text.push_str(&String::from_utf8_lossy(&data));
        }
        if text.contains("connect") {
            break;
        }
    }
    assert!(text.contains("event: invalidation"), "{text}");
    assert!(text.contains("connect"), "{text}");
    let _ = h.account_id;
}

#[tokio::test]
async fn invalidation_stream_is_unavailable_without_the_hub() {
    let h = harness(false).await;
    let response = h
        .router
        .clone()
        .oneshot(request(&h, "/sync/events"))
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}
