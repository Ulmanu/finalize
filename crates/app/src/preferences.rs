//! Typed settings registry with declared scopes, defaults, and validation.
//!
//! Every preference declares its domain and whether it is account-synced, device-local, or
//! browser-local. Values are typed (boolean or choice) with defaults; persisted values are
//! sanitized on read so unknown or invalid data can never break startup.

use async_trait::async_trait;
use momentum_domain::AccountId;

use crate::commands::CommandError;

/// Where a preference lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferenceScope {
    /// Synchronized with the account.
    Account,
    /// Stored on this device by the client.
    Device,
    /// Stored in this browser by the client.
    Browser,
}

impl PreferenceScope {
    /// Stable identifier used by storage and markup.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Device => "device",
            Self::Browser => "browser",
        }
    }
}

/// Settings domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferenceDomain {
    /// General behavior.
    General,
    /// Appearance.
    Appearance,
    /// Today behavior.
    Today,
    /// Calendar behavior.
    Calendar,
    /// Notifications.
    Notifications,
    /// Quick Entry and keyboard behavior.
    QuickEntry,
    /// Logbook behavior.
    Logbook,
    /// Account, sync, and integrations.
    AccountSync,
    /// Data controls.
    Data,
}

impl PreferenceDomain {
    /// Stable identifier used by markup and tests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Appearance => "appearance",
            Self::Today => "today",
            Self::Calendar => "calendar",
            Self::Notifications => "notifications",
            Self::QuickEntry => "quick_entry",
            Self::Logbook => "logbook",
            Self::AccountSync => "account_sync",
            Self::Data => "data",
        }
    }

    /// Every domain, in display order.
    pub const ALL: [Self; 9] = [
        Self::General,
        Self::Appearance,
        Self::Today,
        Self::Calendar,
        Self::Notifications,
        Self::QuickEntry,
        Self::Logbook,
        Self::AccountSync,
        Self::Data,
    ];
}

/// A typed preference value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreferenceValue {
    /// A boolean toggle.
    Bool(bool),
    /// A choice from the definition's allow-list.
    Choice(String),
}

impl PreferenceValue {
    /// Canonical wire form.
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::Bool(value) => value.to_string(),
            Self::Choice(value) => value.clone(),
        }
    }
}

/// One declared preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreferenceDefinition {
    /// Stable key (`domain.name`).
    pub key: &'static str,
    /// Human-readable title.
    pub title: &'static str,
    /// Domain grouping.
    pub domain: PreferenceDomain,
    /// Storage scope.
    pub scope: PreferenceScope,
    /// Default value (always valid).
    pub default: PreferenceDefault,
    /// Allowed choices for choice preferences.
    pub choices: &'static [&'static str],
}

/// Compile-time default values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferenceDefault {
    /// Boolean default.
    Bool(bool),
    /// Choice default (must be in `choices`).
    Choice(&'static str),
}

const REGISTRY: &[PreferenceDefinition] = &[
    PreferenceDefinition {
        key: "general.week_start",
        title: "Week starts on",
        domain: PreferenceDomain::General,
        scope: PreferenceScope::Account,
        default: PreferenceDefault::Choice("monday"),
        choices: &["monday", "sunday"],
    },
    PreferenceDefinition {
        key: "general.default_list",
        title: "Default list after sign-in",
        domain: PreferenceDomain::General,
        scope: PreferenceScope::Device,
        default: PreferenceDefault::Choice("today"),
        choices: &["today", "inbox", "upcoming"],
    },
    PreferenceDefinition {
        key: "appearance.theme",
        title: "Theme",
        domain: PreferenceDomain::Appearance,
        scope: PreferenceScope::Browser,
        default: PreferenceDefault::Choice("system"),
        choices: &["system", "light", "dark"],
    },
    PreferenceDefinition {
        key: "appearance.density",
        title: "Density",
        domain: PreferenceDomain::Appearance,
        scope: PreferenceScope::Browser,
        default: PreferenceDefault::Choice("comfortable"),
        choices: &["comfortable", "compact"],
    },
    PreferenceDefinition {
        key: "today.show_evening",
        title: "Show This Evening",
        domain: PreferenceDomain::Today,
        scope: PreferenceScope::Account,
        default: PreferenceDefault::Bool(true),
        choices: &[],
    },
    PreferenceDefinition {
        key: "today.progress",
        title: "Show progress",
        domain: PreferenceDomain::Today,
        scope: PreferenceScope::Account,
        default: PreferenceDefault::Bool(true),
        choices: &[],
    },
    PreferenceDefinition {
        key: "calendar.show_events",
        title: "Show calendar events",
        domain: PreferenceDomain::Calendar,
        scope: PreferenceScope::Account,
        default: PreferenceDefault::Bool(true),
        choices: &[],
    },
    PreferenceDefinition {
        key: "notifications.reminders",
        title: "Reminder notifications",
        domain: PreferenceDomain::Notifications,
        scope: PreferenceScope::Browser,
        default: PreferenceDefault::Bool(true),
        choices: &[],
    },
    PreferenceDefinition {
        key: "quick_entry.after_add",
        title: "Keep Quick Entry open after adding",
        domain: PreferenceDomain::QuickEntry,
        scope: PreferenceScope::Device,
        default: PreferenceDefault::Bool(false),
        choices: &[],
    },
    PreferenceDefinition {
        key: "logbook.manual",
        title: "Log completed items manually",
        domain: PreferenceDomain::Logbook,
        scope: PreferenceScope::Account,
        default: PreferenceDefault::Bool(false),
        choices: &[],
    },
    PreferenceDefinition {
        key: "account.sync_enabled",
        title: "Sync this device",
        domain: PreferenceDomain::AccountSync,
        scope: PreferenceScope::Device,
        default: PreferenceDefault::Bool(true),
        choices: &[],
    },
    PreferenceDefinition {
        key: "data.auto_backup",
        title: "Weekly backup reminder",
        domain: PreferenceDomain::Data,
        scope: PreferenceScope::Device,
        default: PreferenceDefault::Bool(false),
        choices: &[],
    },
];

