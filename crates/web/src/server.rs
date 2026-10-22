//! Axum server wiring for the Leptos SSR application.

use std::sync::Arc;

use axum::extract::{FromRef, Request, State};
use axum::http::header;
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderValue;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use leptos::config::LeptosOptions;
use leptos::context::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use momentum_app::api_tokens::ApiTokenService;
use momentum_app::areas::AreaService;
use momentum_app::calendar::CalendarService;
use momentum_app::inbound::InboundService;
use momentum_app::logbook::LogbookService;
use momentum_app::metrics::Metrics;
use momentum_app::preferences::PreferenceStore;
use momentum_app::projects::ProjectService;
use momentum_app::realtime::HintHub;
use momentum_app::recurring::RecurringService;
use momentum_app::reminders::PushSubscriptionStore;
use momentum_app::replica::OfflineCoordinator;
use momentum_app::search::RecentsStore;
use momentum_app::sync::SyncService;
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
use momentum_infra::Database;
use serde::Serialize;

use crate::api_routes;
use crate::app::{shell, App};
use crate::area_routes;
use crate::auth_routes::{self, AuthContext};
use crate::capture_routes;
use crate::command_routes;
use crate::export_import_routes;
use crate::inbound_routes;
use crate::logbook_routes;
use crate::notes_routes;
use crate::observability;
use crate::preferences_routes;
use crate::project_routes;
use crate::push_routes;
use crate::search_routes;
use crate::security::{headers as security_headers, origin_guard, LoginThrottle};
use crate::session::{AnonymousSessionResolver, SessionResolver};
use crate::share_routes;
use crate::sync_routes;
use crate::tag_routes;
use crate::todo_routes;

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
    /// To-do service, when persistence is configured.
    pub todos: Option<Arc<TodoService>>,
    /// Area service, when persistence is configured.
    pub areas: Option<Arc<AreaService>>,
    /// Project service, when persistence is configured.
    pub projects: Option<Arc<ProjectService>>,
    /// Tag service, when persistence is configured.
    pub tags: Option<Arc<TagService>>,
    /// Logbook service, when persistence is configured.
    pub logbook: Option<Arc<LogbookService>>,
    /// Recurrence reads for the Repeating special view.
    pub recurring: Option<Arc<RecurringService>>,
    /// Browser push subscriptions for reminder delivery.
    pub push_subscriptions: Option<Arc<dyn PushSubscriptionStore>>,
    /// Recent destinations for Quick Find ranking.
    pub recents: Option<Arc<dyn RecentsStore>>,
    /// Calendar projection service, when a provider is configured.
    pub calendar: Option<Arc<CalendarService>>,
    /// Offline replica coordinator, when a local store is configured.
    pub replica: Option<Arc<OfflineCoordinator>>,
    /// Sync protocol service, when persistence is configured.
    pub sync: Option<Arc<SyncService>>,
    /// Realtime invalidation hint hub, when realtime sync is enabled.
    pub hints: Option<Arc<HintHub>>,
    /// Inbound mail service, when configured.
    pub inbound: Option<Arc<InboundService>>,
    /// Shared secret expected from the mail provider webhook.
    pub inbound_secret: Option<String>,
    /// Automation API token service, when configured.
    pub api_tokens: Option<Arc<ApiTokenService>>,
    /// Preference store for account-synced settings.
    pub preferences: Option<Arc<dyn PreferenceStore>>,
    /// Per-account login attempt throttle.
    pub login_throttle: Arc<LoginThrottle>,
    /// Privacy-safe operational metrics.
    pub metrics: Arc<Metrics>,
}

