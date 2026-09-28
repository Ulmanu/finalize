//! PostgreSQL recurrence template store for materialization.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::materializer::{MaterializerStore, PersistedTemplate};
use momentum_domain::recurrence::{Recurrence, RecurrenceTemplate};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use sqlx::{PgPool, Row};

/// PostgreSQL implementation of [`MaterializerStore`].
#[derive(Debug, Clone)]
pub struct PgMaterializerStore {
    pool: PgPool,
}

impl PgMaterializerStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MaterializerStore for PgMaterializerStore {
    async fn templates_due(
        &self,
        account_id: AccountId,
        horizon: LocalDate,
    ) -> Result<Vec<PersistedTemplate>, CommandError> {
        let rows = sqlx::query(
            "select entity_id, rule, template, next_occurrence, horizon_date \
             from recurrence_templates \
             where account_id = $1 \
               and ((next_occurrence is not null and next_occurrence <= $2) \
                    or (next_occurrence is null and horizon_date is null)) \
             order by next_occurrence asc nulls first, entity_id asc",
        )
        .bind(account_id.as_uuid())
        .bind(horizon.as_naive())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        let mut templates = Vec::with_capacity(rows.len());
        for row in rows {
            let rule = serde_json::from_value::<Recurrence>(row.get("rule"))
                .map_err(|error| CommandError::Storage(error.to_string()))?;
            let template = serde_json::from_value::<RecurrenceTemplate>(row.get("template"))
                .map_err(|error| CommandError::Storage(error.to_string()))?;
            templates.push(PersistedTemplate {
                id: EntityId::from_uuid(row.get("entity_id")),
                rule,
                template,
                next_occurrence: row
                    .get::<Option<chrono::NaiveDate>, _>("next_occurrence")
                    .map(LocalDate::from_naive),
                horizon: row
                    .get::<Option<chrono::NaiveDate>, _>("horizon_date")
                    .map(LocalDate::from_naive),
            });
        }
        Ok(templates)
    }

    async fn advance(
        &self,
        account_id: AccountId,
        template_id: EntityId,
        next_occurrence: Option<LocalDate>,
        horizon: LocalDate,
    ) -> Result<(), CommandError> {
        sqlx::query(
            "update recurrence_templates set next_occurrence = $3, horizon_date = $4, \
             updated_at = now() where account_id = $1 and entity_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(template_id.as_uuid())
        .bind(next_occurrence.map(|date| date.as_naive()))
        .bind(horizon.as_naive())
        .execute(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok(())
    }
}
