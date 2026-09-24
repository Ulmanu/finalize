//! PostgreSQL read store for tags.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::tags::{TagReadStore, TagSummary};
use momentum_domain::{AccountId, EntityId};
use sqlx::{PgPool, Row};

/// PostgreSQL implementation of [`TagReadStore`].
#[derive(Debug, Clone)]
pub struct PgTagReadStore {
    pool: PgPool,
}

impl PgTagReadStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagReadStore for PgTagReadStore {
    async fn list_tags(&self, account_id: AccountId) -> Result<Vec<TagSummary>, CommandError> {
        let rows = sqlx::query(
            "select e.id, e.title, e.tag_parent, e.tag_position, \
             (select count(*) from tag_assignments ta \
              join entities t on t.id = ta.entity_id and t.status <> 'deleted' \
              where ta.tag_id = e.id) as assigned_count \
             from entities e \
             where e.account_id = $1 and e.kind = 'tag' \
             order by e.tag_position asc, e.created_at asc, e.id asc",
        )
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok(rows
            .into_iter()
            .map(|row| TagSummary {
                id: EntityId::from_uuid(row.get("id")),
                title: row.get("title"),
                parent: row
                    .get::<Option<uuid::Uuid>, _>("tag_parent")
                    .map(EntityId::from_uuid),
                position: row.get("tag_position"),
                assigned_count: usize::try_from(row.get::<i64, _>("assigned_count")).unwrap_or(0),
            })
            .collect())
    }
}
