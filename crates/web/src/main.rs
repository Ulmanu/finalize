//! Momentum HTTP server binary.

use std::error::Error;
use std::sync::Arc;

use momentum_infra::{Database, ServerConfig};
use momentum_web::server::{self, AppState};
use momentum_web::session::AnonymousSessionResolver;
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

    let state = AppState::new(server::leptos_options(), database);
    let router = server::router_with_state(state, Arc::new(AnonymousSessionResolver));
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
