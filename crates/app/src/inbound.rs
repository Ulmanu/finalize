//! Inbound email capture: private aliases, deterministic mail mapping, abuse protection.
//!
//! Each account owns a high-entropy alias that can be rotated or disabled. Inbound mail maps
//! subject to title and supported body text to notes through deterministic normalization; size
//! limits, a sliding-window rate limit, and a persisted message-id ledger protect against abuse
//! and duplicates.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use momentum_domain::{AccountId, EntityId};
use uuid::Uuid;

use crate::capture::sanitize_text;
use crate::commands::CommandError;
use crate::todos::TodoService;

/// Domain used for alias addresses.
pub const INBOUND_DOMAIN: &str = "inbox.momentum.example";

/// A high-entropy inbound alias.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundAlias {
    /// Secret token.
    pub token: String,
    /// Whether the alias currently accepts mail.
    pub enabled: bool,
}

impl InboundAlias {
    /// The full email address.
    #[must_use]
    pub fn address(&self) -> String {
        format!("{}@{}", self.token, INBOUND_DOMAIN)
    }
}

/// Alias persistence.
#[async_trait]
pub trait AliasStore: Send + Sync {
    /// The account's most recent alias, when any exists.
    async fn alias_for(&self, account_id: AccountId) -> Result<Option<InboundAlias>, CommandError>;

    /// Resolves a token to its account and enabled state.
    async fn lookup_token(&self, token: &str) -> Result<Option<(AccountId, bool)>, CommandError>;

    /// Replaces the account's alias with a new token (enabled).
    async fn rotate(
        &self,
        account_id: AccountId,
        token: &str,
    ) -> Result<InboundAlias, CommandError>;

    /// Disables the account's alias without deleting it.
    async fn disable(&self, account_id: AccountId) -> Result<(), CommandError>;
}

/// Persisted message-id ledger for idempotent ingress.
#[async_trait]
pub trait MessageLedger: Send + Sync {
    /// Whether the message was already ingested for the account.
    async fn seen(&self, account_id: AccountId, message_id: &str) -> Result<bool, CommandError>;

    /// Records an ingested message id.
    async fn record(&self, account_id: AccountId, message_id: &str) -> Result<(), CommandError>;
}

/// Source of alias tokens.
pub trait TokenSource: Send + Sync {
    /// Produces a fresh high-entropy token.
    fn token(&self) -> String;
}

/// CSPRNG tokens: 128 random bits rendered as 32 hex characters.
#[derive(Debug, Default)]
pub struct RandomTokenSource;

impl TokenSource for RandomTokenSource {
    fn token(&self) -> String {
        Uuid::new_v4().simple().to_string()
    }
}

/// Abuse limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InboundLimits {
    /// Maximum accepted messages per hour per account.
    pub per_hour: usize,
    /// Maximum subject bytes.
    pub max_subject: usize,
    /// Maximum body bytes.
    pub max_body: usize,
}

impl Default for InboundLimits {
    fn default() -> Self {
        Self {
            per_hour: 30,
            max_subject: 200,
            max_body: 65_536,
        }
    }
}

/// One inbound message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundMail {
    /// RFC message id used for deduplication.
    pub message_id: String,
    /// Mail subject.
    pub subject: String,
    /// Plain-text body.
    pub body: String,
}

/// Result of one ingress attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestOutcome {
    /// A to-do was created.
    Delivered {
        /// Created to-do identity.
        todo: EntityId,
    },
    /// The message id was already ingested.
    Duplicate,
    /// The token is unknown.
    UnknownToken,
    /// The alias is disabled.
    Disabled,
    /// The account exceeded its rate limit.
    RateLimited,
    /// The message exceeded size limits.
    TooLarge,
}

/// Deterministic subject/body normalization: quoted reply lines dropped, whitespace collapsed,
/// title fallback applied, lengths bounded.
#[must_use]
pub fn map_mail(subject: &str, body: &str) -> (String, String) {
    let title = sanitize_text(subject);
    let title = if title.is_empty() {
        "Email".to_string()
    } else {
        title
            .chars()
            .take(InboundLimits::default().max_subject)
            .collect()
    };
    let body = body
        .lines()
        .map(str::trim_start)
        .filter(|line| !line.starts_with('>'))
        .collect::<Vec<_>>()
        .join(" ");
    (title, sanitize_text(&body))
}

