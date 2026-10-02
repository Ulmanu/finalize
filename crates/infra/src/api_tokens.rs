//! PostgreSQL automation-token store.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_app::api_tokens::{ApiScope, ApiTokenStore, ApiTokenSummary};
use momentum_app::commands::CommandError;
use momentum_domain::{AccountId, EntityId};
use sqlx::{PgPool, Postgres, Row};

/// PostgreSQL implementation of [`ApiTokenStore`].
#[derive(Debug, Clone)]
pub struct PgApiTokenStore {
    pool: PgPool,
}

impl PgApiTokenStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn storage(error: sqlx::Error) -> CommandError {
    CommandError::Storage(error.to_string())
}

fn summary_from_row(row: &sqlx::postgres::PgRow) -> ApiTokenSummary {
    let scopes: Vec<String> = row.get("scopes");
    ApiTokenSummary {
        id: EntityId::from_uuid(row.get("id")),
        name: row.get("name"),
        scopes: scopes
            .iter()
            .filter_map(|scope| ApiScope::parse(scope))
            .collect(),
        created_at: row.get("created_at"),
        last_used_at: row.get("last_used_at"),
        revoked_at: row.get("revoked_at"),
    }
}

const SUMMARY_COLUMNS: &str = "id, name, scopes, created_at, last_used_at, revoked_at";

#[async_trait]
impl ApiTokenStore for PgApiTokenStore {
    async fn create(
        &self,
        account_id: AccountId,
        id: EntityId,
        name: &str,
        token_hash: &str,
        scopes: &[ApiScope],
        now: DateTime<Utc>,
    ) -> Result<ApiTokenSummary, CommandError> {
        let scopes: Vec<String> = scopes
            .iter()
            .map(|scope| scope.as_str().to_string())
            .collect();
        let row = sqlx::query(&format!(
            "insert into api_tokens (id, account_id, name, token_hash, scopes, created_at) \
             values ($1, $2, $3, $4, $5, $6) returning {SUMMARY_COLUMNS}"
        ))
        .bind(id.as_uuid())
        .bind(account_id.as_uuid())
        .bind(name)
        .bind(token_hash)
        .bind(scopes)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(storage)?;
        Ok(summary_from_row(&row))
    }

    async fn list(&self, account_id: AccountId) -> Result<Vec<ApiTokenSummary>, CommandError> {
        let rows = sqlx::query(&format!(
            "select {SUMMARY_COLUMNS} from api_tokens \
             where account_id = $1 order by created_at desc, id desc"
        ))
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(storage)?;
        Ok(rows.iter().map(summary_from_row).collect())
    }

    async fn revoke(
        &self,
        account_id: AccountId,
        id: EntityId,
        now: DateTime<Utc>,
    ) -> Result<bool, CommandError> {
        let result = sqlx::query(
            "update api_tokens set revoked_at = $3 \
             where account_id = $1 and id = $2 and revoked_at is null",
        )
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        Ok(result.rows_affected() == 1)
    }

    async fn rotate(
        &self,
        account_id: AccountId,
        id: EntityId,
        token_hash: &str,
    ) -> Result<Option<ApiTokenSummary>, CommandError> {
        let row = sqlx::query(&format!(
            "update api_tokens set token_hash = $3, last_used_at = null \
             where account_id = $1 and id = $2 and revoked_at is null \
             returning {SUMMARY_COLUMNS}"
        ))
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(storage)?;
        Ok(row.map(|row| summary_from_row(&row)))
    }

    async fn find_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<(EntityId, AccountId, Vec<ApiScope>, bool)>, CommandError> {
        let row = sqlx::query(
            "select id, account_id, scopes, revoked_at from api_tokens where token_hash = $1",
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(storage)?;
        Ok(row.map(|row| {
            let scopes: Vec<String> = row.get("scopes");
            (
                EntityId::from_uuid(row.get("id")),
                AccountId::from_uuid(row.get("account_id")),
                scopes
                    .iter()
                    .filter_map(|scope| ApiScope::parse(scope))
                    .collect(),
                row.get::<Option<DateTime<Utc>>, _>("revoked_at").is_some(),
            )
        }))
    }

    async fn touch(
        &self,
        account_id: AccountId,
        id: EntityId,
        now: DateTime<Utc>,
    ) -> Result<(), CommandError> {
        sqlx::query("update api_tokens set last_used_at = $3 where account_id = $1 and id = $2")
            .bind(account_id.as_uuid())
            .bind(id.as_uuid())
            .bind(now)
            .execute(&self.pool)
            .await
            .map_err(storage)?;
        Ok(())
    }
}

// Keeps Postgres imported for transaction-free simple queries.
#[allow(dead_code)]
fn _pg(_pool: &PgPool) -> Option<Postgres> {
    None
}
