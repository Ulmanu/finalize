//! Runtime configuration adapters.

use std::env;
use std::net::SocketAddr;

use thiserror::Error;

/// Environment variable that overrides the default bind address.
pub const BIND_ADDR_ENV: &str = "MOMENTUM_BIND_ADDR";

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
        Ok(Self { bind_addr })
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
