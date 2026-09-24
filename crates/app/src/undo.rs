//! Undo/redo engine built on inverse commands.
//!
//! Undo never writes snapshots: it derives inverse commands from the pre-state and executes
//! them through the same [`CommandDispatcher`] as ordinary mutations, so validation, events,
//! versioning, and sync hooks stay intact. Redo is the inverse of the inverse.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use momentum_domain::model::{ScheduleState, Status, Todo};
use momentum_domain::AccountId;
use momentum_domain::EntityId;
use uuid::Uuid;

use crate::auth::Clock;
use crate::commands::{
    Command, CommandDispatcher, CommandEnvelope, CommandError, ScheduleInput, UnitOfWorkFactory,
};

/// Default number of groups kept in the active history window.
pub const DEFAULT_WINDOW: usize = 100;

/// A command that was executed together with the commands that revert it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndoEntry {
    /// The command that was executed.
    pub command: Command,
    /// Commands that revert it, in execution order.
    pub inverse: Vec<Command>,
}

/// A group of related commands that undo/redo as one step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndoGroup {
    /// Group identity.
    pub id: Uuid,
    /// Human-readable label.
    pub label: String,
    /// Executed commands and their inverses.
    pub entries: Vec<UndoEntry>,
    /// Creation instant.
    pub created_at: DateTime<Utc>,
}

impl UndoGroup {
    /// Number of commands in the group.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the group has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Executes command groups and computes their inverses.
pub struct UndoEngine {
    factory: Arc<dyn UnitOfWorkFactory>,
    dispatcher: Arc<CommandDispatcher>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for UndoEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UndoEngine")
    }
}

impl UndoEngine {
    /// Creates the engine.
    #[must_use]
    pub fn new(
        factory: Arc<dyn UnitOfWorkFactory>,
        dispatcher: Arc<CommandDispatcher>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            factory,
            dispatcher,
            clock,
        }
    }

    /// Executes a group of commands, capturing the inverse of each.
    ///
    /// # Errors
    ///
    /// Propagates command errors; the caller should not record a partially executed group.
    pub async fn apply_group(
        &self,
        account_id: AccountId,
        label: impl Into<String>,
        commands: Vec<Command>,
    ) -> Result<UndoGroup, CommandError> {
        let mut entries = Vec::with_capacity(commands.len());
        for command in commands {
            let pre_state = self.pre_state(account_id, &command).await?;
            let outcome = self
                .dispatcher
                .execute(CommandEnvelope::new(
                    account_id,
                    command.clone(),
                    self.clock.now(),
                ))
                .await?;
            let mut inverse = inverse_commands(pre_state.as_ref(), &command);
            if inverse.is_empty() {
                if let (Command::CreateTodo { .. }, Some(created)) = (&command, outcome.entity_id) {
                    inverse = inverse_of_create(created);
                }
            }
            entries.push(UndoEntry { command, inverse });
        }
        Ok(UndoGroup {
            id: Uuid::now_v7(),
            label: label.into(),
            entries,
            created_at: self.clock.now(),
        })
    }

    /// Reverts a group by executing its inverse commands in reverse order.
    ///
    /// The returned group can be passed to [`UndoEngine::redo`].
    ///
    /// # Errors
    ///
    /// Propagates command errors from the inverse commands.
    pub async fn undo(
        &self,
        account_id: AccountId,
        group: &UndoGroup,
    ) -> Result<UndoGroup, CommandError> {
        let mut commands: Vec<Command> = group
            .entries
            .iter()
            .rev()
            .flat_map(|entry| entry.inverse.clone())
            .collect();
        commands.retain(|command| command.target().is_some());
        self.apply_group(account_id, format!("Undo {}", group.label), commands)
            .await
    }

    /// Re-executes the state that was undone.
    ///
    /// # Errors
    ///
    /// Propagates command errors from the redo commands.
    pub async fn redo(
        &self,
        account_id: AccountId,
        undo_group: &UndoGroup,
    ) -> Result<UndoGroup, CommandError> {
        let commands: Vec<Command> = undo_group
            .entries
            .iter()
            .flat_map(|entry| entry.inverse.clone())
            .collect();
        self.apply_group(account_id, "Redo", commands).await
    }

    async fn pre_state(
        &self,
        account_id: AccountId,
        command: &Command,
    ) -> Result<Option<Todo>, CommandError> {
        let Some(id) = command.target() else {
            return Ok(None);
        };
        let mut unit = self.factory.begin(account_id).await?;
        let todo = unit.load_todo(account_id, id).await?;
        unit.rollback().await?;
        Ok(todo)
    }
}

