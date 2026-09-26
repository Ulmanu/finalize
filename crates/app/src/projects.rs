//! Project queries and lifecycle service.

use std::sync::Arc;

use async_trait::async_trait;
use momentum_domain::model::{Deadline, ProjectCompletionPolicy, Schedule, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};

use crate::auth::Clock;
use crate::commands::{
    Command, CommandDispatcher, CommandEnvelope, CommandError, CommandOutcome,
    ProjectDeletionPolicy, ScheduleInput,
};

/// Summary of a heading inside a project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingSummary {
    /// Canonical identity.
    pub id: EntityId,
    /// Title.
    pub title: String,
    /// Whether the heading is archived.
    pub archived: bool,
    /// Ordered to-do identities filed under this heading.
    pub todo_ids: Vec<EntityId>,
}

/// An item in a project's top-level order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectOrderItem {
    /// A heading.
    Heading(EntityId),
    /// A to-do filed directly in the project.
    Todo(EntityId),
}

/// Read model for a project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectSummary {
    /// Canonical identity.
    pub id: EntityId,
    /// Title.
    pub title: String,
    /// Notes.
    pub notes: String,
    /// Lifecycle status.
    pub status: Status,
    /// Schedule.
    pub schedule: Schedule,
    /// Optional deadline.
    pub deadline: Option<Deadline>,
    /// Owning area, when known.
    pub area: Option<EntityId>,
    /// Number of open direct to-dos.
    pub open_count: usize,
    /// Completion instant, when completed.
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Cancellation instant, when canceled.
    pub canceled_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Logging instant, when explicitly logged.
    pub logged_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of completed direct to-dos.
    pub completed_count: usize,
    /// Headings in the project, archived ones included.
    pub headings: Vec<HeadingSummary>,
    /// Top-level order of headings and unheaded to-dos.
    pub order: Vec<ProjectOrderItem>,
}

impl ProjectSummary {
    /// Eligible children (open + completed; canceled and deleted excluded).
    #[must_use]
    pub const fn eligible_count(&self) -> usize {
        self.open_count + self.completed_count
    }

    /// Derived completion ratio in `0.0..=1.0`, when the project has eligible children.
    #[must_use]
    pub fn progress(&self) -> Option<f32> {
        let total = self.eligible_count();
        if total == 0 {
            None
        } else {
            #[allow(clippy::cast_precision_loss)]
            Some(self.completed_count as f32 / total as f32)
        }
    }
}

/// Read-store port for projects.
#[async_trait]
pub trait ProjectReadStore: Send + Sync {
    /// Lists non-deleted projects.
    async fn list_projects(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<ProjectSummary>, CommandError>;

    /// Lists trashed (soft-deleted) projects.
    async fn list_deleted_projects(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<ProjectSummary>, CommandError>;

    /// Loads one project.
    async fn get_project(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<ProjectSummary>, CommandError>;
}

/// Project lifecycle service.
pub struct ProjectService {
    dispatcher: Arc<CommandDispatcher>,
    reader: Arc<dyn ProjectReadStore>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for ProjectService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ProjectService")
    }
}

impl ProjectService {
    /// Creates the service.
    #[must_use]
    pub fn new(
        dispatcher: Arc<CommandDispatcher>,
        reader: Arc<dyn ProjectReadStore>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            dispatcher,
            reader,
            clock,
        }
    }

    /// Lists projects.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn list(&self, account_id: AccountId) -> Result<Vec<ProjectSummary>, CommandError> {
        self.reader.list_projects(account_id).await
    }

