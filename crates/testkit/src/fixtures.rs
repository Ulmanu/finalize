//! Canonical fixtures used by tests.

use momentum_domain::BuildInfo;

/// Returns the build identity fixture used by server and application tests.
#[must_use]
pub const fn build_info() -> BuildInfo {
    BuildInfo::current()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_matches_domain_identity() {
        assert_eq!(build_info(), BuildInfo::current());
    }
}
