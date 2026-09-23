//! Axum server wiring for the Leptos SSR application.

use std::sync::Arc;

use axum::extract::{FromRef, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use leptos::config::LeptosOptions;
use leptos::context::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use momentum_infra::Database;
use serde::Serialize;

use crate::app::{shell, App};
use crate::session::{AnonymousSessionResolver, SessionResolver};

/// Base stylesheet embedded in the server binary.
///
/// Keeping the foundation stylesheet in Rust-managed assets avoids a JavaScript build step;
/// the design-system change expands this file.
pub const APP_CSS: &str = include_str!("../assets/app.css");

/// Design-token and primitive stylesheet embedded in the server binary.
pub const DESIGN_CSS: &str = include_str!("../assets/design.css");

/// Shared application state: Leptos options plus optional persistence.
#[derive(Clone)]
pub struct AppState {
    /// Leptos SSR configuration.
    pub leptos_options: LeptosOptions,
    /// Database pool, when persistence is configured.
    pub database: Option<Database>,
}

impl AppState {
    /// Creates state with the given options and no database.
    #[must_use]
    pub fn new(leptos_options: LeptosOptions, database: Option<Database>) -> Self {
        Self {
            leptos_options,
            database,
        }
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

/// JSON payload returned by liveness probes.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

/// JSON payload returned by readiness probes.
#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    status: &'static str,
    persistence: &'static str,
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

/// Builds the application router with the anonymous session resolver.
pub fn router(options: LeptosOptions) -> Router {
    router_with_session(options, Arc::new(AnonymousSessionResolver))
}

/// Builds the application router with an explicit session resolver.
///
/// Exposed so tests and the account-authentication change can inject the real resolver
/// without changing route or layout code.
pub fn router_with_session(options: LeptosOptions, resolver: Arc<dyn SessionResolver>) -> Router {
    router_with_state(AppState::new(options, None), resolver)
}

/// Builds the application router with explicit state and session resolver.
pub fn router_with_state(state: AppState, resolver: Arc<dyn SessionResolver>) -> Router {
    init_runtime();
    let paths = generate_route_list(App);
    let shell_options = state.leptos_options.clone();

    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/static/app.css", get(app_css))
        .route("/static/design.css", get(design_css))
        .leptos_routes_with_context(
            &state,
            paths,
            move || {
                provide_context(resolver.clone());
            },
            move || shell(shell_options.clone()),
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state)
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: momentum_app::health().as_str(),
    })
}

async fn readyz(State(state): State<AppState>) -> Json<ReadyResponse> {
    let Some(database) = state.database else {
        return Json(ReadyResponse {
            status: "ok",
            persistence: "disabled",
        });
    };

    match database.health().await {
        Ok(()) => Json(ReadyResponse {
            status: "ok",
            persistence: "ok",
        }),
        Err(error) => {
            tracing::warn!(%error, "readiness database check failed");
            Json(ReadyResponse {
                status: "degraded",
                persistence: "unavailable",
            })
        }
    }
}

async fn app_css() -> impl IntoResponse {
    (
        [(
            CONTENT_TYPE,
            HeaderValue::from_static("text/css; charset=utf-8"),
        )],
        APP_CSS,
    )
}

async fn design_css() -> impl IntoResponse {
    (
        [(
            CONTENT_TYPE,
            HeaderValue::from_static("text/css; charset=utf-8"),
        )],
        DESIGN_CSS,
    )
}
