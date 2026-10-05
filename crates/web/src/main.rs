//! Finalize HTTP server binary.

use std::error::Error;
use std::sync::Arc;

use momentum_app::api_tokens::ApiTokenService;
use momentum_app::areas::AreaService;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::inbound::{InboundLimits, InboundService, RandomTokenSource};
use momentum_app::logbook::LogbookService;
use momentum_app::metrics::Metrics;
use momentum_app::projects::ProjectService;
use momentum_app::realtime::HintHub;
use momentum_app::recurring::RecurringService;
#[cfg(feature = "local")]
use momentum_app::session::{AccountSession, SessionState};
use momentum_app::sync::SyncService;
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
#[cfg(feature = "local")]
use momentum_domain::AccountId;
use momentum_infra::PgApiTokenStore;
use momentum_infra::PgChangeLogReader;
use momentum_infra::PgInboundStore;
use momentum_infra::PgPreferenceStore;
use momentum_infra::{
    Argon2Hasher, Database, PgAreaReadStore, PgAuthStore, PgLoggingPolicyStore, PgProjectReadStore,
    PgRecurrenceReadStore, PgTagReadStore, PgTodoReadStore, PgUnitOfWorkFactory,
    RandomTokenGenerator, ServerConfig, SystemClock,
};
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
#[cfg(not(feature = "local"))]
use momentum_web::session::AnonymousSessionResolver;
#[cfg(feature = "local")]
use momentum_web::session::FixedSessionResolver;
use momentum_web::session::{CookieSessionResolver, SessionResolver};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("migrate") {
        return migrate_command().await;
    }
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info")),
        )
        .init();

    let config = ServerConfig::from_env()?;
    let database = connect_database(&config).await?;
    let metrics = Arc::new(Metrics::new());

    let (session_resolver, auth, todos, areas, projects, tags, logbook, recurring, sync, hints): (
        Arc<dyn SessionResolver>,
        _,
        _,
        _,
        _,
        _,
        _,
        _,
        _,
        _,
    ) = match &database {
        Some(database) => {
            let store = Arc::new(PgAuthStore::new(database.pool().clone()));
            let (accounts, sessions, tokens) = store.ports();
            let service = Arc::new(AuthService::new(
                accounts,
                sessions,
                tokens,
                Arc::new(Argon2Hasher),
                Arc::new(RandomTokenGenerator),
                Arc::new(SystemClock),
            ));
            let context = Arc::new(AuthContext::new(service.clone(), config.secure_cookies));

            let clock = Arc::new(SystemClock);
            let dispatcher = Arc::new(CommandDispatcher::new(
                Arc::new(PgUnitOfWorkFactory::new(database.pool().clone())),
                clock.clone(),
            ));
            let reader = Arc::new(PgTodoReadStore::new(database.pool().clone()));
            let todo_service =
                Arc::new(TodoService::new(dispatcher.clone(), reader, clock.clone()));

            let area_reader = Arc::new(PgAreaReadStore::new(database.pool().clone()));
            let area_service = Arc::new(AreaService::new(
                dispatcher.clone(),
                area_reader,
                clock.clone(),
            ));

            let project_reader = Arc::new(PgProjectReadStore::new(database.pool().clone()));
            let project_service = Arc::new(ProjectService::new(
                dispatcher.clone(),
                project_reader,
                clock.clone(),
            ));

            let recurring = Arc::new(RecurringService::new(Arc::new(PgRecurrenceReadStore::new(
                database.pool().clone(),
            ))));

            let policy_store = Arc::new(PgLoggingPolicyStore::new(database.pool().clone()));
            let logbook_service = Arc::new(LogbookService::new(
                dispatcher.clone(),
                todo_service.clone(),
                project_service.clone(),
                policy_store,
                clock.clone(),
            ));

            let hints = Arc::new(HintHub::new());
            let sync = Arc::new(
                SyncService::new(
                    dispatcher.clone(),
                    todo_service.clone(),
                    Arc::new(PgChangeLogReader::new(database.pool().clone())),
                    clock.clone(),
                )
                .with_hints(hints.clone())
                .with_metrics(metrics.clone()),
            );

            let tag_reader = Arc::new(PgTagReadStore::new(database.pool().clone()));
            let tag_service = Arc::new(TagService::new(dispatcher, tag_reader, clock));

            (
                Arc::new(CookieSessionResolver::new(service)),
                Some(context),
                Some(todo_service),
                Some(area_service),
                Some(project_service),
                Some(tag_service),
                Some(logbook_service),
                Some(recurring),
                Some(sync),
                Some(hints),
            )
        }
        None => {
            #[cfg(feature = "local")]
            {
                use momentum_testkit::areas::MemoryAreaReadStore;
                use momentum_testkit::commands::MemoryDatabase;
                use momentum_testkit::logbook::MemoryLoggingPolicyStore;
                use momentum_testkit::projects::MemoryProjectReadStore;
                use momentum_testkit::recurring::MemoryRecurrenceReadStore;
                use momentum_testkit::sync::MemoryChangeLogReader;
                use momentum_testkit::tags::MemoryTagReadStore;
                use momentum_testkit::todos::MemoryTodoReadStore;

                tracing::warn!(
                    "DATABASE_URL is not set: running in local mode with in-memory storage; \
                     data is not persisted"
                );
                let database = MemoryDatabase::new();
                let clock = Arc::new(SystemClock);
                let dispatcher = Arc::new(CommandDispatcher::new(
                    Arc::new(database.clone()),
                    clock.clone(),
                ));
                let todo_service = Arc::new(TodoService::new(
                    dispatcher.clone(),
                    Arc::new(MemoryTodoReadStore::new(database.clone())),
                    clock.clone(),
                ));
                let area_service = Arc::new(AreaService::new(
                    dispatcher.clone(),
                    Arc::new(MemoryAreaReadStore::new(database.clone())),
                    clock.clone(),
                ));
                let project_service = Arc::new(ProjectService::new(
                    dispatcher.clone(),
                    Arc::new(MemoryProjectReadStore::new(database.clone())),
                    clock.clone(),
                ));
                let tag_service = Arc::new(TagService::new(
                    dispatcher.clone(),
                    Arc::new(MemoryTagReadStore::new(database.clone())),
                    clock.clone(),
                ));
                let recurring = Arc::new(RecurringService::new(Arc::new(
                    MemoryRecurrenceReadStore::new(),
                )));
                let logbook_service = Arc::new(LogbookService::new(
                    dispatcher.clone(),
                    todo_service.clone(),
                    project_service.clone(),
                    Arc::new(MemoryLoggingPolicyStore::new()),
                    clock.clone(),
                ));
                let hints = Arc::new(HintHub::new());
                let sync = Arc::new(
                    SyncService::new(
                        dispatcher,
                        todo_service.clone(),
                        Arc::new(MemoryChangeLogReader::new(database)),
                        clock,
                    )
                    .with_hints(hints.clone()),
                );
                let local_account = AccountId::from_uuid(
                    uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001")
                        .expect("local account uuid"),
                );
                let session: Arc<dyn SessionResolver> =
                    Arc::new(FixedSessionResolver(SessionState::Authenticated(
                        AccountSession::new(local_account, "local@finalize"),
                    )));
                (
                    session,
                    None,
                    Some(todo_service),
                    Some(area_service),
                    Some(project_service),
                    Some(tag_service),
                    Some(logbook_service),
                    Some(recurring),
                    Some(sync),
                    Some(hints),
                )
            }
            #[cfg(not(feature = "local"))]
            {
                (
                    Arc::new(AnonymousSessionResolver) as Arc<dyn SessionResolver>,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
            }
        }
    };

    let inbound_pool = database.as_ref().map(|database| database.pool().clone());
    let todos_for_inbound = todos.clone();
    let state = AppState::new(
        server::leptos_options(),
        database,
        session_resolver,
        auth,
        todos,
        areas,
        projects,
    )
    .with_metrics(metrics.clone());
    let state = match tags {
        Some(tags) => state.with_tags(tags),
        None => state,
    };
    let state = match logbook {
        Some(logbook) => state.with_logbook(logbook),
        None => state,
    };
    let state = match recurring {
        Some(recurring) => state.with_recurring(recurring),
        None => state,
    };
    let state = match sync {
        Some(sync) => state.with_sync(sync),
        None => state,
    };
    let state = match hints {
        Some(hints) => state.with_hints(hints),
        None => state,
    };
    let state = match &inbound_pool {
        Some(pool) => state.with_api_tokens(Arc::new(ApiTokenService::new(
            Arc::new(PgApiTokenStore::new(pool.clone())),
            Arc::new(RandomTokenGenerator),
        ))),
        None => state,
    };
    let state = match &inbound_pool {
        Some(pool) => state.with_preferences(Arc::new(PgPreferenceStore::new(pool.clone()))),
        None => state,
    };
    let state = match (inbound_pool, todos_for_inbound) {
        (Some(pool), Some(todos)) => {
            let service = Arc::new(
                InboundService::new(
                    Arc::new(PgInboundStore::new(pool.clone())),
                    Arc::new(PgInboundStore::new(pool)),
                    todos,
                    Arc::new(RandomTokenSource),
                    InboundLimits::default(),
                )
                .with_metrics(metrics.clone()),
            );
            let secret = std::env::var("MOMENTUM_INBOUND_SECRET").ok();
            state.with_inbound(service, secret)
        }
        _ => state,
    };
    let router = server::router_with_state(state);
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;

    tracing::info!(addr = %config.bind_addr, "momentum server listening");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Applies pending migrations and exits (the deployment migration step).
async fn migrate_command() -> Result<(), Box<dyn Error>> {
    let config = ServerConfig::from_env()?;
    let Some(url) = config.database_url.as_deref() else {
        return Err("DATABASE_URL is required for migrations".into());
    };
    let database = Database::connect(url).await?;
    let applied = database.migrate().await?;
    if applied.is_empty() {
        println!("database schema is up to date");
    } else {
        println!("applied migrations: {applied:?}");
    }
    Ok(())
}

async fn connect_database(config: &ServerConfig) -> Result<Option<Database>, Box<dyn Error>> {
    let Some(url) = config.database_url.as_deref() else {
        tracing::info!("DATABASE_URL is not set; starting without persistence");
        return Ok(None);
    };

    let database = Database::connect(url).await?;
    // Safe migrations: never serve with pending migrations; operators run
    // `momentum-server migrate` as a separate deployment step.
    let pending = momentum_infra::db::migrations::pending(database.pool()).await?;
    if !pending.is_empty() {
        return Err(format!(
            "pending database migrations {pending:?}; run `momentum-server migrate` first"
        )
        .into());
    }
    Ok(Some(database))
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::warn!(%error, "failed to install Ctrl+C handler");
    }
}
