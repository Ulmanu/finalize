//! Authentication adapters: Argon2id hashing, random token generation, and PostgreSQL stores.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_app::auth::{
    AccountRecord, AccountStore, AuthError, AuthTokenKind, AuthTokenRecord, AuthTokenStore, Clock,
    PasswordHasher, SessionRecord, SessionStore, TokenGenerator,
};
use momentum_domain::ids::SessionId;
use momentum_domain::AccountId;
use sqlx::PgPool;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString};
use argon2::Argon2;

/// Argon2id password hasher.
#[derive(Debug, Default, Clone, Copy)]
pub struct Argon2Hasher;

impl PasswordHasher for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|error| AuthError::Storage(format!("password hashing failed: {error}")))
    }

    fn verify(&self, hash: &str, password: &str) -> bool {
        let Ok(parsed) = PasswordHash::new(hash) else {
            return false;
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }
}

/// Random opaque token generator using the OS random source and SHA-256 storage hashes.
#[derive(Debug, Default, Clone, Copy)]
pub struct RandomTokenGenerator;

impl TokenGenerator for RandomTokenGenerator {
    fn generate(&self) -> String {
        use rand::RngCore;

        let mut bytes = [0_u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn hash(&self, token: &str) -> String {
        use sha2::{Digest, Sha256};

        let digest = Sha256::digest(token.as_bytes());
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

/// System clock adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// PostgreSQL implementation of the auth store ports.
#[derive(Debug, Clone)]
pub struct PgAuthStore {
    pool: PgPool,
}

impl PgAuthStore {
    /// Creates a store over an existing pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Returns the store as the three port objects used by the auth service.
    #[must_use]
    pub fn ports(
        self: &Arc<Self>,
    ) -> (
        Arc<dyn AccountStore>,
        Arc<dyn SessionStore>,
        Arc<dyn AuthTokenStore>,
    ) {
        (self.clone(), self.clone(), self.clone())
    }
}

fn storage(error: sqlx::Error) -> AuthError {
    AuthError::Storage(error.to_string())
}

fn account_from_row(row: &sqlx::postgres::PgRow) -> AccountRecord {
    use sqlx::Row;

    AccountRecord {
        id: AccountId::from_uuid(row.get("id")),
        email: row.get("email"),
        email_normalized: row.get("email_normalized"),
        display_name: row.get("display_name"),
        password_hash: row.get("password_hash"),
        email_verified_at: row.get("email_verified_at"),
        created_at: row.get("created_at"),
        version: row.get("version"),
        deleted_at: row.get("deleted_at"),
    }
}

const ACCOUNT_COLUMNS: &str = "id, email, email_normalized, display_name, password_hash, \
     email_verified_at, created_at, version, deleted_at";

#[async_trait]
impl AccountStore for PgAuthStore {
    async fn find_by_email(&self, normalized: &str) -> Result<Option<AccountRecord>, AuthError> {
        let row = sqlx::query(&format!(
            "select {ACCOUNT_COLUMNS} from accounts where email_normalized = $1"
        ))
        .bind(normalized)
        .fetch_optional(&self.pool)
        .await
        .map_err(storage)?;
        Ok(row.as_ref().map(account_from_row))
    }

    async fn find_by_id(&self, id: AccountId) -> Result<Option<AccountRecord>, AuthError> {
        let row = sqlx::query(&format!(
            "select {ACCOUNT_COLUMNS} from accounts where id = $1"
        ))
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(storage)?;
        Ok(row.as_ref().map(account_from_row))
    }

    async fn insert(&self, account: &AccountRecord) -> Result<(), AuthError> {
        sqlx::query(
            "insert into accounts (id, email, email_normalized, display_name, password_hash, \
             email_verified_at, created_at, updated_at, version) \
             values ($1, $2, $3, $4, $5, $6, $7, $7, $8)",
        )
        .bind(account.id.as_uuid())
        .bind(&account.email)
        .bind(&account.email_normalized)
        .bind(&account.display_name)
        .bind(&account.password_hash)
        .bind(account.email_verified_at)
        .bind(account.created_at)
        .bind(account.version)
        .execute(&self.pool)
        .await
        .map_err(|error| match error {
            sqlx::Error::Database(ref db)
                if db.constraint() == Some("accounts_email_normalized_key") =>
            {
                AuthError::EmailTaken
            }
            other => storage(other),
        })?;
        Ok(())
    }

    async fn update_password(
        &self,
        id: AccountId,
        hash: &str,
        expected_version: i64,
        updated_at: DateTime<Utc>,
    ) -> Result<(), AuthError> {
        let result = sqlx::query(
            "update accounts set password_hash = $2, version = version + 1, updated_at = $3 \
             where id = $1 and version = $4",
        )
        .bind(id.as_uuid())
        .bind(hash)
        .bind(updated_at)
        .bind(expected_version)
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        if result.rows_affected() == 0 {
            return Err(AuthError::Storage("version conflict".to_string()));
        }
        Ok(())
    }

    async fn mark_verified(&self, id: AccountId, at: DateTime<Utc>) -> Result<(), AuthError> {
        sqlx::query(
            "update accounts set email_verified_at = coalesce(email_verified_at, $2), \
             version = version + 1, updated_at = $2 where id = $1",
        )
        .bind(id.as_uuid())
        .bind(at)
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn mark_deleted(&self, id: AccountId, at: DateTime<Utc>) -> Result<(), AuthError> {
        sqlx::query(
            "update accounts set deleted_at = $2, version = version + 1, updated_at = $2 \
             where id = $1",
        )
        .bind(id.as_uuid())
        .bind(at)
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        Ok(())
    }
}

fn session_from_row(row: &sqlx::postgres::PgRow) -> SessionRecord {
    use sqlx::Row;

    SessionRecord {
        id: SessionId::from_uuid(row.get("id")),
        account_id: AccountId::from_uuid(row.get("account_id")),
        token_hash: row.get("token_hash"),
        created_at: row.get("created_at"),
        expires_at: row.get("expires_at"),
        revoked_at: row.get("revoked_at"),
    }
}

const SESSION_COLUMNS: &str = "id, account_id, token_hash, created_at, expires_at, revoked_at";

#[async_trait]
impl SessionStore for PgAuthStore {
    async fn insert(&self, session: &SessionRecord) -> Result<(), AuthError> {
        sqlx::query(
            "insert into sessions (id, account_id, token_hash, created_at, expires_at) \
             values ($1, $2, $3, $4, $5)",
        )
        .bind(session.id.as_uuid())
        .bind(session.account_id.as_uuid())
        .bind(&session.token_hash)
        .bind(session.created_at)
        .bind(session.expires_at)
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn find_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<SessionRecord>, AuthError> {
        let row = sqlx::query(&format!(
            "select {SESSION_COLUMNS} from sessions where token_hash = $1"
        ))
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(storage)?;
        Ok(row.as_ref().map(session_from_row))
    }

    async fn revoke(&self, id: SessionId, at: DateTime<Utc>) -> Result<(), AuthError> {
        sqlx::query("update sessions set revoked_at = coalesce(revoked_at, $2) where id = $1")
            .bind(id.as_uuid())
            .bind(at)
            .execute(&self.pool)
            .await
            .map_err(storage)?;
        Ok(())
    }

    async fn revoke_all_for_account(
        &self,
        account_id: AccountId,
        at: DateTime<Utc>,
    ) -> Result<(), AuthError> {
        sqlx::query(
            "update sessions set revoked_at = coalesce(revoked_at, $2) \
             where account_id = $1 and revoked_at is null",
        )
        .bind(account_id.as_uuid())
        .bind(at)
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn list_for_account(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<SessionRecord>, AuthError> {
        let rows = sqlx::query(&format!(
            "select {SESSION_COLUMNS} from sessions where account_id = $1 order by created_at desc"
        ))
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(storage)?;
        Ok(rows.iter().map(session_from_row).collect())
    }
}

fn token_kind_from_str(value: &str) -> AuthTokenKind {
    match value {
        "password_reset" => AuthTokenKind::PasswordReset,
        _ => AuthTokenKind::EmailVerification,
    }
}

#[async_trait]
impl AuthTokenStore for PgAuthStore {
    async fn insert(&self, token: &AuthTokenRecord) -> Result<(), AuthError> {
        sqlx::query(
            "insert into auth_tokens (id, account_id, kind, token_hash, expires_at, consumed_at) \
             values ($1, $2, $3, $4, $5, $6)",
        )
        .bind(token.id)
        .bind(token.account_id.as_uuid())
        .bind(token.kind.as_str())
        .bind(&token.token_hash)
        .bind(token.expires_at)
        .bind(token.consumed_at)
        .execute(&self.pool)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn consume(
        &self,
        kind: AuthTokenKind,
        token_hash: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<AuthTokenRecord>, AuthError> {
        use sqlx::Row;

        let row = sqlx::query(
            "update auth_tokens set consumed_at = $3 \
             where kind = $1 and token_hash = $2 and consumed_at is null and expires_at > $3 \
             returning id, account_id, kind, token_hash, expires_at, consumed_at",
        )
        .bind(kind.as_str())
        .bind(token_hash)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(storage)?;

        Ok(row.map(|row| AuthTokenRecord {
            id: row.get("id"),
            account_id: AccountId::from_uuid(row.get("account_id")),
            kind: token_kind_from_str(row.get::<String, _>("kind").as_str()),
            token_hash: row.get("token_hash"),
            expires_at: row.get("expires_at"),
            consumed_at: row.get("consumed_at"),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argon2_hashes_verify_and_reject() {
        let hasher = Argon2Hasher;
        let hash = hasher.hash("correct horse battery").expect("hash");
        assert!(hash.starts_with("$argon2id$"), "must be argon2id: {hash}");
        assert!(hasher.verify(&hash, "correct horse battery"));
        assert!(!hasher.verify(&hash, "wrong password"));
        assert!(!hasher.verify("not-a-hash", "correct horse battery"));
    }

    #[test]
    fn argon2_salts_differ_between_hashes() {
        let hasher = Argon2Hasher;
        let first = hasher.hash("same password").expect("hash");
        let second = hasher.hash("same password").expect("hash");
        assert_ne!(first, second);
    }

    #[test]
    fn random_tokens_are_high_entropy_and_hashed_deterministically() {
        let generator = RandomTokenGenerator;
        let first = generator.generate();
        let second = generator.generate();
        assert_eq!(first.len(), 64);
        assert_ne!(first, second);
        assert_eq!(generator.hash(&first), generator.hash(&first));
        assert_ne!(generator.hash(&first), generator.hash(&second));
        assert_eq!(generator.hash(&first).len(), 64);
    }
}
