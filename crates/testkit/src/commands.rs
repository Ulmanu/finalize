//! In-memory unit of work for deterministic command-layer tests.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use momentum_app::commands::{
    AreaChildren, ChangeRecord, CommandError, ProjectChildren, UnitOfWork, UnitOfWorkFactory,
};
use momentum_app::ordering::{OrderedItem, OrderingScope, Rank, ScopeOrder};
use momentum_domain::model::{Area, ContextRef, Heading, Project, Todo};
use momentum_domain::{AccountId, EntityId};
use uuid::Uuid;

#[derive(Default)]
struct MemoryState {
    todos: HashMap<(AccountId, EntityId), Todo>,
    areas: HashMap<(AccountId, EntityId), Area>,
    projects: HashMap<(AccountId, EntityId), Project>,
    headings: HashMap<(AccountId, EntityId), Heading>,
    project_areas: HashMap<(AccountId, EntityId), EntityId>,
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

    /// Returns a stored heading.
    #[must_use]
    pub fn heading(&self, account_id: AccountId, id: EntityId) -> Option<Heading> {
        self.state
            .lock()
            .expect("memory state lock")
            .headings
            .get(&(account_id, id))
            .cloned()
    }

    /// Returns every heading stored for an account.
    #[must_use]
    pub fn headings_for(&self, account_id: AccountId) -> Vec<Heading> {
        self.state
            .lock()
            .expect("memory state lock")
            .headings
            .iter()
            .filter(|((account, _), _)| *account == account_id)
            .map(|(_, heading)| heading.clone())
            .collect()
    }

    /// Returns a stored project.
    #[must_use]
    pub fn project(&self, account_id: AccountId, id: EntityId) -> Option<Project> {
        self.state
            .lock()
            .expect("memory state lock")
            .projects
            .get(&(account_id, id))
            .cloned()
    }

    /// Returns every stored project for an account.
    #[must_use]
    pub fn projects_for(&self, account_id: AccountId) -> Vec<Project> {
        let state = self.state.lock().expect("memory state lock");
        let mut projects: Vec<Project> = state
            .projects
            .iter()
            .filter(|((account, _), _)| *account == account_id)
            .map(|(_, project)| project.clone())
            .collect();
        projects.sort_by_key(|project| project.meta.created_at);
        projects
    }

    /// Returns a stored area.
    #[must_use]
    pub fn area(&self, account_id: AccountId, id: EntityId) -> Option<Area> {
        self.state
            .lock()
            .expect("memory state lock")
            .areas
            .get(&(account_id, id))
            .cloned()
    }

    /// Returns every stored area for an account.
    #[must_use]
    pub fn areas_for(&self, account_id: AccountId) -> Vec<Area> {
        let state = self.state.lock().expect("memory state lock");
        let mut areas: Vec<Area> = state
            .areas
            .iter()
            .filter(|((account, _), _)| *account == account_id)
            .map(|(_, area)| area.clone())
            .collect();
        areas.sort_by_key(|area| area.meta.created_at);
        areas
    }

    /// Returns `(project_count, todo_count)` for an area.
    #[must_use]
    pub fn area_children(&self, account_id: AccountId, area_id: EntityId) -> (usize, usize) {
        let state = self.state.lock().expect("memory state lock");
        let todos = state
            .todos
            .iter()
            .filter(|((account, _), todo)| {
                *account == account_id
                    && todo.context == ContextRef::Area(area_id)
                    && todo.status != momentum_domain::model::Status::Deleted
            })
            .count();
        let projects = state
            .project_areas
            .iter()
            .filter(|((account, _), assigned)| *account == account_id && **assigned == area_id)
            .count();
        (projects, todos)
    }

    /// Returns a project's area assignment.
    #[must_use]
    pub fn project_area(&self, account_id: AccountId, project_id: EntityId) -> Option<EntityId> {
        self.state
            .lock()
            .expect("memory state lock")
            .project_areas
            .get(&(account_id, project_id))
            .copied()
    }

