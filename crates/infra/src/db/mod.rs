//! PostgreSQL persistence foundation.
//!
//! The [`Database`] wrapper owns the connection pool and exposes migration and health
//! operations. Repository implementations for domain entities arrive with the domain and
//! command-layer changes.

pub mod migrations;
pub mod schema;

use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use thiserror::Error;

pub use migrations::{
    applied, apply, checksum, pending, rollback_last, Migration, MigrationError, MIGRATIONS,
};

/// Database connection failures.
#[derive(Debug, Error)]
pub enum DbError {
    /// Connecting to PostgreSQL failed.
    #[error("failed to connect to the database: {0}")]
    Connect(#[source] sqlx::Error),
    /// A health query failed.
    #[error("database health check failed: {0}")]
    Health(#[source] sqlx::Error),
}

/// PostgreSQL connection pool wrapper.
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Connects to PostgreSQL and verifies the connection.
    ///
    /// # Errors
    ///
    /// Returns [`DbError::Connect`] when the pool cannot be created or a connection cannot be
    /// acquired.
    pub async fn connect(url: &str) -> Result<Self, DbError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(url)
            .await
            .map_err(DbError::Connect)?;
        Ok(Self { pool })
    }

    /// Every account id (worker loops iterate accounts).
    ///
    /// # Errors
    ///
    /// Propagates query failures.
    pub async fn account_ids(&self) -> Result<Vec<momentum_domain::AccountId>, DbError> {
        let rows = sqlx::query("select id from accounts order by created_at asc, id asc")
            .fetch_all(&self.pool)
            .await
            .map_err(DbError::Health)?;
        Ok(rows
            .into_iter()
            .map(|row| momentum_domain::AccountId::from_uuid(row.get("id")))
            .collect())
    }

    /// Returns the underlying pool for repositories.
    #[must_use]
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Verifies the database is reachable.
    ///
    /// # Errors
    ///
    /// Returns [`DbError::Health`] when the probe query fails.
    pub async fn health(&self) -> Result<(), DbError> {
        sqlx::query_scalar::<_, i32>("select 1")
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::Health)?;
        Ok(())
    }

    /// Applies pending migrations.
    ///
    /// # Errors
    ///
    /// Propagates [`MigrationError`].
    pub async fn migrate(&self) -> Result<Vec<i64>, MigrationError> {
        migrations::apply(&self.pool).await
    }

    /// Returns pending migration versions.
    ///
    /// # Errors
    ///
    /// Propagates [`MigrationError`].
    pub async fn pending_migrations(&self) -> Result<Vec<i64>, MigrationError> {
        migrations::pending(&self.pool).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Live PostgreSQL checks are conditional on `DATABASE_URL`; this environment has no
    /// database server, so the test records the skip and returns successfully.
    async fn live_database() -> Option<Database> {
        let Ok(url) = std::env::var("DATABASE_URL") else {
            eprintln!(
                "SKIP: DATABASE_URL is not set; live PostgreSQL migration tests were not run"
            );
            return None;
        };
        match Database::connect(&url).await {
            Ok(database) => Some(database),
            Err(error) => {
                eprintln!("SKIP: could not connect to DATABASE_URL: {error}");
                None
            }
        }
    }

    #[tokio::test]
    async fn migrations_apply_idempotently_and_roll_back() {
        let Some(database) = live_database().await else {
            return;
        };

        // Clean slate for this isolated test database.
        while rollback_last(database.pool())
            .await
            .expect("rollback")
            .is_some()
        {}

        let applied = database.migrate().await.expect("apply migrations");
        assert!(applied.contains(&1), "migration 0001 must apply");

        let second = database.migrate().await.expect("re-apply migrations");
        assert!(second.is_empty(), "second apply must be a no-op");

        let tables: Vec<String> = sqlx::query_scalar(
            "select table_name from information_schema.tables where table_schema = 'public'",
        )
        .fetch_all(database.pool())
        .await
        .expect("table listing");
        for expected in [
            "accounts",
            "entities",
            "placements",
            "tombstones",
            "change_log",
        ] {
            assert!(
                tables.iter().any(|table| table == expected),
                "missing {expected}"
            );
        }

        let rolled_back = rollback_last(database.pool())
            .await
            .expect("rollback")
            .expect("a migration to roll back");
        assert_eq!(rolled_back, 1);

        database.migrate().await.expect("re-apply after rollback");
    }
}
