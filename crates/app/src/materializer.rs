//! Idempotent, restart-safe recurrence materialization.
//!
//! Generation is driven by persisted template state: each run materializes the occurrences that
//! fall inside a bounded horizon, records them through the command layer (whose transaction
//! guards duplicates), and persists progress so a restart resumes rather than restarts.

use std::sync::Arc;

use async_trait::async_trait;
use momentum_domain::recurrence::{Recurrence, RecurrenceTemplate};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};

use crate::commands::CommandError;
use crate::tags::TagService;
use crate::todos::TodoService;

/// How many days ahead materialization eagerly generates.
pub const HORIZON_DAYS: i64 = 30;

/// Upper bound on occurrences generated per template in one run.
pub const MAX_OCCURRENCES_PER_RUN: u32 = 90;

/// A persisted recurrence template with its materialization progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedTemplate {
    /// Template identity (also the entities row of the recurring item).
    pub id: EntityId,
    /// The recurrence rule.
    pub rule: Recurrence,
    /// The inheritance payload.
    pub template: RecurrenceTemplate,
    /// Next occurrence still to generate, when the rule has not ended.
    pub next_occurrence: Option<LocalDate>,
    /// Horizon already materialized, when any.
    pub horizon: Option<LocalDate>,
}

/// Persistence for recurrence templates and generated occurrences.
#[async_trait]
pub trait MaterializerStore: Send + Sync {
    /// Templates that still need generation up to `horizon`.
    async fn templates_due(
        &self,
        account_id: AccountId,
        horizon: LocalDate,
    ) -> Result<Vec<PersistedTemplate>, CommandError>;

    /// Persists generation progress for one template.
    async fn advance(
        &self,
        account_id: AccountId,
        template_id: EntityId,
        next_occurrence: Option<LocalDate>,
        horizon: LocalDate,
    ) -> Result<(), CommandError>;
}

/// Per-run results.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MaterializationRun {
    /// Generated copy identities.
    pub materialized: Vec<EntityId>,
    /// Occurrences skipped because they already existed.
    pub skipped: Vec<LocalDate>,
    /// Occurrences that failed with their reasons.
    pub failed: Vec<(LocalDate, String)>,
}

/// Generates recurrence copies within a bounded horizon.
pub struct RecurrenceMaterializer {
    todos: Arc<TodoService>,
    tags: Option<Arc<TagService>>,
    store: Arc<dyn MaterializerStore>,
}

impl RecurrenceMaterializer {
    /// Creates the materializer.
    #[must_use]
    pub fn new(
        todos: Arc<TodoService>,
        tags: Option<Arc<TagService>>,
        store: Arc<dyn MaterializerStore>,
    ) -> Self {
        Self { todos, tags, store }
    }

    /// Materializes every due template up to `today + HORIZON_DAYS`.
    ///
    /// Repeated runs are no-ops for already materialized occurrences, and progress is persisted
    /// so restarts continue from the stored template state.
    ///
    /// # Errors
    ///
    /// Propagates template read/write failures.
    pub async fn run(
        &self,
        account_id: AccountId,
        today: LocalDate,
    ) -> Result<MaterializationRun, CommandError> {
        let horizon = advance(today, HORIZON_DAYS);
        let mut run = MaterializationRun::default();
        for template in self.store.templates_due(account_id, horizon).await? {
            let mut date = match template.next_occurrence {
                Some(date) if date > today => date,
                _ => today,
            };
            let mut generated = 0_u32;
            let mut next = None;
            while date <= horizon && generated < MAX_OCCURRENCES_PER_RUN {
                match self
                    .todos
                    .materialize_occurrence(account_id, template.id, date)
                    .await
                {
                    Ok(outcome) => {
                        if let Some(copy) = outcome.entity_id {
                            run.materialized.push(copy);
                            self.apply_tags(account_id, copy, &template).await;
                        } else {
                            run.skipped.push(date);
                        }
                    }
                    Err(error) => run.failed.push((date, error.to_string())),
                }
                generated += 1;
                let occurrences = generated;
                match template.rule.next_occurrence(date, occurrences) {
                    Some(following) => {
                        next = Some(following);
                        date = following;
                    }
                    None => {
                        next = None;
                        break;
                    }
                }
            }
            self.store
                .advance(account_id, template.id, next, horizon)
                .await?;
        }
        Ok(run)
    }

    async fn apply_tags(
        &self,
        account_id: AccountId,
        copy: EntityId,
        template: &PersistedTemplate,
    ) {
        let (Some(tags), Some(_)) = (&self.tags, template.template.tag_ids.first()) else {
            return;
        };
        for tag in &template.template.tag_ids {
            let _ = tags.assign(account_id, copy, *tag).await;
        }
    }
}

fn advance(base: LocalDate, days: i64) -> LocalDate {
    let mut date = base;
    for _ in 0..days.max(0) {
        date = date.next_day();
    }
    date
}
