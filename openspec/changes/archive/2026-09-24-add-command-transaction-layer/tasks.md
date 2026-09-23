# Tasks — add-command-transaction-layer

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Command layer

- [x] 2.1 Define typed commands, envelopes, events, projection targets, and typed errors.
- [x] 2.2 Define the `UnitOfWork`/`UnitOfWorkFactory` ports with replay lookup and change-record append.
- [x] 2.3 Implement `CommandDispatcher` with atomic execution, optimistic concurrency, event emission, invalidation, and idempotent replay.

## 3. Adapters

- [x] 3.1 Implement `PgUnitOfWork` in `momentum-infra` (entity upsert + change log + sequence in one transaction).
- [x] 3.2 Implement `MemoryDatabase`/`MemoryUnitOfWork` in `momentum-testkit` with failure injection.

## 4. Tests and verification

- [x] 4.1 Add dispatcher tests for every command variant (events, invalidations, state).
- [x] 4.2 Add idempotent-replay, version-conflict, account-isolation, and failure-injection tests.
- [x] 4.3 Add a conditional PostgreSQL unit-of-work integration test.
- [x] 4.4 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 4.5 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md` (including the unverified live-PostgreSQL path).
- [x] 4.6 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-command-transaction-layer --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive add-command-transaction-layer --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
