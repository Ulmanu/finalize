//! Bulk application of supported commands over a multi-selection.
//!
//! Bulk operations run each item through the same version-guarded, undoable command used by the
//! single-item paths and collect explicit per-item results, so a selection containing a
//! now-invalid member cannot silently produce partial corruption: every failure is reported by
//! entity ID alongside the successes.

use std::sync::Arc;

use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};

use crate::commands::{CommandError, ScheduleInput};
use crate::movement::Destination;
use crate::tags::TagService;
use crate::todos::TodoService;

/// A command applied to every selected item.
#[derive(Debug, Clone)]
pub enum BulkAction {
    /// Completes each item.
    Complete,
    /// Cancels each item.
    Cancel,
    /// Moves each item to Trash.
    Delete,
    /// Replaces each item's schedule.
    Schedule(ScheduleInput),
    /// Replaces each item's deadline.
    SetDeadline(Option<LocalDate>),
    /// Moves each item to a destination.
    Move(Destination),
    /// Assigns one tag to each item.
    AssignTag(EntityId),
}

/// One failed item inside a bulk operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulkFailure {
    /// The entity that could not be processed.
    pub id: EntityId,
    /// Human-readable failure reason.
    pub message: String,
}

/// Deterministic per-item results for a bulk operation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BulkOutcome {
    /// Items processed successfully.
    pub succeeded: Vec<EntityId>,
    /// Items that failed with their reasons.
    pub failed: Vec<BulkFailure>,
}

impl BulkOutcome {
    /// Creates an empty outcome.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns how many items were attempted.
    #[must_use]
    pub fn processed(&self) -> usize {
        self.succeeded.len() + self.failed.len()
    }

    /// Returns whether every attempted item succeeded.
    #[must_use]
    pub fn is_full_success(&self) -> bool {
        self.failed.is_empty()
    }

    /// Records one item result.
    pub fn record(&mut self, id: EntityId, result: Result<(), CommandError>) {
        match result {
            Ok(()) => self.succeeded.push(id),
            Err(error) => self.failed.push(BulkFailure {
                id,
                message: error.to_string(),
            }),
        }
    }
}

/// Applies bulk actions with explicit per-item results.
#[derive(Debug)]
pub struct BulkService {
    todos: Arc<TodoService>,
    tags: Option<Arc<TagService>>,
}

impl BulkService {
    /// Creates the service.
    #[must_use]
    pub fn new(todos: Arc<TodoService>, tags: Option<Arc<TagService>>) -> Self {
        Self { todos, tags }
    }

    /// Applies one action to every item, returning deterministic per-item results.
    pub async fn apply(
        &self,
        account_id: AccountId,
        ids: &[EntityId],
        action: &BulkAction,
    ) -> BulkOutcome {
        let mut outcome = BulkOutcome::new();
        for id in ids {
            let result = match action {
                BulkAction::Complete => self.todos.complete(account_id, *id).await.map(|_| ()),
                BulkAction::Cancel => self.todos.cancel(account_id, *id).await.map(|_| ()),
                BulkAction::Delete => self.todos.delete(account_id, *id).await.map(|_| ()),
                BulkAction::Schedule(schedule) => self
                    .todos
                    .schedule(account_id, *id, *schedule)
                    .await
                    .map(|_| ()),
                BulkAction::SetDeadline(deadline) => self
                    .todos
                    .set_deadline(account_id, *id, *deadline)
                    .await
                    .map(|_| ()),
                BulkAction::Move(destination) => self
                    .todos
                    .move_to(account_id, *id, destination.context())
                    .await
                    .map(|_| ()),
                BulkAction::AssignTag(tag) => match &self.tags {
                    Some(tags) => tags.assign(account_id, *id, *tag).await.map(|_| ()),
                    None => Err(CommandError::NotFound),
                },
            };
            outcome.record(*id, result);
        }
        outcome
    }
}
