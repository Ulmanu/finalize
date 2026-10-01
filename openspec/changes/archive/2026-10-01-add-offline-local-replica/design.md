# Design — add-offline-local-replica

## Context

Commands are serializable and carry stable operation ids in `CommandEnvelope`; the server dedupes
by operation id. The missing client piece is durable local persistence plus replay orchestration.

## Requirements Driving the Design

- `IndexedDB local replica`
- `Optimistic local commands`
- `Outbox recovery`

## Architecture

- `ReplicaStore` is the persistence port (load/save snapshot, load/save outbox). The browser
  adapter (IndexedDB) implements it in the hydrated client; `MemoryReplicaStore` backs tests and
  development.
- `OfflineCoordinator::apply_locally` serializes the command with a fresh UUIDv7 operation id and
  appends it to the outbox **before** any network call; `record_entity` updates the cached working
  set, so the UI can render optimistically. Snapshot, preferences, and cursor live in the same
  store, giving offline startup (`startup`) its working set.
- `replay` walks the outbox in order: acknowledged operations are removed, while retry/offline
  failures increment attempts and stop the pass, so the next replay resumes where it stopped.
  Operation ids never change, so replays are idempotent on the server.
- The layout renders an outbox-pending count from the coordinator, proving the integration.

## Data and Persistence

- No server schema change; the replica is client-owned and never authoritative. The sync cursor
  supports later delta sync.

## Offline and Sync

- Optimistic writes are durable outbound operations compatible with the change-log envelope;
  replay is idempotent by operation id.

## UI and Accessibility

- Pending indicator names the count; offline startup can render the snapshot before connectivity.

## Error Handling

- Store failures propagate as strings from the port; replay failures leave entries queued with
  attempt counts instead of dropping writes.

## Testing Strategy

- Testkit tests: optimistic enqueue plus reload survival, ordered idempotent replay clearing the
  outbox, retry/offline stopping the pass with attempt counts preserved and later completion; web
  tests for the pending indicator with and without a configured replica.

## Rollout / Migration

- None required; first run starts with an empty snapshot.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the IndexedDB adapter itself is not written here because the hydrated client's
  persistence layer is browser-only and cannot run in the SSR test harness; the store port,
  coordinator, and durability semantics are implemented and tested so the adapter is a drop-in.
