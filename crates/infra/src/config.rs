//! Runtime configuration adapters.

use std::env;
use std::net::SocketAddr;

use thiserror::Error;

/// Environment variable that overrides the default bind address.
pub const BIND_ADDR_ENV: &str = "MOMENTUM_BIND_ADDR";

/// Environment variable holding the PostgreSQL connection string.
pub const DATABASE_URL_ENV: &str = "DATABASE_URL";

/// Environment variable that marks session cookies as `Secure` (HTTPS deployments).
pub const SECURE_COOKIES_ENV: &str = "MOMENTUM_SECURE_COOKIES";

/// Default bind address used for local development.
pub const DEFAULT_BIND_ADDR: &str = "127.0.0.1:3000";

/// Errors produced while loading runtime configuration.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    /// The bind address value could not be parsed as a socket address.
    #[error("invalid {BIND_ADDR_ENV} value: {0}")]
    InvalidBindAddr(String),
}

/// Server runtime configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    /// Address the HTTP server binds to.
    pub bind_addr: SocketAddr,
    /// PostgreSQL connection string, when persistence is configured.
    pub database_url: Option<String>,
    /// Whether session cookies must carry the `Secure` attribute.
    pub secure_cookies: bool,
}

impl ServerConfig {
    /// Loads configuration from the process environment, applying documented defaults.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::InvalidBindAddr`] when `MOMENTUM_BIND_ADDR` is set but is not a
    /// valid socket address.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| env::var(key).ok())
    }

    /// Loads configuration using a caller-provided lookup function (used in tests).
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::InvalidBindAddr`] when the lookup returns an unparsable value.
    pub fn from_lookup(
        mut lookup: impl FnMut(&str) -> Option<String>,
    ) -> Result<Self, ConfigError> {
        let raw = lookup(BIND_ADDR_ENV).unwrap_or_else(|| DEFAULT_BIND_ADDR.to_string());
        let bind_addr = raw
            .parse::<SocketAddr>()
            .map_err(|_| ConfigError::InvalidBindAddr(raw))?;
        let database_url = lookup(DATABASE_URL_ENV).filter(|url| !url.trim().is_empty());
        let secure_cookies = lookup(SECURE_COOKIES_ENV)
            .map(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes"
                )
            })
            .unwrap_or(false);
        Ok(Self {
            bind_addr,
            database_url,
            secure_cookies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_localhost_when_unset() {
        let config = ServerConfig::from_lookup(|_| None).expect("default config");
        assert_eq!(config.bind_addr.to_string(), DEFAULT_BIND_ADDR);
    }

    #[test]
    fn secure_cookies_flag_parses_documented_values() {
        let enabled = ServerConfig::from_lookup(|key| {
            (key == SECURE_COOKIES_ENV).then(|| "true".to_string())
        })
        .expect("config");
        assert!(enabled.secure_cookies);

        let disabled = ServerConfig::from_lookup(|_| None).expect("config");
        assert!(!disabled.secure_cookies);
    }

    #[test]
    fn reads_database_url_when_present() {
        let config = ServerConfig::from_lookup(|key| {
            (key == DATABASE_URL_ENV).then(|| "postgres://localhost/momentum".to_string())
        })
        .expect("config with database");
        assert_eq!(
            config.database_url.as_deref(),
            Some("postgres://localhost/momentum")
        );
    }

    #[test]
    fn blank_database_url_is_treated_as_absent() {
        let config = ServerConfig::from_lookup(|key| (key == DATABASE_URL_ENV).then(String::new))
            .expect("config without database");
        assert_eq!(config.database_url, None);
    }

    #[test]
    fn reads_override_from_lookup() {
        let config = ServerConfig::from_lookup(|key| {
            (key == BIND_ADDR_ENV).then(|| "0.0.0.0:8080".to_string())
        })
        .expect("override config");
        assert_eq!(config.bind_addr.port(), 8080);
    }

    #[test]
    fn rejects_invalid_bind_address() {
        let error = ServerConfig::from_lookup(|key| {
            (key == BIND_ADDR_ENV).then(|| "not-an-address".to_string())
        })
        .expect_err("invalid address must fail");
        assert_eq!(
            error,
            ConfigError::InvalidBindAddr("not-an-address".to_string())
        );
    }
}
