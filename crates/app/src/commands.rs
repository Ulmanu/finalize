//! Typed command layer: the single authoritative mutation boundary.
//!
//! Every mutation enters through a [`Command`] carried by a [`CommandEnvelope`]. The
//! [`CommandDispatcher`] executes it inside a [`UnitOfWork`] transaction: domain rules are
//! applied to account-scoped entities, persistence and change-log metadata are written
//! atomically, and the outcome carries domain events plus projection invalidations. Replaying
//! an operation id returns the recorded outcome without re-applying side effects.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_domain::model::{ContextRef, Deadline, Reminder, Schedule, Status, Title, Todo};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_domain::EntityId;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::auth::Clock;

/// Schedule input accepted by commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ScheduleInput {
    /// Actionable with no start date.
    Anytime,
    /// Deferred to Someday.
    Someday,
    /// Scheduled for a local date.
    On(LocalDate),
    /// Scheduled for a local date in the This Evening section.
    ThisEvening(LocalDate),
}

impl ScheduleInput {
    /// Converts the input into a domain schedule.
    ///
    /// # Errors
    ///
    /// Propagates domain schedule errors (evening without a date cannot be expressed here).
    pub fn to_schedule(self) -> Result<Schedule, CommandError> {
        Ok(match self {
            Self::Anytime => Schedule::anytime(),
            Self::Someday => Schedule::someday(),
            Self::On(date) => Schedule::on(date),
            Self::ThisEvening(date) => Schedule::this_evening(date),
        })
    }
}

/// Reminder input accepted by commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReminderInput {
    /// Instant the reminder fires.
    pub at: DateTime<Utc>,
    /// IANA timezone used to present and snooze it.
    pub timezone: String,
}

/// Typed application command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Command {
    /// Creates a to-do.
    CreateTodo {
        /// Title text.
        title: String,
        /// Optional notes.
        notes: Option<String>,
        /// Contextual parent.
        context: ContextRef,
        /// Initial schedule.
        schedule: ScheduleInput,
        /// Optional deadline.
        deadline: Option<LocalDate>,
        /// Optional reminder.
        reminder: Option<ReminderInput>,
    },
    /// Duplicates a to-do into a new canonical identity.
    DuplicateTodo {
        /// Entity to duplicate.
        id: EntityId,
    },
    /// Replaces the title.
    SetTodoTitle {
        /// Entity to update.
        id: EntityId,
        /// New title text.
        title: String,
    },
    /// Replaces the notes.
    SetTodoNotes {
        /// Entity to update.
        id: EntityId,
        /// New notes text.
        notes: String,
    },
    /// Completes a to-do.
    CompleteTodo {
        /// Entity to complete.
        id: EntityId,
    },
    /// Cancels a to-do.
    CancelTodo {
        /// Entity to cancel.
        id: EntityId,
    },
    /// Reopens a completed or canceled to-do.
    ReopenTodo {
        /// Entity to reopen.
        id: EntityId,
    },
    /// Moves a to-do to Trash.
    DeleteTodo {
        /// Entity to delete.
        id: EntityId,
    },
    /// Restores a to-do from Trash.
    RestoreTodo {
        /// Entity to restore.
        id: EntityId,
    },
    /// Moves a to-do to a new contextual parent.
    MoveTodo {
        /// Entity to move.
        id: EntityId,
        /// Destination context.
        context: ContextRef,
    },
    /// Replaces the schedule.
    ScheduleTodo {
        /// Entity to schedule.
        id: EntityId,
        /// New schedule.
        schedule: ScheduleInput,
    },
    /// Replaces the deadline.
    SetTodoDeadline {
        /// Entity to update.
        id: EntityId,
        /// New deadline, or `None` to clear.
        deadline: Option<LocalDate>,
    },
    /// Replaces the reminder.
    SetTodoReminder {
        /// Entity to update.
        id: EntityId,
        /// New reminder, or `None` to clear.
        reminder: Option<ReminderInput>,
    },
}

