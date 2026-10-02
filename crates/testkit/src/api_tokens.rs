//! Memory automation-token store for API tests.

use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_app::api_tokens::{ApiScope, ApiTokenStore, ApiTokenSummary};
use momentum_app::commands::CommandError;
use momentum_domain::{AccountId, EntityId};

#[derive(Debug, Default)]
struct State {
    tokens: Vec<(AccountId, ApiTokenSummary, String)>,
}

/// Memory implementation of [`ApiTokenStore`].
#[derive(Debug, Default)]
pub struct MemoryApiTokenStore {
    state: Mutex<State>,
}

impl MemoryApiTokenStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ApiTokenStore for MemoryApiTokenStore {
    async fn create(
        &self,
        account_id: AccountId,
        id: EntityId,
        name: &str,
        token_hash: &str,
        scopes: &[ApiScope],
        now: DateTime<Utc>,
    ) -> Result<ApiTokenSummary, CommandError> {
        let summary = ApiTokenSummary {
            id,
            name: name.to_string(),
            scopes: scopes.to_vec(),
            created_at: now,
            last_used_at: None,
            revoked_at: None,
        };
        self.state.lock().expect("api token lock").tokens.push((
            account_id,
            summary.clone(),
            token_hash.to_string(),
        ));
        Ok(summary)
    }

    async fn list(&self, account_id: AccountId) -> Result<Vec<ApiTokenSummary>, CommandError> {
        let mut summaries: Vec<ApiTokenSummary> = self
            .state
            .lock()
            .expect("api token lock")
            .tokens
            .iter()
            .filter(|(account, _, _)| *account == account_id)
            .map(|(_, summary, _)| summary.clone())
            .collect();
        summaries.sort_by_key(|summary| std::cmp::Reverse(summary.created_at));
        Ok(summaries)
    }

    async fn revoke(
        &self,
        account_id: AccountId,
        id: EntityId,
        now: DateTime<Utc>,
    ) -> Result<bool, CommandError> {
        let mut state = self.state.lock().expect("api token lock");
        let Some((_, summary, _)) = state
            .tokens
            .iter_mut()
            .find(|(account, summary, _)| *account == account_id && summary.id == id)
        else {
            return Ok(false);
        };
        summary.revoked_at = Some(now);
        Ok(true)
    }

    async fn rotate(
        &self,
        account_id: AccountId,
        id: EntityId,
        token_hash: &str,
    ) -> Result<Option<ApiTokenSummary>, CommandError> {
        let mut state = self.state.lock().expect("api token lock");
        let Some((_, summary, hash)) = state.tokens.iter_mut().find(|(account, summary, _)| {
            *account == account_id && summary.id == id && summary.revoked_at.is_none()
        }) else {
            return Ok(None);
        };
        *hash = token_hash.to_string();
        summary.last_used_at = None;
        Ok(Some(summary.clone()))
    }

    async fn find_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<(EntityId, AccountId, Vec<ApiScope>, bool)>, CommandError> {
        Ok(self
            .state
            .lock()
            .expect("api token lock")
            .tokens
            .iter()
            .find(|(_, _, hash)| hash == token_hash)
            .map(|(account, summary, _)| {
                (
                    summary.id,
                    *account,
                    summary.scopes.clone(),
                    summary.revoked_at.is_some(),
                )
            }))
    }

    async fn touch(
        &self,
        account_id: AccountId,
        id: EntityId,
        now: DateTime<Utc>,
    ) -> Result<(), CommandError> {
        let mut state = self.state.lock().expect("api token lock");
        if let Some((_, summary, _)) = state
            .tokens
            .iter_mut()
            .find(|(account, summary, _)| *account == account_id && summary.id == id)
        {
            summary.last_used_at = Some(now);
        }
        Ok(())
    }
}
