# Proposal

## Why

The repository currently contains no Rust code, so no later roadmap capability can be
implemented, tested, or validated. This change establishes the full-stack Rust foundation:
a Cargo workspace with enforced dependency direction, a Leptos/Axum server plus Rust/WASM
client path, and repeatable quality gates that an unattended agent can run for every
subsequent change.

## What Changes

- Add the `momentum` Cargo workspace with crates: `momentum-domain`, `momentum-app`,
  `momentum-infra`, `momentum-web` (Leptos UI + Axum server + WASM client), and
  `momentum-testkit` (test support), plus a workspace-level `xtask`-style quality runner.
- Add a minimal but real Axum server that renders a Leptos SSR page and exposes health
  endpoints, with a `hydrate` client build path for the same Rust view code.
- Add repeatable quality commands: `scripts/quality-gates.sh` (fmt, clippy, tests,
  OpenSpec strict validation) and `scripts/build-web.sh` (WASM build attempt with clear
  reporting when the WASM target is unavailable).
- Add an architecture test that fails if workspace dependency direction is violated
  (domain/app must not depend on web/infra; infra must not depend on web).
- Add `rustfmt.toml`, workspace lint configuration, and README build/run instructions.
- Record the original-branding policy ("Momentum") and Rust-only source policy in the
  repository documentation.

## Capabilities

### New Capabilities

- `platform/rust-workspace`: Cargo workspace topology with enforced dependency direction,
  full-stack Rust server/WASM execution path, and repeatable baseline quality gates.

### Modified Capabilities

(none)

## Impact

- New files: root `Cargo.toml`, `rustfmt.toml`, `README.md`, `scripts/`, `crates/*`.
- No persistence, no database schema, no user data yet; no migration impact.
- `cargo build`, `cargo test --workspace`, `cargo clippy`, and `cargo fmt` become available
  as repository quality gates; OpenSpec strict validation remains required per change.
- Non-goals: no feature-domain behavior (tasks, lists, auth) is implemented here; no
  handwritten JavaScript/TypeScript; no Cultured Code branding or assets.
