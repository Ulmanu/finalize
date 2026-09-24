//! In-memory to-do read store for tests.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::todos::{TodoReadStore, TodoScope, TodoSummary};
use momentum_domain::model::{ScheduleState, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};

use crate::commands::MemoryDatabase;

/// Read store over a [`MemoryDatabase`].
#[derive(Debug, Clone)]
pub struct MemoryTodoReadStore {
    database: MemoryDatabase,
}

impl MemoryTodoReadStore {
    /// Creates the read store.
    #[must_use]
    pub fn new(database: MemoryDatabase) -> Self {
        Self { database }
    }
}

fn matches_scope(todo: &TodoSummary, scope: TodoScope, today: LocalDate) -> bool {
    match scope {
        TodoScope::Inbox => {
            todo.status == Status::Open
                && todo.context.is_inbox()
                && match todo.schedule.state() {
                    ScheduleState::Anytime => true,
                    ScheduleState::On(date) => date <= today,
                    ScheduleState::Someday => false,
                }
        }
        TodoScope::AllOpen => todo.status == Status::Open,
        TodoScope::Logbook => todo.status.is_terminal(),
        TodoScope::Trash => todo.status == Status::Deleted,
    }
}

#[async_trait]
impl TodoReadStore for MemoryTodoReadStore {
    async fn list_todos(
        &self,
        account_id: AccountId,
        scope: TodoScope,
        today: LocalDate,
    ) -> Result<Vec<TodoSummary>, CommandError> {
        let mut todos = self.database.todos_for(account_id);
        todos.retain(|todo| matches_scope(todo, scope, today));
        if scope == TodoScope::Inbox {
            todos.sort_by_key(|todo| {
                (
                    self.database
                        .context_rank(account_id, todo.id)
                        .map_or(i64::MAX, |rank| rank.value()),
                    todo.created_at,
                )
            });
        } else {
            todos.sort_by_key(|todo| todo.created_at);
        }
        Ok(todos)
    }

    async fn get_todo(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<TodoSummary>, CommandError> {
        Ok(self.database.todo(account_id, id).map(|todo| TodoSummary {
            id: todo.id(),
            title: todo.title.as_str().to_string(),
            notes: todo.notes,
            status: todo.status,
            schedule: todo.schedule,
            deadline: todo.deadline,
            reminder: todo.reminder,
            context: todo.context,
            completed_at: todo.completed_at,
            created_at: todo.meta.created_at,
            version: todo.meta.version,
            checklist: todo
                .checklist
                .iter()
                .map(|item| momentum_app::todos::ChecklistItemSummary {
                    id: item.id,
                    title: item.title.as_str().to_string(),
                    completed: item.completed,
                })
                .collect(),
        }))
    }
}
