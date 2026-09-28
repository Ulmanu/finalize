//! Memory-backed recurrence template store for materializer tests.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::materializer::{MaterializerStore, PersistedTemplate, TemplateState};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};

use crate::commands::MemoryDatabase;

/// Memory implementation of [`MaterializerStore`].
#[derive(Debug, Clone)]
pub struct MemoryMaterializerStore {
    database: MemoryDatabase,
}

impl MemoryMaterializerStore {
    /// Creates the store over a database.
    #[must_use]
    pub fn new(database: MemoryDatabase) -> Self {
        Self { database }
    }
}

#[async_trait]
impl MaterializerStore for MemoryMaterializerStore {
    async fn active_templates(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<PersistedTemplate>, CommandError> {
        Ok(self
            .database
            .recurrence_templates_for(account_id)
            .into_iter()
            .filter(
                |template| match (template.next_occurrence, template.horizon) {
                    (Some(_), _) => true,
                    (None, None) => true,
                    (None, Some(_)) => false,
                },
            )
            .collect())
    }

    async fn template_state(
        &self,
        account_id: AccountId,
        template_id: EntityId,
    ) -> Result<TemplateState, CommandError> {
        Ok(self
            .database
            .recurrence_template_state(account_id, template_id))
    }

    async fn advance(
        &self,
        account_id: AccountId,
        template_id: EntityId,
        next_occurrence: Option<LocalDate>,
        horizon: LocalDate,
    ) -> Result<(), CommandError> {
        self.database.advance_recurrence_template(
            account_id,
            template_id,
            next_occurrence,
            horizon,
        );
        Ok(())
    }
}