/// Derives the commands that revert `command`, given the entity state before execution.
#[must_use]
pub fn inverse_commands(pre_state: Option<&Todo>, command: &Command) -> Vec<Command> {
    match command {
        Command::DuplicateTodo { .. } | Command::CreateTodo { .. } => {
            // The created identity is only known after execution; the engine re-derives the
            // inverse from the post-state through the created event, so a create is undone by
            // deleting the entity the command produced.
            Vec::new()
        }
        Command::SetTodoTitle { id, .. } => pre_state.map_or_else(Vec::new, |todo| {
            vec![Command::SetTodoTitle {
                id: *id,
                title: todo.title.as_str().to_string(),
            }]
        }),
        Command::SetTodoNotes { id, .. } => pre_state.map_or_else(Vec::new, |todo| {
            vec![Command::SetTodoNotes {
                id: *id,
                notes: todo.notes.clone(),
            }]
        }),
        Command::CompleteTodo { id } => vec![Command::ReopenTodo { id: *id }],
        Command::CancelTodo { id } => vec![Command::ReopenTodo { id: *id }],
        Command::ReopenTodo { id } => pre_state.map_or_else(Vec::new, |todo| match todo.status {
            Status::Canceled => vec![Command::CancelTodo { id: *id }],
            _ => vec![Command::CompleteTodo { id: *id }],
        }),
        Command::DeleteTodo { id } => vec![Command::RestoreTodo { id: *id }],
        Command::RestoreTodo { id } => vec![Command::DeleteTodo { id: *id }],
        Command::MoveTodo { id, .. } => pre_state.map_or_else(Vec::new, |todo| {
            vec![Command::MoveTodo {
                id: *id,
                context: todo.context,
            }]
        }),
        Command::ScheduleTodo { id, .. } => pre_state.map_or_else(Vec::new, |todo| {
            vec![Command::ScheduleTodo {
                id: *id,
                schedule: schedule_input(todo),
            }]
        }),
        Command::SetTodoDeadline { id, .. } => pre_state.map_or_else(Vec::new, |todo| {
            vec![Command::SetTodoDeadline {
                id: *id,
                deadline: todo.deadline.map(|deadline| deadline.date),
            }]
        }),
        Command::SetTodoReminder { id, .. } => pre_state.map_or_else(Vec::new, |todo| {
            vec![Command::SetTodoReminder {
                id: *id,
                reminder: todo
                    .reminder
                    .as_ref()
                    .map(|reminder| crate::commands::ReminderInput {
                        at: reminder.at,
                        timezone: reminder.timezone.clone(),
                    }),
            }]
        }),
    }
}

fn schedule_input(todo: &Todo) -> ScheduleInput {
    match todo.schedule.state() {
        ScheduleState::Anytime => ScheduleInput::Anytime,
        ScheduleState::Someday => ScheduleInput::Someday,
        ScheduleState::On(date) => {
            if todo.schedule.is_evening() {
                ScheduleInput::ThisEvening(date)
            } else {
                ScheduleInput::On(date)
            }
        }
    }
}

/// Builds the inverse of a create once the created identity is known.
#[must_use]
pub fn inverse_of_create(created: EntityId) -> Vec<Command> {
    vec![Command::DeleteTodo { id: created }]
}

