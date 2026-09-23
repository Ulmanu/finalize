# Proposal

## Why

There is no mutation path yet: nothing enforces that changes go through one validated,
atomic, account-scoped, idempotent command layer. Every later feature (CRUD, projections,
sync, automation) must share that layer or invariants will drift.

## What Changes

- Add `momentum-app::commands`: typed `Command` variants, `CommandEnvelope` with a stable
  operation id, typed `CommandError`, `DomainEvent` values, `Projection` invalidation targets,
  and a `CommandDispatcher` that executes commands against a `UnitOfWork`.
- Execute each command inside one transaction: load the account-scoped entity, apply domain
  rules with optimistic concurrency, persist the entity, append change-log metadata with a
  per-account sequence, and return projection invalidations plus events.
- Make execution idempotent: a replayed operation id returns the recorded outcome without
  re-applying side effects.
- Enforce account scoping: commands for entities owned by another account fail as not-found.
- Add a PostgreSQL `UnitOfWork` in `momentum-infra` (entities + change log + sync sequence in
  one transaction) and an in-memory `UnitOfWork` in `momentum-testkit` for deterministic tests.

## Capabilities

### New Capabilities

- `app/commands`: typed command layer with atomic transactions, deterministic errors, events,
  projection invalidation, and idempotent sync hooks.

### Modified Capabilities

(none)

## Impact

- `crates/app` (commands module), `crates/infra` (PG unit of work), `crates/testkit`
  (in-memory unit of work + tests).
- No schema change (change 05 already has `entities`, `change_log`, `sync_metadata`).
- Non-goals: no UI wiring (changes 11+), no full sync protocol (change 50), no undo (change
  10), no recurrence materialization (change 41).