/// Ingress service.
pub struct InboundService {
    aliases: Arc<dyn AliasStore>,
    ledger: Arc<dyn MessageLedger>,
    todos: Arc<TodoService>,
    tokens: Arc<dyn TokenSource>,
    limits: InboundLimits,
    recent: Mutex<HashMap<AccountId, Vec<DateTime<Utc>>>>,
}

impl std::fmt::Debug for InboundService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InboundService")
    }
}

impl InboundService {
    /// Creates the service.
    #[must_use]
    pub fn new(
        aliases: Arc<dyn AliasStore>,
        ledger: Arc<dyn MessageLedger>,
        todos: Arc<TodoService>,
        tokens: Arc<dyn TokenSource>,
        limits: InboundLimits,
    ) -> Self {
        Self {
            aliases,
            ledger,
            todos,
            tokens,
            limits,
            recent: Mutex::new(HashMap::new()),
        }
    }

    /// Returns the account's alias, creating one on first use.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn ensure_alias(&self, account_id: AccountId) -> Result<InboundAlias, CommandError> {
        if let Some(alias) = self.aliases.alias_for(account_id).await? {
            return Ok(alias);
        }
        self.aliases.rotate(account_id, &self.tokens.token()).await
    }

    /// Rotates the alias to a fresh token.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn rotate(&self, account_id: AccountId) -> Result<InboundAlias, CommandError> {
        self.aliases.rotate(account_id, &self.tokens.token()).await
    }

    /// Disables the account's alias.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn disable(&self, account_id: AccountId) -> Result<(), CommandError> {
        self.aliases.disable(account_id).await
    }

    /// Ingests one message addressed to `token`.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn ingest(
        &self,
        token: &str,
        mail: &InboundMail,
        now: DateTime<Utc>,
    ) -> Result<IngestOutcome, CommandError> {
        if mail.subject.len() > self.limits.max_subject || mail.body.len() > self.limits.max_body {
            return Ok(IngestOutcome::TooLarge);
        }
        let Some((account_id, enabled)) = self.aliases.lookup_token(token).await? else {
            return Ok(IngestOutcome::UnknownToken);
        };
        if !enabled {
            return Ok(IngestOutcome::Disabled);
        }
        if self.ledger.seen(account_id, &mail.message_id).await? {
            return Ok(IngestOutcome::Duplicate);
        }
        {
            let mut recent = self.recent.lock().expect("rate lock");
            let window = recent.entry(account_id).or_default();
            window.retain(|at| now - *at < Duration::hours(1));
            if window.len() >= self.limits.per_hour {
                return Ok(IngestOutcome::RateLimited);
            }
            window.push(now);
        }
        let (title, notes) = map_mail(&mail.subject, &mail.body);
        let notes = (!notes.is_empty()).then_some(notes);
        let outcome = self
            .todos
            .create_in(
                account_id,
                &title,
                notes,
                momentum_domain::model::ContextRef::Inbox,
            )
            .await?;
        self.ledger.record(account_id, &mail.message_id).await?;
        Ok(IngestOutcome::Delivered {
            todo: outcome.entity_id.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{map_mail, InboundLimits};

    #[test]
    fn mail_mapping_is_deterministic_and_strips_quotes() {
        let (title, notes) = map_mail(
            "  Weekly\u{7} report ",
            "First line\n> quoted reply\nSecond\tline\n\n",
        );
        assert_eq!(title, "Weekly report");
        assert_eq!(notes, "First line Second line");

        let (title, notes) = map_mail("", "");
        assert_eq!(title, "Email");
        assert!(notes.is_empty());
    }

    #[test]
    fn subject_is_bounded_by_the_configured_limit() {
        let long = "s".repeat(InboundLimits::default().max_subject + 50);
        let (title, _) = map_mail(&long, "");
        assert_eq!(title.len(), InboundLimits::default().max_subject);
    }
}
