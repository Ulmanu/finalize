//! Account authentication: lifecycle commands over store ports.
//!
//! The service owns the rules (validation, hashing delegation, token lifecycle, session
//! expiry, uniform failure behavior). Adapters (PostgreSQL, in-memory) implement the ports;
//! password hashing and token generation are ports so the application layer stays free of
//! crypto and I/O dependencies.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use momentum_domain::ids::SessionId;
use momentum_domain::{AccountId, Email, EmailError, Password, PasswordError};
use thiserror::Error;

/// How long a session remains valid without renewal.
pub const SESSION_TTL_DAYS: i64 = 30;

/// How long an email verification token remains valid.
pub const VERIFICATION_TTL_HOURS: i64 = 24;

/// How long a password reset token remains valid.
pub const RESET_TTL_HOURS: i64 = 1;

/// Typed authentication failures.
#[derive(Debug, Error)]
pub enum AuthError {
    /// The email address failed validation.
    #[error("invalid email address")]
    InvalidEmail(#[from] EmailError),
    /// The password failed policy validation.
    #[error("password does not meet the policy")]
    WeakPassword(#[from] PasswordError),
    /// The normalized email is already registered (including soft-deleted accounts).
    #[error("email address is already registered")]
    EmailTaken,
    /// Credentials did not match; deliberately uniform for unknown email and wrong password.
    #[error("invalid credentials")]
    InvalidCredentials,
    /// Credentials were valid but the email has not been verified.
    #[error("email address is not verified")]
    EmailNotVerified,
    /// The session token is unknown, expired, or revoked.
    #[error("session is not valid")]
    InvalidSession,
    /// The verification/reset token is unknown, expired, or already used.
    #[error("authentication token is invalid or expired")]
    InvalidToken,
    /// The account does not exist or is deleted.
    #[error("account not found")]
    AccountNotFound,
    /// The storage adapter failed.
    #[error("authentication storage error")]
    Storage(String),
}

/// Kind of single-use authentication token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthTokenKind {
    /// Email verification token.
    EmailVerification,
    /// Password reset token.
    PasswordReset,
}

impl AuthTokenKind {
    /// Stable database representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmailVerification => "email_verification",
            Self::PasswordReset => "password_reset",
        }
    }
}

/// Persisted account row.
#[derive(Debug, Clone)]
pub struct AccountRecord {
    /// Canonical identity.
    pub id: AccountId,
    /// User-visible email.
    pub email: String,
    /// Normalized email used for lookup/uniqueness.
    pub email_normalized: String,
    /// Optional display name.
    pub display_name: Option<String>,
    /// Argon2id hash; `None` only for accounts created by future invitation flows.
    pub password_hash: Option<String>,
    /// When the email was verified.
    pub email_verified_at: Option<DateTime<Utc>>,
    /// Creation instant.
    pub created_at: DateTime<Utc>,
    /// Optimistic version.
    pub version: i64,
    /// Soft-deletion instant.
    pub deleted_at: Option<DateTime<Utc>>,
}

impl AccountRecord {
    /// Whether the account is deleted.
    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Whether the email has been verified.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.email_verified_at.is_some()
    }
}

/// Persisted session row.
#[derive(Debug, Clone)]
pub struct SessionRecord {
    /// Session identity.
    pub id: SessionId,
    /// Owning account.
    pub account_id: AccountId,
    /// SHA-256 hash of the opaque session token.
    pub token_hash: String,
    /// Creation instant.
    pub created_at: DateTime<Utc>,
    /// Expiry instant.
    pub expires_at: DateTime<Utc>,
    /// Revocation instant.
    pub revoked_at: Option<DateTime<Utc>>,
}

impl SessionRecord {
    /// Whether the session is currently usable at `now`.
    #[must_use]
    pub fn is_active(&self, now: DateTime<Utc>) -> bool {
        self.revoked_at.is_none() && self.expires_at > now
    }
}

/// Persisted single-use auth token row.
#[derive(Debug, Clone)]
pub struct AuthTokenRecord {
    /// Token identity.
    pub id: uuid::Uuid,
    /// Owning account.
    pub account_id: AccountId,
    /// Token kind.
    pub kind: AuthTokenKind,
    /// SHA-256 hash of the opaque token.
    pub token_hash: String,
    /// Expiry instant.
    pub expires_at: DateTime<Utc>,
    /// Consumption instant.
    pub consumed_at: Option<DateTime<Utc>>,
}

