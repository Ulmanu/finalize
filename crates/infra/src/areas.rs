//! PostgreSQL read store for areas.

use async_trait::async_trait;
use momentum_app::areas::{AreaReadStore, AreaSummary};
use momentum_app::commands::CommandError;
use momentum_domain::{AccountId, EntityId};
use sqlx::{PgPool, Row};

/// PostgreSQL implementation of [`AreaReadStore`].
#[derive(Debug, Clone)]
pub struct PgAreaReadStore {
    pool: PgPool,
}

impl PgAreaReadStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn summarize(
        &self,
        account_id: AccountId,
        area_id: EntityId,
        title: String,
    ) -> Result<AreaSummary, CommandError> {
        let todo_count: i64 = sqlx::query_scalar(
            "select count(*) from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'todo' and e.status <> 'deleted' \
             and p.parent_kind = 'area' and p.parent_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(area_id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;

        let project_count: i64 = sqlx::query_scalar(
            "select count(*) from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'project' and e.status = 'open' \
             and p.parent_kind = 'area' and p.parent_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(area_id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;

        Ok(AreaSummary {
            id: area_id,
            title,
            project_count: usize::try_from(project_count).unwrap_or(0),
            todo_count: usize::try_from(todo_count).unwrap_or(0),
        })
    }
}

#[async_trait]
impl AreaReadStore for PgAreaReadStore {
    async fn list_areas(&self, account_id: AccountId) -> Result<Vec<AreaSummary>, CommandError> {
        let rows = sqlx::query(
            "select id, title from entities \
             where account_id = $1 and kind = 'area' and deleted_at is null \
             order by created_at asc, id asc",
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;

        let mut areas = Vec::with_capacity(rows.len());
        for row in rows {
            let id = EntityId::from_uuid(row.get("id"));
            let title = row.get("title");
            areas.push(self.summarize(account_id, id, title).await?);
        }
        Ok(areas)
    }

    async fn list_deleted_areas(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<AreaSummary>, CommandError> {
        let rows = sqlx::query(
            "select id, title from entities \
             where account_id = $1 and kind = 'area' and deleted_at is not null \
             order by deleted_at desc, id desc",
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        let mut areas = Vec::with_capacity(rows.len());
        for row in rows {
            let id = EntityId::from_uuid(row.get("id"));
            let title = row.get("title");
            areas.push(self.summarize(account_id, id, title).await?);
        }
        Ok(areas)
    }

    async fn get_area(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<AreaSummary>, CommandError> {
        let row = sqlx::query(
            "select title from entities \
             where account_id = $1 and id = $2 and kind = 'area' and deleted_at is null",
        )
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;

        match row {
            Some(row) => Ok(Some(
                self.summarize(account_id, id, row.get("title")).await?,
            )),
            None => Ok(None),
        }
    }
}
