# Tasks — bootstrap-fullstack-rust-workspace

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Workspace implementation

- [x] 2.1 Create the root Cargo workspace with members `crates/{domain,app,infra,web,testkit,xtask}` and shared workspace dependency/lint configuration.
- [x] 2.2 Implement `momentum-domain` as a compiling pure-Rust crate with crate-level lints and no workspace-local dependencies.
- [x] 2.3 Implement `momentum-app` with the application-layer crate skeleton depending only on `momentum-domain`.
- [x] 2.4 Implement `momentum-infra` with adapter-layer skeleton depending only on `domain`/`app`.
- [x] 2.5 Implement `momentum-testkit` fixtures plus the `workspace_topology` architecture test that rejects forbidden dependency direction.
- [x] 2.6 Implement `momentum-xtask` with `arch-check` and `web-build` subcommands used by scripts.

## 3. Full-stack Rust path

- [x] 3.1 Implement the shared Leptos view tree in `momentum-web` compiling under both `ssr` and `hydrate` features.
- [x] 3.2 Implement the Axum server binary with `/healthz`, `/readyz`, and an SSR application route.
- [x] 3.3 Add `scripts/build-web.sh` that builds the WASM client when the target is installed and reports a recorded non-fatal limitation otherwise.
- [x] 3.4 Confirm no handwritten JavaScript/TypeScript application logic exists in the repository.

## 4. Baseline quality gates

- [x] 4.1 Add `rustfmt.toml`, workspace lint configuration, and README build/run/test instructions.
- [x] 4.2 Add `scripts/quality-gates.sh` running fmt check, Clippy `-D warnings`, workspace tests, and OpenSpec strict validation.
- [x] 4.3 Add server smoke integration test (`/healthz` and `/` SSR assertions) and run it.
- [x] 4.4 Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace --all-features`.
- [x] 4.5 Attempt the WASM target build and record availability/result in `OVERNIGHT_REPORT.md`.
- [x] 4.6 Perform explicit artifact-to-implementation verification (no `/opsx:verify` skill installed) and record it in `OVERNIGHT_REPORT.md`.
- [x] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate bootstrap-fullstack-rust-workspace --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive bootstrap-fullstack-rust-workspace --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
