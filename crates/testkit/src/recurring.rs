//! In-memory recurrence read store for tests.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::recurring::{RecurrenceReadStore, RecurrenceTemplateSummary};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;

/// In-memory recurrence template store.
#[derive(Debug, Clone, Default)]
pub struct MemoryRecurrenceReadStore {
    templates: Arc<Mutex<Vec<(AccountId, RecurrenceTemplateSummary)>>>,
}

impl MemoryRecurrenceReadStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Seeds a template for tests.
    pub fn add_template(&self, account_id: AccountId, summary: RecurrenceTemplateSummary) {
        self.templates
            .lock()
            .expect("recurrence lock")
            .push((account_id, summary));
    }

    /// Convenience seeding helper.
    pub fn add(
        &self,
        account_id: AccountId,
        id: momentum_domain::EntityId,
        title: &str,
        next_occurrence: Option<LocalDate>,
    ) {
        self.add_template(
            account_id,
            RecurrenceTemplateSummary {
                id,
                title: title.to_string(),
                next_occurrence,
            },
        );
    }
}

#[async_trait]
impl RecurrenceReadStore for MemoryRecurrenceReadStore {
    async fn list_templates(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<RecurrenceTemplateSummary>, CommandError> {
        Ok(self
            .templates
            .lock()
            .expect("recurrence lock")
            .iter()
            .filter(|(account, _)| *account == account_id)
            .map(|(_, summary)| summary.clone())
            .collect())
    }
}