/// Account persistence port.
#[async_trait]
pub trait AccountStore: Send + Sync {
    /// Finds an account by normalized email.
    async fn find_by_email(&self, normalized: &str) -> Result<Option<AccountRecord>, AuthError>;
    /// Finds an account by identity.
    async fn find_by_id(&self, id: AccountId) -> Result<Option<AccountRecord>, AuthError>;
    /// Inserts a new account.
    async fn insert(&self, account: &AccountRecord) -> Result<(), AuthError>;
    /// Replaces the password hash using optimistic concurrency.
    async fn update_password(
        &self,
        id: AccountId,
        hash: &str,
        expected_version: i64,
        updated_at: DateTime<Utc>,
    ) -> Result<(), AuthError>;
    /// Marks the email verified.
    async fn mark_verified(&self, id: AccountId, at: DateTime<Utc>) -> Result<(), AuthError>;
    /// Marks the account deleted.
    async fn mark_deleted(&self, id: AccountId, at: DateTime<Utc>) -> Result<(), AuthError>;
}

/// Session persistence port.
#[async_trait]
pub trait SessionStore: Send + Sync {
    /// Inserts a session.
    async fn insert(&self, session: &SessionRecord) -> Result<(), AuthError>;
    /// Finds a session by token hash.
    async fn find_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<SessionRecord>, AuthError>;
    /// Revokes a single session.
    async fn revoke(&self, id: SessionId, at: DateTime<Utc>) -> Result<(), AuthError>;
    /// Revokes every session for an account.
    async fn revoke_all_for_account(
        &self,
        account_id: AccountId,
        at: DateTime<Utc>,
    ) -> Result<(), AuthError>;
    /// Lists active sessions for an account.
    async fn list_for_account(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<SessionRecord>, AuthError>;
}

/// Single-use token persistence port.
#[async_trait]
pub trait AuthTokenStore: Send + Sync {
    /// Inserts a token.
    async fn insert(&self, token: &AuthTokenRecord) -> Result<(), AuthError>;
    /// Consumes a token atomically, returning it when valid at `now`.
    async fn consume(
        &self,
        kind: AuthTokenKind,
        token_hash: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<AuthTokenRecord>, AuthError>;
}

/// Password hashing port.
pub trait PasswordHasher: Send + Sync {
    /// Hashes a password with a fresh salt.
    fn hash(&self, password: &str) -> Result<String, AuthError>;
    /// Verifies a password against a stored hash.
    fn verify(&self, hash: &str, password: &str) -> bool;
}

/// Opaque token generation port.
pub trait TokenGenerator: Send + Sync {
    /// Generates a high-entropy opaque token.
    fn generate(&self) -> String;
    /// Hashes a token for storage and lookup.
    fn hash(&self, token: &str) -> String;
}

/// Time source port.
pub trait Clock: Send + Sync {
    /// Returns the current instant.
    fn now(&self) -> DateTime<Utc>;
}

/// Successful registration result.
#[derive(Debug, Clone)]
pub struct RegistrationOutcome {
    /// Created account identity.
    pub account_id: AccountId,
    /// Verification token to deliver out of band.
    pub verification_token: String,
}

/// Successful login result.
#[derive(Debug, Clone)]
pub struct LoginOutcome {
    /// Account summary.
    pub account: AccountRecord,
    /// Session identity.
    pub session_id: SessionId,
    /// Opaque session token for the cookie.
    pub token: String,
    /// Session expiry.
    pub expires_at: DateTime<Utc>,
}

/// Password reset request result.
#[derive(Debug, Clone)]
pub struct ResetRequestOutcome {
    /// Reset token to deliver out of band.
    pub reset_token: String,
    /// Expiry instant.
    pub expires_at: DateTime<Utc>,
}

/// Account authentication service.
pub struct AuthService {
    accounts: Arc<dyn AccountStore>,
    sessions: Arc<dyn SessionStore>,
    tokens: Arc<dyn AuthTokenStore>,
    hasher: Arc<dyn PasswordHasher>,
    token_generator: Arc<dyn TokenGenerator>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for AuthService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AuthService")
    }
}

impl AuthService {
    /// Creates the service from its ports.
    #[must_use]
    pub fn new(
        accounts: Arc<dyn AccountStore>,
        sessions: Arc<dyn SessionStore>,
        tokens: Arc<dyn AuthTokenStore>,
        hasher: Arc<dyn PasswordHasher>,
        token_generator: Arc<dyn TokenGenerator>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            accounts,
            sessions,
            tokens,
            hasher,
            token_generator,
            clock,
        }
    }

