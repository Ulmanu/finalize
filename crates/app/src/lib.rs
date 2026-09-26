//! Application layer for Momentum: commands, queries, and projections.
//!
//! Every authoritative mutation in later roadmap changes enters through typed commands
//! defined here. This crate depends only on [`momentum_domain`]; adapters (PostgreSQL,
//! browser storage, providers) implement traits exposed by this layer.

#![forbid(unsafe_code)]

pub mod areas;
pub mod auth;
pub mod commands;
pub mod logbook;
pub mod ordering;
pub mod projects;
pub mod recurring;
pub mod session;
pub mod tags;
pub mod todos;
pub mod undo;

use momentum_domain::BuildInfo;
use serde::Serialize;

/// Health status reported by the application layer for liveness/readiness probes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    /// The application layer is initialized and able to serve requests.
    Ok,
}

impl HealthStatus {
    /// Machine-readable status label used in HTTP responses.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
        }
    }
}

/// Returns the current application health status.
#[must_use]
pub const fn health() -> HealthStatus {
    HealthStatus::Ok
}

/// Returns product/build identity for diagnostics and user-visible about surfaces.
#[must_use]
pub const fn build_info() -> BuildInfo {
    BuildInfo::current()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_reports_ok_label() {
        assert_eq!(health().as_str(), "ok");
    }

    #[test]
    fn build_info_delegates_to_domain_identity() {
        assert_eq!(build_info().product, momentum_domain::PRODUCT_NAME);
    }
}
