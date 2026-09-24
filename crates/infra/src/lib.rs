//! Infrastructure adapters for Momentum.
//!
//! This crate owns PostgreSQL/sqlx repositories, the Rust-managed migration runner, and
//! provider-specific integrations. Those arrive with the persistence roadmap change; this
//! foundation provides runtime configuration that the server reads from the environment.

#![forbid(unsafe_code)]

pub mod areas;
pub mod auth;
pub mod commands;
pub mod config;
pub mod db;
pub mod projects;
pub mod tags;
pub mod todos;

pub use areas::PgAreaReadStore;
pub use auth::{Argon2Hasher, PgAuthStore, RandomTokenGenerator, SystemClock};
pub use commands::PgUnitOfWorkFactory;
pub use config::{ConfigError, ServerConfig};
pub use db::{Database, DbError, Migration, MigrationError, MIGRATIONS};
pub use projects::PgProjectReadStore;
pub use tags::PgTagReadStore;
pub use todos::PgTodoReadStore;
