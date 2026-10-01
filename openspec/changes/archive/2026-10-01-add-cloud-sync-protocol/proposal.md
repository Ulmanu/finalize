# Proposal

## Why

Offline replicas need a server protocol: push idempotent operations, pull changes after a
monotonic cursor, and resolve conflicts by explicit domain rules instead of destructive
last-write-wins.

## What Changes

- Add `momentum_app::sync`: `SyncRequest`/`SyncResponse` with ordered `SyncOperation`s (stable
  operation ids), `OperationAck` (sequence, replayed), `ChangeEntry` (sequence, events, entity),
  and a `ChangeLogReader` port over the existing per-account `change_log`.
- Push: each operation executes through the command dispatcher with its client operation id, so
  retries deduplicate by id; acknowledgements carry assigned sequences and replay flags.
- Pull: changes after the cursor return ascending with a monotonic new cursor.
- Conflict policy: explicit rules (`CompletionIsMonotonic`, `DeleteBeatsEdit`, `FieldVersion`,
  `OrderingRelative`, `Rejected`) with resolutions (`ServerWins`, `ClientRefreshes`, `Ignored`);
  edits to trashed entities are classified via a pre-check, ordering conflicts keep server ranks,
  and field version conflicts keep server values.
- Add `PgChangeLogReader` and a testkit memory reader, plus a JSON `POST /sync` route wired
  through `AppState::with_sync` and the server bootstrap.

## Capabilities

### New Capabilities

- `features/cloud-sync-protocol`: incremental push/pull, idempotent operation ids, conflict
  policy.

### Modified Capabilities

(none)

## Impact

- `crates/app` (sync module + unit tests), `crates/infra` (PG change-log reader), `crates/web`
  (route, state wiring, tests), `crates/testkit` (memory reader + protocol tests). No schema
  change (`change_log` already exists with a unique operation index).
- Non-goals: the hydrated client's delta application/cache merge is replica work (49) and later
  UI hardening; the wire protocol, dedupe, cursors, and conflict rules are complete.
