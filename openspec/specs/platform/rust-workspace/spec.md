# platform/rust-workspace Specification

## Purpose

Provide the Cargo workspace foundation for Momentum: a dependency-direction-enforced Rust
workspace, a full-stack Rust execution path (Axum server plus Rust/WASM client), and
repeatable baseline quality gates that every later capability builds on.

## Requirements

### Requirement: Rust workspace topology

The system SHALL provide a Cargo workspace separating web UI, domain/application logic,
persistence/infrastructure, and test support so dependency direction can be enforced.

#### Scenario: Workspace builds from a clean checkout

- **GIVEN** a clean checkout of the repository with no build artifacts
- **WHEN** the workspace is built with `cargo build --workspace`
- **THEN** every workspace crate compiles successfully
- **AND** the domain and application crates do not depend on the web or infrastructure crates

#### Scenario: Dependency direction violation is rejected

- **GIVEN** the workspace manifest graph
- **WHEN** the architecture test inspects crate dependencies
- **THEN** a violation of the allowed dependency direction fails the test with the offending crate named
- **AND** unrelated workspace state is left unchanged

### Requirement: Full-stack Rust execution path

The web application SHALL use Rust for server logic and Rust-to-WASM for client application
logic, with no handwritten JavaScript or TypeScript business/application layer.

#### Scenario: Server binary is produced from Rust sources

- **GIVEN** the workspace on a supported native target
- **WHEN** the server crate is built and started
- **THEN** it serves a server-rendered application page and a health endpoint produced by Rust code

#### Scenario: Client build path compiles Rust for the browser

- **GIVEN** the `wasm32-unknown-unknown` target is installed
- **WHEN** the web crate is compiled for the browser target with the hydration feature
- **THEN** compilation succeeds without any handwritten JavaScript or TypeScript sources
- **AND** when the target is unavailable, the build script reports the missing target as a recorded,
  non-fatal environmental limitation instead of silently skipping the check

### Requirement: Baseline quality gates

The repository SHALL expose repeatable formatting, linting, test, and web-build commands
suitable for unattended validation.

#### Scenario: Quality gate script runs the baseline suite

- **GIVEN** a developer or agent shell in the repository root
- **WHEN** `scripts/quality-gates.sh` is executed
- **THEN** it runs formatting check, Clippy with warnings denied, workspace tests, and OpenSpec strict validation
- **AND** it exits non-zero if any gate fails

#### Scenario: Formatting and lint configuration is enforced

- **GIVEN** workspace sources that violate formatting or deny-level Clippy lints
- **WHEN** `cargo fmt --all -- --check` or `cargo clippy --workspace --all-targets --all-features -- -D warnings` runs
- **THEN** the command fails and identifies the offending file or lint
