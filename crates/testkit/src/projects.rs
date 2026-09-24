//! In-memory project read store for tests.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::projects::{ProjectReadStore, ProjectSummary};
use momentum_domain::model::Status;
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
