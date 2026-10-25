//! Normalized client-side workspace state.

use std::collections::HashMap;

use leptos::prelude::*;
use momentum_app::areas::AreaSummary;
use momentum_app::projects::ProjectSummary;
use momentum_app::tags::TagSummary;
use momentum_app::todos::TodoSummary;
use momentum_domain::EntityId;

use crate::protocol::projection::{ProjectionKey, ProjectionSnapshot};
use crate::protocol::sidebar::SidebarSnapshot;

/// Client-side lifecycle of one cached projection.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProjectionState {
    /// Most recently received normalized projection payload.
    pub snapshot: Option<ProjectionSnapshot>,
    /// Whether a fetch is currently in progress.
    pub loading: bool,
    /// Whether a command invalidated the cached value.
    pub stale: bool,
}

/// Normalized workspace data shared by hydrated feature components.
#[derive(Clone)]
pub struct ClientStore {
    /// To-do entities, indexed by stable identity.
    pub todos: RwSignal<HashMap<EntityId, TodoSummary>>,
    /// Project entities, indexed by stable identity.
    pub projects: RwSignal<HashMap<EntityId, ProjectSummary>>,
    /// Area entities, indexed by stable identity.
    pub areas: RwSignal<HashMap<EntityId, AreaSummary>>,
    /// Tag entities, indexed by stable identity.
    pub tags: RwSignal<HashMap<EntityId, TagSummary>>,
    /// Cached projection membership and load state.
    pub projections: RwSignal<HashMap<ProjectionKey, ProjectionState>>,
    /// Last loaded sidebar hierarchy.
    pub sidebar: RwSignal<Option<SidebarSnapshot>>,
    /// Highest server change sequence observed by this client.
    pub server_sequence: RwSignal<i64>,
}

impl Default for ClientStore {
    fn default() -> Self {
        Self {
            todos: RwSignal::new(HashMap::new()),
            projects: RwSignal::new(HashMap::new()),
            areas: RwSignal::new(HashMap::new()),
            tags: RwSignal::new(HashMap::new()),
            projections: RwSignal::new(HashMap::new()),
            sidebar: RwSignal::new(None),
            server_sequence: RwSignal::new(0),
        }
    }
}

impl ClientStore {
    /// Creates and provides the workspace store to the current Leptos owner.
    pub fn provide() -> Self {
        let store = Self::default();
        provide_context(store.clone());
        store
    }

    /// Returns the provided workspace store.
    ///
    /// # Panics
    ///
    /// Panics when used outside [`ClientStore::provide`].
    #[must_use]
    pub fn use_store() -> Self {
        use_context::<Self>().expect("ClientStore is provided by App")
    }

    /// Merges a projection snapshot into the normalized entity and membership stores.
    pub fn merge_projection(&self, snapshot: ProjectionSnapshot) {
        let key = snapshot.key;

        self.todos.update(|todos| {
            for todo in &snapshot.todos {
                todos.insert(todo.id, todo.clone());
            }
        });
        self.projections.update(|projections| {
            projections.insert(
                key,
                ProjectionState {
                    snapshot: Some(snapshot),
                    loading: false,
                    stale: false,
                },
            );
        });
    }

    /// Inserts or replaces one normalized to-do entity.
    pub fn upsert_todo(&self, todo: TodoSummary) {
        self.todos.update(|todos| {
            todos.insert(todo.id, todo);
        });
    }

    /// Marks projections affected by an authoritative command as stale.
    pub fn invalidate(&self, projections: &[momentum_app::commands::Projection]) {
        self.projections.update(|states| {
            for projection in projections {
                let key = match projection {
                    momentum_app::commands::Projection::Inbox => Some(ProjectionKey::Inbox),
                    momentum_app::commands::Projection::Today => Some(ProjectionKey::Today),
                    momentum_app::commands::Projection::Upcoming => Some(ProjectionKey::Upcoming),
                    momentum_app::commands::Projection::Anytime => Some(ProjectionKey::Anytime),
                    momentum_app::commands::Projection::Someday => Some(ProjectionKey::Someday),
                    momentum_app::commands::Projection::Logbook => Some(ProjectionKey::Logbook),
                    momentum_app::commands::Projection::Deadlines => Some(ProjectionKey::Deadlines),
                    momentum_app::commands::Projection::Project(id) => {
                        Some(ProjectionKey::Project(*id))
                    }
                    momentum_app::commands::Projection::Area(id) => Some(ProjectionKey::Area(*id)),
                    momentum_app::commands::Projection::Trash
                    | momentum_app::commands::Projection::Repeating
                    | momentum_app::commands::Projection::AllProjects
                    | momentum_app::commands::Projection::LoggedProjects => None,
                };
                if let Some(key) = key {
                    states.entry(key).or_default().stale = true;
                }
            }
        });
    }
}
