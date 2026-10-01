//! PostgreSQL change-log reader for the sync protocol.

use async_trait::async_trait;
use momentum_app::commands::{ChangeRecord, CommandError, DomainEvent};
use momentum_app::sync::ChangeLogReader;
use momentum_domain::{AccountId, EntityId};
use serde_json::Value;
use sqlx::{PgPool, Row};

/// Reads persisted change-log entries.
#[derive(Debug, Clone)]
pub struct PgChangeLogReader {
    pool: PgPool,
}

impl PgChangeLogReader {
    /// Creates the reader over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChangeLogReader for PgChangeLogReader {
    async fn changes_after(
        &self,
        account_id: AccountId,
        cursor: i64,
    ) -> Result<Vec<ChangeRecord>, CommandError> {
        let rows = sqlx::query(
            "select sequence, operation_id, entity_id, payload, created_at \
             from change_log where account_id = $1 and sequence > $2 \
             order by sequence asc",
        )
        .bind(account_id.as_uuid())
        .bind(cursor)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        let mut records = Vec::with_capacity(rows.len());
        for row in rows {
            let events = serde_json::from_value::<Vec<DomainEvent>>(row.get::<Value, _>("payload"))
                .map_err(|error| CommandError::Storage(error.to_string()))?;
            records.push(ChangeRecord {
                operation_id: row.get("operation_id"),
                sequence: row.get("sequence"),
                entity_id: row
                    .get::<Option<uuid::Uuid>, _>("entity_id")
                    .map(EntityId::from_uuid),
                events,
                created_at: row.get("created_at"),
            });
        }
        Ok(records)
    }
}
