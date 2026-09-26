//! PostgreSQL logging-policy store backed by the preferences table.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::logbook::LoggingPolicyStore;
use momentum_domain::model::LoggingPolicy;
use momentum_domain::AccountId;
use sqlx::PgPool;

/// Preferences key storing the account logging policy.
pub const LOGBOOK_POLICY_KEY: &str = "logbook.policy";

/// PostgreSQL implementation of [`LoggingPolicyStore`].
#[derive(Debug, Clone)]
pub struct PgLoggingPolicyStore {
    pool: PgPool,
}

impl PgLoggingPolicyStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LoggingPolicyStore for PgLoggingPolicyStore {
    async fn load_policy(&self, account_id: AccountId) -> Result<LoggingPolicy, CommandError> {
        let value: Option<String> = sqlx::query_scalar(
            "select value #>> '{}' from preferences where account_id = $1 and key = $2",
        )
        .bind(account_id.as_uuid())
        .bind(LOGBOOK_POLICY_KEY)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok(value
            .as_deref()
            .map_or(LoggingPolicy::Automatic, LoggingPolicy::parse))
    }

    async fn save_policy(
        &self,
        account_id: AccountId,
        policy: LoggingPolicy,
    ) -> Result<(), CommandError> {
        sqlx::query(
            "insert into preferences (account_id, key, scope, value, updated_at) \
             values ($1, $2, 'account', to_jsonb($3::text), now()) \
             on conflict (account_id, key) do update \
             set value = excluded.value, updated_at = now()",
        )
        .bind(account_id.as_uuid())
        .bind(LOGBOOK_POLICY_KEY)
        .bind(policy.as_str())
        .execute(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok(())
    }
}
