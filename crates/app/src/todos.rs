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

/// Quick schedule destinations offered by list processing controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickSchedule {
    /// Schedule for today.
    Today,
    /// Schedule for today in the This Evening section.
    ThisEvening,
    /// Schedule for the next local date.
    Tomorrow,
    /// Defer to Someday.
    Someday,
    /// Clear the start date (Anytime).
    Anytime,
}

/// Read model for one checklist item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecklistItemSummary {
    /// Item identity.
    pub id: momentum_domain::ChecklistItemId,
    /// Item text.
    pub title: String,
    /// Completion state.
    pub completed: bool,
}

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
    /// Ordered checklist items.
    pub checklist: Vec<ChecklistItemSummary>,
    /// Directly assigned tags.
    pub tags: Vec<EntityId>,
    /// Tags inherited from the contextual parent (project/area), without direct assignment.
    pub inherited_tags: Vec<EntityId>,
}

/// Scope of a to-do listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoScope {
    /// Open to-dos with no contextual parent.
    Inbox,
    /// Every open to-do, ordered by creation.
    AllOpen,
    /// Open to-dos scheduled on or before the local today (This Evening included).
    Today,
    /// Open to-dos scheduled on a future date.
    Upcoming,
    /// Open to-dos with an Anytime schedule.
    Anytime,
    /// Open to-dos deferred to Someday.
    Someday,
    /// Completed and canceled to-dos.
    Logbook,
    /// Soft-deleted to-dos.
    Trash,
}

/// Read-store port for to-dos.
#[async_trait]
pub trait TodoReadStore: Send + Sync {
    /// Lists to-dos for a scope, relative to the caller's local date.
    async fn list_todos(
        &self,
        account_id: AccountId,
        scope: TodoScope,
        today: LocalDate,
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
        // The user's timezone preference arrives with the localization capability; until then
        // the local date is derived at UTC.
        let today = LocalDate::today_in_timezone(self.clock.now(), "UTC");
        self.reader.list_todos(account_id, scope, today).await
    }

    /// Lists to-dos for a scope using an explicit local-UTC offset in minutes.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn list_at(
        &self,
        account_id: AccountId,
        scope: TodoScope,
        offset_minutes: i32,
    ) -> Result<Vec<TodoSummary>, CommandError> {
        let today = LocalDate::today_from_utc(self.clock.now(), offset_minutes)
            .unwrap_or_else(|_| LocalDate::today_in_timezone(self.clock.now(), "UTC"));
        self.reader.list_todos(account_id, scope, today).await
    }

    /// Returns the service's current local date (UTC until localization).
    #[must_use]
    pub fn local_today(&self) -> LocalDate {
        LocalDate::today_in_timezone(self.clock.now(), "UTC")
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

    /// Creates a to-do in the Inbox.
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
        self.create_in(account_id, title, notes, ContextRef::Inbox)
            .await
    }

    /// Creates a to-do in a specific context.
    ///
    /// # Errors
    ///
    /// Propagates command validation and storage failures.
    pub async fn create_in(
        &self,
        account_id: AccountId,
        title: &str,
        notes: Option<String>,
        context: ContextRef,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::CreateTodo {
                title: title.to_string(),
                notes,
                context,
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

    /// Moves a to-do up within its contextual scope.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn reorder_up(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::ReorderTodo {
                id,
                direction: crate::commands::ReorderDirection::Up,
            },
        )
        .await
    }

    /// Moves a to-do down within its contextual scope.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn reorder_down(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::ReorderTodo {
                id,
                direction: crate::commands::ReorderDirection::Down,
            },
        )
        .await
    }

    /// Applies a quick schedule relative to `today`.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn schedule_quick(
        &self,
        account_id: AccountId,
        id: EntityId,
        quick: QuickSchedule,
        today: LocalDate,
    ) -> Result<CommandOutcome, CommandError> {
        let schedule = match quick {
            QuickSchedule::Today => ScheduleInput::On(today),
            QuickSchedule::ThisEvening => ScheduleInput::ThisEvening(today),
            QuickSchedule::Tomorrow => ScheduleInput::On(today.next_day()),
            QuickSchedule::Someday => ScheduleInput::Someday,
            QuickSchedule::Anytime => ScheduleInput::Anytime,
        };
        self.schedule(account_id, id, schedule).await
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

    /// Adds a checklist item.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn add_checklist_item(
        &self,
        account_id: AccountId,
        todo: EntityId,
        title: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::AddChecklistItem {
                todo,
                title: title.to_string(),
            },
        )
        .await
    }

    /// Renames a checklist item.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn rename_checklist_item(
        &self,
        account_id: AccountId,
        todo: EntityId,
        item: momentum_domain::ChecklistItemId,
        title: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::SetChecklistItemTitle {
                todo,
                item,
                title: title.to_string(),
            },
        )
        .await
    }

    /// Sets checklist item completion.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn set_checklist_item_completed(
        &self,
        account_id: AccountId,
        todo: EntityId,
        item: momentum_domain::ChecklistItemId,
        completed: bool,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::SetChecklistItemCompleted {
                todo,
                item,
                completed,
            },
        )
        .await
    }

    /// Removes a checklist item.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn remove_checklist_item(
        &self,
        account_id: AccountId,
        todo: EntityId,
        item: momentum_domain::ChecklistItemId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::RemoveChecklistItem { todo, item })
            .await
    }

    /// Moves a checklist item up or down.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn reorder_checklist_item(
        &self,
        account_id: AccountId,
        todo: EntityId,
        item: momentum_domain::ChecklistItemId,
        direction: crate::commands::ReorderDirection,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::ReorderChecklistItem {
                todo,
                item,
                direction,
            },
        )
        .await
    }

    /// Expands pasted multi-line content into checklist items.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn expand_checklist(
        &self,
        account_id: AccountId,
        todo: EntityId,
        text: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::ExpandChecklist {
                todo,
                text: text.to_string(),
            },
        )
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