impl AppState {
    /// Creates state with explicit parts.
    #[must_use]
    pub fn new(
        leptos_options: LeptosOptions,
        database: Option<Database>,
        session_resolver: Arc<dyn SessionResolver>,
        auth: Option<Arc<AuthContext>>,
        todos: Option<Arc<TodoService>>,
        areas: Option<Arc<AreaService>>,
        projects: Option<Arc<ProjectService>>,
    ) -> Self {
        Self {
            leptos_options,
            database,
            session_resolver,
            auth,
            todos,
            areas,
            projects,
            tags: None,
            logbook: None,
            recurring: None,
            push_subscriptions: None,
            recents: None,
            calendar: None,
            replica: None,
            sync: None,
            hints: None,
            inbound: None,
            inbound_secret: None,
            api_tokens: None,
            preferences: None,
            login_throttle: Arc::new(LoginThrottle::default()),
            metrics: Arc::new(Metrics::new()),
        }
    }

    /// Replaces the metrics registry.
    #[must_use]
    pub fn with_metrics(mut self, metrics: Arc<Metrics>) -> Self {
        self.metrics = metrics;
        self
    }

    /// Replaces the login throttle (tests use small windows).
    #[must_use]
    pub fn with_login_throttle(mut self, throttle: Arc<LoginThrottle>) -> Self {
        self.login_throttle = throttle;
        self
    }

    /// Adds the account preference store.
    #[must_use]
    pub fn with_preferences(mut self, preferences: Arc<dyn PreferenceStore>) -> Self {
        self.preferences = Some(preferences);
        self
    }

    /// Adds the automation API token service.
    #[must_use]
    pub fn with_api_tokens(mut self, api_tokens: Arc<ApiTokenService>) -> Self {
        self.api_tokens = Some(api_tokens);
        self
    }

    /// Adds the inbound mail service and its webhook secret.
    #[must_use]
    pub fn with_inbound(mut self, inbound: Arc<InboundService>, secret: Option<String>) -> Self {
        self.inbound = Some(inbound);
        self.inbound_secret = secret;
        self
    }

    /// Adds the realtime hint hub.
    #[must_use]
    pub fn with_hints(mut self, hints: Arc<HintHub>) -> Self {
        self.hints = Some(hints);
        self
    }

    /// Adds the sync service.
    #[must_use]
    pub fn with_sync(mut self, sync: Arc<SyncService>) -> Self {
        self.sync = Some(sync);
        self
    }

    /// Adds the offline replica coordinator.
    #[must_use]
    pub fn with_replica(mut self, replica: Arc<OfflineCoordinator>) -> Self {
        self.replica = Some(replica);
        self
    }

    /// Adds the calendar projection service.
    #[must_use]
    pub fn with_calendar(mut self, calendar: Arc<CalendarService>) -> Self {
        self.calendar = Some(calendar);
        self
    }

    /// Adds the Quick Find recents store.
    #[must_use]
    pub fn with_recents(mut self, recents: Arc<dyn RecentsStore>) -> Self {
        self.recents = Some(recents);
        self
    }

    /// Adds the push subscription store.
    #[must_use]
    pub fn with_push_subscriptions(
        mut self,
        push_subscriptions: Arc<dyn PushSubscriptionStore>,
    ) -> Self {
        self.push_subscriptions = Some(push_subscriptions);
        self
    }

    /// Adds the tag service.
    #[must_use]
    pub fn with_tags(mut self, tags: Arc<TagService>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Adds the logbook service.
    #[must_use]
    pub fn with_logbook(mut self, logbook: Arc<LogbookService>) -> Self {
        self.logbook = Some(logbook);
        self
    }

    /// Adds the recurrence read service.
    #[must_use]
    pub fn with_recurring(mut self, recurring: Arc<RecurringService>) -> Self {
        self.recurring = Some(recurring);
        self
    }

    /// Creates anonymous state without persistence or authentication.
    #[must_use]
    pub fn anonymous(leptos_options: LeptosOptions) -> Self {
        Self::new(
            leptos_options,
            None,
            Arc::new(AnonymousSessionResolver),
            None,
            None,
            None,
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
    router_with_state(AppState::new(
        options, None, resolver, None, None, None, None,
    ))
}

/// PWA manifest: installable shell, share target, and app shortcuts over command URLs.
const MANIFEST: &str = r##"{"name":"Finalize","short_name":"Finalize","start_url":"/today","display":"standalone","theme_color":"#1d4ed8","background_color":"#ffffff","icons":[{"src":"/static/icon.svg","sizes":"any","type":"image/svg+xml","purpose":"any maskable"}],"share_target":{"action":"/capture","method":"GET","params":{"title":"title","text":"text","url":"url"}},"shortcuts":[{"name":"New to-do","url":"/add"},{"name":"Quick Find","url":"/search"},{"name":"Today","url":"/today"}]}"##;

async fn service_worker() -> impl IntoResponse {
    (
        [
            (
                CONTENT_TYPE,
                HeaderValue::from_static("application/javascript"),
            ),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("no-cache, no-store, must-revalidate"),
            ),
        ],
        crate::pwa::service_worker_script(crate::pwa::SHELL_VERSION),
    )
}

async fn icon() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"))],
        crate::pwa::ICON_SVG,
    )
}

