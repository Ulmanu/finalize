//! Axum server wiring for the Leptos SSR application.

use std::sync::Arc;

use axum::extract::{FromRef, Request, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderValue;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use leptos::config::LeptosOptions;
use leptos_axum::{generate_route_list, LeptosRoutes};
use momentum_infra::Database;
use serde::Serialize;

use crate::app::{shell, App};
use crate::auth_routes::{self, AuthContext};
use crate::session::{AnonymousSessionResolver, SessionResolver};

/// Base stylesheet embedded in the server binary.
///
/// Keeping the foundation stylesheet in Rust-managed assets avoids a JavaScript build step;
/// the design-system change expands this file.
pub const APP_CSS: &str = include_str!("../assets/app.css");

/// Design-token and primitive stylesheet embedded in the server binary.
pub const DESIGN_CSS: &str = include_str!("../assets/design.css");

/// Shared application state: Leptos options, optional persistence, session resolution, and
/// optional authentication.
#[derive(Clone)]
pub struct AppState {
    /// Leptos SSR configuration.
    pub leptos_options: LeptosOptions,
    /// Database pool, when persistence is configured.
    pub database: Option<Database>,
    /// Session resolver used by the session middleware.
    pub session_resolver: Arc<dyn SessionResolver>,
    /// Authentication context, when the server has an account store.
    pub auth: Option<Arc<AuthContext>>,
}

impl AppState {
    /// Creates state with explicit parts.
    #[must_use]
    pub fn new(
        leptos_options: LeptosOptions,
        database: Option<Database>,
        session_resolver: Arc<dyn SessionResolver>,
        auth: Option<Arc<AuthContext>>,
    ) -> Self {
        Self {
            leptos_options,
            database,
            session_resolver,
            auth,
        }
    }

    /// Creates anonymous state without persistence or authentication.
    #[must_use]
    pub fn anonymous(leptos_options: LeptosOptions) -> Self {
        Self::new(
            leptos_options,
            None,
            Arc::new(AnonymousSessionResolver),
            None,
        )
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
    authentication: &'static str,
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

/// Builds the application router with anonymous sessions.
pub fn router(options: LeptosOptions) -> Router {
    router_with_state(AppState::anonymous(options))
}

/// Builds the application router with an explicit session resolver.
///
/// Exposed so tests can inject deterministic resolvers without changing route code.
pub fn router_with_session(options: LeptosOptions, resolver: Arc<dyn SessionResolver>) -> Router {
    router_with_state(AppState::new(options, None, resolver, None))
}

/// Builds the application router with explicit state.
pub fn router_with_state(state: AppState) -> Router {
    init_runtime();
    let paths = generate_route_list(App);
    let shell_options = state.leptos_options.clone();

    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/static/app.css", get(app_css))
        .route("/static/design.css", get(design_css))
        .merge(auth_routes::routes())
        .leptos_routes_with_context(&state, paths, provide_context_placeholder, move || {
            shell(shell_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            session_middleware,
        ))
        .with_state(state)
}

fn provide_context_placeholder() {}

/// Resolves the session for every request and stores it in request extensions.
async fn session_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let session = state.session_resolver.resolve(request.headers()).await;
    request.extensions_mut().insert(session);
    next.run(request).await
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: momentum_app::health().as_str(),
    })
}

async fn readyz(State(state): State<AppState>) -> Json<ReadyResponse> {
    let authentication = if state.auth.is_some() {
        "ok"
    } else {
        "disabled"
    };

    let Some(database) = state.database else {
        return Json(ReadyResponse {
            status: "ok",
            persistence: "disabled",
            authentication,
        });
    };

    match database.health().await {
        Ok(()) => Json(ReadyResponse {
            status: "ok",
            persistence: "ok",
            authentication,
        }),
        Err(error) => {
            tracing::warn!(%error, "readiness database check failed");
            Json(ReadyResponse {
                status: "degraded",
                persistence: "unavailable",
                authentication,
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
