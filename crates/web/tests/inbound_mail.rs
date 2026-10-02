#![cfg(feature = "ssr")]

//! Inbound mail webhook and alias settings surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::inbound::{InboundLimits, InboundService};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::AccountId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::inbound::{DeterministicTokenSource, MemoryInboundStore};
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

const SECRET: &str = "webhook-secret";

struct Harness {
    router: axum::Router,
    service: Arc<InboundService>,
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
        .register("inbound@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("inbound@example.com", "correct horse battery")
        .await
        .expect("login");

    let service = Arc::new(InboundService::new(
        Arc::new(MemoryInboundStore::new()),
        Arc::new(MemoryInboundStore::new()),
        todos.clone(),
        Arc::new(DeterministicTokenSource::new()),
        InboundLimits::default(),
    ));
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        None,
    )
    .with_inbound(service.clone(), Some(SECRET.to_string()));

    Harness {
        router: server::router_with_state(state),
        service,
        todos,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn raw_request(
    h: &Harness,
    method: &str,
    uri: &str,
    content_type: Option<&str>,
    body: &str,
    secret: Option<&str>,
) -> (StatusCode, Option<String>, String) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, &h.cookie);
    if let Some(content_type) = content_type {
        builder = builder.header(header::CONTENT_TYPE, content_type);
    }
    if let Some(secret) = secret {
        builder = builder.header("x-inbound-secret", secret);
    }
    let request = builder.body(Body::from(body.to_string())).expect("request");
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

async fn inbox_count(h: &Harness) -> usize {
    h.todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .len()
}

#[tokio::test]
async fn webhook_delivers_duplicates_and_disabled_aliases() {
    let h = harness().await;
    let alias = h.service.ensure_alias(h.account_id).await.expect("alias");
    let payload = |message_id: &str| {
        format!(
            "{{\"to\":\"{}\",\"subject\":\"From mail\",\"text\":\"Body text\",\
             \"message_id\":\"{message_id}\"}}",
            alias.address()
        )
    };

    let (status, _, body) = raw_request(
        &h,
        "POST",
        "/inbound/mail",
        Some("application/json"),
        &payload("m1"),
        Some(SECRET),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("delivered"), "{body}");
    assert_eq!(inbox_count(&h).await, 1);

    let (status, _, body) = raw_request(
        &h,
        "POST",
        "/inbound/mail",
        Some("application/json"),
        &payload("m1"),
        Some(SECRET),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("duplicate"), "{body}");
    assert_eq!(inbox_count(&h).await, 1);

    h.service.disable(h.account_id).await.expect("disable");
    let (status, _, body) = raw_request(
        &h,
        "POST",
        "/inbound/mail",
        Some("application/json"),
        &payload("m2"),
        Some(SECRET),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert!(body.contains("disabled"), "{body}");
    assert_eq!(inbox_count(&h).await, 1);
}

#[tokio::test]
async fn webhook_requires_the_shared_secret() {
    let h = harness().await;
    let alias = h.service.ensure_alias(h.account_id).await.expect("alias");
    let payload = format!(
        "{{\"to\":\"{}\",\"subject\":\"No secret\",\"text\":\"\",\"message_id\":\"m1\"}}",
        alias.address()
    );
    let (status, _, _) = raw_request(
        &h,
        "POST",
        "/inbound/mail",
        Some("application/json"),
        &payload,
        Some("wrong"),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _, _) = raw_request(
        &h,
        "POST",
        "/inbound/mail",
        Some("application/json"),
        &payload,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(inbox_count(&h).await, 0);
}

#[tokio::test]
async fn settings_shows_and_rotates_the_private_address() {
    let h = harness().await;
    let (status, _, body) = raw_request(&h, "GET", "/settings", None, "", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"inbound-alias\""), "{body}");
    let first = h.service.ensure_alias(h.account_id).await.expect("alias");
    assert!(body.contains(&first.token), "{body}");

    let (status, location, _) =
        raw_request(&h, "POST", "/settings/inbound/rotate", None, "", None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings"));
    let rotated = h.service.ensure_alias(h.account_id).await.expect("alias");
    assert_ne!(rotated.token, first.token);

    let (status, location, _) =
        raw_request(&h, "POST", "/settings/inbound/disable", None, "", None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings"));
    let disabled = h.service.ensure_alias(h.account_id).await.expect("alias");
    assert!(!disabled.enabled);
}
