//! Memory preference store for settings tests.

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::preferences::{PreferenceScope, PreferenceStore};
use momentum_domain::AccountId;

/// Memory implementation of [`PreferenceStore`].
#[derive(Debug, Default)]
pub struct MemoryPreferenceStore {
    values: Mutex<HashMap<(AccountId, String), (PreferenceScope, String)>>,
}

impl MemoryPreferenceStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl PreferenceStore for MemoryPreferenceStore {
    async fn load_all(&self, account_id: AccountId) -> Result<Vec<(String, String)>, CommandError> {
        Ok(self
            .values
            .lock()
            .expect("preference lock")
            .iter()
            .filter(|((account, _), _)| *account == account_id)
            .map(|((_, key), (_, raw))| (key.clone(), raw.clone()))
            .collect())
    }

    async fn save(
        &self,
        account_id: AccountId,
        key: &str,
        scope: PreferenceScope,
        raw: &str,
    ) -> Result<(), CommandError> {
        self.values
            .lock()
            .expect("preference lock")
            .insert((account_id, key.to_string()), (scope, raw.to_string()));
        Ok(())
    }
}
