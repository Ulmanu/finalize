//! Deterministic in-memory auth adapters for tests.
//!
//! These are test doubles: the deterministic hasher/token generator exist so lifecycle tests
//! are fast and reproducible. Production uses the Argon2id hasher and random token generator
//! in `momentum-infra`.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use momentum_app::auth::{
    AccountRecord, AccountStore, AuthError, AuthTokenKind, AuthTokenRecord, AuthTokenStore, Clock,
    PasswordHasher, SessionRecord, SessionStore, TokenGenerator,
};
use momentum_domain::ids::SessionId;
use momentum_domain::AccountId;

#[derive(Default)]
struct MemoryState {
    accounts: Vec<AccountRecord>,
    sessions: Vec<SessionRecord>,
    tokens: Vec<AuthTokenRecord>,
}

/// In-memory implementation of all auth stores.
#[derive(Default)]
pub struct MemoryAuthStore {
    state: Mutex<MemoryState>,
}

impl std::fmt::Debug for MemoryAuthStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MemoryAuthStore")
    }
}

impl MemoryAuthStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a snapshot of an account by normalized email.
    #[must_use]
    pub fn account_by_email(&self, normalized: &str) -> Option<AccountRecord> {
        self.state
            .lock()
            .expect("memory store lock")
            .accounts
            .iter()
            .find(|account| account.email_normalized == normalized)
            .cloned()
    }

    /// Returns a snapshot of an account by id.
    #[must_use]
    pub fn account_by_id(&self, id: AccountId) -> Option<AccountRecord> {
        self.state
            .lock()
            .expect("memory store lock")
            .accounts
            .iter()
            .find(|account| account.id == id)
            .cloned()
    }

    /// Returns all sessions for an account.
    #[must_use]
    pub fn sessions_for(&self, account_id: AccountId) -> Vec<SessionRecord> {
        self.state
            .lock()
            .expect("memory store lock")
            .sessions
            .iter()
            .filter(|session| session.account_id == account_id)
            .cloned()
            .collect()
    }
}

#[async_trait]
impl AccountStore for MemoryAuthStore {
    async fn find_by_email(&self, normalized: &str) -> Result<Option<AccountRecord>, AuthError> {
        Ok(self.account_by_email(normalized))
    }

    async fn find_by_id(&self, id: AccountId) -> Result<Option<AccountRecord>, AuthError> {
        Ok(self.account_by_id(id))
    }

    async fn insert(&self, account: &AccountRecord) -> Result<(), AuthError> {
        let mut state = self.state.lock().expect("memory store lock");
        if state
            .accounts
            .iter()
            .any(|existing| existing.email_normalized == account.email_normalized)
        {
            return Err(AuthError::EmailTaken);
        }
        state.accounts.push(account.clone());
        Ok(())
    }

    async fn update_password(
        &self,
        id: AccountId,
        hash: &str,
        expected_version: i64,
        updated_at: DateTime<Utc>,
    ) -> Result<(), AuthError> {
        let mut state = self.state.lock().expect("memory store lock");
        let account = state
            .accounts
            .iter_mut()
            .find(|account| account.id == id)
            .ok_or(AuthError::AccountNotFound)?;
        if account.version != expected_version {
            return Err(AuthError::Storage("version conflict".to_string()));
        }
        account.password_hash = Some(hash.to_string());
        account.version += 1;
        account.created_at = account.created_at.min(updated_at);
        Ok(())
    }

    async fn mark_verified(&self, id: AccountId, at: DateTime<Utc>) -> Result<(), AuthError> {
        let mut state = self.state.lock().expect("memory store lock");
        let account = state
            .accounts
            .iter_mut()
            .find(|account| account.id == id)
            .ok_or(AuthError::AccountNotFound)?;
        account.email_verified_at = Some(at);
        account.version += 1;
        Ok(())
    }

