//! Memory change-log reader for sync protocol tests.

use async_trait::async_trait;
use momentum_app::commands::{ChangeRecord, CommandError};
use momentum_app::sync::ChangeLogReader;
use momentum_domain::AccountId;

use crate::commands::MemoryDatabase;

/// Reads the in-memory change log.
#[derive(Debug, Clone)]
pub struct MemoryChangeLogReader {
    database: MemoryDatabase,
}

impl MemoryChangeLogReader {
    /// Creates the reader over a database.
    #[must_use]
    pub fn new(database: MemoryDatabase) -> Self {
        Self { database }
    }
}

#[async_trait]
impl ChangeLogReader for MemoryChangeLogReader {
    async fn changes_after(
        &self,
        account_id: AccountId,
        cursor: i64,
    ) -> Result<Vec<ChangeRecord>, CommandError> {
        Ok(self.database.changes_after(account_id, cursor))
    }
}
