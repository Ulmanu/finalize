# Design — add-trash

## Context

Soft deletion already existed per entity kind (to-do status, project status, area `deleted_at`)
and a `TodoScope::Trash` existed, but there was no Trash page, projects/areas had no restore,
and nothing could be permanently deleted.

## Requirements Driving the Design

- `Soft deletion`
- `Restore`
- `Permanent deletion`

## Architecture

- Trash membership: `TodoScope::Trash` for to-dos plus `list_deleted_projects` /
  `list_deleted_areas` read-store methods for the other kinds.
- Restore commands: `RestoreProject`, `RestoreArea`, and the existing `RestoreTodo` now validate
  trashed state, clear terminal metadata, and fall back to Inbox when the to-do's prior context
  no longer exists (checked inside the same unit of work so exactly one save occurs).
- Permanent deletion: `PurgeTodo`/`PurgeProject`/`PurgeArea` require trashed state; `EmptyTrash`
  removes every trashed row for the account. PostgreSQL hard deletes cascade placements,
  checklist items, and tag assignments; memory adapters remove the corresponding map entries and
  ordering ranks.
- The web Trash page is server-rendered with per-row Restore / Delete permanently forms and a
  single explicit Empty Trash form; no pointer-only affordance is required.

## Data and Persistence

- No schema change; purges are physical row deletions and existing change-log records remain as
  sync history (retention handled by the later sync/tombstone capability).

## Offline and Sync

- All transitions are ordinary idempotent commands; repeats after the first succeed as no-ops or
  not-found without corrupting state.

## UI and Accessibility

- Sections per entity kind with labeled buttons and an explicit empty state; navigation gains a
  Trash destination.

## Error Handling

- Purging non-trashed items is validation; unknown ids are not found; restore validates trashed
  state; fallback context recovery never fails silently into a dangling parent.

## Testing Strategy

- Testkit tests for soft deletion across kinds, restore with missing-context fallback, and
  guarded permanent deletion plus empty trash invariants.
- Web tests for the Trash page, restore round trip, purge guard, and empty-trash flow.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: restore fallback runs inside the same unit of work as the restore mutation because
  adapters observe pre-commit state during a command; that keeps exactly one optimistic-version
  save per restore.
