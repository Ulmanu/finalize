//! Axum server wiring for the Leptos SSR application.

use axum::routing::get;
use axum::{Json, Router};
use leptos::config::LeptosOptions;
use serde::Serialize;

use crate::app::shell;

/// JSON payload returned by liveness and readiness probes.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

/// Builds the Leptos options used by the server.
///
/// The defaults match the paths used by `scripts/build-web.sh`; they can be overridden with
/// the standard Leptos environment variables in later deployment changes.
#[must_use]
pub fn leptos_options() -> LeptosOptions {
    LeptosOptions::builder()
        .output_name("momentum-web")
        .site_root("target/site")
        .site_pkg_dir("pkg")
        .build()
}

/// Initializes the global async executor used by Leptos SSR tasks.
///
/// Safe to call repeatedly; later calls are ignored by the executor registry.
pub fn init_runtime() {
    let _ = any_spawner::Executor::init_tokio();
}

/// Builds the application router without binding a socket.
///
/// Exposed so tests and future deployment changes can mount the same routes in-process.
pub fn router(options: LeptosOptions) -> Router {
    init_runtime();
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .fallback(leptos_axum::file_and_error_handler(move |options| {
            shell(options)
        }))
        .with_state(options)
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: momentum_app::health().as_str(),
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: momentum_app::health().as_str(),
    })
}
