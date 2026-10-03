#![cfg(feature = "ssr")]

//! Theme and text sizing: preference registry entries and accessibility-aware CSS.

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
        .register("theme@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("theme@example.com", "correct horse battery")
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

async fn get(h: &Harness, uri: &str) -> (StatusCode, String) {
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
async fn appearance_preferences_cover_theme_text_size_and_motion() {
    let h = harness().await;
    let (status, body) = get(&h, "/settings").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("data-preference-key=\"appearance.theme\" data-scope=\"browser\""),
        "{body}"
    );
    assert!(
        body.contains("data-preference-key=\"appearance.text_size\" data-scope=\"browser\""),
        "{body}"
    );
    assert!(
        body.contains("data-preference-key=\"appearance.reduced_motion\" data-scope=\"browser\""),
        "{body}"
    );
    assert!(body.contains("extra_large"), "{body}");

    // Text size is a browser-local preference: acknowledged without server storage.
    let request = Request::builder()
        .method("POST")
        .uri("/settings/preferences")
        .header(header::COOKIE, &h.cookie)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(
            "key=appearance.text_size&value=extra_large".to_string(),
        ))
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let stored = h.store.load_all(h.account_id).await.expect("load");
    assert!(stored.iter().all(|(key, _)| key != "appearance.text_size"));
}

#[tokio::test]
async fn stylesheets_carry_theme_text_size_contrast_and_motion_rules() {
    let h = harness().await;
    let (status, design) = get(&h, "/static/design.css").await;
    assert_eq!(status, StatusCode::OK);
    for needle in [
        "[data-theme=\"dark\"]",
        "[data-theme=\"light\"]",
        "html[data-text-size=\"large\"]",
        "html[data-text-size=\"extra_large\"]",
        "prefers-contrast: more",
        "[data-reduced-motion=\"true\"]",
        "prefers-color-scheme: dark",
    ] {
        assert!(design.contains(needle), "missing {needle}");
    }

    let (status, app) = get(&h, "/static/app.css").await;
    assert_eq!(status, StatusCode::OK);
    assert!(app.contains("prefers-reduced-motion: reduce"), "{app}");
}