    /// Lists logged/completed and canceled projects.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn logged(&self, account_id: AccountId) -> Result<Vec<ProjectSummary>, CommandError> {
        let mut projects: Vec<ProjectSummary> = self
            .list(account_id)
            .await?
            .into_iter()
            .filter(|project| project.status.is_terminal())
            .collect();
        projects.sort_by_key(|project| {
            std::cmp::Reverse(
                project
                    .logged_at
                    .or(project.completed_at)
                    .or(project.canceled_at),
            )
        });
        Ok(projects)
    }

    /// Lists trashed projects.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn trash(&self, account_id: AccountId) -> Result<Vec<ProjectSummary>, CommandError> {
        self.reader.list_deleted_projects(account_id).await
    }

    /// Restores a trashed project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn restore(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::RestoreProject { id })
            .await
    }

    /// Permanently deletes a trashed project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn purge(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::PurgeProject { id })
            .await
    }

    /// Lists open projects deferred to the Someday schedule state.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn someday(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<ProjectSummary>, CommandError> {
        Ok(self
            .list(account_id)
            .await?
            .into_iter()
            .filter(|project| project.status == Status::Open && project.schedule.is_someday())
            .collect())
    }

    /// Returns the service's current local date (UTC until localization).
    #[must_use]
    pub fn local_today(&self) -> LocalDate {
        LocalDate::today_in_timezone(self.clock.now(), "UTC")
    }

    /// Loads one project.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn get(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<ProjectSummary>, CommandError> {
        self.reader.get_project(account_id, id).await
    }

    /// Creates a heading inside a project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn create_heading(
        &self,
        account_id: AccountId,
        project: EntityId,
        title: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::CreateHeading {
                project,
                title: title.to_string(),
            },
        )
        .await
    }

    /// Renames a heading.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn rename_heading(
        &self,
        account_id: AccountId,
        id: EntityId,
        title: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::RenameHeading {
                id,
                title: title.to_string(),
            },
        )
        .await
    }

    /// Moves a heading one position up or down, taking its children with it.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn reorder_heading(
        &self,
        account_id: AccountId,
        id: EntityId,
        direction: crate::commands::ReorderDirection,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::ReorderHeading { id, direction })
            .await
    }

    /// Archives a heading without deleting its children.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn archive_heading(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::ArchiveHeading { id })
            .await
    }

    /// Restores an archived heading.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn restore_heading(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::RestoreHeading { id })
            .await
    }

    /// Creates a project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn create(
        &self,
        account_id: AccountId,
        title: &str,
        area: Option<EntityId>,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::CreateProject {
                title: title.to_string(),
                area,
            },
        )
        .await
    }

    /// Renames a project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn rename(
        &self,
        account_id: AccountId,
        id: EntityId,
        title: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::RenameProject {
                id,
                title: title.to_string(),
            },
        )
        .await
    }

    /// Replaces project notes.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn set_notes(
        &self,
        account_id: AccountId,
        id: EntityId,
        notes: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::SetProjectNotes {
                id,
                notes: notes.to_string(),
            },
        )
        .await
    }

    /// Schedules a project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn schedule(
        &self,
        account_id: AccountId,
        id: EntityId,
        schedule: ScheduleInput,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::ScheduleProject { id, schedule })
            .await
    }

    /// Sets a project deadline.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn set_deadline(
        &self,
        account_id: AccountId,
        id: EntityId,
        deadline: Option<LocalDate>,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::SetProjectDeadline { id, deadline })
            .await
    }

    /// Completes a project under an explicit policy.
    ///
    /// # Errors
    ///
    /// Returns a conflict when the reject policy sees open children.
    pub async fn complete(
        &self,
        account_id: AccountId,
        id: EntityId,
        policy: ProjectCompletionPolicy,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::CompleteProject { id, policy })
            .await
    }

    /// Cancels a project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn cancel(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::CancelProject { id })
            .await
    }

    /// Reopens a project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn reopen(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::ReopenProject { id })
            .await
    }

    /// Assigns or clears a project's area.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn move_to_area(
        &self,
        account_id: AccountId,
        id: EntityId,
        area: Option<EntityId>,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::MoveProject { id, area })
            .await
    }

    /// Deletes a project under an explicit direct-children policy.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn delete(
        &self,
        account_id: AccountId,
        id: EntityId,
        policy: ProjectDeletionPolicy,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::DeleteProject { id, policy })
            .await
    }

    async fn dispatch(
        &self,
        account_id: AccountId,
        command: Command,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatcher
            .execute(CommandEnvelope::new(account_id, command, self.clock.now()))
            .await
    }
}
