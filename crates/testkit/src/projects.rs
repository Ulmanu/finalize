//! In-memory project read store for tests.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::projects::{HeadingSummary, ProjectOrderItem, ProjectReadStore, ProjectSummary};
use momentum_domain::model::{ContextRef, Status};
use momentum_domain::{AccountId, EntityId};

use crate::commands::MemoryDatabase;

/// Read store over a [`MemoryDatabase`].
#[derive(Debug, Clone)]
pub struct MemoryProjectReadStore {
    database: MemoryDatabase,
}

impl MemoryProjectReadStore {
    /// Creates the read store.
    #[must_use]
    pub fn new(database: MemoryDatabase) -> Self {
        Self { database }
    }

    fn summarize(
        &self,
        account_id: AccountId,
        project: momentum_domain::model::Project,
    ) -> ProjectSummary {
        let id = project.id();
        let open_count = self
            .database
            .todos_for(account_id)
            .into_iter()
            .filter(|todo| {
                todo.context == momentum_domain::model::ContextRef::Project(id)
                    && todo.status == Status::Open
            })
            .count();
        let completed_count = self
            .database
            .todos_for(account_id)
            .into_iter()
            .filter(|todo| {
                todo.context == momentum_domain::model::ContextRef::Project(id)
                    && todo.status.is_terminal()
            })
            .count();
        let mut headings: Vec<HeadingSummary> = self
            .database
            .headings_for(account_id)
            .into_iter()
            .filter(|heading| heading.project == id)
            .map(|heading| {
                let mut todo_ids: Vec<(i64, chrono::DateTime<chrono::Utc>, EntityId)> = self
                    .database
                    .todos_for(account_id)
                    .into_iter()
                    .filter(|todo| {
                        todo.context == ContextRef::Heading(heading.id())
                            && todo.status != Status::Deleted
                    })
                    .map(|todo| {
                        (
                            self.database
                                .context_rank(account_id, todo.id)
                                .map_or(i64::MAX, momentum_app::ordering::Rank::value),
                            todo.created_at,
                            todo.id,
                        )
                    })
                    .collect();
                todo_ids.sort_by_key(|(rank, created_at, id)| (*rank, *created_at, *id));
                HeadingSummary {
                    id: heading.id(),
                    title: heading.title.as_str().to_string(),
                    archived: heading.archived,
                    todo_ids: todo_ids.into_iter().map(|(_, _, id)| id).collect(),
                }
            })
            .collect();
        headings.sort_by_key(|heading| heading.id);

        let mut order_items: Vec<(
            i64,
            chrono::DateTime<chrono::Utc>,
            EntityId,
            ProjectOrderItem,
        )> = self
            .database
            .todos_for(account_id)
            .into_iter()
            .filter(|todo| {
                todo.context == ContextRef::Project(id) && todo.status != Status::Deleted
            })
            .map(|todo| {
                (
                    self.database
                        .context_rank(account_id, todo.id)
                        .map_or(i64::MAX, momentum_app::ordering::Rank::value),
                    todo.created_at,
                    todo.id,
                    ProjectOrderItem::Todo(todo.id),
                )
            })
            .collect();
        for heading in headings.iter().filter(|heading| !heading.archived) {
            let created_at = self
                .database
                .heading(account_id, heading.id)
                .map_or_else(chrono::Utc::now, |heading| heading.meta.created_at);
            order_items.push((
                self.database
                    .context_rank(account_id, heading.id)
                    .map_or(i64::MAX, momentum_app::ordering::Rank::value),
                created_at,
                heading.id,
                ProjectOrderItem::Heading(heading.id),
            ));
        }
        order_items.sort_by_key(|(rank, created_at, id, _)| (*rank, *created_at, *id));
        let order = order_items
            .into_iter()
            .map(|(_, _, _, item)| item)
            .collect();

        ProjectSummary {
            id,
            title: project.title.as_str().to_string(),
            notes: project.notes,
            status: project.status,
            schedule: project.schedule,
            deadline: project.deadline,
            area: self.database.project_area(account_id, id),
            open_count,
            completed_count,
            headings,
            order,
        }
    }
}

#[async_trait]
impl ProjectReadStore for MemoryProjectReadStore {
    async fn list_projects(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<ProjectSummary>, CommandError> {
        Ok(self
            .database
            .projects_for(account_id)
            .into_iter()
            .filter(|project| project.status != Status::Deleted)
            .map(|project| self.summarize(account_id, project))
            .collect())
    }

    async fn get_project(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<ProjectSummary>, CommandError> {
        Ok(self
            .database
            .project(account_id, id)
            .filter(|project| project.status != Status::Deleted)
            .map(|project| self.summarize(account_id, project)))
    }
}
