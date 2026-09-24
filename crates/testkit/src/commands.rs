//! In-memory unit of work for deterministic command-layer tests.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use momentum_app::commands::{ChangeRecord, CommandError, UnitOfWork, UnitOfWorkFactory};
use momentum_app::ordering::{OrderedItem, OrderingScope, Rank, ScopeOrder};
use momentum_domain::model::{ContextRef, Todo};
use momentum_domain::{AccountId, EntityId};
use uuid::Uuid;

#[derive(Default)]
struct MemoryState {
    todos: HashMap<(AccountId, EntityId), Todo>,
    changes: Vec<(AccountId, ChangeRecord)>,
    context_ranks: HashMap<(AccountId, EntityId), Rank>,
    next_ranks: HashMap<(AccountId, ContextRef), i64>,
}

/// Shared in-memory database backing command-layer tests.
#[derive(Default)]
pub struct MemoryDatabase {
    state: Arc<Mutex<MemoryState>>,
    fail_next_save: Arc<AtomicBool>,
}

impl std::fmt::Debug for MemoryDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MemoryDatabase")
    }
}

impl Clone for MemoryDatabase {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            fail_next_save: Arc::clone(&self.fail_next_save),
        }
    }
}

impl MemoryDatabase {
    /// Creates an empty database.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Makes the next `save_todo` fail, simulating a persistence error.
    pub fn fail_next_save(&self) {
        self.fail_next_save.store(true, Ordering::SeqCst);
    }

    /// Returns a stored to-do.
    #[must_use]
    pub fn todo(&self, account_id: AccountId, id: EntityId) -> Option<Todo> {
        self.state
            .lock()
            .expect("memory state lock")
            .todos
            .get(&(account_id, id))
            .cloned()
    }

    /// Returns an entity's context-scope rank, when assigned.
    #[must_use]
    pub fn context_rank(&self, account_id: AccountId, id: EntityId) -> Option<Rank> {
        self.state
            .lock()
            .expect("memory state lock")
            .context_ranks
            .get(&(account_id, id))
            .copied()
    }

    /// Returns every stored to-do for an account as read models.
    #[must_use]
    pub fn todos_for(&self, account_id: AccountId) -> Vec<momentum_app::todos::TodoSummary> {
        let state = self.state.lock().expect("memory state lock");
        let mut todos: Vec<&Todo> = state
            .todos
            .iter()
            .filter(|((account, _), _)| *account == account_id)
            .map(|(_, todo)| todo)
            .collect();
        todos.sort_by_key(|todo| (todo.meta.created_at, todo.id()));
        todos
            .into_iter()
            .map(|todo| momentum_app::todos::TodoSummary {
                id: todo.id(),
                title: todo.title.as_str().to_string(),
                notes: todo.notes.clone(),
                status: todo.status,
                schedule: todo.schedule,
                deadline: todo.deadline,
                reminder: todo.reminder.clone(),
                context: todo.context,
                completed_at: todo.completed_at,
                created_at: todo.meta.created_at,
                version: todo.meta.version,
            })
            .collect()
    }

    /// Returns the change records for an account in sequence order.
    #[must_use]
    pub fn changes(&self, account_id: AccountId) -> Vec<ChangeRecord> {
        let state = self.state.lock().expect("memory state lock");
        let mut records: Vec<ChangeRecord> = state
            .changes
            .iter()
            .filter(|(account, _)| *account == account_id)
            .map(|(_, record)| record.clone())
            .collect();
        records.sort_by_key(|record| record.sequence);
        records
    }

    fn next_sequence(&self, account_id: AccountId) -> i64 {
        let state = self.state.lock().expect("memory state lock");
        state
            .changes
            .iter()
            .filter(|(account, _)| *account == account_id)
            .map(|(_, record)| record.sequence)
            .max()
            .unwrap_or(0)
            + 1
    }
}

#[async_trait]
impl UnitOfWorkFactory for MemoryDatabase {
    async fn begin(&self, account_id: AccountId) -> Result<Box<dyn UnitOfWork>, CommandError> {
        Ok(Box::new(MemoryUnitOfWork {
            database: self.clone(),
            account_id,
            staged_saves: Vec::new(),
            staged_changes: Vec::new(),
        }))
    }
}

