#![cfg(feature = "ssr")]

//! PWA surface: manifest, generated service worker, icon, and shell registration.

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
use momentum_web::pwa::SHELL_VERSION;
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
async fn manifest_is_installable_with_shortcuts() {
    let router = router().await;
    let (status, content_type, body) = get(&router, "/manifest.webmanifest").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        content_type.contains("application/manifest+json"),
        "{content_type}"
    );
    assert!(body.contains("\"display\":\"standalone\""), "{body}");
    assert!(body.contains("\"theme_color\":\"#1d4ed8\""), "{body}");
    assert!(body.contains("\"background_color\":\"#ffffff\""), "{body}");
    assert!(body.contains("/static/icon.svg"), "{body}");
    assert!(body.contains("\"shortcuts\""), "{body}");
    assert!(body.contains("\"url\":\"/add\""), "{body}");
    assert!(body.contains("\"share_target\""), "{body}");
}

#[tokio::test]
async fn service_worker_is_versioned_and_uncached() {
    let router = router().await;
    let (status, content_type, body) = get(&router, "/service-worker.js").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        content_type.contains("application/javascript"),
        "{content_type}"
    );
    assert!(
        body.contains(&format!("momentum-shell-{SHELL_VERSION}")),
        "{body}"
    );
    assert!(body.contains("self.skipWaiting()"), "{body}");
    assert!(body.contains("self.clients.claim()"), "{body}");
    assert!(body.contains("/pkg/momentum-web_bg.wasm"), "{body}");
    assert!(
        !body.contains("/api/") || body.contains("startsWith(\"/api/\")"),
        "{body}"
    );
}

#[tokio::test]
async fn icon_is_served_as_svg() {
    let router = router().await;
    let (status, content_type, body) = get(&router, "/static/icon.svg").await;
    assert_eq!(status, StatusCode::OK);
    assert!(content_type.contains("image/svg+xml"), "{content_type}");
    assert!(body.contains("<svg"), "{body}");
}

#[tokio::test]
async fn shell_links_the_manifest_icon_and_registration() {
    let router = router().await;
    let (status, _, body) = get(&router, "/inbox").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("rel=\"manifest\" href=\"/manifest.webmanifest\""),
        "{body}"
    );
    assert!(
        body.contains("rel=\"icon\" href=\"/static/icon.svg\""),
        "{body}"
    );
    assert!(
        body.contains("navigator.serviceWorker.register(\"/service-worker.js\")"),
        "{body}"
    );
    assert!(body.contains("controllerchange"), "{body}");
}