/// Bounded grouped undo history.
#[derive(Debug, Clone)]
pub struct UndoStack {
    window: usize,
    undo: Vec<UndoGroup>,
    redo: Vec<UndoGroup>,
}

impl UndoStack {
    /// Creates a stack with the default window.
    #[must_use]
    pub fn new() -> Self {
        Self::with_window(DEFAULT_WINDOW)
    }

    /// Creates a stack with a custom window size (minimum 1).
    #[must_use]
    pub fn with_window(window: usize) -> Self {
        Self {
            window: window.max(1),
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    /// Records an executed group, clearing the redo history.
    pub fn push(&mut self, group: UndoGroup) {
        if group.is_empty() {
            return;
        }
        self.redo.clear();
        self.undo.push(group);
        while self.undo.len() > self.window {
            self.undo.remove(0);
        }
    }

    /// Takes the most recent group to undo.
    pub fn pop_undo(&mut self) -> Option<UndoGroup> {
        self.undo.pop()
    }

    /// Records a group that was undone, making it available for redo.
    pub fn push_redo(&mut self, group: UndoGroup) {
        self.redo.push(group);
    }

    /// Takes the most recent group to redo.
    pub fn pop_redo(&mut self) -> Option<UndoGroup> {
        self.redo.pop()
    }

    /// Clears both histories.
    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }

    /// Number of undoable groups.
    #[must_use]
    pub fn undo_depth(&self) -> usize {
        self.undo.len()
    }

    /// Number of redoable groups.
    #[must_use]
    pub fn redo_depth(&self) -> usize {
        self.redo.len()
    }
}

impl Default for UndoStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use momentum_domain::model::{ContextRef, Deadline, Reminder, Title};
    use momentum_domain::time::LocalDate;

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 1, 9, 0, 0).unwrap()
    }

    fn todo() -> Todo {
        Todo::new(Title::parse("Original").expect("title"), now())
    }

    #[test]
    fn inverse_of_edits_restores_previous_values() {
        let mut item = todo();
        item.notes = "old notes".to_string();
        item.set_schedule(
            crate::commands::ScheduleInput::On(LocalDate::from_ymd(2026, 6, 5).expect("date"))
                .to_schedule()
                .expect("schedule"),
            now(),
        );
        item.set_deadline(
            Some(Deadline::new(
                LocalDate::from_ymd(2026, 6, 6).expect("date"),
            )),
            now(),
        );
        item.set_reminder(Some(Reminder::scheduled(now(), "UTC")), now());

        let inverse = inverse_commands(
            Some(&item),
            &Command::SetTodoTitle {
                id: item.id(),
                title: "Changed".to_string(),
            },
        );
        assert_eq!(
            inverse,
            vec![Command::SetTodoTitle {
                id: item.id(),
                title: "Original".to_string()
            }]
        );

        let inverse = inverse_commands(
            Some(&item),
            &Command::SetTodoNotes {
                id: item.id(),
                notes: "new".to_string(),
            },
        );
        assert_eq!(
            inverse,
            vec![Command::SetTodoNotes {
                id: item.id(),
                notes: "old notes".to_string()
            }]
        );

        let inverse = inverse_commands(
            Some(&item),
            &Command::ScheduleTodo {
                id: item.id(),
                schedule: ScheduleInput::Someday,
            },
        );
        assert_eq!(
            inverse,
            vec![Command::ScheduleTodo {
                id: item.id(),
                schedule: ScheduleInput::On(LocalDate::from_ymd(2026, 6, 5).expect("date"))
            }]
        );

        let inverse = inverse_commands(
            Some(&item),
            &Command::SetTodoDeadline {
                id: item.id(),
                deadline: None,
            },
        );
        assert_eq!(
            inverse,
            vec![Command::SetTodoDeadline {
                id: item.id(),
                deadline: Some(LocalDate::from_ymd(2026, 6, 6).expect("date"))
            }]
        );

        let inverse = inverse_commands(
            Some(&item),
            &Command::SetTodoReminder {
                id: item.id(),
                reminder: None,
            },
        );
        assert!(matches!(
            inverse.as_slice(),
            [Command::SetTodoReminder {
                reminder: Some(_),
                ..
            }]
        ));
    }

    #[test]
    fn inverse_of_status_transitions_uses_pre_state() {
        let mut item = todo();
        assert_eq!(
            inverse_commands(Some(&item), &Command::CompleteTodo { id: item.id() }),
            vec![Command::ReopenTodo { id: item.id() }]
        );

        item.cancel(now());
        assert_eq!(
            inverse_commands(Some(&item), &Command::ReopenTodo { id: item.id() }),
            vec![Command::CancelTodo { id: item.id() }]
        );

        let mut completed = todo();
        completed.complete(now());
        assert_eq!(
            inverse_commands(
                Some(&completed),
                &Command::ReopenTodo { id: completed.id() }
            ),
            vec![Command::CompleteTodo { id: completed.id() }]
        );
    }

    #[test]
    fn inverse_of_move_and_delete() {
        let item = todo();
        let project = EntityId::new();
        let mut moved = item.clone();
        moved.move_to(ContextRef::Project(project), now());

        assert_eq!(
            inverse_commands(
                Some(&moved),
                &Command::MoveTodo {
                    id: moved.id(),
                    context: ContextRef::Inbox
                }
            ),
            vec![Command::MoveTodo {
                id: moved.id(),
                context: ContextRef::Project(project)
            }]
        );
        assert_eq!(
            inverse_commands(Some(&item), &Command::DeleteTodo { id: item.id() }),
            vec![Command::RestoreTodo { id: item.id() }]
        );
        assert_eq!(
            inverse_commands(Some(&item), &Command::RestoreTodo { id: item.id() }),
            vec![Command::DeleteTodo { id: item.id() }]
        );
    }

    #[test]
    fn inverse_of_create_requires_the_created_identity() {
        assert!(inverse_commands(None, &create_command()).is_empty());
        let created = EntityId::new();
        assert_eq!(
            inverse_of_create(created),
            vec![Command::DeleteTodo { id: created }]
        );
    }

    fn create_command() -> Command {
        Command::CreateTodo {
            title: "New".to_string(),
            notes: None,
            context: ContextRef::Inbox,
            schedule: ScheduleInput::Anytime,
            deadline: None,
            reminder: None,
        }
    }

    #[test]
    fn stack_is_bounded_and_clears_redo_on_push() {
        let mut stack = UndoStack::with_window(2);
        for index in 0..3 {
            stack.push(UndoGroup {
                id: Uuid::now_v7(),
                label: format!("group {index}"),
                entries: vec![UndoEntry {
                    command: create_command(),
                    inverse: vec![],
                }],
                created_at: now(),
            });
        }
        assert_eq!(stack.undo_depth(), 2);
        let group = stack.pop_undo().expect("group");
        assert_eq!(group.label, "group 2");

        stack.push_redo(group);
        assert_eq!(stack.redo_depth(), 1);
        stack.push(UndoGroup {
            id: Uuid::now_v7(),
            label: "new".to_string(),
            entries: vec![UndoEntry {
                command: create_command(),
                inverse: vec![],
            }],
            created_at: now(),
        });
        assert_eq!(stack.redo_depth(), 0, "a new action clears redo history");
    }

    #[test]
    fn empty_groups_are_not_recorded() {
        let mut stack = UndoStack::new();
        stack.push(UndoGroup {
            id: Uuid::now_v7(),
            label: "empty".to_string(),
            entries: vec![],
            created_at: now(),
        });
        assert_eq!(stack.undo_depth(), 0);
        assert!(stack.pop_undo().is_none());
    }
}