    /// Assigns a project to an area (or clears it).
    pub fn assign_project_area(
        &self,
        account_id: AccountId,
        project_id: EntityId,
        area_id: Option<EntityId>,
    ) {
        let mut state = self.state.lock().expect("memory state lock");
        match area_id {
            Some(area_id) => {
                state
                    .project_areas
                    .insert((account_id, project_id), area_id);
            }
            None => {
                state.project_areas.remove(&(account_id, project_id));
            }
        }
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

    async fn load_project(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Project>, CommandError> {
        Ok(self.database.project(account_id, id))
    }

    async fn save_project(
        &mut self,
        account_id: AccountId,
        project: &Project,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        let mut state = self.database.state.lock().expect("memory state lock");
        if let Some(expected) = expected_version {
            let actual = state
                .projects
                .get(&(account_id, project.id()))
                .map_or(0, |current| current.meta.version);
            if actual != expected {
                return Err(CommandError::Conflict { expected, actual });
            }
        }
        state
            .projects
            .insert((account_id, project.id()), project.clone());
        Ok(())
    }

    async fn load_heading(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Heading>, CommandError> {
        Ok(self.database.heading(account_id, id))
    }

    async fn save_heading(
        &mut self,
        account_id: AccountId,
        heading: &Heading,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        let mut state = self.database.state.lock().expect("memory state lock");
        if let Some(expected) = expected_version {
            let actual = state
                .headings
                .get(&(account_id, heading.id()))
                .map_or(0, |current| current.meta.version);
            if actual != expected {
                return Err(CommandError::Conflict { expected, actual });
            }
        }
        state
            .headings
            .insert((account_id, heading.id()), heading.clone());
        Ok(())
    }

    async fn list_project_children(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<ProjectChildren, CommandError> {
        let state = self.database.state.lock().expect("memory state lock");
        let todo_ids = state
            .todos
            .iter()
            .filter(|((account, _), todo)| {
                *account == account_id && todo.context == ContextRef::Project(project_id)
            })
            .map(|((_, id), _)| *id)
            .collect();
        let heading_ids = state
            .headings
            .iter()
            .filter(|((account, _), heading)| {
                *account == account_id && heading.project == project_id
            })
            .map(|((_, id), _)| *id)
            .collect();
        Ok(ProjectChildren {
            todo_ids,
            heading_ids,
        })
    }

    async fn load_area(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Area>, CommandError> {
        Ok(self.database.area(account_id, id))
    }

    async fn save_area(
        &mut self,
        account_id: AccountId,
        area: &Area,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        let mut state = self.database.state.lock().expect("memory state lock");
        if let Some(expected) = expected_version {
            let actual = state
                .areas
                .get(&(account_id, area.id()))
                .map_or(0, |current| current.meta.version);
            if actual != expected {
                return Err(CommandError::Conflict { expected, actual });
            }
        }
        state.areas.insert((account_id, area.id()), area.clone());
        Ok(())
    }

    async fn list_area_children(
        &mut self,
        account_id: AccountId,
        area_id: EntityId,
    ) -> Result<AreaChildren, CommandError> {
        let state = self.database.state.lock().expect("memory state lock");
        let todo_ids = state
            .todos
            .iter()
            .filter(|((account, _), todo)| {
                *account == account_id
                    && todo.context == ContextRef::Area(area_id)
                    && todo.status != momentum_domain::model::Status::Deleted
            })
            .map(|((_, id), _)| *id)
            .collect();
        let project_ids = state
            .project_areas
            .iter()
            .filter(|((account, _), assigned)| *account == account_id && **assigned == area_id)
            .map(|((_, project), _)| *project)
            .collect();
        Ok(AreaChildren {
            todo_ids,
            project_ids,
        })
    }

    async fn project_area(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<Option<Option<EntityId>>, CommandError> {
        Ok(Some(self.database.project_area(account_id, project_id)))
    }

    async fn set_project_area(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
        area_id: Option<EntityId>,
    ) -> Result<(), CommandError> {
        self.database
            .assign_project_area(account_id, project_id, area_id);
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
        if let ContextRef::Project(project_id) = context {
            entities.extend(
                state
                    .headings
                    .iter()
                    .filter_map(|((account, id), heading)| {
                        (*account == account_id && heading.project == project_id)
                            .then_some((heading.meta.created_at, *id))
                    }),
            );
        }
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
