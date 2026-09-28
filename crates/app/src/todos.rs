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
    /// Cancellation instant.
    pub canceled_at: Option<DateTime<Utc>>,
    /// Logging instant, when explicitly logged.
    pub logged_at: Option<DateTime<Utc>>,
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

/// Derived progress for a set of completable items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progress {
    /// Completed eligible items.
    pub completed: usize,
    /// Total eligible items (open + completed; canceled and deleted excluded).
    pub total: usize,
}

impl Progress {
    /// Creates progress from counts.
    #[must_use]
    pub const fn new(completed: usize, total: usize) -> Self {
        Self { completed, total }
    }

    /// Whether there is anything to measure (neutral state when false).
    #[must_use]
    pub const fn is_neutral(&self) -> bool {
        self.total == 0
    }

    /// Completion ratio in `0.0..=1.0`, or `None` in the neutral state.
    #[must_use]
    pub fn ratio(&self) -> Option<f32> {
        if self.total == 0 {
            None
        } else {
            #[allow(clippy::cast_precision_loss)]
            Some(self.completed as f32 / self.total as f32)
        }
    }

    /// Rounded completion percentage for display, or `None` in the neutral state.
    #[must_use]
    pub fn percent(&self) -> Option<u32> {
        self.ratio().map(|ratio| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                (ratio * 100.0).round() as u32
            }
        })
    }
}

impl TodoSummary {
    /// Derives the deadline state relative to a local date.
    #[must_use]
    pub fn deadline_state(
        &self,
        today: LocalDate,
    ) -> Option<momentum_domain::model::DeadlineState> {
        self.deadline.map(|deadline| deadline.state(today))
    }
}

/// One date group of the Upcoming agenda.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpcomingDay {
    /// Local calendar date shared by the group.
    pub date: LocalDate,
    /// Open items scheduled for that date, in canonical order.
    pub todos: Vec<TodoSummary>,
}

/// A read-only row contributed by an external calendar projection.
///
/// Calendar rows carry no entity identity, so task mutations cannot target them; an integration
/// that supports editing must introduce its own provider-neutral command path first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarRow {
    /// Local calendar date of the event.
    pub date: LocalDate,
    /// Display title.
    pub title: String,
}

/// The Upcoming agenda: date-grouped task rows plus read-only calendar rows.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UpcomingAgenda {
    /// Future scheduled items grouped by local date, ascending.
    pub days: Vec<UpcomingDay>,
    /// External calendar rows; empty until a calendar integration exists.
    pub calendar: Vec<CalendarRow>,
}

impl UpcomingAgenda {
    /// Groups future-scheduled items by their local schedule date, preserving input order.
    #[must_use]
    pub fn from_todos(todos: Vec<TodoSummary>) -> Self {
        let mut days: Vec<UpcomingDay> = Vec::new();
        for todo in todos {
            let Some(date) = todo.schedule.date() else {
                continue;
            };
            match days.last_mut() {
                Some(day) if day.date == date => day.todos.push(todo),
                _ => days.push(UpcomingDay {
                    date,
                    todos: vec![todo],
                }),
            }
        }
        days.sort_by_key(|day| day.date);
        Self {
            days,
            calendar: Vec::new(),
        }
    }
}

/// One context group of the Anytime projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnytimeGroup {
    /// Existing contextual parent shared by the group (Inbox, area, or project).
    pub context: ContextRef,
    /// Actionable items in canonical order.
    pub todos: Vec<TodoSummary>,
}

/// The Anytime projection grouped by existing context.
///
/// Grouping reflects contexts that already exist; rendering never creates parent relationships.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AnytimeAgenda {
    /// Groups ordered Inbox first, then areas, then projects, deterministically by identity.
    pub groups: Vec<AnytimeGroup>,
}

impl AnytimeAgenda {
    /// Groups actionable items by their existing context, preserving input order within groups.
    #[must_use]
    pub fn from_todos(todos: Vec<TodoSummary>) -> Self {
        let mut groups: Vec<AnytimeGroup> = Vec::new();
        for todo in todos {
            match groups
                .iter_mut()
                .find(|group| group.context == todo.context)
            {
                Some(group) => group.todos.push(todo),
                None => groups.push(AnytimeGroup {
                    context: todo.context,
                    todos: vec![todo],
                }),
            }
        }
        groups.sort_by_key(|group| match group.context {
            ContextRef::Inbox => (0, None),
            ContextRef::Area(id) => (1, Some(id)),
            ContextRef::Project(id) => (2, Some(id)),
            ContextRef::Heading(id) => (3, Some(id)),
        });
        Self { groups }
    }
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
    /// Open to-dos scheduled for the next local calendar date.
    Tomorrow,
    /// Open and completed to-dos scheduled on or before the local today (progress reads).
    TodayAll,
    /// Open to-dos with an Anytime schedule.
    Anytime,
    /// Open to-dos deferred to Someday.
    Someday,
    /// Open to-dos carrying a deadline.
    Deadlines,
    /// Completed and canceled to-dos; under the manual logging policy only already-logged
    /// entries are included.
    Logbook {
        /// Whether to include only explicitly logged entries.
        logged_only: bool,
    },
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

