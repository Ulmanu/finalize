//! PostgreSQL recurrence template read store.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::recurring::{RecurrenceReadStore, RecurrenceTemplateSummary};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use sqlx::{PgPool, Row};

/// PostgreSQL implementation of [`RecurrenceReadStore`].
#[derive(Debug, Clone)]
pub struct PgRecurrenceReadStore {
    pool: PgPool,
}

impl PgRecurrenceReadStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecurrenceReadStore for PgRecurrenceReadStore {
    async fn list_templates(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<RecurrenceTemplateSummary>, CommandError> {
        let rows = sqlx::query(
            "select rt.entity_id, e.title, rt.next_occurrence \
             from recurrence_templates rt \
             join entities e on e.id = rt.entity_id \
             where rt.account_id = $1 \
             order by rt.next_occurrence asc nulls last, e.title asc, rt.entity_id asc",
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok(rows
            .into_iter()
            .map(|row| RecurrenceTemplateSummary {
                id: EntityId::from_uuid(row.get("entity_id")),
                title: row.get("title"),
                next_occurrence: row
                    .get::<Option<chrono::NaiveDate>, _>("next_occurrence")
                    .map(LocalDate::from_naive),
            })
            .collect())
    }
}
