//! Memory inbound alias/ledger stores and a deterministic token source for tests.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::inbound::{AliasStore, InboundAlias, MessageLedger, TokenSource};
use momentum_domain::AccountId;

#[derive(Debug, Default)]
struct State {
    aliases: HashMap<AccountId, InboundAlias>,
    messages: HashSet<(AccountId, String)>,
}

/// Memory implementation of both inbound store ports.
#[derive(Debug, Default)]
pub struct MemoryInboundStore {
    state: Mutex<State>,
}

impl MemoryInboundStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl AliasStore for MemoryInboundStore {
    async fn alias_for(&self, account_id: AccountId) -> Result<Option<InboundAlias>, CommandError> {
        Ok(self
            .state
            .lock()
            .expect("inbound state lock")
            .aliases
            .get(&account_id)
            .cloned())
    }

    async fn lookup_token(&self, token: &str) -> Result<Option<(AccountId, bool)>, CommandError> {
        Ok(self
            .state
            .lock()
            .expect("inbound state lock")
            .aliases
            .iter()
            .find(|(_, alias)| alias.token == token)
            .map(|(account, alias)| (*account, alias.enabled)))
    }

    async fn rotate(
        &self,
        account_id: AccountId,
        token: &str,
    ) -> Result<InboundAlias, CommandError> {
        let alias = InboundAlias {
            token: token.to_string(),
            enabled: true,
        };
        self.state
            .lock()
            .expect("inbound state lock")
            .aliases
            .insert(account_id, alias.clone());
        Ok(alias)
    }

    async fn disable(&self, account_id: AccountId) -> Result<(), CommandError> {
        if let Some(alias) = self
            .state
            .lock()
            .expect("inbound state lock")
            .aliases
            .get_mut(&account_id)
        {
            alias.enabled = false;
        }
        Ok(())
    }
}

#[async_trait]
impl MessageLedger for MemoryInboundStore {
    async fn seen(&self, account_id: AccountId, message_id: &str) -> Result<bool, CommandError> {
        Ok(self
            .state
            .lock()
            .expect("inbound state lock")
            .messages
            .contains(&(account_id, message_id.to_string())))
    }

    async fn record(&self, account_id: AccountId, message_id: &str) -> Result<(), CommandError> {
        self.state
            .lock()
            .expect("inbound state lock")
            .messages
            .insert((account_id, message_id.to_string()));
        Ok(())
    }
}

/// Deterministic 32-character tokens for tests.
#[derive(Debug, Default)]
pub struct DeterministicTokenSource {
    next: AtomicU64,
}

impl DeterministicTokenSource {
    /// Creates the source.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TokenSource for DeterministicTokenSource {
    fn token(&self) -> String {
        let value = self.next.fetch_add(1, Ordering::SeqCst) + 1;
        format!("{value:032x}")
    }
}
