//! Client command dispatch, optimistic completion, and rollback state.

use std::collections::{BTreeMap, HashMap};

use chrono::Utc;
use leptos::prelude::*;
use leptos::task::spawn_local;
use momentum_app::commands::Command;
use momentum_app::todos::TodoSummary;
use momentum_domain::model::Status;
use momentum_domain::EntityId;
use uuid::Uuid;

use crate::protocol::mutation::UiCommandResponse;
use crate::protocol::projection::{ProjectionKey, ProjectionRow};
use crate::server_fn::commands::execute_ui_command;
use crate::state::client_store::{ClientStore, ProjectionState};

/// Local state captured before applying an optimistic command.
#[derive(Debug, Clone, Default)]
pub struct RollbackState {
    todo_id: Option<EntityId>,
    todo: Option<TodoSummary>,
    projections: HashMap<ProjectionKey, ProjectionState>,
}

/// A command awaiting authoritative confirmation from the server.
#[derive(Debug, Clone)]
pub struct PendingMutation {
    /// Stable operation identity used by server-side idempotency.
    pub operation_id: Uuid,
    /// Command submitted to the application layer.
    pub command: Command,
    /// State restored if the command fails.
    pub rollback: RollbackState,
}

/// Client-side command dispatcher with optimistic todo lifecycle updates.
#[derive(Clone)]
pub struct ClientCommandBus {
    store: ClientStore,
    pending: RwSignal<BTreeMap<Uuid, PendingMutation>>,
    /// Most recent user-visible mutation failure.
    pub last_error: RwSignal<Option<String>>,
}

impl ClientCommandBus {
    /// Creates and provides the command bus for the application lifetime.
    pub fn provide(store: ClientStore) -> Self {
        let bus = Self {
            store,
            pending: RwSignal::new(BTreeMap::new()),
            last_error: RwSignal::new(None),
        };
        provide_context(bus.clone());
        bus
    }

    /// Returns the command bus provided by [`ClientCommandBus::provide`].
    ///
    /// # Panics
    ///
    /// Panics when called outside the application root.
    #[must_use]
    pub fn use_bus() -> Self {
        use_context::<Self>().expect("ClientCommandBus is provided by App")
    }

    /// Dispatches a command through the authenticated Leptos server function.
    ///
    /// Completion and reopen actions update cached entities and list membership immediately.
    /// The supplied callback can keep a control in sync with rollback if the server rejects it.
    /// A second lifecycle mutation for the same todo is ignored until its first request settles.
    pub fn dispatch(
        &self,
        command: Command,
        on_complete: Option<Callback<Result<UiCommandResponse, String>>>,
    ) -> Option<Uuid> {
        let rollback = self.capture_rollback(&command);
        if let Some(todo_id) = rollback.todo_id {
            let already_pending = self.pending.get_untracked().values().any(|mutation| {
                optimistic_todo_status(&mutation.command)
                    .is_some_and(|(pending_todo, _)| pending_todo == todo_id)
            });
            if already_pending {
                return None;
            }
        }

        let request = crate::server_fn::commands::new_ui_command_request(command.clone());
        let operation_id = request.operation_id;
        self.last_error.set(None);
        self.apply_optimistic(&command);
        self.pending.update(|pending| {
            pending.insert(
                operation_id,
                PendingMutation {
                    operation_id,
                    command: command.clone(),
                    rollback: rollback.clone(),
                },
            );
        });

        let bus = self.clone();
        spawn_local(async move {
            let result = execute_ui_command(request)
                .await
                .map_err(|error| error.to_string());

            bus.pending.update(|pending| {
                pending.remove(&operation_id);
            });

            match result {
                Ok(response) => {
                    bus.store
                        .server_sequence
                        .update(|sequence| *sequence = (*sequence).max(response.sequence));
                    bus.store.invalidate(&response.invalidated);
                    if let Some(callback) = on_complete {
                        callback.run(Ok(response));
                    }
                }
                Err(error) => {
                    bus.restore(rollback);
                    bus.last_error.set(Some(error.clone()));
                    if let Some(callback) = on_complete {
                        callback.run(Err(error));
                    }
                }
            }
        });

        Some(operation_id)
    }

    fn capture_rollback(&self, command: &Command) -> RollbackState {
        let Some((todo_id, _)) = optimistic_todo_status(command) else {
            return RollbackState::default();
        };

        let todo = self.store.todos.get_untracked().get(&todo_id).cloned();
        let projections =
            self.store
                .projections
                .get_untracked()
                .into_iter()
                .filter(|(_, state)| {
                    state.snapshot.as_ref().is_some_and(|snapshot| {
                        snapshot.todos.iter().any(|todo| todo.id == todo_id)
                            || snapshot.sections.iter().any(|section| {
                                section.rows.iter().any(
                                    |row| matches!(row, ProjectionRow::Todo(id) if *id == todo_id),
                                )
                            })
                    })
                })
                .collect();

        RollbackState {
            todo_id: Some(todo_id),
            todo,
            projections,
        }
    }

    fn apply_optimistic(&self, command: &Command) {
        let Some((todo_id, status)) = optimistic_todo_status(command) else {
            return;
        };

        self.store.todos.update(|todos| {
            if let Some(todo) = todos.get_mut(&todo_id) {
                todo.status = status;
                todo.completed_at = (status == Status::Completed).then(Utc::now);
                if status == Status::Open {
                    todo.canceled_at = None;
                }
            }
        });

        self.store.projections.update(|projections| {
            for (key, state) in projections {
                let should_remove = match status {
                    Status::Completed => *key != ProjectionKey::Logbook,
                    Status::Open => *key == ProjectionKey::Logbook,
                    Status::Canceled | Status::Deleted => false,
                };
                if !should_remove {
                    continue;
                }
                let Some(snapshot) = state.snapshot.as_mut() else {
                    continue;
                };
                snapshot.todos.retain(|todo| todo.id != todo_id);
                for section in &mut snapshot.sections {
                    section
                        .rows
                        .retain(|row| !matches!(row, ProjectionRow::Todo(id) if *id == todo_id));
                }
            }
        });
    }

    fn restore(&self, rollback: RollbackState) {
        if let Some(todo_id) = rollback.todo_id {
            self.store.todos.update(|todos| {
                if let Some(todo) = rollback.todo {
                    todos.insert(todo_id, todo);
                }
            });
            self.store.projections.update(|projections| {
                for (key, state) in rollback.projections {
                    projections.insert(key, state);
                }
            });
        }
    }
}

fn optimistic_todo_status(command: &Command) -> Option<(EntityId, Status)> {
    match command {
        Command::CompleteTodo { id } => Some((*id, Status::Completed)),
        Command::ReopenTodo { id } => Some((*id, Status::Open)),
        _ => None,
    }
}
