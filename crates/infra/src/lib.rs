//! Infrastructure adapters for Momentum.
//!
//! This crate owns PostgreSQL/sqlx repositories, the Rust-managed migration runner, and
//! provider-specific integrations. Those arrive with the persistence roadmap change; this
//! foundation provides runtime configuration that the server reads from the environment.

#![forbid(unsafe_code)]

pub mod config;

pub use config::{ConfigError, ServerConfig};
