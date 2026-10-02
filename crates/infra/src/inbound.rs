//! PostgreSQL inbound alias and message-ledger stores.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::inbound::{AliasStore, InboundAlias, MessageLedger};
use momentum_domain::AccountId;
use sqlx::{PgPool, Row};

/// PostgreSQL implementation of both inbound ports.
#[derive(Debug, Clone)]
pub struct PgInboundStore {
    pool: PgPool,
}

impl PgInboundStore {
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
impl AliasStore for PgInboundStore {
    async fn alias_for(&self, account_id: AccountId) -> Result<Option<InboundAlias>, CommandError> {
        let row = sqlx::query(
            "select token, enabled from inbound_aliases \
             where account_id = $1 order by created_at desc limit 1",
        )
        .bind(account_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(storage)?;
        Ok(row.map(|row| InboundAlias {
            token: row.get("token"),
            enabled: row.get("enabled"),
        }))
    }

    async fn lookup_token(&self, token: &str) -> Result<Option<(AccountId, bool)>, CommandError> {
        let row = sqlx::query("select account_id, enabled from inbound_aliases where token = $1")
            .bind(token)
            .fetch_optional(&self.pool)
            .await
            .map_err(storage)?;
        Ok(row.map(|row| {
            (
                AccountId::from_uuid(row.get("account_id")),
                row.get("enabled"),
            )
        }))
    }

    async fn rotate(
        &self,
        account_id: AccountId,
        token: &str,
    ) -> Result<InboundAlias, CommandError> {
        let mut tx = self.pool.begin().await.map_err(storage)?;
        sqlx::query("delete from inbound_aliases where account_id = $1")
            .bind(account_id.as_uuid())
            .execute(&mut *tx)
            .await
            .map_err(storage)?;
        sqlx::query(
            "insert into inbound_aliases (token, account_id, enabled) values ($1, $2, true)",
        )
        .bind(token)
        .bind(account_id.as_uuid())
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
        tx.commit().await.map_err(storage)?;
        Ok(InboundAlias {
            token: token.to_string(),
            enabled: true,
        })
    }

    async fn disable(&self, account_id: AccountId) -> Result<(), CommandError> {
        sqlx::query("update inbound_aliases set enabled = false where account_id = $1")
            .bind(account_id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(storage)?;
        Ok(())
    }
}

#[async_trait]
impl MessageLedger for PgInboundStore {
    async fn seen(&self, account_id: AccountId, message_id: &str) -> Result<bool, CommandError> {
        let exists: bool = sqlx::query_scalar(
            "select exists(select 1 from inbound_messages \
             where account_id = $1 and message_id = $2)",
        )
        .bind(account_id.as_uuid())
        .bind(message_id)
        .fetch_one(&self.pool)
        .await
        .map_err(storage)?;
        Ok(exists)
    }

    async fn record(&self, account_id: AccountId, message_id: &str) -> Result<(), CommandError> {
        sqlx::query(
            "insert into inbound_messages (account_id, message_id) values ($1, $2) \
             on conflict (account_id, message_id) do nothing",
        )
        .bind(account_id.as_uuid())
        .bind(message_id)
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        Ok(())
    }
}
