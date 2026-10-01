//! Cloud sync protocol: incremental push/pull, monotonic cursors, idempotent operation ids, and
//! explicit conflict rules.
//!
//! Clients push operations carrying their stable operation ids (deduplicated by the command
//! dispatcher) and pull changes after a cursor. Conflicts are never resolved by generic
//! destructive last-write-wins: each failure is classified into an explicit domain rule with a
//! documented resolution.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_domain::{AccountId, EntityId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::Clock;
use crate::commands::{
    ChangeRecord, Command, CommandDispatcher, CommandEnvelope, CommandError, DomainEvent,
};
use crate::realtime::HintHub;
use crate::todos::TodoService;

/// One client operation in a push batch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncOperation {
    /// Stable client operation id reused across retries.
    pub operation_id: Uuid,
    /// The command to execute.
    pub command: Command,
}

/// A push/pull request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncRequest {
    /// Last change-log sequence the client already holds.
    pub cursor: i64,
    /// Outbound operations, applied in order.
    pub operations: Vec<SyncOperation>,
}

/// Acknowledgement of one pushed operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationAck {
    /// Client operation id.
    pub operation_id: Uuid,
    /// Entity the command created or changed.
    pub entity_id: Option<EntityId>,
    /// Change-log sequence assigned to this operation.
    pub sequence: i64,
    /// Whether the operation was deduplicated as an earlier replay.
    pub replayed: bool,
}

/// One server change after the cursor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeEntry {
    /// Monotonic per-account sequence.
    pub sequence: i64,
    /// Operation that produced the change.
    pub operation_id: Uuid,
    /// Entity the change concerns.
    pub entity_id: Option<EntityId>,
    /// Emitted domain events.
    pub events: Vec<DomainEvent>,
    /// Record creation instant.
    pub created_at: DateTime<Utc>,
}

impl ChangeEntry {
    /// Converts a persisted change record into a wire entry.
    #[must_use]
    pub fn from_record(record: ChangeRecord) -> Self {
        Self {
            sequence: record.sequence,
            operation_id: record.operation_id,
            entity_id: record.entity_id,
            events: record.events,
            created_at: record.created_at,
        }
    }
}

/// Explicit domain rules used to classify sync conflicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictRule {
    /// Completing is monotonic: replaying a completion is an idempotent no-op.
    CompletionIsMonotonic,
    /// Delete beats edit: edits to deleted entities never resurrect them.
    DeleteBeatsEdit,
    /// Field edits resolve by server version (client refreshes).
    FieldVersion,
    /// Ordering is relative: the server rank is kept and the client refreshes.
    OrderingRelative,
    /// The operation was rejected for another reason.
    Rejected,
}

impl ConflictRule {
    /// Stable identifier used by clients and tests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompletionIsMonotonic => "completion_is_monotonic",
            Self::DeleteBeatsEdit => "delete_beats_edit",
            Self::FieldVersion => "field_version",
            Self::OrderingRelative => "ordering_relative",
            Self::Rejected => "rejected",
        }
    }
}

/// What the client should do about a conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// The server state wins; the client refreshes.
    ServerWins,
    /// The client must refresh its local replica before retrying.
    ClientRefreshes,
    /// The operation is ignored by design (ordering, monotonic no-ops).
    Ignored,
}

impl ConflictResolution {
    /// Stable identifier used by clients and tests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServerWins => "server_wins",
            Self::ClientRefreshes => "client_refreshes",
            Self::Ignored => "ignored",
        }
    }
}

/// One classified conflict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflict {
    /// Client operation id that failed.
    pub operation_id: Uuid,
    /// Entity concerned, when known.
    pub entity_id: Option<EntityId>,
    /// Domain rule applied.
    pub rule: ConflictRule,
    /// Resolution guidance.
    pub resolution: ConflictResolution,
    /// Human-readable detail.
    pub detail: String,
}

/// A push/pull response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncResponse {
    /// New cursor after applying pulled changes.
    pub cursor: i64,
    /// Acknowledgements in push order.
    pub acknowledged: Vec<OperationAck>,
    /// Server changes after the request cursor, ascending.
    pub changes: Vec<ChangeEntry>,
    /// Classified conflicts in push order.
    pub conflicts: Vec<SyncConflict>,
}

/// Read port over the change log.
#[async_trait]
pub trait ChangeLogReader: Send + Sync {
    /// Lists changes with a sequence greater than `cursor`, ascending.
    async fn changes_after(
        &self,
        account_id: AccountId,
        cursor: i64,
    ) -> Result<Vec<ChangeRecord>, CommandError>;
}

/// Push/pull sync endpoint logic.
pub struct SyncService {
    dispatcher: Arc<CommandDispatcher>,
    todos: Arc<TodoService>,
    changes: Arc<dyn ChangeLogReader>,
    clock: Arc<dyn Clock>,
    hints: Option<Arc<HintHub>>,
}

impl std::fmt::Debug for SyncService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SyncService")
    }
}

