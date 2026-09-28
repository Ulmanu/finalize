//! PostgreSQL reminder source for the delivery scheduler.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_app::commands::CommandError;
use momentum_app::reminders::{DueReminder, ReminderSource};
use momentum_domain::model::Reminder;
use momentum_domain::{AccountId, EntityId};
use serde_json::Value;
use sqlx::{PgPool, Row};

/// Reads due reminders from persisted to-do rows.
#[derive(Debug, Clone)]
pub struct PgReminderSource {
    pool: PgPool,
}

impl PgReminderSource {
    /// Creates the source over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReminderSource for PgReminderSource {
    async fn due_reminders(
        &self,
        account_id: AccountId,
        now: DateTime<Utc>,
    ) -> Result<Vec<DueReminder>, CommandError> {
        let rows = sqlx::query(
            "select id, title, reminder_rule from entities \
             where account_id = $1 and kind = 'todo' and reminder_rule is not null",
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        let mut due = Vec::new();
        for row in rows {
            let Some(reminder) = row
                .get::<Option<Value>, _>("reminder_rule")
                .and_then(|value| serde_json::from_value::<Reminder>(value).ok())
            else {
                continue;
            };
            if !reminder.is_due(now) {
                continue;
            }
            let id: uuid::Uuid = row.get("id");
            let todo_id = EntityId::from_uuid(id);
            due.push(DueReminder {
                todo_id,
                account_id,
                title: row.get("title"),
                at: reminder.at,
                timezone: reminder.timezone,
                deep_link: format!("/todos/{todo_id}"),
            });
        }
        Ok(due)
    }
}
