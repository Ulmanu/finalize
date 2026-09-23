//! Momentum HTTP server binary.

use std::error::Error;
use std::sync::Arc;

use momentum_app::auth::AuthService;
use momentum_infra::{
    Argon2Hasher, Database, PgAuthStore, RandomTokenGenerator, ServerConfig, SystemClock,
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

    let (session_resolver, auth): (Arc<dyn SessionResolver>, _) = match &database {
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
            (Arc::new(CookieSessionResolver::new(service)), Some(context))
        }
        None => (Arc::new(AnonymousSessionResolver), None),
    };

    let state = AppState::new(server::leptos_options(), database, session_resolver, auth);
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
