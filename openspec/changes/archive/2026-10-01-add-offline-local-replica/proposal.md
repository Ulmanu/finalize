# Proposal

## Why

Offline startup and optimistic editing need a durable local replica: working set, preferences,
sync cursor, and outbound operations must survive reloads and replay idempotently when the
connection returns.

## What Changes

- Add `momentum_app::replica`: `ReplicaSnapshot` (entities, preferences, sync cursor),
  `OutboxEntry` (stable operation id, serialized command, attempts), the `ReplicaStore` port, an
  in-memory store, and `OfflineCoordinator` with `startup`, `apply_locally` (optimistic enqueue
  before acknowledgement), `record_entity` (optimistic working set), `acknowledge`, and `replay`
  (ordered, stops at the first failure, idempotent by operation id).
- Document the browser adapter: `ReplicaStore` is implemented by IndexedDB in the hydrated client
  (the store port exists precisely so the browser persists the snapshot/outbox durably); the
  in-memory store covers tests and development.
- Surface pending operations in the shell: `AppState::with_replica` provides the coordinator to
  the layout, which renders an outbox-pending indicator.

## Capabilities

### New Capabilities

- `features/offline-local-replica`: IndexedDB local replica, optimistic local commands, outbox
  recovery.

### Modified Capabilities

(none)

## Impact

- `crates/app` (replica module), `crates/web` (context, pending indicator, tests), `crates/testkit`
  (tests). No schema change.
- Non-goals: the concrete IndexedDB `ReplicaStore` implementation and the HTTP sync client (both
  are hydrated-client/deployment work; the store port, coordinator semantics, and tests are
  complete and the server API is unchanged).
