# Momentum

Momentum is an original-branding, Things-style personal task manager implemented as a
full-stack Rust web application. It is developed in this repository as a sequence of
OpenSpec-governed changes; see `openspec/` for the canonical specifications.

> Momentum is an independent project. It does not use Cultured Code trademarks, logos,
> marketing artwork, or proprietary icon assets. Observable workflow and information
> architecture are used as a behavioral reference only, with original branding and assets.

## Workspace layout

| Crate | Path | Responsibility |
|---|---|---|
| `momentum-domain` | `crates/domain` | Pure domain model and invariants |
| `momentum-app` | `crates/app` | Commands, queries, projections |
| `momentum-infra` | `crates/infra` | PostgreSQL/provider adapters, configuration |
| `momentum-web` | `crates/web` | Leptos UI, Axum server, WASM client |
| `momentum-testkit` | `crates/testkit` | Shared test support |
| `momentum-xtask` | `crates/xtask` | Workspace automation |

Dependency direction is one-way (`domain ← app ← infra ← web`) and enforced by
`cargo xtask arch-check`, which is also exercised by an integration test.

## Requirements

- Rust 1.85+ (`cargo`, `rustc`)
- OpenSpec CLI (`openspec`) for spec validation
- Optional: `wasm32-unknown-unknown` target for the browser client build

## Build and run

```bash
cargo build --workspace
cargo run -p momentum-web --bin momentum-server
# server listens on http://127.0.0.1:3000 (override with MOMENTUM_BIND_ADDR)
```

Endpoints:

- `GET /healthz` — liveness JSON
- `GET /readyz` — readiness JSON
- `GET /` — server-rendered application shell

## Quality gates

```bash
scripts/quality-gates.sh          # fmt, arch-check, OpenSpec specs, clippy, tests
scripts/quality-gates.sh --quick  # fmt, arch-check, OpenSpec specs only
scripts/build-web.sh              # WASM client build (reports missing target as SKIPPED)
```

Individual gates:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
openspec validate --specs --strict --no-interactive
```

## Source policy

Application and business logic is Rust. Handwritten JavaScript/TypeScript application code is
not permitted; browser behavior comes from Rust compiled to WASM plus toolchain-emitted glue.
