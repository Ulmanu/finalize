//! PostgreSQL preference store over the generic preferences table.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::preferences::{PreferenceScope, PreferenceStore};
use momentum_domain::AccountId;
use serde_json::Value;
use sqlx::{PgPool, Row};

/// PostgreSQL implementation of [`PreferenceStore`].
#[derive(Debug, Clone)]
pub struct PgPreferenceStore {
    pool: PgPool,
}

impl PgPreferenceStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn storage(error: sqlx::Error) -> CommandError {
    CommandError::Storage(error.to_string())
}

#[async_trait]
impl PreferenceStore for PgPreferenceStore {
    async fn load_all(&self, account_id: AccountId) -> Result<Vec<(String, String)>, CommandError> {
        let rows = sqlx::query(
            "select key, value from preferences where account_id = $1 and scope = 'account'",
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(storage)?;
        Ok(rows
            .into_iter()
            .map(|row| {
                let key: String = row.get("key");
                let value: Value = row.get("value");
                let raw = match value {
                    Value::String(text) => text,
                    other => other.to_string(),
                };
                (key, raw)
            })
            .collect())
    }

    async fn save(
        &self,
        account_id: AccountId,
        key: &str,
        scope: PreferenceScope,
        raw: &str,
    ) -> Result<(), CommandError> {
        sqlx::query(
            "insert into preferences (account_id, key, scope, value) values ($1, $2, $3, $4) \
             on conflict (account_id, key) do update set scope = excluded.scope, \
             value = excluded.value, updated_at = now()",
        )
        .bind(account_id.as_uuid())
        .bind(key)
        .bind(scope.as_str())
        .bind(Value::String(raw.to_string()))
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        Ok(())
    }
}
