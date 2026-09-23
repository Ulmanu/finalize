# Design — add-persistence-foundation

## Context

PostgreSQL is the server-side source of truth. Offline clients create records before they
reach the server, so identifiers must be client-generatable (UUIDv7) and updates must carry
optimistic version metadata. Sync (change 50) needs a per-account sequence and change log, so
those tables are created now to avoid a later destructive migration.

## Requirements Driving the Design

- `Canonical persistence schema`
- `Stable identifiers and versions`
- `Migration safety`

## Architecture

- `crates/infra/src/db/mod.rs` — `Database` wrapper (`connect`, `health`, `pool`).
- `crates/infra/src/db/migrations.rs` — Rust migration engine:
  - `Migration { version, name, up, down: Option<&str>, irreversible_reason: Option<&str> }`.
  - `MIGRATIONS` ordered list; version 1 creates the canonical schema.
  - `apply(pool)` runs pending migrations in a transaction each, recording
    `schema_migrations(version, name, checksum, applied_at)`.
  - `rollback_last(pool)` executes the down SQL and deletes the record.
  - `pending(pool)` and `applied(pool)` for readiness/diagnostics.
  - FNV-1a checksum over the up SQL detects edited migrations.
- Schema conventions:
  - `uuid` primary keys (UUIDv7 from clients), `account_id` on every user-owned row,
    `version bigint not null default 1`, `created_at`/`updated_at timestamptz`.
  - Date-only concepts use `date` columns (`schedule_date`, `deadline`), never fake UTC
    midnight instants; reminder instants use `timestamptz` plus an IANA `timezone` column.
  - `placements` separates contextual ordering (`scope = 'context'`) from projection ordering
    (`scope = 'today'`), preserving the ordering-independence invariant.
  - `tombstones` records deletions for sync; `entities.status`/`deleted_at` provide soft
    deletion; `change_log` + `sync_metadata.sequence` provide the incremental sync spine.
- Configuration: `ServerConfig.database_url` from `DATABASE_URL`; absent means the server runs
  without persistence (foundation builds/tests still pass) and `/readyz` reports `ok` with
  `persistence: "disabled"`.

## Data and Persistence

This change is the persistence foundation. Migrations are forward-only at runtime; rollback is
an operator action used by tests. `0001` down SQL drops every created table (documented as
data-destroying and intended only for isolated development/test databases).

## Offline and Sync

Tables created here anticipate offline/sync: client-generated UUIDs, `version`, tombstones,
change log with per-account sequence, and operation ids for idempotent replay.

## UI and Accessibility

No UI change; `/readyz` gains a `persistence` field for operators.

## Error Handling

- `MigrationError` distinguishes connection, checksum mismatch, and SQL failures, including the
  failing version.
- No migration runs without a transaction; partial application cannot be recorded.
- Readiness never panics; a failing database check reports `degraded`.

## Testing Strategy

- Pure tests: ordering, uniqueness, non-empty SQL, rollback/irreversibility documented,
  checksum determinism.
- Conditional integration tests (`DATABASE_URL`): apply, idempotent re-apply, table existence,
  rollback. Skipped (recorded) when no database is available; this environment has no
  PostgreSQL server or Docker daemon.
- SQL text assertions: no `timestamp` used for date-only columns; tombstone/version columns
  present.

## Rollout / Migration

Greenfield. Applying to an existing development database is safe because the migration table is
created if absent and version 1 is only applied when not recorded.

## Open Questions Resolved From the Repository

- **sqlx version**: 0.8.6 (workspace) with runtime queries (no compile-time database needed).
- **Migration format**: Rust-defined SQL strings rather than external files so migrations ship
  in the binary and are testable without a database.
- **No Docker/Postgres in this environment**: integration tests are conditional; the migration
  engine's pure invariants are always tested.

## Tasks / Definition of Done

See `tasks.md`.
