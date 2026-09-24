//! In-memory tag read store for tests.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::tags::{TagReadStore, TagSummary};
use momentum_domain::AccountId;

use crate::commands::MemoryDatabase;

/// Read store over a [`MemoryDatabase`].
#[derive(Debug, Clone)]
pub struct MemoryTagReadStore {
    database: MemoryDatabase,
}

impl MemoryTagReadStore {
    /// Creates the read store.
    #[must_use]
    pub fn new(database: MemoryDatabase) -> Self {
        Self { database }
    }
}

#[async_trait]
impl TagReadStore for MemoryTagReadStore {
    async fn list_tags(&self, account_id: AccountId) -> Result<Vec<TagSummary>, CommandError> {
        let mut tags: Vec<TagSummary> = self
            .database
            .tags_for(account_id)
            .into_iter()
            .map(|(tag, position)| TagSummary {
                id: tag.id(),
                title: tag.title.as_str().to_string(),
                parent: tag.parent,
                position,
                assigned_count: self
                    .database
                    .todos_for(account_id)
                    .into_iter()
                    .filter(|todo| todo.tags.contains(&tag.id()))
                    .count()
                    + self
                        .database
                        .projects_for(account_id)
                        .into_iter()
                        .filter(|project| project.tags.contains(&tag.id()))
                        .count()
                    + self
                        .database
                        .areas_for(account_id)
                        .into_iter()
                        .filter(|area| area.tags.contains(&tag.id()))
                        .count(),
            })
            .collect();
        tags.sort_by_key(|tag| (tag.position, tag.id));
        Ok(tags)
    }
}
