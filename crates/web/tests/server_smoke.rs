#![cfg(feature = "ssr")]

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn test_router() -> axum::Router {
    momentum_web::server::router(momentum_web::server::leptos_options())
}

async fn body_text(response: axum::response::Response) -> String {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("read response body")
        .to_bytes();
    String::from_utf8(bytes.to_vec()).expect("utf-8 body")
}

#[tokio::test]
async fn healthz_reports_ok() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("router response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(
        body.contains("\"status\":\"ok\""),
        "unexpected body: {body}"
    );
}

#[tokio::test]
async fn readyz_reports_ok() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("router response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn root_renders_server_side_product_identity() {
    let response = test_router()
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("router response");

    let status = response.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::NOT_FOUND,
        "unexpected status: {status}"
    );

    let body = body_text(response).await;
    assert!(
        body.contains(momentum_domain::PRODUCT_NAME),
        "SSR body must contain product name: {body}"
    );
    assert!(
        body.contains("data-testid=\"landing-page\""),
        "SSR body must contain the landing page: {body}"
    );
    assert!(
        body.trim_start().starts_with("<!DOCTYPE html>"),
        "SSR body must be a complete document: {body}"
    );
}

#[test]
fn repository_contains_no_handwritten_javascript_or_typescript() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("web crate lives in crates/web");
    let mut offenders = Vec::new();
    let mut stack = vec![root.join("crates"), root.join("scripts")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let extension = path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default();
            if matches!(
                extension,
                "js" | "mjs" | "ts" | "tsx" | "jsx" | "vue" | "svelte"
            ) {
                offenders.push(path);
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "handwritten JS/TS application sources are forbidden: {offenders:?}"
    );
}
