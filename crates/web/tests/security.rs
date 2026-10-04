#![cfg(feature = "ssr")]

//! Security hardening: headers, origin checks, and login throttling.

use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_web::auth_routes::AuthContext;
use momentum_web::security::LoginThrottle;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

async fn router() -> axum::Router {
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
    auth.register("secure@example.com", "correct horse battery", None)
        .await
        .expect("register");
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        None,
        None,
        None,
    )
    .with_login_throttle(Arc::new(LoginThrottle::new(Duration::from_secs(60), 2)));
    server::router_with_state(state)
}

async fn request(
    router: &axum::Router,
    method: &str,
    uri: &str,
    origin: Option<&str>,
    body: Option<&str>,
) -> (StatusCode, Option<String>, axum::http::HeaderMap, String) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::HOST, "localhost:3000");
    if let Some(origin) = origin {
        builder = builder.header(header::ORIGIN, origin);
    }
    if body.is_some() {
        builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    }
    let request = builder
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
        .expect("request");
    let response = router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let headers = response.headers().clone();
    let location = headers
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
        headers,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

#[tokio::test]
async fn every_response_carries_security_headers() {
    let router = router().await;
    let (status, _, headers, _) = request(&router, "GET", "/login", None, None).await;
    assert_eq!(status, StatusCode::OK);
    let csp = headers
        .get(header::CONTENT_SECURITY_POLICY)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    assert!(csp.contains("frame-ancestors 'none'"), "{csp}");
    assert!(csp.contains("object-src 'none'"), "{csp}");
    assert_eq!(
        headers
            .get(header::X_CONTENT_TYPE_OPTIONS)
            .and_then(|value| value.to_str().ok()),
        Some("nosniff")
    );
    assert_eq!(
        headers
            .get(header::X_FRAME_OPTIONS)
            .and_then(|value| value.to_str().ok()),
        Some("DENY")
    );
    assert!(headers.get(header::REFERRER_POLICY).is_some());
    assert!(headers.get("permissions-policy").is_some());
}

#[tokio::test]
async fn foreign_origins_cannot_submit_mutations() {
    let router = router().await;
    let (status, _, _, _) = request(
        &router,
        "POST",
        "/auth/login",
        Some("https://evil.example"),
        Some("email=secure%40example.com&password=correct+horse+battery"),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // Matching origin (and same-origin posts without an Origin header) proceed normally.
    let (status, location, _, _) = request(
        &router,
        "POST",
        "/auth/login",
        Some("http://localhost:3000"),
        Some("email=secure%40example.com&password=wrong"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/login?error=invalid"));

    let (status, _, _, _) = request(
        &router,
        "POST",
        "/auth/login",
        None,
        Some("email=secure%40example.com&password=wrong"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn repeated_failed_logins_are_throttled() {
    let router = router().await;
    let body = "email=secure%40example.com&password=wrong";
    for _ in 0..2 {
        let (status, location, _, _) =
            request(&router, "POST", "/auth/login", None, Some(body)).await;
        assert_eq!(status, StatusCode::SEE_OTHER);
        assert_eq!(location.as_deref(), Some("/login?error=invalid"));
    }
    let (status, location, _, _) = request(&router, "POST", "/auth/login", None, Some(body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/login?error=throttled"));

    // Even a correct password stays throttled until the window clears (clear-on-success only
    // applies after an allowed attempt).
    let (status, location, _, _) = request(
        &router,
        "POST",
        "/auth/login",
        None,
        Some("email=secure%40example.com&password=correct+horse+battery"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/login?error=throttled"));
}
