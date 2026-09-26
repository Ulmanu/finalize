//! In-memory logging-policy store for tests.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::logbook::LoggingPolicyStore;
use momentum_domain::model::LoggingPolicy;
use momentum_domain::AccountId;

/// In-memory policy store.
#[derive(Debug, Clone, Default)]
pub struct MemoryLoggingPolicyStore {
    policies: Arc<Mutex<HashMap<AccountId, LoggingPolicy>>>,
}

impl MemoryLoggingPolicyStore {
    /// Creates an empty store defaulting to the automatic policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl LoggingPolicyStore for MemoryLoggingPolicyStore {
    async fn load_policy(&self, account_id: AccountId) -> Result<LoggingPolicy, CommandError> {
        Ok(self
            .policies
            .lock()
            .expect("policy lock")
            .get(&account_id)
            .copied()
            .unwrap_or_default())
    }

    async fn save_policy(
        &self,
        account_id: AccountId,
        policy: LoggingPolicy,
    ) -> Result<(), CommandError> {
        self.policies
            .lock()
            .expect("policy lock")
            .insert(account_id, policy);
        Ok(())
    }
}
