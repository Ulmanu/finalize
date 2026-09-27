//! Pure domain model for Momentum.
//!
//! This crate holds entities, value objects, and invariants. It deliberately has no
//! dependency on UI frameworks, database drivers, or browser APIs so that domain rules can
//! be unit-tested and reused by every adapter.
//!
//! The first roadmap change only establishes the crate and product identity; task, project,
//! area, ordering, scheduling, and recurrence models are added by later changes.

#![forbid(unsafe_code)]

pub mod email;
pub mod ids;
pub mod markdown;
pub mod model;
pub mod natural_date;
pub mod notes;
pub mod password;
pub mod time;

pub use email::{Email, EmailError};
pub use ids::{AccountId, ChecklistItemId, EntityId, SessionId};
pub use password::{Password, PasswordError};

/// Original product name used for all user-visible branding.
///
/// Momentum is an original product; it does not use Cultured Code trademarks, logos, or
/// proprietary assets.
pub const PRODUCT_NAME: &str = "Momentum";

/// Build and product identity shared by application, infrastructure, and web layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildInfo {
    /// User-visible product name.
    pub product: &'static str,
    /// Cargo package version of the domain crate.
    pub version: &'static str,
}

impl BuildInfo {
    /// Returns the build identity compiled into this binary.
    #[must_use]
    pub const fn current() -> Self {
        Self {
            product: PRODUCT_NAME,
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_info_reports_product_identity() {
        let info = BuildInfo::current();
        assert_eq!(info.product, PRODUCT_NAME);
        assert!(!info.version.is_empty());
    }
}