struct MemoryUnitOfWork {
    database: MemoryDatabase,
    account_id: AccountId,
    staged_saves: Vec<(EntityId, Todo, Option<i64>)>,
    staged_changes: Vec<ChangeRecord>,
}

impl std::fmt::Debug for MemoryUnitOfWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MemoryUnitOfWork")
    }
}

#[async_trait]
impl UnitOfWork for MemoryUnitOfWork {
    async fn load_todo(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Todo>, CommandError> {
        Ok(self.database.todo(account_id, id))
    }

    async fn save_todo(
        &mut self,
        account_id: AccountId,
        todo: &Todo,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        if self.database.fail_next_save.swap(false, Ordering::SeqCst) {
            return Err(CommandError::Storage("injected failure".to_string()));
        }
        if let Some(expected) = expected_version {
            let current = self.database.todo(account_id, todo.id());
            let actual = current.map_or(0, |current| current.meta.version);
            if actual != expected {
                return Err(CommandError::Conflict { expected, actual });
            }
        }
        self.staged_saves
            .push((todo.id(), todo.clone(), expected_version));
        Ok(())
    }

    async fn load_context_order(
        &mut self,
        account_id: AccountId,
        context: ContextRef,
    ) -> Result<Vec<(EntityId, Rank)>, CommandError> {
        let mut state = self.database.state.lock().expect("memory state lock");
        let mut entities: Vec<(chrono::DateTime<chrono::Utc>, EntityId)> = state
            .todos
            .iter()
            .filter(|((account, _), todo)| {
                *account == account_id
                    && todo.context == context
                    && todo.status != momentum_domain::model::Status::Deleted
            })
            .map(|((_, id), todo)| (todo.meta.created_at, *id))
            .collect();
        entities.sort_by_key(|(created_at, id)| (*created_at, *id));
        let entities: Vec<EntityId> = entities.into_iter().map(|(_, id)| id).collect();

        let mut next = *state.next_ranks.get(&(account_id, context)).unwrap_or(&0);
        for entity in &entities {
            state
                .context_ranks
                .entry((account_id, *entity))
                .or_insert_with(|| {
                    next += momentum_app::ordering::STEP;
                    Rank::new(next).unwrap_or_else(|_| Rank::start())
                });
        }
        state.next_ranks.insert((account_id, context), next);

        let mut order = ScopeOrder::new(OrderingScope::Context(context));
        for entity in entities {
            if let Some(rank) = state.context_ranks.get(&(account_id, entity)) {
                order.items_mut().push(OrderedItem {
                    entity_id: entity,
                    rank: *rank,
                });
            }
        }
        order.sort_by_rank();
        Ok(order
            .items()
            .iter()
            .map(|item| (item.entity_id, item.rank))
            .collect())
    }

    async fn set_context_rank(
        &mut self,
        account_id: AccountId,
        entity_id: EntityId,
        rank: Rank,
    ) -> Result<(), CommandError> {
        self.database
            .state
            .lock()
            .expect("memory state lock")
            .context_ranks
            .insert((account_id, entity_id), rank);
        Ok(())
    }

    async fn change_by_operation(
        &mut self,
        account_id: AccountId,
        operation_id: Uuid,
    ) -> Result<Option<ChangeRecord>, CommandError> {
        Ok(self
            .database
            .changes(account_id)
            .into_iter()
            .find(|record| record.operation_id == operation_id))
    }

    async fn append_change(
        &mut self,
        account_id: AccountId,
        record: &ChangeRecord,
    ) -> Result<ChangeRecord, CommandError> {
        let mut stored = record.clone();
        stored.sequence = self.database.next_sequence(account_id);
        self.staged_changes.push(stored.clone());
        Ok(stored)
    }

    async fn commit(self: Box<Self>) -> Result<(), CommandError> {
        let mut state = self.database.state.lock().expect("memory state lock");
        for (id, todo, expected_version) in self.staged_saves {
            if let Some(expected) = expected_version {
                let actual = state
                    .todos
                    .get(&(self.account_id, id))
                    .map_or(0, |current| current.meta.version);
                if actual != expected {
                    return Err(CommandError::Conflict { expected, actual });
                }
            }
            state.todos.insert((self.account_id, id), todo);
        }
        for record in self.staged_changes {
            state.changes.push((self.account_id, record));
        }
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), CommandError> {
        Ok(())
    }
}
