#![cfg(feature = "ssr")]

//! Deployment probes: liveness and readiness endpoints.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_web::auth_routes::AuthContext;
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
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        None,
        None,
        None,
    );
    server::router_with_state(state)
}

async fn get(router: &axum::Router, uri: &str) -> (StatusCode, String, String) {
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .expect("request");
    let response = router.clone().oneshot(request).await.expect("response");
    let status = response.status();
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
    (
        status,
        content_type,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

#[tokio::test]
async fn probes_report_liveness_and_readiness_without_persistence() {
    let router = router().await;
    let (status, content_type, body) = get(&router, "/healthz").await;
    assert_eq!(status, StatusCode::OK);
    assert!(content_type.contains("application/json"), "{content_type}");
    assert_eq!(body, "{\"status\":\"ok\"}");

    let (status, _, body) = get(&router, "/readyz").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("\"persistence\":\"disabled\""), "{body}");
    assert!(body.contains("\"authentication\":\"ok\""), "{body}");
}

#[tokio::test]
async fn probes_require_no_session() {
    let router = router().await;
    for path in ["/healthz", "/readyz"] {
        let (status, _, _) = get(&router, path).await;
        assert_eq!(status, StatusCode::OK, "{path}");
    }
}
