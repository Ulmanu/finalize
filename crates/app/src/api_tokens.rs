//! Scoped personal automation API tokens.
//!
//! Tokens are generated with the same opaque-token generator used by authentication, stored only
//! as deterministic hashes (so lookup is by hash and the plaintext is shown once at creation or
//! rotation), carry explicit scopes, and can be revoked. REST automation resolves a bearer token
//! to an [`ApiPrincipal`] and delegates every mutation to the same application commands as the
//! first-party UI.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_domain::{AccountId, EntityId};

use crate::auth::TokenGenerator;
use crate::commands::CommandError;

/// Permission granted to an automation token.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum ApiScope {
    /// Read access to list and show endpoints.
    Read,
    /// Write access to mutating endpoints (implies read).
    Write,
}

impl ApiScope {
    /// Stable identifier used by routes and storage.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
        }
    }

    /// Parses a stable identifier.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "read" => Some(Self::Read),
            "write" => Some(Self::Write),
            _ => None,
        }
    }

    /// Every scope, for validation messages.
    pub const ALL: [Self; 2] = [Self::Read, Self::Write];
}

/// A token as shown to its owner (never includes the plaintext or hash).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ApiTokenSummary {
    /// Token identity.
    pub id: EntityId,
    /// Human-readable name.
    pub name: String,
    /// Granted scopes.
    pub scopes: Vec<ApiScope>,
    /// Creation instant.
    pub created_at: DateTime<Utc>,
    /// Last authenticated use, when any.
    pub last_used_at: Option<DateTime<Utc>>,
    /// Revocation instant, when revoked.
    pub revoked_at: Option<DateTime<Utc>>,
}

/// A freshly created/rotated token; the plaintext is returned exactly once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedApiToken {
    /// Token metadata.
    pub summary: ApiTokenSummary,
    /// View-once plaintext token.
    pub token: String,
}

/// Persistence for automation tokens.
#[async_trait]
pub trait ApiTokenStore: Send + Sync {
    /// Inserts a token record.
    async fn create(
        &self,
        account_id: AccountId,
        id: EntityId,
        name: &str,
        token_hash: &str,
        scopes: &[ApiScope],
        now: DateTime<Utc>,
    ) -> Result<ApiTokenSummary, CommandError>;

    /// Lists the account's tokens, newest first.
    async fn list(&self, account_id: AccountId) -> Result<Vec<ApiTokenSummary>, CommandError>;

    /// Revokes a token, returning whether it existed.
    async fn revoke(
        &self,
        account_id: AccountId,
        id: EntityId,
        now: DateTime<Utc>,
    ) -> Result<bool, CommandError>;

    /// Replaces a token's hash, returning its summary when it existed and was not revoked.
    async fn rotate(
        &self,
        account_id: AccountId,
        id: EntityId,
        token_hash: &str,
    ) -> Result<Option<ApiTokenSummary>, CommandError>;

    /// Resolves a token hash to its identity, account, scopes, and revocation state.
    async fn find_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<(EntityId, AccountId, Vec<ApiScope>, bool)>, CommandError>;

    /// Records a successful use.
    async fn touch(
        &self,
        account_id: AccountId,
        id: EntityId,
        now: DateTime<Utc>,
    ) -> Result<(), CommandError>;
}

/// Authenticated automation principal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiPrincipal {
    /// Token identity (for audit).
    pub token_id: EntityId,
    /// Owning account.
    pub account_id: AccountId,
    /// Granted scopes.
    pub scopes: Vec<ApiScope>,
}

impl ApiPrincipal {
    /// Whether the principal holds a scope (write implies read).
    #[must_use]
    pub fn has(&self, scope: ApiScope) -> bool {
        match scope {
            ApiScope::Read => {
                self.scopes.contains(&ApiScope::Read) || self.scopes.contains(&ApiScope::Write)
            }
            ApiScope::Write => self.scopes.contains(&ApiScope::Write),
        }
    }
}

/// Token lifecycle and authentication.
pub struct ApiTokenService {
    store: Arc<dyn ApiTokenStore>,
    tokens: Arc<dyn TokenGenerator>,
}

impl std::fmt::Debug for ApiTokenService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApiTokenService")
    }
}

impl ApiTokenService {
    /// Creates the service.
    #[must_use]
    pub fn new(store: Arc<dyn ApiTokenStore>, tokens: Arc<dyn TokenGenerator>) -> Self {
        Self { store, tokens }
    }

    /// Creates a token, returning the plaintext exactly once.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn create(
        &self,
        account_id: AccountId,
        name: &str,
        scopes: &[ApiScope],
        now: DateTime<Utc>,
    ) -> Result<CreatedApiToken, CommandError> {
        let token = self.tokens.generate();
        let hash = self.tokens.hash(&token);
        let summary = self
            .store
            .create(account_id, EntityId::new(), name, &hash, scopes, now)
            .await?;
        Ok(CreatedApiToken { summary, token })
    }

    /// Lists tokens.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn list(&self, account_id: AccountId) -> Result<Vec<ApiTokenSummary>, CommandError> {
        self.store.list(account_id).await
    }

    /// Revokes a token.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn revoke(
        &self,
        account_id: AccountId,
        id: EntityId,
        now: DateTime<Utc>,
    ) -> Result<bool, CommandError> {
        self.store.revoke(account_id, id, now).await
    }

    /// Rotates a token, returning the new plaintext exactly once.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn rotate(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<CreatedApiToken>, CommandError> {
        let token = self.tokens.generate();
        let hash = self.tokens.hash(&token);
        Ok(self
            .store
            .rotate(account_id, id, &hash)
            .await?
            .map(|summary| CreatedApiToken { summary, token }))
    }

    /// Authenticates a presented bearer token.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn authenticate(
        &self,
        presented: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<ApiPrincipal>, CommandError> {
        let hash = self.tokens.hash(presented);
        let Some((id, account_id, scopes, revoked)) = self.store.find_by_hash(&hash).await? else {
            return Ok(None);
        };
        if revoked {
            return Ok(None);
        }
        self.store.touch(account_id, id, now).await?;
        Ok(Some(ApiPrincipal {
            token_id: id,
            account_id,
            scopes,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiPrincipal, ApiScope};
    use momentum_domain::{AccountId, EntityId};

    #[test]
    fn scopes_parse_and_write_implies_read() {
        assert_eq!(ApiScope::parse("read"), Some(ApiScope::Read));
        assert_eq!(ApiScope::parse("write"), Some(ApiScope::Write));
        assert_eq!(ApiScope::parse("admin"), None);

        let read_only = ApiPrincipal {
            token_id: EntityId::new(),
            account_id: AccountId::new(),
            scopes: vec![ApiScope::Read],
        };
        assert!(read_only.has(ApiScope::Read));
        assert!(!read_only.has(ApiScope::Write));

        let writer = ApiPrincipal {
            token_id: EntityId::new(),
            account_id: AccountId::new(),
            scopes: vec![ApiScope::Write],
        };
        assert!(writer.has(ApiScope::Read));
        assert!(writer.has(ApiScope::Write));
    }
}
