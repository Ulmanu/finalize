//! Workspace automation used by scripts and tests.
//!
//! Two tasks live here:
//!
//! * [`arch::check_workspace`] enforces the crate dependency direction required by the
//!   architecture invariants (domain ← app ← infra ← web, with test support as leaves).
//! * [`webbuild::build_web_client`] compiles the Leptos client crate for
//!   `wasm32-unknown-unknown` and reports an explicit, non-fatal limitation when the target
//!   is not installed.

#![forbid(unsafe_code)]

pub mod arch;
pub mod webbuild;