impl Command {
    /// Entity the command targets, when it is not a creation.
    #[must_use]
    pub const fn target(&self) -> Option<EntityId> {
        match self {
            Self::CreateTodo { .. } => None,
            Self::DuplicateTodo { id }
            | Self::SetTodoTitle { id, .. }
            | Self::SetTodoNotes { id, .. }
            | Self::CompleteTodo { id }
            | Self::CancelTodo { id }
            | Self::ReopenTodo { id }
            | Self::DeleteTodo { id }
            | Self::RestoreTodo { id }
            | Self::MoveTodo { id, .. }
            | Self::ScheduleTodo { id, .. }
            | Self::SetTodoDeadline { id, .. }
            | Self::SetTodoReminder { id, .. } => Some(*id),
        }
    }

    /// Stable command name used in diagnostics.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::CreateTodo { .. } => "create_todo",
            Self::DuplicateTodo { .. } => "duplicate_todo",
            Self::SetTodoTitle { .. } => "set_todo_title",
            Self::SetTodoNotes { .. } => "set_todo_notes",
            Self::CompleteTodo { .. } => "complete_todo",
            Self::CancelTodo { .. } => "cancel_todo",
            Self::ReopenTodo { .. } => "reopen_todo",
            Self::DeleteTodo { .. } => "delete_todo",
            Self::RestoreTodo { .. } => "restore_todo",
            Self::MoveTodo { .. } => "move_todo",
            Self::ScheduleTodo { .. } => "schedule_todo",
            Self::SetTodoDeadline { .. } => "set_todo_deadline",
            Self::SetTodoReminder { .. } => "set_todo_reminder",
        }
    }
}

/// A command with its idempotency and authorization envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    /// Stable client operation id; retries reuse it.
    pub operation_id: Uuid,
    /// Account issuing the command.
    pub account_id: AccountId,
    /// Issue instant.
    pub issued_at: DateTime<Utc>,
    /// The command itself.
    pub command: Command,
}

impl CommandEnvelope {
    /// Creates an envelope with a fresh operation id.
    #[must_use]
    pub fn new(account_id: AccountId, command: Command, issued_at: DateTime<Utc>) -> Self {
        Self {
            operation_id: Uuid::now_v7(),
            account_id,
            issued_at,
            command,
        }
    }
}