/// Every declared preference.
#[must_use]
pub const fn registry() -> &'static [PreferenceDefinition] {
    REGISTRY
}

/// Looks up a preference definition.
#[must_use]
pub fn definition(key: &str) -> Option<&'static PreferenceDefinition> {
    REGISTRY.iter().find(|definition| definition.key == key)
}

/// The compile-time default for a definition.
#[must_use]
pub fn default_value(definition: &PreferenceDefinition) -> PreferenceValue {
    match definition.default {
        PreferenceDefault::Bool(value) => PreferenceValue::Bool(value),
        PreferenceDefault::Choice(value) => PreferenceValue::Choice(value.to_string()),
    }
}

/// Parses a raw value against a definition; unknown keys or invalid values are rejected.
#[must_use]
pub fn parse_value(key: &str, raw: &str) -> Option<PreferenceValue> {
    let definition = definition(key)?;
    match definition.default {
        PreferenceDefault::Bool(_) => match raw {
            "true" => Some(PreferenceValue::Bool(true)),
            "false" => Some(PreferenceValue::Bool(false)),
            _ => None,
        },
        PreferenceDefault::Choice(_) => definition
            .choices
            .contains(&raw)
            .then(|| PreferenceValue::Choice(raw.to_string())),
    }
}

/// Sanitizes a persisted value, falling back to the default so bad data cannot break startup.
#[must_use]
pub fn sanitize(key: &str, raw: Option<&str>) -> Option<PreferenceValue> {
    let definition = definition(key)?;
    raw.and_then(|raw| parse_value(key, raw))
        .or_else(|| Some(default_value(definition)))
}

/// Persistence for account-synced preferences (device/browser scopes are client-side).
#[async_trait]
pub trait PreferenceStore: Send + Sync {
    /// Loads every persisted account preference as `(key, raw)`.
    async fn load_all(&self, account_id: AccountId) -> Result<Vec<(String, String)>, CommandError>;

    /// Persists one account preference.
    async fn save(
        &self,
        account_id: AccountId,
        key: &str,
        scope: PreferenceScope,
        raw: &str,
    ) -> Result<(), CommandError>;
}

/// Resolves the effective value of a preference for an account.
///
/// # Errors
///
/// Propagates store failures.
pub async fn effective(
    store: &dyn PreferenceStore,
    account_id: AccountId,
    key: &str,
) -> Result<Option<PreferenceValue>, CommandError> {
    let Some(definition) = definition(key) else {
        return Ok(None);
    };
    if definition.scope != PreferenceScope::Account {
        return Ok(sanitize(key, None));
    }
    let stored = store
        .load_all(account_id)
        .await?
        .into_iter()
        .find(|(stored_key, _)| stored_key == key)
        .map(|(_, raw)| raw);
    Ok(sanitize(key, stored.as_deref()))
}

#[cfg(test)]
mod tests {
    use super::{
        default_value, definition, parse_value, registry, sanitize, PreferenceDomain,
        PreferenceScope, PreferenceValue,
    };

    #[test]
    fn registry_is_unique_valid_and_covers_every_domain() {
        let mut keys: Vec<&str> = registry().iter().map(|definition| definition.key).collect();
        let total = keys.len();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), total, "duplicate preference keys");
        for definition in registry() {
            match definition.default {
                super::PreferenceDefault::Bool(_) => {
                    assert!(definition.choices.is_empty(), "{}", definition.key);
                }
                super::PreferenceDefault::Choice(value) => {
                    assert!(
                        definition.choices.contains(&value),
                        "{} default not in choices",
                        definition.key
                    );
                }
            }
            assert!(!definition.title.is_empty());
        }
        for domain in PreferenceDomain::ALL {
            assert!(
                registry()
                    .iter()
                    .any(|definition| definition.domain == domain),
                "no preference for {}",
                domain.as_str()
            );
        }
        assert!(registry()
            .iter()
            .any(|definition| definition.scope == PreferenceScope::Browser));
        assert!(registry()
            .iter()
            .any(|definition| definition.scope == PreferenceScope::Device));
        assert!(registry()
            .iter()
            .any(|definition| definition.scope == PreferenceScope::Account));
    }

    #[test]
    fn parsing_rejects_unknown_keys_and_values_and_sanitizes_fallbacks() {
        assert_eq!(
            parse_value("appearance.theme", "dark"),
            Some(PreferenceValue::Choice("dark".to_string()))
        );
        assert_eq!(parse_value("appearance.theme", "neon"), None);
        assert_eq!(
            parse_value("today.progress", "false"),
            Some(PreferenceValue::Bool(false))
        );
        assert_eq!(parse_value("today.progress", "yes"), None);
        assert_eq!(parse_value("nope", "true"), None);

        assert_eq!(
            sanitize("appearance.theme", Some("neon")),
            Some(default_value(
                definition("appearance.theme").expect("definition")
            ))
        );
        assert_eq!(
            sanitize("today.progress", None),
            Some(PreferenceValue::Bool(true))
        );
        assert_eq!(sanitize("nope", Some("x")), None);
    }
}
