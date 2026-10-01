# Design — add-cloud-sync-protocol

## Context

`CommandEnvelope` carries stable operation ids and the dispatcher already deduplicates by them;
`change_log` stores a per-account monotonic sequence with a unique `(account, operation_id)`
index. The missing layer was the client-facing push/pull envelope and an explicit conflict policy.

## Requirements Driving the Design

- `Incremental push/pull protocol`
- `Idempotent operation IDs`
- `Conflict policy`

## Architecture

- `SyncService::sync` executes pushed operations in order through the dispatcher using the
  client's operation id; acknowledgements echo the assigned sequence and `replayed` flag. Changes
  after the cursor are read through `ChangeLogReader` (PG joins `change_log`; memory reads the
  test database) and returned ascending with `cursor = max(sequence, request.cursor)`.
- Conflicts never fall back to generic last-write-wins: `classify_conflict` maps
  `CommandError::Conflict` to `FieldVersion`/`ServerWins` (or `OrderingRelative`/`Ignored` for
  ordering commands), `CommandError::NotFound` to `DeleteBeatsEdit`/`ClientRefreshes`, and other
  rejections to `Rejected`. A pre-check classifies edits targeting trashed entities before
  dispatch, so tombs are never mutated silently.
- Completion is monotonic by domain design: replaying a completion is an idempotent no-op
  acknowledged without conflict and without touching `completed_at`.

## Data and Persistence

- No schema change; the change log is the source of truth for pulled changes. Operation ids remain
  the dedupe key on both push and replay.

## Offline and Sync

- The outbox (49) feeds `operations`; cursors persist in the replica snapshot; replays are
  idempotent by construction.

## Error Handling

- Per-operation failures become classified conflicts; log-read failures fail the request; a server
  without a configured sync service answers 503.

## Testing Strategy

- App unit tests for ordering recognition and conflict classification; testkit tests for monotonic
  acknowledgements, operation-id dedupe, cursor-bounded pulls, monotonic completion, delete-beats-
  edit, ordering-relative conflicts; web tests for the JSON route and absent-service behavior.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: entity-kind-specific pre-checks only cover trashed to-dos (the only tombstone type the
  todo read store exposes); other missing entities flow through dispatcher errors into the same
  explicit classification.
