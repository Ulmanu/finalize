//! Background worker: reminder delivery scheduling and recurrence materialization.
//!
//! The worker is a separate process (topology documented in DEPLOYMENT.md) sharing the same
//! PostgreSQL database as the web server. It refuses to start with pending migrations, iterates
//! every account, and shuts down gracefully on SIGINT/SIGTERM. Delivery logging is
//! privacy-safe: only entity ids and timestamps are recorded, never titles or notes.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use momentum_app::commands::CommandDispatcher;
use momentum_app::materializer::{RecurrenceMaterializer, HORIZON_DAYS};
use momentum_app::reminders::{DueReminder, ReminderNotifier, ReminderScheduler};
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
use momentum_domain::time::LocalDate;
use momentum_infra::db::Database;
use momentum_infra::{
    PgMaterializerStore, PgReminderSource, PgTagReadStore, PgTodoReadStore, PgUnitOfWorkFactory,
    ServerConfig, SystemClock,
};

/// Interval between worker passes.
const PASS_INTERVAL: Duration = Duration::from_secs(60);

/// Notifier that logs delivery (privacy-safe: ids and instants only).
#[derive(Debug)]
struct LoggingNotifier;

#[async_trait]
impl ReminderNotifier for LoggingNotifier {
    async fn notify(&self, reminder: &DueReminder) -> Result<(), String> {
        tracing::info!(
            todo = %reminder.todo_id,
            at = %reminder.at,
            "reminder due"
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = ServerConfig::from_env()?;
    let Some(url) = config.database_url.as_deref() else {
        return Err("DATABASE_URL is required for the worker".into());
    };
    let database = Database::connect(url).await?;
    let pending = momentum_infra::db::migrations::pending(database.pool()).await?;
    if !pending.is_empty() {
        return Err(format!(
            "pending database migrations {pending:?}; run `momentum-server migrate` first"
        )
        .into());
    }

    let pool = database.pool().clone();
    let clock = Arc::new(SystemClock);
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(PgUnitOfWorkFactory::new(pool.clone())),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher.clone(),
        Arc::new(PgTodoReadStore::new(pool.clone())),
        clock.clone(),
    ));
    let tags = Arc::new(TagService::new(
        dispatcher,
        Arc::new(PgTagReadStore::new(pool.clone())),
        clock,
    ));
    let scheduler = Arc::new(ReminderScheduler::new(
        todos.clone(),
        Arc::new(PgReminderSource::new(pool.clone())),
        Arc::new(LoggingNotifier),
    ));
    let materializer = Arc::new(RecurrenceMaterializer::new(
        todos.clone(),
        Some(tags),
        Arc::new(PgMaterializerStore::new(pool)),
    ));

    tracing::info!(horizon_days = HORIZON_DAYS, "worker started");
    let mut shutdown = Box::pin(async {
        let _ = tokio::signal::ctrl_c().await;
    });
    loop {
        let now = Utc::now();
        let today = LocalDate::today_in_timezone(now, "UTC");
        match database.account_ids().await {
            Ok(accounts) => {
                for account in accounts {
                    match scheduler.run_once(account, now).await {
                        Ok(run) => {
                            if !run.delivered.is_empty() || !run.failed.is_empty() {
                                tracing::info!(
                                    delivered = run.delivered.len(),
                                    failed = run.failed.len(),
                                    "reminder pass complete"
                                );
                            }
                        }
                        Err(error) => {
                            tracing::warn!(error = %error, "reminder pass failed");
                        }
                    }
                    if let Err(error) = materializer.run(account, today).await {
                        tracing::warn!(error = %error, "recurrence pass failed");
                    }
                }
            }
            Err(error) => tracing::warn!(error = %error, "account enumeration failed"),
        }

        tokio::select! {
            () = &mut shutdown => {
                tracing::info!("worker shutting down");
                break;
            }
            () = tokio::time::sleep(PASS_INTERVAL) => {}
        }
    }
    Ok(())
}