/// Domain event emitted by a command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum DomainEvent {
    /// A to-do was created.
    TodoCreated {
        /// Entity identity.
        id: EntityId,
    },
    /// A to-do was duplicated.
    TodoDuplicated {
        /// Identity of the new to-do.
        id: EntityId,
        /// Identity of the source to-do.
        source_id: EntityId,
    },
    /// The title changed.
    TodoTitleChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// The notes changed.
    TodoNotesChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was completed.
    TodoCompleted {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was canceled.
    TodoCanceled {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was reopened.
    TodoReopened {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was deleted.
    TodoDeleted {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was restored.
    TodoRestored {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do moved to another context.
    TodoMoved {
        /// Entity identity.
        id: EntityId,
    },
    /// The schedule changed.
    TodoScheduled {
        /// Entity identity.
        id: EntityId,
    },
    /// The deadline changed.
    TodoDeadlineChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// The reminder changed.
    TodoReminderChanged {
        /// Entity identity.
        id: EntityId,
    },
}

impl DomainEvent {
    /// Entity the event refers to.
    #[must_use]
    pub const fn entity_id(&self) -> EntityId {
        match self {
            Self::TodoCreated { id }
            | Self::TodoDuplicated { id, .. }
            | Self::TodoTitleChanged { id }
            | Self::TodoNotesChanged { id }
            | Self::TodoCompleted { id }
            | Self::TodoCanceled { id }
            | Self::TodoReopened { id }
            | Self::TodoDeleted { id }
            | Self::TodoRestored { id }
            | Self::TodoMoved { id }
            | Self::TodoScheduled { id }
            | Self::TodoDeadlineChanged { id }
            | Self::TodoReminderChanged { id } => *id,
        }
    }
}

/// Projection that must be re-derived after a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "projection", rename_all = "snake_case")]
pub enum Projection {
    /// Inbox list.
    Inbox,
    /// Today list (including This Evening).
    Today,
    /// Upcoming list.
    Upcoming,
    /// Anytime list.
    Anytime,
    /// Someday list.
    Someday,
    /// Logbook list.
    Logbook,
    /// Trash list.
    Trash,
    /// Deadlines special list.
    Deadlines,
    /// Repeating special list.
    Repeating,
    /// All Projects special list.
    AllProjects,
    /// Logged Projects special list.
    LoggedProjects,
    /// A specific project.
    Project(EntityId),
    /// A specific area.
    Area(EntityId),
}

/// Typed command failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommandError {
    /// The command input failed validation.
    #[error("validation failed: {0}")]
    Validation(String),
    /// The entity does not exist for this account.
    #[error("entity not found")]
    NotFound,
    /// The entity changed since the caller last read it.
    #[error("version conflict: expected {expected}, actual {actual}")]
    Conflict {
        /// Version the caller expected.
        expected: i64,
        /// Version currently stored.
        actual: i64,
    },
    /// The storage adapter failed.
    #[error("storage error")]
    Storage(String),
}

/// Persisted change record used for sync and idempotent replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeRecord {
    /// Operation id of the command that produced the record.
    pub operation_id: Uuid,
    /// Per-account monotonic sequence.
    pub sequence: i64,
    /// Entity the change concerns.
    pub entity_id: Option<EntityId>,
    /// Emitted events.
    pub events: Vec<DomainEvent>,
    /// Record creation instant.
    pub created_at: DateTime<Utc>,
}

/// Result of executing a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    /// Operation id of the executed command.
    pub operation_id: Uuid,
    /// Entity the command created or changed.
    pub entity_id: Option<EntityId>,
    /// Emitted domain events.
    pub events: Vec<DomainEvent>,
    /// Projections to invalidate.
    pub invalidated: Vec<Projection>,
    /// Change-log sequence assigned to this command.
    pub sequence: i64,
    /// Whether this outcome came from a replayed operation id.
    pub replayed: bool,
}

/// Transactional unit of work for command execution.
#[async_trait]
pub trait UnitOfWork: Send {
    /// Loads an account-scoped to-do.
    async fn load_todo(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Todo>, CommandError>;

    /// Persists a to-do, enforcing the expected version.
    async fn save_todo(
        &mut self,
        account_id: AccountId,
        todo: &Todo,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError>;

    /// Looks up a change record by operation id (idempotency check).
    async fn change_by_operation(
        &mut self,
        account_id: AccountId,
        operation_id: Uuid,
    ) -> Result<Option<ChangeRecord>, CommandError>;

    /// Appends a change record, assigning the next per-account sequence.
    async fn append_change(
        &mut self,
        account_id: AccountId,
        record: &ChangeRecord,
    ) -> Result<ChangeRecord, CommandError>;

    /// Commits the transaction.
    async fn commit(self: Box<Self>) -> Result<(), CommandError>;

    /// Rolls the transaction back.
    async fn rollback(self: Box<Self>) -> Result<(), CommandError>;
}

/// Creates unit-of-work transactions.
#[async_trait]
pub trait UnitOfWorkFactory: Send + Sync {
    /// Begins a transaction for an account.
    async fn begin(&self, account_id: AccountId) -> Result<Box<dyn UnitOfWork>, CommandError>;
}

/// Executes commands atomically.
pub struct CommandDispatcher {
    factory: Arc<dyn UnitOfWorkFactory>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for CommandDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CommandDispatcher")
    }
}

impl CommandDispatcher {
    /// Creates a dispatcher from its ports.
    #[must_use]
    pub fn new(factory: Arc<dyn UnitOfWorkFactory>, clock: Arc<dyn Clock>) -> Self {
        Self { factory, clock }
    }

    /// Executes a command inside one transaction.
    ///
    /// # Errors
    ///
    /// Returns a typed [`CommandError`] for validation, not-found, conflict, and storage
    /// failures.
    pub async fn execute(&self, envelope: CommandEnvelope) -> Result<CommandOutcome, CommandError> {
        let mut unit = self.factory.begin(envelope.account_id).await?;

        if let Some(record) = unit
            .change_by_operation(envelope.account_id, envelope.operation_id)
            .await?
        {
            unit.rollback().await?;
            return Ok(CommandOutcome {
                operation_id: envelope.operation_id,
                entity_id: record.entity_id,
                invalidated: invalidations_for_events(&record.events),
                events: record.events,
                sequence: record.sequence,
                replayed: true,
            });
        }

        let now = self.clock.now();
        let outcome = self.apply(&mut *unit, &envelope, now).await;

        match outcome {
            Ok((entity_id, events)) => {
                let record = ChangeRecord {
                    operation_id: envelope.operation_id,
                    sequence: 0,
                    entity_id,
                    events: events.clone(),
                    created_at: now,
                };
                let stored = unit.append_change(envelope.account_id, &record).await?;
                unit.commit().await?;
                Ok(CommandOutcome {
                    operation_id: envelope.operation_id,
                    entity_id,
                    invalidated: invalidations_for_events(&events),
                    events,
                    sequence: stored.sequence,
                    replayed: false,
                })
            }
            Err(error) => {
                unit.rollback().await?;
                Err(error)
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn apply(
        &self,
        unit: &mut dyn UnitOfWork,
        envelope: &CommandEnvelope,
        now: DateTime<Utc>,
    ) -> Result<(Option<EntityId>, Vec<DomainEvent>), CommandError> {
        let account_id = envelope.account_id;
        match &envelope.command {
            Command::CreateTodo {
                title,
                notes,
                context,
                schedule,
                deadline,
                reminder,
            } => {
                let title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                let mut todo = Todo::new(title, now);
                if let Some(notes) = notes {
                    todo.notes = notes.clone();
                }
                todo.context = *context;
                todo.schedule = schedule.to_schedule()?;
                todo.deadline = deadline.map(Deadline::new);
                todo.reminder = reminder
                    .clone()
                    .map(|reminder| Reminder::scheduled(reminder.at, reminder.timezone));
                unit.save_todo(account_id, &todo, None).await?;
                Ok((
                    Some(todo.id()),
                    vec![DomainEvent::TodoCreated { id: todo.id() }],
                ))
            }
            Command::DuplicateTodo { id } => {
                let source = unit
                    .load_todo(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let mut copy = Todo::new(source.title.clone(), now);
                copy.notes = source.notes.clone();
                copy.schedule = source.schedule;
                copy.deadline = source.deadline;
                copy.reminder = source.reminder.clone();
                copy.context = source.context;
                copy.tags = source.tags.clone();
                unit.save_todo(account_id, &copy, None).await?;
                Ok((
                    Some(copy.id()),
                    vec![DomainEvent::TodoDuplicated {
                        id: copy.id(),
                        source_id: source.id(),
                    }],
                ))
            }
            command => {
                let id = command.target().ok_or(CommandError::NotFound)?;
                let mut todo = unit
                    .load_todo(account_id, id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let expected_version = todo.meta.version;
                let event = apply_to_todo(&mut todo, command, now)?;
                unit.save_todo(account_id, &todo, Some(expected_version))
                    .await?;
                Ok((Some(todo.id()), vec![event]))
            }
        }
    }
}

fn apply_to_todo(
    todo: &mut Todo,
    command: &Command,
    now: DateTime<Utc>,
) -> Result<DomainEvent, CommandError> {
    let id = todo.id();
    match command {
        Command::CreateTodo { .. } | Command::DuplicateTodo { .. } => Err(
            CommandError::Validation("creation commands are handled separately".to_string()),
        ),
        Command::SetTodoTitle { title, .. } => {
            todo.title =
                Title::parse(title).map_err(|error| CommandError::Validation(error.to_string()))?;
            todo.meta.touch(now);
            Ok(DomainEvent::TodoTitleChanged { id })
        }
        Command::SetTodoNotes { notes, .. } => {
            if notes.len() > momentum_domain::model::MAX_NOTES_LEN {
                return Err(CommandError::Validation("notes are too long".to_string()));
            }
            todo.notes = notes.clone();
            todo.meta.touch(now);
            Ok(DomainEvent::TodoNotesChanged { id })
        }
        Command::CompleteTodo { .. } => {
            todo.complete(now);
            Ok(DomainEvent::TodoCompleted { id })
        }
        Command::CancelTodo { .. } => {
            todo.cancel(now);
            Ok(DomainEvent::TodoCanceled { id })
        }
        Command::ReopenTodo { .. } => {
            todo.reopen(now);
            Ok(DomainEvent::TodoReopened { id })
        }
        Command::DeleteTodo { .. } => {
            if todo.status != Status::Deleted {
                todo.status = Status::Deleted;
                todo.meta.deleted_at = Some(now);
                todo.meta.touch(now);
            }
            Ok(DomainEvent::TodoDeleted { id })
        }
        Command::RestoreTodo { .. } => {
            if todo.status == Status::Deleted {
                todo.status = Status::Open;
                todo.meta.deleted_at = None;
                todo.meta.touch(now);
            }
            Ok(DomainEvent::TodoRestored { id })
        }
        Command::MoveTodo { context, .. } => {
            todo.move_to(*context, now);
            Ok(DomainEvent::TodoMoved { id })
        }
        Command::ScheduleTodo { schedule, .. } => {
            todo.set_schedule(schedule.to_schedule()?, now);
            Ok(DomainEvent::TodoScheduled { id })
        }
        Command::SetTodoDeadline { deadline, .. } => {
            todo.set_deadline(deadline.map(Deadline::new), now);
            Ok(DomainEvent::TodoDeadlineChanged { id })
        }
        Command::SetTodoReminder { reminder, .. } => {
            let reminder = reminder
                .clone()
                .map(|reminder| Reminder::scheduled(reminder.at, reminder.timezone));
            todo.set_reminder(reminder, now);
            Ok(DomainEvent::TodoReminderChanged { id })
        }
    }
}

/// Maps events to the projections that must be re-derived.
#[must_use]
pub fn invalidations_for_events(events: &[DomainEvent]) -> Vec<Projection> {
    let mut projections = Vec::new();
    for event in events {
        for projection in invalidations_for(event) {
            if !projections.contains(&projection) {
                projections.push(projection);
            }
        }
    }
    projections
}

/// Maps one event to projections.
#[must_use]
pub fn invalidations_for(event: &DomainEvent) -> Vec<Projection> {
    match event {
        DomainEvent::TodoCreated { .. } | DomainEvent::TodoDuplicated { .. } => {
            vec![Projection::Inbox]
        }
        DomainEvent::TodoTitleChanged { .. } | DomainEvent::TodoNotesChanged { .. } => {
            vec![Projection::Inbox, Projection::Today, Projection::Anytime]
        }
        DomainEvent::TodoCompleted { .. }
        | DomainEvent::TodoCanceled { .. }
        | DomainEvent::TodoReopened { .. } => vec![
            Projection::Inbox,
            Projection::Today,
            Projection::Upcoming,
            Projection::Anytime,
            Projection::Someday,
            Projection::Logbook,
        ],
        DomainEvent::TodoDeleted { .. } | DomainEvent::TodoRestored { .. } => vec![
            Projection::Inbox,
            Projection::Today,
            Projection::Anytime,
            Projection::Trash,
        ],
        DomainEvent::TodoMoved { .. } => vec![Projection::Inbox, Projection::Anytime],
        DomainEvent::TodoScheduled { .. } => vec![
            Projection::Inbox,
            Projection::Today,
            Projection::Upcoming,
            Projection::Anytime,
            Projection::Someday,
        ],
        DomainEvent::TodoDeadlineChanged { .. } => vec![Projection::Deadlines, Projection::Today],
        DomainEvent::TodoReminderChanged { .. } => vec![Projection::Today],
    }
}