    /// Registers a new account and issues an email verification token.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::EmailTaken`] for an existing normalized email (including deleted
    /// accounts), and validation errors for invalid input.
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        display_name: Option<String>,
    ) -> Result<RegistrationOutcome, AuthError> {
        let email = Email::parse(email)?;
        let password = Password::parse_for_email(password, email.normalized())?;

        if self
            .accounts
            .find_by_email(email.normalized())
            .await?
            .is_some()
        {
            return Err(AuthError::EmailTaken);
        }

        let now = self.clock.now();
        let account = AccountRecord {
            id: AccountId::new(),
            email: email.as_str().to_string(),
            email_normalized: email.normalized().to_string(),
            display_name: display_name.filter(|name| !name.trim().is_empty()),
            password_hash: Some(self.hasher.hash(password.expose())?),
            email_verified_at: None,
            created_at: now,
            version: 1,
            deleted_at: None,
        };
        self.accounts.insert(&account).await?;

        let token = self
            .issue_token(
                account.id,
                AuthTokenKind::EmailVerification,
                VERIFICATION_TTL_HOURS,
            )
            .await?;

        Ok(RegistrationOutcome {
            account_id: account.id,
            verification_token: token,
        })
    }

    /// Verifies an email address using a single-use token.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidToken`] for unknown, expired, or consumed tokens.
    pub async fn verify_email(&self, token: &str) -> Result<AccountId, AuthError> {
        let now = self.clock.now();
        let record = self
            .tokens
            .consume(
                AuthTokenKind::EmailVerification,
                &self.token_generator.hash(token),
                now,
            )
            .await?
            .ok_or(AuthError::InvalidToken)?;

        self.accounts.mark_verified(record.account_id, now).await?;
        Ok(record.account_id)
    }

    /// Authenticates credentials and creates a session.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidCredentials`] for unknown email or wrong password (uniform),
    /// and [`AuthError::EmailNotVerified`] when credentials are valid but verification is
    /// pending.
    pub async fn login(&self, email: &str, password: &str) -> Result<LoginOutcome, AuthError> {
        let email = Email::parse(email)?;
        let account = self
            .accounts
            .find_by_email(email.normalized())
            .await?
            .filter(|account| !account.is_deleted())
            .ok_or(AuthError::InvalidCredentials)?;

        let Some(hash) = account.password_hash.as_deref() else {
            return Err(AuthError::InvalidCredentials);
        };
        if !self.hasher.verify(hash, password) {
            return Err(AuthError::InvalidCredentials);
        }
        if !account.is_verified() {
            return Err(AuthError::EmailNotVerified);
        }

        let now = self.clock.now();
        let token = self.token_generator.generate();
        let session = SessionRecord {
            id: SessionId::new(),
            account_id: account.id,
            token_hash: self.token_generator.hash(&token),
            created_at: now,
            expires_at: now + Duration::days(SESSION_TTL_DAYS),
            revoked_at: None,
        };
        self.sessions.insert(&session).await?;

        Ok(LoginOutcome {
            account,
            session_id: session.id,
            token,
            expires_at: session.expires_at,
        })
    }

    /// Validates a session token and returns the owning account.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidSession`] for unknown, expired, or revoked tokens and for
    /// deleted accounts.
    pub async fn authenticate_session(&self, token: &str) -> Result<AccountRecord, AuthError> {
        let now = self.clock.now();
        let session = self
            .sessions
            .find_by_token_hash(&self.token_generator.hash(token))
            .await?
            .filter(|session| session.is_active(now))
            .ok_or(AuthError::InvalidSession)?;

        let account = self
            .accounts
            .find_by_id(session.account_id)
            .await?
            .filter(|account| !account.is_deleted())
            .ok_or(AuthError::InvalidSession)?;
        Ok(account)
    }

    /// Revokes the session identified by its opaque token.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidSession`] when the token is unknown or already revoked.
    pub async fn logout(&self, token: &str) -> Result<(), AuthError> {
        let now = self.clock.now();
        let session = self
            .sessions
            .find_by_token_hash(&self.token_generator.hash(token))
            .await?
            .filter(|session| session.is_active(now))
            .ok_or(AuthError::InvalidSession)?;
        self.sessions.revoke(session.id, now).await
    }

    /// Issues a password reset token when the account exists.
    ///
    /// Unknown emails return `Ok(None)` so callers cannot enumerate accounts.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn request_password_reset(
        &self,
        email: &str,
    ) -> Result<Option<ResetRequestOutcome>, AuthError> {
        let email = Email::parse(email)?;
        let Some(account) = self
            .accounts
            .find_by_email(email.normalized())
            .await?
            .filter(|account| !account.is_deleted())
        else {
            return Ok(None);
        };

        let token = self
            .issue_token(account.id, AuthTokenKind::PasswordReset, RESET_TTL_HOURS)
            .await?;
        Ok(Some(ResetRequestOutcome {
            reset_token: token,
            expires_at: self.clock.now() + Duration::hours(RESET_TTL_HOURS),
        }))
    }

    /// Completes a password reset and revokes every session.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidToken`] for invalid tokens and password policy errors for
    /// weak replacements.
    pub async fn reset_password(
        &self,
        token: &str,
        new_password: &str,
    ) -> Result<AccountId, AuthError> {
        let now = self.clock.now();
        let record = self
            .tokens
            .consume(
                AuthTokenKind::PasswordReset,
                &self.token_generator.hash(token),
                now,
            )
            .await?
            .ok_or(AuthError::InvalidToken)?;

        let account = self
            .accounts
            .find_by_id(record.account_id)
            .await?
            .filter(|account| !account.is_deleted())
            .ok_or(AuthError::AccountNotFound)?;

        let password = Password::parse_for_email(new_password, &account.email_normalized)?;
        let hash = self.hasher.hash(password.expose())?;
        self.accounts
            .update_password(account.id, &hash, account.version, now)
            .await?;
        // Completing a reset proves control of the mailbox, so the email is verified.
        self.accounts.mark_verified(account.id, now).await?;
        self.sessions
            .revoke_all_for_account(account.id, now)
            .await?;
        Ok(account.id)
    }

    /// Changes the password for an authenticated account and revokes every session.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidCredentials`] when the current password is wrong.
    pub async fn change_password(
        &self,
        account_id: AccountId,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), AuthError> {
        let account = self
            .accounts
            .find_by_id(account_id)
            .await?
            .filter(|account| !account.is_deleted())
            .ok_or(AuthError::AccountNotFound)?;

        let Some(hash) = account.password_hash.as_deref() else {
            return Err(AuthError::InvalidCredentials);
        };
        if !self.hasher.verify(hash, current_password) {
            return Err(AuthError::InvalidCredentials);
        }

        let password = Password::parse_for_email(new_password, &account.email_normalized)?;
        let new_hash = self.hasher.hash(password.expose())?;
        let now = self.clock.now();
        self.accounts
            .update_password(account.id, &new_hash, account.version, now)
            .await?;
        self.sessions.revoke_all_for_account(account.id, now).await
    }

    /// Revokes a specific session owned by the account.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidSession`] when the session does not belong to the account.
    pub async fn revoke_session(
        &self,
        account_id: AccountId,
        session_id: SessionId,
    ) -> Result<(), AuthError> {
        let sessions = self.sessions.list_for_account(account_id).await?;
        let session = sessions
            .iter()
            .find(|session| session.id == session_id)
            .ok_or(AuthError::InvalidSession)?;
        self.sessions.revoke(session.id, self.clock.now()).await
    }

    /// Deletes an account after verifying its password, revoking all sessions.
    ///
    /// # Errors
    ///
    /// Returns [`AuthError::InvalidCredentials`] when the password is wrong.
    pub async fn delete_account(
        &self,
        account_id: AccountId,
        password: &str,
    ) -> Result<(), AuthError> {
        let account = self
            .accounts
            .find_by_id(account_id)
            .await?
            .filter(|account| !account.is_deleted())
            .ok_or(AuthError::AccountNotFound)?;

        let Some(hash) = account.password_hash.as_deref() else {
            return Err(AuthError::InvalidCredentials);
        };
        if !self.hasher.verify(hash, password) {
            return Err(AuthError::InvalidCredentials);
        }

        let now = self.clock.now();
        self.accounts.mark_deleted(account.id, now).await?;
        self.sessions.revoke_all_for_account(account.id, now).await
    }

    /// Lists active sessions for account settings.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn list_sessions(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<SessionRecord>, AuthError> {
        Ok(self
            .sessions
            .list_for_account(account_id)
            .await?
            .into_iter()
            .filter(|session| session.is_active(self.clock.now()))
            .collect())
    }

    async fn issue_token(
        &self,
        account_id: AccountId,
        kind: AuthTokenKind,
        ttl_hours: i64,
    ) -> Result<String, AuthError> {
        let token = self.token_generator.generate();
        let now = self.clock.now();
        self.tokens
            .insert(&AuthTokenRecord {
                id: uuid::Uuid::now_v7(),
                account_id,
                kind,
                token_hash: self.token_generator.hash(&token),
                expires_at: now + Duration::hours(ttl_hours),
                consumed_at: None,
            })
            .await?;
        Ok(token)
    }
}
