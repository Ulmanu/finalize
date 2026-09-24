//! To-do queries and the lifecycle service.
//!
//! Reads go through [`TodoReadStore`]; writes are built as typed commands and executed by the
//! [`CommandDispatcher`], so this module never touches persistence directly.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_domain::model::{ContextRef, Deadline, Reminder, Schedule, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};

use crate::auth::Clock;
use crate::commands::{
    Command, CommandDispatcher, CommandEnvelope, CommandError, CommandOutcome, ReminderInput,
    ScheduleInput,
};

/// Read model for a to-do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoSummary {
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
    /// Optional reminder.
    pub reminder: Option<Reminder>,
    /// Contextual parent.
    pub context: ContextRef,
    /// Completion instant.
    pub completed_at: Option<DateTime<Utc>>,
    /// Creation instant.
    pub created_at: DateTime<Utc>,
    /// Optimistic version.
    pub version: i64,
}

/// Scope of a to-do listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoScope {
    /// Open to-dos with no contextual parent.
    Inbox,
    /// Every open to-do, ordered by creation.
    AllOpen,
    /// Completed and canceled to-dos.
    Logbook,
    /// Soft-deleted to-dos.
    Trash,
}

/// Read-store port for to-dos.
#[async_trait]
pub trait TodoReadStore: Send + Sync {
    /// Lists to-dos for a scope.
    async fn list_todos(
        &self,
        account_id: AccountId,
        scope: TodoScope,
    ) -> Result<Vec<TodoSummary>, CommandError>;

    /// Loads one to-do.
    async fn get_todo(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<TodoSummary>, CommandError>;
}

/// To-do lifecycle service.
pub struct TodoService {
    dispatcher: Arc<CommandDispatcher>,
    reader: Arc<dyn TodoReadStore>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for TodoService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TodoService")
    }
}

impl TodoService {
    /// Creates the service.
    #[must_use]
    pub fn new(
        dispatcher: Arc<CommandDispatcher>,
        reader: Arc<dyn TodoReadStore>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            dispatcher,
            reader,
            clock,
        }
    }

    /// Lists to-dos for a scope.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn list(
        &self,
        account_id: AccountId,
        scope: TodoScope,
    ) -> Result<Vec<TodoSummary>, CommandError> {
        self.reader.list_todos(account_id, scope).await
    }

    /// Loads one to-do.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn get(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<TodoSummary>, CommandError> {
        self.reader.get_todo(account_id, id).await
    }

    /// Creates a to-do.
    ///
    /// # Errors
    ///
    /// Propagates command validation and storage failures.
    pub async fn create(
        &self,
        account_id: AccountId,
        title: &str,
        notes: Option<String>,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::CreateTodo {
                title: title.to_string(),
                notes,
                context: ContextRef::Inbox,
                schedule: ScheduleInput::Anytime,
                deadline: None,
                reminder: None,
            },
        )
        .await
    }

    /// Replaces the title.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn set_title(
        &self,
        account_id: AccountId,
        id: EntityId,
        title: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::SetTodoTitle {
                id,
                title: title.to_string(),
            },
        )
        .await
    }

    /// Replaces the notes.
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
            Command::SetTodoNotes {
                id,
                notes: notes.to_string(),
            },
        )
        .await
    }

    /// Duplicates a to-do.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn duplicate(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::DuplicateTodo { id })
            .await
    }

    /// Completes a to-do.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn complete(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::CompleteTodo { id })
            .await
    }

    /// Cancels a to-do.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn cancel(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::CancelTodo { id }).await
    }

    /// Reopens a to-do.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn reopen(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::ReopenTodo { id }).await
    }

    /// Moves a to-do to Trash.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn delete(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::DeleteTodo { id }).await
    }

    /// Restores a to-do from Trash.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn restore(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::RestoreTodo { id }).await
    }

    /// Moves a to-do to a context.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn move_to(
        &self,
        account_id: AccountId,
        id: EntityId,
        context: ContextRef,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::MoveTodo { id, context })
            .await
    }

    /// Replaces the schedule.
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
        self.dispatch(account_id, Command::ScheduleTodo { id, schedule })
            .await
    }

    /// Replaces the deadline.
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
        self.dispatch(account_id, Command::SetTodoDeadline { id, deadline })
            .await
    }

    /// Replaces the reminder.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn set_reminder(
        &self,
        account_id: AccountId,
        id: EntityId,
        reminder: Option<ReminderInput>,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::SetTodoReminder { id, reminder })
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
