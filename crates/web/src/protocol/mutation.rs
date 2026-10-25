//! First-party UI mutation request and response types.

use momentum_app::commands::{Command, Projection};
use momentum_domain::EntityId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A command submitted by the hydrated first-party UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiCommandRequest {
    /// Stable operation identity reused when retrying a request.
    pub operation_id: Uuid,
    /// Typed application command.
    pub command: Command,
}

/// Minimal authoritative result needed by the client after a mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiCommandResponse {
    /// Entity created or changed by the command, when applicable.
    pub entity_id: Option<EntityId>,
    /// Projections that need refreshing.
    pub invalidated: Vec<Projection>,
    /// Per-account change-log cursor assigned to the operation.
    pub sequence: i64,
    /// Whether the server returned an idempotent replay.
    pub replayed: bool,
}