    /// Permanently deletes a trashed to-do.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn purge(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::PurgeTodo { id }).await
    }

    /// Permanently deletes every trashed entity for the account.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn empty_trash(&self, account_id: AccountId) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::EmptyTrash).await
    }

    /// Moves a to-do to an explicit insertion position within its contextual scope.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn move_to_position(
        &self,
        account_id: AccountId,
        id: EntityId,
        position: usize,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::MoveTodoToPosition { id, position })
            .await
    }

    /// Moves a Today item to an explicit insertion position within its Today/Evening bucket.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn move_today_to_position(
        &self,
        account_id: AccountId,
        id: EntityId,
        position: usize,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::MoveTodayToPosition { id, position })
            .await
    }

    /// Moves a Today item within its Today/This Evening ordering bucket.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn reorder_today(
        &self,
        account_id: AccountId,
        id: EntityId,
        direction: crate::commands::ReorderDirection,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::ReorderToday { id, direction })
            .await
    }

    /// Loads the Upcoming agenda: future items grouped by local date plus read-only
    /// calendar rows (none until an external integration exists).
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn upcoming_agenda(
        &self,
        account_id: AccountId,
    ) -> Result<UpcomingAgenda, CommandError> {
        let todos = self.list(account_id, TodoScope::Upcoming).await?;
        Ok(UpcomingAgenda::from_todos(todos))
    }

    /// Derives Today completion progress over completable items (open and completed to-dos
    /// scheduled on or before the local today; canceled and deleted rows are excluded).
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn today_progress(&self, account_id: AccountId) -> Result<Progress, CommandError> {
        let todos = self.list(account_id, TodoScope::TodayAll).await?;
        let total = todos.len();
        let completed = todos
            .iter()
            .filter(|todo| todo.status == momentum_domain::model::Status::Completed)
            .count();
        Ok(Progress::new(completed, total))
    }

    /// Loads the Anytime projection grouped by existing context.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn anytime_agenda(
        &self,
        account_id: AccountId,
    ) -> Result<AnytimeAgenda, CommandError> {
        let todos = self.list(account_id, TodoScope::Anytime).await?;
        Ok(AnytimeAgenda::from_todos(todos))
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

    /// Replaces a recurrence rule for future occurrences.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn update_recurrence_rule(
        &self,
        account_id: AccountId,
        template_id: EntityId,
        rule: momentum_domain::recurrence::Recurrence,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::UpdateRecurrenceRule { template_id, rule },
        )
        .await
    }

    /// Materializes one recurrence occurrence as a fresh copy (idempotent).
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn materialize_occurrence(
        &self,
        account_id: AccountId,
        template_id: EntityId,
        occurrence: LocalDate,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::MaterializeRecurrenceOccurrence {
                template_id,
                occurrence,
            },
        )
        .await
    }

    /// Snoozes the attached reminder using a configured choice.
    ///
    /// # Errors
    ///
    /// Fails when the to-do or reminder is missing or the timezone cannot be resolved.
    pub async fn snooze(
        &self,
        account_id: AccountId,
        id: EntityId,
        choice: crate::reminders::SnoozeChoice,
    ) -> Result<CommandOutcome, CommandError> {
        let todo = self
            .reader
            .get_todo(account_id, id)
            .await?
            .ok_or(CommandError::NotFound)?;
        let Some(reminder) = todo.reminder else {
            return Err(CommandError::Validation(
                "to-do has no reminder to snooze".to_string(),
            ));
        };
        let until = crate::reminders::resolve_snooze(&reminder, self.clock.now(), choice)?;
        self.dispatch(account_id, Command::SnoozeTodoReminder { id, until })
            .await
    }

    /// Marks the attached reminder delivered (idempotent).
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn mark_reminder_delivered(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::MarkTodoReminderDelivered { id })
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

    pub(crate) async fn dispatch(
        &self,
        account_id: AccountId,
        command: Command,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatcher
            .execute(CommandEnvelope::new(account_id, command, self.clock.now()))
            .await
    }
}