    async fn mark_deleted(&self, id: AccountId, at: DateTime<Utc>) -> Result<(), AuthError> {
        let mut state = self.state.lock().expect("memory store lock");
        let account = state
            .accounts
            .iter_mut()
            .find(|account| account.id == id)
            .ok_or(AuthError::AccountNotFound)?;
        account.deleted_at = Some(at);
        account.version += 1;
        Ok(())
    }
}

#[async_trait]
impl SessionStore for MemoryAuthStore {
    async fn insert(&self, session: &SessionRecord) -> Result<(), AuthError> {
        self.state
            .lock()
            .expect("memory store lock")
            .sessions
            .push(session.clone());
        Ok(())
    }

    async fn find_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<SessionRecord>, AuthError> {
        Ok(self
            .state
            .lock()
            .expect("memory store lock")
            .sessions
            .iter()
            .find(|session| session.token_hash == token_hash)
            .cloned())
    }

    async fn revoke(&self, id: SessionId, at: DateTime<Utc>) -> Result<(), AuthError> {
        let mut state = self.state.lock().expect("memory store lock");
        let session = state
            .sessions
            .iter_mut()
            .find(|session| session.id == id)
            .ok_or(AuthError::InvalidSession)?;
        session.revoked_at = Some(at);
        Ok(())
    }

    async fn revoke_all_for_account(
        &self,
        account_id: AccountId,
        at: DateTime<Utc>,
    ) -> Result<(), AuthError> {
        let mut state = self.state.lock().expect("memory store lock");
        for session in state
            .sessions
            .iter_mut()
            .filter(|session| session.account_id == account_id)
        {
            session.revoked_at = Some(at);
        }
        Ok(())
    }

    async fn list_for_account(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<SessionRecord>, AuthError> {
        Ok(self.sessions_for(account_id))
    }
}

#[async_trait]
impl AuthTokenStore for MemoryAuthStore {
    async fn insert(&self, token: &AuthTokenRecord) -> Result<(), AuthError> {
        self.state
            .lock()
            .expect("memory store lock")
            .tokens
            .push(token.clone());
        Ok(())
    }

    async fn consume(
        &self,
        kind: AuthTokenKind,
        token_hash: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<AuthTokenRecord>, AuthError> {
        let mut state = self.state.lock().expect("memory store lock");
        let token = state
            .tokens
            .iter_mut()
            .find(|token| token.kind == kind && token.token_hash == token_hash);
        let Some(token) = token else {
            return Ok(None);
        };
        if token.consumed_at.is_some() || token.expires_at <= now {
            return Ok(None);
        }
        token.consumed_at = Some(now);
        Ok(Some(token.clone()))
    }
}

/// Deterministic password hasher for tests.
#[derive(Debug, Default, Clone, Copy)]
pub struct DeterministicHasher;

impl PasswordHasher for DeterministicHasher {
    fn hash(&self, password: &str) -> Result<String, AuthError> {
        Ok(format!("test-hash:{password}"))
    }

    fn verify(&self, hash: &str, password: &str) -> bool {
        hash == format!("test-hash:{password}")
    }
}

/// Deterministic token generator for tests.
#[derive(Debug, Default)]
pub struct DeterministicTokenGenerator {
    counter: AtomicU64,
}

impl DeterministicTokenGenerator {
    /// Creates a generator starting at zero.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TokenGenerator for DeterministicTokenGenerator {
    fn generate(&self) -> String {
        let next = self.counter.fetch_add(1, Ordering::SeqCst);
        format!("test-token-{next}")
    }

    fn hash(&self, token: &str) -> String {
        format!("test-hash:{token}")
    }
}

/// Adjustable clock for tests.
#[derive(Debug)]
pub struct TestClock {
    now: Mutex<DateTime<Utc>>,
}

impl TestClock {
    /// Creates a clock at a fixed instant.
    #[must_use]
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            now: Mutex::new(now),
        }
    }

    /// Advances the clock by a duration.
    pub fn advance(&self, duration: Duration) {
        let mut now = self.now.lock().expect("clock lock");
        *now += duration;
    }
}

impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.now.lock().expect("clock lock")
    }
}
