//! Session contract shared by the server, the WASM client, and tests.
//!
//! Authentication (registration, cookies, sessions) arrives in the account-authentication
//! change. This module defines the canonical state that the shell uses to decide between the
//! public and authenticated layouts, so route and layout code does not change when the real
//! cookie-backed resolver is added.

use momentum_domain::AccountId;
use serde::{Deserialize, Serialize};

/// Authenticated account summary exposed to the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountSession {
    /// Canonical account identity.
    pub account_id: AccountId,
    /// Account email used for sign-in.
    pub email: String,
    /// Optional display name.
    pub display_name: Option<String>,
}

impl AccountSession {
    /// Creates a session summary.
    #[must_use]
    pub fn new(account_id: AccountId, email: impl Into<String>) -> Self {
        Self {
            account_id,
            email: email.into(),
            display_name: None,
        }
    }
}

/// Whether the current request has an authenticated account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum SessionState {
    /// No authenticated account is present.
    Anonymous,
    /// An authenticated account is present.
    Authenticated(AccountSession),
}

impl SessionState {
    /// Returns the authenticated account summary, if any.
    #[must_use]
    pub const fn account(&self) -> Option<&AccountSession> {
        match self {
            Self::Anonymous => None,
            Self::Authenticated(account) => Some(account),
        }
    }

    /// Returns whether an account is authenticated.
    #[must_use]
    pub const fn is_authenticated(&self) -> bool {
        self.account().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anonymous_has_no_account() {
        assert!(!SessionState::Anonymous.is_authenticated());
        assert!(SessionState::Anonymous.account().is_none());
    }

    #[test]
    fn authenticated_round_trips_through_json() {
        let state = SessionState::Authenticated(AccountSession::new(
            AccountId::new(),
            "person@example.com",
        ));
        let json = serde_json::to_string(&state).expect("serialize");
        let decoded: SessionState = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, state);
    }

    #[test]
    fn anonymous_serializes_with_tag() {
        let json = serde_json::to_string(&SessionState::Anonymous).expect("serialize");
        assert_eq!(json, "{\"state\":\"anonymous\"}");
    }
}
