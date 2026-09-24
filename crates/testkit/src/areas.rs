//! In-memory area read store for tests.

use async_trait::async_trait;
use momentum_app::areas::{AreaReadStore, AreaSummary};
use momentum_app::commands::CommandError;
use momentum_domain::{AccountId, EntityId};

use crate::commands::MemoryDatabase;

/// Read store over a [`MemoryDatabase`].
#[derive(Debug, Clone)]
pub struct MemoryAreaReadStore {
    database: MemoryDatabase,
}

impl MemoryAreaReadStore {
    /// Creates the read store.
    #[must_use]
    pub fn new(database: MemoryDatabase) -> Self {
        Self { database }
    }

    fn summarize(&self, account_id: AccountId, area_id: EntityId, title: String) -> AreaSummary {
        let children = self.database.area_children(account_id, area_id);
        AreaSummary {
            id: area_id,
            title,
            project_count: children.0,
            todo_count: children.1,
        }
    }
}

#[async_trait]
impl AreaReadStore for MemoryAreaReadStore {
    async fn list_areas(&self, account_id: AccountId) -> Result<Vec<AreaSummary>, CommandError> {
        Ok(self
            .database
            .areas_for(account_id)
            .into_iter()
            .filter(|area| area.meta.deleted_at.is_none())
            .map(|area| self.summarize(account_id, area.id(), area.title.as_str().to_string()))
            .collect())
    }

    async fn get_area(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<AreaSummary>, CommandError> {
        Ok(self
            .database
            .area(account_id, id)
            .filter(|area| area.meta.deleted_at.is_none())
            .map(|area| self.summarize(account_id, id, area.title.as_str().to_string())))
    }
}
