//! Repeating special view: recurrence template reads.

use std::sync::Arc;

use async_trait::async_trait;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};

use crate::commands::CommandError;

/// Read model for a recurrence template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecurrenceTemplateSummary {
    /// Canonical identity of the recurring entity.
    pub id: EntityId,
    /// Title of the recurring entity.
    pub title: String,
    /// Next planned occurrence, when scheduled.
    pub next_occurrence: Option<LocalDate>,
}

/// Read-store port for recurrence templates.
#[async_trait]
pub trait RecurrenceReadStore: Send + Sync {
    /// Lists recurrence templates for the account.
    async fn list_templates(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<RecurrenceTemplateSummary>, CommandError>;
}

/// Repeating special-view service.
pub struct RecurringService {
    reader: Arc<dyn RecurrenceReadStore>,
}

impl std::fmt::Debug for RecurringService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RecurringService")
    }
}

impl RecurringService {
    /// Creates the service.
    #[must_use]
    pub fn new(reader: Arc<dyn RecurrenceReadStore>) -> Self {
        Self { reader }
    }

    /// Lists recurrence templates.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn list(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<RecurrenceTemplateSummary>, CommandError> {
        self.reader.list_templates(account_id).await
    }
}
