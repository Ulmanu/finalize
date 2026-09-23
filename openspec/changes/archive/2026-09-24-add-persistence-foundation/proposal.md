# Proposal

## Why

No persistence exists yet, so nothing survives a restart and later capabilities (auth, tasks,
sync, offline) have no durable schema. This change establishes the canonical PostgreSQL schema,
Rust-managed migrations with checksums and rollback, and stable identifier/version conventions
that offline creation and synchronization depend on.

## What Changes

- Add `momentum-infra::db` with a Rust migration engine: ordered migrations, checksum
  verification, transactional apply, rollback of the last migration, and a recorded migration
  table.
- Add migration `0001` creating the canonical schema: accounts, sessions, entities,
  placements, tag assignments, checklist items, recurrence templates/instances, reminders,
  preferences, sync metadata, change log, and tombstones.
- Use UUIDv7 stable identifiers, `version` optimistic-concurrency columns, `updated_at`
  timestamps, and deletion/tombstone fields on persisted user entities.
- Add optional `DATABASE_URL` configuration; `/readyz` reports database readiness when a pool
  is configured and remains available without one (no persistence configured).
- Add migration tests: pure invariants always run; live PostgreSQL application/rollback tests
  run when `DATABASE_URL` is present and otherwise report a recorded environmental skip.

## Capabilities

### New Capabilities

- `persistence/schema`: canonical PostgreSQL schema, Rust-managed migrations with safety
  guarantees, and stable identifiers/versions for offline and sync use.

### Modified Capabilities

(none)

## Impact

- `crates/infra` (sqlx, migration engine, database wrapper), `crates/infra/src/config.rs`,
  `crates/web/src/server.rs` readiness, tests.
- New dependency: `sqlx` (postgres, runtime-tokio-rustls, uuid, chrono).
- Non-goals: no repository implementations or command layer yet (changes 07/09), no live
  PostgreSQL server available in this environment (integration tests are conditional and
  recorded), no handwritten JS/TS.