async fn manifest() -> impl IntoResponse {
    (
        [(
            CONTENT_TYPE,
            HeaderValue::from_static("application/manifest+json"),
        )],
        MANIFEST,
    )
}

/// Builds the application router with explicit state.
pub fn router_with_state(state: AppState) -> Router {
    init_runtime();
    let paths = generate_route_list(App);
    let shell_options = state.leptos_options.clone();
    let todo_service = state.todos.clone();
    let area_service = state.areas.clone();
    let project_service = state.projects.clone();
    let tag_service = state.tags.clone();
    let logbook_service = state.logbook.clone();
    let recurring_service = state.recurring.clone();
    let recents = state.recents.clone();
    let calendar = state.calendar.clone();
    let replica = state.replica.clone();
    let inbound = state.inbound.clone();
    let api_tokens = state.api_tokens.clone();
    let preferences = state.preferences.clone();

    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/static/app.css", get(app_css))
        .route("/manifest.webmanifest", get(manifest))
        .route("/service-worker.js", get(service_worker))
        .route("/static/icon.svg", get(icon))
        .route("/static/design.css", get(design_css))
        .merge(auth_routes::routes())
        .merge(capture_routes::routes())
        .merge(command_routes::routes())
        .merge(api_routes::routes())
        .merge(export_import_routes::routes())
        .merge(share_routes::routes())
        .merge(preferences_routes::routes())
        .merge(observability::routes())
        .merge(inbound_routes::routes())
        .merge(todo_routes::routes())
        .merge(area_routes::routes())
        .merge(project_routes::routes())
        .merge(tag_routes::routes())
        .merge(notes_routes::routes())
        .merge(logbook_routes::routes())
        .merge(push_routes::routes())
        .merge(search_routes::routes())
        .merge(sync_routes::routes())
        .leptos_routes_with_context(
            &state,
            paths,
            move || {
                provide_context(todo_service.clone());
                provide_context(area_service.clone());
                provide_context(project_service.clone());
                provide_context(tag_service.clone());
                provide_context(logbook_service.clone());
                provide_context(recurring_service.clone());
                provide_context(recents.clone());
                provide_context(calendar.clone());
                provide_context(replica.clone());
                provide_context(inbound.clone());
                provide_context(api_tokens.clone());
                provide_context(preferences.clone());
            },
            move || shell(shell_options.clone()),
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            session_middleware,
        ))
        .layer(middleware::from_fn(origin_guard))
        .layer(middleware::from_fn(security_headers))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            observability::count_requests,
        ))
        .layer(middleware::from_fn(observability::request_id))
        .with_state(state)
}

/// Resolves the session for every request and stores it in request extensions.
async fn session_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let session = state.session_resolver.resolve(request.headers()).await;
    // Local mode already supplies a workspace; entry and account URLs should open it.
    if state.auth.is_none()
        && session.account().is_some()
        && matches!(request.uri().path(), "/" | "/login" | "/signup")
    {
        return axum::response::Redirect::to("/today").into_response();
    }
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
