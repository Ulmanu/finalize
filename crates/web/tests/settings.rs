#![cfg(feature = "ssr")]

//! Settings: typed preference registry, declared scopes, and validated persistence.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::preferences::PreferenceStore;
use momentum_domain::AccountId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::preferences::MemoryPreferenceStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    store: Arc<MemoryPreferenceStore>,
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
        clock,
    ));

    let registration = auth
        .register("settings@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("settings@example.com", "correct horse battery")
        .await
        .expect("login");

    let preferences = Arc::new(MemoryPreferenceStore::new());
    let store_handle: Arc<dyn PreferenceStore> = preferences.clone();
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        None,
        None,
        None,
    )
    .with_preferences(store_handle);

    Harness {
        router: server::router_with_state(state),
        store: preferences,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(
    h: &Harness,
    method: &str,
    uri: &str,
    body: Option<&str>,
) -> (StatusCode, Option<String>, String) {
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

#[tokio::test]
async fn settings_lists_typed_preferences_with_declared_scopes() {
    let h = harness().await;
    let (status, _, body) = request(&h, "GET", "/settings", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"preferences\""), "{body}");
    assert!(
        body.contains("data-preference-key=\"general.week_start\" data-scope=\"account\""),
        "{body}"
    );
    assert!(
        body.contains("data-preference-key=\"appearance.theme\" data-scope=\"browser\""),
        "{body}"
    );
    assert!(
        body.contains("data-preference-key=\"account.sync_enabled\" data-scope=\"device\""),
        "{body}"
    );
    for domain in [
        "general",
        "appearance",
        "today",
        "calendar",
        "notifications",
        "quick_entry",
        "logbook",
        "account_sync",
        "data",
    ] {
        assert!(
            body.contains(&format!("data-domain=\"{domain}\"")),
            "{body}"
        );
    }
}

#[tokio::test]
async fn account_preferences_persist_and_invalid_values_are_rejected() {
    let h = harness().await;

    let (status, location, _) = request(
        &h,
        "POST",
        "/settings/preferences",
        Some("key=general.week_start&value=sunday"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings"));
    let stored = h.store.load_all(h.account_id).await.expect("load");
    assert!(stored.contains(&("general.week_start".to_string(), "sunday".to_string())));

    let (_, _, body) = request(&h, "GET", "/settings", None).await;
    assert!(body.contains("sunday"), "{body}");

    let (status, location, _) = request(
        &h,
        "POST",
        "/settings/preferences",
        Some("key=general.week_start&value=neon"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings?error=preference"));
    assert!(h
        .store
        .load_all(h.account_id)
        .await
        .expect("load")
        .iter()
        .all(|(key, value)| key != "general.week_start" || value == "sunday"));

    let (status, location, _) = request(
        &h,
        "POST",
        "/settings/preferences",
        Some("key=unknown.key&value=true"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings?error=preference"));
}

#[tokio::test]
async fn browser_local_preferences_are_acknowledged_without_server_storage() {
    let h = harness().await;
    let (status, location, _) = request(
        &h,
        "POST",
        "/settings/preferences",
        Some("key=appearance.theme&value=dark"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings"));
    let stored = h.store.load_all(h.account_id).await.expect("load");
    assert!(stored.iter().all(|(key, _)| key != "appearance.theme"));
}
