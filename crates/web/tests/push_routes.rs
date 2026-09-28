#![cfg(feature = "ssr")]

//! Push subscription route tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::reminders::{MemoryPushSubscriptionStore, PushSubscriptionStore};
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
    store: Option<Arc<MemoryPushSubscriptionStore>>,
    account_id: AccountId,
    cookie: String,
}

async fn harness(with_store: bool) -> Harness {
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
        .register("push@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("push@example.com", "correct horse battery")
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
    let push_store = with_store.then(|| Arc::new(MemoryPushSubscriptionStore::new()));
    if let Some(push_store) = &push_store {
        let store: Arc<dyn PushSubscriptionStore> = push_store.clone();
        state = state.with_push_subscriptions(store);
    }

    Harness {
        router: server::router_with_state(state),
        store: push_store,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(
    h: &Harness,
    method: &str,
    uri: &str,
    body: Option<&str>,
) -> (StatusCode, Option<String>) {
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
    let _ = response.into_body().collect().await.expect("body");
    (status, location)
}

#[tokio::test]
async fn push_subscriptions_register_and_refresh_by_endpoint() {
    let h = harness(true).await;
    let (status, location) = request(
        &h,
        "POST",
        "/push/subscriptions",
        Some(
            "endpoint=https%3A%2F%2Fpush.example%2Fa&p256dh=key&auth=secret&return_to=%2Fsettings",
        ),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings"));

    let store = h.store.as_ref().expect("store");
    let subscriptions = store.list(h.account_id).await.expect("list");
    assert_eq!(subscriptions.len(), 1);
    assert_eq!(subscriptions[0].endpoint, "https://push.example/a");

    let (_, _) = request(
        &h,
        "POST",
        "/push/subscriptions",
        Some("endpoint=https%3A%2F%2Fpush.example%2Fa&p256dh=key2&auth=secret2"),
    )
    .await;
    let subscriptions = store.list(h.account_id).await.expect("list");
    assert_eq!(subscriptions.len(), 1);
    assert_eq!(subscriptions[0].p256dh, "key2");

    let (_, _) = request(
        &h,
        "POST",
        "/push/subscriptions",
        Some("endpoint=https%3A%2F%2Fpush.example%2Fb&p256dh=key&auth=secret"),
    )
    .await;
    assert_eq!(store.list(h.account_id).await.expect("list").len(), 2);
}

#[tokio::test]
async fn push_subscriptions_without_store_report_error() {
    let h = harness(false).await;
    let (status, location) = request(
        &h,
        "POST",
        "/push/subscriptions",
        Some(
            "endpoint=https%3A%2F%2Fpush.example%2Fa&p256dh=key&auth=secret&return_to=%2Fsettings",
        ),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings?error=push"));
}
