# Design — bootstrap-fullstack-rust-workspace

## Context

The repository starts empty. Everything later depends on a stable workspace shape, so this
change chooses crate boundaries that make the roadmap's invariants enforceable in code:
domain invariants are pure Rust, authoritative mutations pass through an application command
layer, and adapters (PostgreSQL, browser APIs, providers) stay behind traits.

## Requirements Driving the Design

- `Rust workspace topology`
- `Full-stack Rust execution path`
- `Baseline quality gates`

## Architecture

Workspace members (product name: **Momentum**, original branding):

| Crate | Path | Responsibility | May depend on |
|---|---|---|---|
| `momentum-domain` | `crates/domain` | Entities, value objects, invariants, projection rules | nothing workspace-local |
| `momentum-app` | `crates/app` | Commands, queries, projections, ordering, undo, sync protocol types | domain |
| `momentum-infra` | `crates/infra` | PostgreSQL/sqlx repositories, migration runner, provider adapters | domain, app |
| `momentum-web` | `crates/web` | Leptos components, routes, Axum server, WASM hydration | domain, app, infra |
| `momentum-testkit` | `crates/testkit` | Fixtures, in-memory repositories, assertions | domain, app |
| `momentum-xtask` | `crates/xtask` | Workspace automation: architecture check, web build | none |

Dependency direction is acyclic and one-way: `domain ← app ← infra ← web`, with `testkit`
and `xtask` as leaves. The architecture test in `crates/testkit/tests/workspace_topology.rs`
parses each member manifest and asserts the allowed edges, so a future change that adds a
forbidden dependency fails CI-style validation.

## Full-stack execution path

- `momentum-web` exposes one shared view tree (`app()`) compiled twice:
  - `ssr` feature → server rendering via `leptos::ssr` and `leptos_axum`.
  - `hydrate` feature → WASM client for the same tree.
- The Axum server (`crates/web/src/server.rs`, binary `momentum-server`) mounts:
  - `GET /healthz` → `{"status":"ok"}` (liveness),
  - `GET /readyz` → readiness placeholder for later persistence checks,
  - `/` and Leptos SSR routes → server-rendered HTML.
- No handwritten JS/TS exists in the repository. `scripts/build-web.sh` builds the WASM
  artifact with `cargo build -p momentum-web --target wasm32-unknown-unknown --features hydrate`
  when the target is installed, and reports a clear non-fatal message when it is not. The
  script never fabricates success.

## Data and Persistence

No persistence in this change. `momentum-infra` exists with a placeholder module documenting
that migrations arrive in change 05; it must still compile under `--all-features`.

## Offline and Sync

Not applicable yet. Command/query type shapes introduced later will be designed for
outbox/change-log reuse; no storage is created now.

## UI and Accessibility

The SSR page renders a semantic `<main>` landmark with a product heading and a
`data-testid` hook for tests. Leptos components are structured so the design system
(change 03) can replace inline markup. Keyboard/focus behavior is minimal but semantic HTML
is used from the start.

## Error Handling

- Server routes return typed HTTP status codes; no panic paths in request handling.
- `momentum-web` compiles with `#![forbid(unsafe_code)]` and workspace lints; Clippy runs with
  `-D warnings`.
- WASM build failures are surfaced by the script with the underlying cargo output.

## Testing Strategy

- `momentum-testkit` integration test `workspace_topology` — dependency direction invariants.
- `momentum-web` integration test `server_smoke` — starts the Axum router in-process and asserts
  `/healthz` returns ok and `/` returns HTML containing the product name.
- `cargo test --workspace --all-features` is the baseline gate; `scripts/quality-gates.sh`
  additionally runs fmt/clippy/OpenSpec validation.

## Rollout / Migration

Greenfield: safe to apply. No data formats exist yet, so no migration or compatibility
strategy is required at this stage.

## Open Questions Resolved From the Repository

- **Toolchain**: Homebrew Rust 1.95 with no rustup; `wasm32-unknown-unknown` is installed via a
  rustup bootstrap in the background. `scripts/build-web.sh` degrades to a recorded warning
  when the target is absent, and the overnight report records the actual state.
- **Database**: no local PostgreSQL/Docker daemon at bootstrap; persistence tests in later
  changes use an in-memory adapter plus conditional `DATABASE_URL` integration tests.
- **Product name**: `Momentum` (original branding; avoids Cultured Code trademarks).

## Tasks / Definition of Done

See `tasks.md`.
