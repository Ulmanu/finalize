//! Root-owned overlay state and host component.

use leptos::prelude::*;
use momentum_domain::EntityId;

use crate::state::pending::ClientCommandBus;

/// A semantic reference to an element used to position a popover.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementAnchor(pub String);

/// Screen coordinates used for pointer-positioned menus.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenPoint {
    /// Horizontal viewport coordinate.
    pub x: f64,
    /// Vertical viewport coordinate.
    pub y: f64,
}

/// Current application overlay.
#[derive(Debug, Clone, PartialEq)]
pub enum Overlay {
    /// No overlay is open.
    None,
    /// Quick Find dialog.
    QuickFind,
    /// Quick Entry dialog.
    QuickEntry,
    /// Schedule picker for one to-do.
    When {
        todo: EntityId,
        anchor: ElementAnchor,
    },
    /// Deadline picker for one to-do.
    Deadline {
        todo: EntityId,
        anchor: ElementAnchor,
    },
    /// Tag picker for one or more entities.
    Tags {
        entities: Vec<EntityId>,
        anchor: ElementAnchor,
    },
    /// Destination picker for one or more entities.
    Move { entities: Vec<EntityId> },
    /// Context menu positioned at a pointer location.
    ContextMenu {
        target: ContextTarget,
        point: ScreenPoint,
    },
}

/// Entity category used as the target of a context menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextTarget {
    /// A to-do row.
    Todo(EntityId),
    /// A project row.
    Project(EntityId),
    /// An area row.
    Area(EntityId),
}

/// Reactive state for the single root overlay host.
#[derive(Clone)]
pub struct OverlayState {
    /// Currently active overlay.
    pub active: RwSignal<Overlay>,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            active: RwSignal::new(Overlay::None),
        }
    }
}

impl OverlayState {
    /// Creates and provides the overlay state to the current Leptos owner.
    pub fn provide() -> Self {
        let state = Self::default();
        provide_context(state.clone());
        state
    }

    /// Returns the provided overlay state.
    ///
    /// # Panics
    ///
    /// Panics when used outside [`OverlayState::provide`].
    #[must_use]
    pub fn use_state() -> Self {
        use_context::<Self>().expect("OverlayState is provided by App")
    }
}

/// Root mount point for application overlays.
#[component]
pub fn OverlayHost() -> impl IntoView {
    let active = OverlayState::use_state().active;
    let last_error = ClientCommandBus::use_bus().last_error;
    view! {
        <div
            id="overlay-host"
            class="overlay-host"
            data-overlay-host="true"
            data-active-overlay=move || match active.get() {
                Overlay::None => "none",
                Overlay::QuickFind => "quick-find",
                Overlay::QuickEntry => "quick-entry",
                Overlay::When { .. } => "when",
                Overlay::Deadline { .. } => "deadline",
                Overlay::Tags { .. } => "tags",
                Overlay::Move { .. } => "move",
                Overlay::ContextMenu { .. } => "context-menu",
            }
            aria-live="polite"
        >
            {move || {
                last_error
                    .get()
                    .map(|message| {
                        view! {
                            <div class="command-error" role="alert" data-testid="command-error">
                                {message}
                            </div>
                        }
                    })
            }}
        </div>
    }
}
