//! Area queries and lifecycle service.
//!
//! Areas are ongoing responsibilities that group projects and loose to-dos. Deletion always
//! carries an explicit policy so contents are never silently orphaned or destroyed.

use std::sync::Arc;

use async_trait::async_trait;
use momentum_domain::{AccountId, EntityId};

use crate::auth::Clock;
use crate::commands::{
    AreaDeletionPolicy, Command, CommandDispatcher, CommandEnvelope, CommandError, CommandOutcome,
};

/// Read model for an area.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreaSummary {
    /// Canonical identity.
    pub id: EntityId,
    /// Title.
    pub title: String,
    /// Number of projects assigned to the area.
    pub project_count: usize,
    /// Number of loose to-dos filed directly under the area.
    pub todo_count: usize,
}

/// Read-store port for areas.
#[async_trait]
pub trait AreaReadStore: Send + Sync {
    /// Lists non-deleted areas with their child counts.
    async fn list_areas(&self, account_id: AccountId) -> Result<Vec<AreaSummary>, CommandError>;

    /// Lists trashed areas.
    async fn list_deleted_areas(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<AreaSummary>, CommandError>;

    /// Loads one area with its child counts.
    async fn get_area(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<AreaSummary>, CommandError>;
}

/// Area lifecycle service.
pub struct AreaService {
    dispatcher: Arc<CommandDispatcher>,
    reader: Arc<dyn AreaReadStore>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for AreaService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AreaService")
    }
}

impl AreaService {
    /// Creates the service.
    #[must_use]
    pub fn new(
        dispatcher: Arc<CommandDispatcher>,
        reader: Arc<dyn AreaReadStore>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            dispatcher,
            reader,
            clock,
        }
    }

    /// Lists areas.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn list(&self, account_id: AccountId) -> Result<Vec<AreaSummary>, CommandError> {
        self.reader.list_areas(account_id).await
    }

    /// Loads one area.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn get(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<AreaSummary>, CommandError> {
        self.reader.get_area(account_id, id).await
    }

    /// Creates an area.
    ///
    /// # Errors
    ///
    /// Propagates command validation and storage failures.
    pub async fn create(
        &self,
        account_id: AccountId,
        title: &str,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(
            account_id,
            Command::CreateArea {
                title: title.to_string(),
            },
        )
        .await
    }

    /// Renames an area.
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
            Command::RenameArea {
                id,
                title: title.to_string(),
            },
        )
        .await
    }

    /// Lists trashed areas.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn trash(&self, account_id: AccountId) -> Result<Vec<AreaSummary>, CommandError> {
        self.reader.list_deleted_areas(account_id).await
    }

    /// Restores a trashed area.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn restore(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::RestoreArea { id }).await
    }

    /// Permanently deletes a trashed area.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn purge(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::PurgeArea { id }).await
    }

    /// Deletes an area under an explicit contents policy.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn delete(
        &self,
        account_id: AccountId,
        id: EntityId,
        policy: AreaDeletionPolicy,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::DeleteArea { id, policy })
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