impl SyncService {
    /// Creates the service.
    #[must_use]
    pub fn new(
        dispatcher: Arc<CommandDispatcher>,
        todos: Arc<TodoService>,
        changes: Arc<dyn ChangeLogReader>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            dispatcher,
            todos,
            changes,
            clock,
            hints: None,
        }
    }

    /// Publishes coalesced invalidation hints after successful pushes.
    #[must_use]
    pub fn with_hints(mut self, hints: Arc<HintHub>) -> Self {
        self.hints = Some(hints);
        self
    }

    /// Applies pushed operations and returns changes after the client cursor.
    ///
    /// # Errors
    ///
    /// Propagates change-log read failures; per-operation failures become classified conflicts
    /// instead of failing the whole batch.
    pub async fn sync(
        &self,
        account_id: AccountId,
        request: SyncRequest,
    ) -> Result<SyncResponse, CommandError> {
        let mut acknowledged = Vec::new();
        let mut conflicts = Vec::new();
        for operation in request.operations {
            // Delete beats edit: an edit pushed against a trashed entity is classified rather
            // than silently mutating the tombstoned record.
            if let Some(entity) = operation.command.target() {
                if let Ok(Some(summary)) = self.todos.get(account_id, entity).await {
                    if summary.status == momentum_domain::model::Status::Deleted {
                        conflicts.push(classify_conflict(
                            operation.operation_id,
                            Some(entity),
                            &operation.command,
                            &CommandError::NotFound,
                        ));
                        continue;
                    }
                }
            }
            let envelope = CommandEnvelope {
                operation_id: operation.operation_id,
                account_id,
                issued_at: self.clock.now(),
                command: operation.command.clone(),
            };
            match self.dispatcher.execute(envelope).await {
                Ok(outcome) => {
                    if let Some(hints) = &self.hints {
                        hints.publish(account_id, outcome.sequence);
                    }
                    acknowledged.push(OperationAck {
                        operation_id: outcome.operation_id,
                        entity_id: outcome.entity_id,
                        sequence: outcome.sequence,
                        replayed: outcome.replayed,
                    });
                }
                Err(error) => conflicts.push(classify_conflict(
                    operation.operation_id,
                    operation.command.target(),
                    &operation.command,
                    &error,
                )),
            }
        }
        let changes = self
            .changes
            .changes_after(account_id, request.cursor)
            .await?;
        let cursor = changes
            .iter()
            .map(|record| record.sequence)
            .max()
            .unwrap_or(request.cursor);
        Ok(SyncResponse {
            cursor,
            acknowledged,
            changes: changes.into_iter().map(ChangeEntry::from_record).collect(),
            conflicts,
        })
    }
}

/// Whether a command changes relative ordering (server rank is authoritative).
#[must_use]
pub fn is_ordering_command(command: &Command) -> bool {
    matches!(
        command,
        Command::ReorderTodo { .. }
            | Command::ReorderToday { .. }
            | Command::MoveTodoToPosition { .. }
            | Command::MoveTodayToPosition { .. }
            | Command::ReorderTag { .. }
            | Command::ReorderChecklistItem { .. }
    )
}

/// Classifies a failed push into an explicit conflict rule and resolution.
#[must_use]
pub fn classify_conflict(
    operation_id: Uuid,
    entity_id: Option<EntityId>,
    command: &Command,
    error: &CommandError,
) -> SyncConflict {
    let ordering = is_ordering_command(command);
    let (rule, resolution, detail) = match error {
        CommandError::Conflict { expected, actual } if ordering => (
            ConflictRule::OrderingRelative,
            ConflictResolution::Ignored,
            format!("server ordering retained (expected {expected}, actual {actual})"),
        ),
        CommandError::Conflict { expected, actual } => (
            ConflictRule::FieldVersion,
            ConflictResolution::ServerWins,
            format!("server version {actual} retained over client {expected}"),
        ),
        CommandError::NotFound if ordering => (
            ConflictRule::OrderingRelative,
            ConflictResolution::Ignored,
            "ordering target no longer exists; refresh the list".to_string(),
        ),
        CommandError::NotFound => (
            ConflictRule::DeleteBeatsEdit,
            ConflictResolution::ClientRefreshes,
            "entity was deleted; refresh and offer restore".to_string(),
        ),
        other => (
            ConflictRule::Rejected,
            ConflictResolution::ClientRefreshes,
            other.to_string(),
        ),
    };
    SyncConflict {
        operation_id,
        entity_id,
        rule,
        resolution,
        detail,
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_conflict, is_ordering_command, ConflictResolution, ConflictRule};
    use crate::commands::{Command, CommandError};
    use momentum_domain::EntityId;
    use uuid::Uuid;

    #[test]
    fn ordering_commands_are_recognized() {
        let id = EntityId::new();
        assert!(is_ordering_command(&Command::MoveTodoToPosition {
            id,
            position: 2,
        }));
        assert!(!is_ordering_command(&Command::CompleteTodo { id }));
    }

    #[test]
    fn conflicts_follow_explicit_domain_rules() {
        let id = EntityId::new();
        let operation = Uuid::now_v7();

        let stale_order = classify_conflict(
            operation,
            Some(id),
            &Command::MoveTodoToPosition { id, position: 1 },
            &CommandError::Conflict {
                expected: 1,
                actual: 3,
            },
        );
        assert_eq!(stale_order.rule, ConflictRule::OrderingRelative);
        assert_eq!(stale_order.resolution, ConflictResolution::Ignored);

        let field = classify_conflict(
            operation,
            Some(id),
            &Command::SetTodoTitle {
                id,
                title: "new".to_string(),
            },
            &CommandError::Conflict {
                expected: 1,
                actual: 3,
            },
        );
        assert_eq!(field.rule, ConflictRule::FieldVersion);
        assert_eq!(field.resolution, ConflictResolution::ServerWins);

        let deleted = classify_conflict(
            operation,
            Some(id),
            &Command::SetTodoTitle {
                id,
                title: "new".to_string(),
            },
            &CommandError::NotFound,
        );
        assert_eq!(deleted.rule, ConflictRule::DeleteBeatsEdit);
        assert_eq!(deleted.resolution, ConflictResolution::ClientRefreshes);
    }
}
