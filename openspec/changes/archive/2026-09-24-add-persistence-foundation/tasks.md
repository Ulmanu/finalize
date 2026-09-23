# Tasks — add-persistence-foundation

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Persistence foundation

- [x] 2.1 Add sqlx to `momentum-infra` with a `Database` wrapper (connect, health, pool).
- [x] 2.2 Implement the Rust migration engine (ordered versions, checksums, transactional apply, rollback, pending/applied queries).
- [x] 2.3 Author migration 0001 creating accounts, sessions, entities, placements, tag assignments, checklist items, recurrence templates/instances, reminders, preferences, sync metadata, change log, and tombstones with UUID/version/timestamp conventions.
- [x] 2.4 Add `DATABASE_URL` configuration and wire `/readyz` persistence reporting.

## 3. Tests and verification

- [x] 3.1 Add pure migration-invariant tests (ordering, uniqueness, non-empty SQL, checksum determinism, documented rollback/irreversibility).
- [x] 3.2 Add schema-text assertions (date-only columns use `date`, version/tombstone columns exist, account scoping columns exist).
- [x] 3.3 Add conditional live PostgreSQL integration tests (apply, idempotent re-apply, table existence, rollback) that report a recorded skip without `DATABASE_URL`.
- [x] 3.4 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 3.5 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md` (including the unverified live-database path).
- [x] 3.6 Resolve all critical/actionable verification findings.

## 4. Archive and commit

- [x] 4.1 Re-run `openspec validate add-persistence-foundation --strict --no-interactive`.
- [x] 4.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 4.3 Archive with `openspec archive add-persistence-foundation --yes` without bypassing validation.
- [x] 4.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 4.5 Review the full Git diff for accidental/unrelated changes.
- [x] 4.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
