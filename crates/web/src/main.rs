//! Momentum HTTP server binary.

use std::error::Error;
use std::sync::Arc;

use momentum_app::areas::AreaService;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::logbook::LogbookService;
use momentum_app::projects::ProjectService;
use momentum_app::recurring::RecurringService;
use momentum_app::sync::SyncService;
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
use momentum_infra::PgChangeLogReader;
use momentum_infra::{
    Argon2Hasher, Database, PgAreaReadStore, PgAuthStore, PgLoggingPolicyStore, PgProjectReadStore,
    PgRecurrenceReadStore, PgTagReadStore, PgTodoReadStore, PgUnitOfWorkFactory,
    RandomTokenGenerator, ServerConfig, SystemClock,
};
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::{AnonymousSessionResolver, CookieSessionResolver, SessionResolver};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info")),
        )
        .init();

    let config = ServerConfig::from_env()?;
    let database = connect_database(&config).await?;

    let (session_resolver, auth, todos, areas, projects, tags, logbook, recurring, sync): (
        Arc<dyn SessionResolver>,
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

            let sync = Arc::new(SyncService::new(
                dispatcher.clone(),
                todo_service.clone(),
                Arc::new(PgChangeLogReader::new(database.pool().clone())),
                clock.clone(),
            ));

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
            )
        }
        None => (
            Arc::new(AnonymousSessionResolver),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    };

    let state = AppState::new(
        server::leptos_options(),
        database,
        session_resolver,
        auth,
        todos,
        areas,
        projects,
    );
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
    let router = server::router_with_state(state);
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;

    tracing::info!(addr = %config.bind_addr, "momentum server listening");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn connect_database(config: &ServerConfig) -> Result<Option<Database>, Box<dyn Error>> {
    let Some(url) = config.database_url.as_deref() else {
        tracing::info!("DATABASE_URL is not set; starting without persistence");
        return Ok(None);
    };

    let database = Database::connect(url).await?;
    let applied = database.migrate().await?;
    if applied.is_empty() {
        tracing::info!("database schema is up to date");
    } else {
        tracing::info!(versions = ?applied, "applied database migrations");
    }
    Ok(Some(database))
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::warn!(%error, "failed to install Ctrl+C handler");
    }
}
