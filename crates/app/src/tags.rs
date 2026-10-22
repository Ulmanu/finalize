//! Tag queries and lifecycle service.

use std::sync::Arc;

use async_trait::async_trait;
use momentum_domain::{AccountId, EntityId};

use crate::auth::Clock;
use crate::commands::{
    Command, CommandDispatcher, CommandEnvelope, CommandError, CommandOutcome, ReorderDirection,
};

/// Read model for a tag.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TagSummary {
    /// Canonical identity.
    pub id: EntityId,
    /// Title.
    pub title: String,
    /// Parent tag, when nested.
    pub parent: Option<EntityId>,
    /// Sibling position.
    pub position: i64,
    /// Number of to-dos directly carrying this tag.
    pub assigned_count: usize,
}

/// Read-store port for tags.
#[async_trait]
pub trait TagReadStore: Send + Sync {
    /// Lists tags ordered by hierarchy position.
    async fn list_tags(&self, account_id: AccountId) -> Result<Vec<TagSummary>, CommandError>;
}

/// Tag lifecycle and assignment service.
pub struct TagService {
    dispatcher: Arc<CommandDispatcher>,
    reader: Arc<dyn TagReadStore>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for TagService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TagService")
    }
}

impl TagService {
    /// Creates the service.
    #[must_use]
    pub fn new(
        dispatcher: Arc<CommandDispatcher>,
        reader: Arc<dyn TagReadStore>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            dispatcher,
            reader,
            clock,
        }
    }

    /// Lists tags.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn list(&self, account_id: AccountId) -> Result<Vec<TagSummary>, CommandError> {
        self.reader.list_tags(account_id).await
    }

    /// Creates a tag.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn create(
        &self,
        account_id: AccountId,
        title: &str,
        parent: Option<EntityId>,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::CreateTag {
                title: title.to_string(),
                parent,
            },
        )
        .await
    }

    /// Renames a tag.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn rename(
        &self,
        account_id: AccountId,
        id: EntityId,
        title: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::RenameTag {
                id,
                title: title.to_string(),
            },
        )
        .await
    }

    /// Moves a tag to a new parent.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn move_to_parent(
        &self,
        account_id: AccountId,
        id: EntityId,
        parent: Option<EntityId>,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::MoveTag { id, parent })
            .await
    }

    /// Moves a tag among its siblings.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn reorder(
        &self,
        account_id: AccountId,
        id: EntityId,
        direction: ReorderDirection,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::ReorderTag { id, direction })
            .await
    }

    /// Deletes a tag, reparenting children to its parent.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn delete(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::DeleteTag { id }).await
    }

    /// Assigns a tag to an open entity (to-do, project, or area).
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn assign(
        &self,
        account_id: AccountId,
        entity: EntityId,
        tag: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::AssignTag { entity, tag })
            .await
    }

    /// Removes a tag from an open entity (to-do, project, or area).
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn unassign(
        &self,
        account_id: AccountId,
        entity: EntityId,
        tag: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::UnassignTag { entity, tag })
            .await
    }

    async fn dispatch(
        &self,
        account_id: AccountId,
        command: Command,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatcher
            .execute(CommandEnvelope::new(account_id, command, self.clock.now()))
            .await
    }
}
