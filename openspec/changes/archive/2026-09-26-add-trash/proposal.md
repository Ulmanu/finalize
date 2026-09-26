# Proposal

## Why

Deletion must never lose work accidentally. Deleted entities belong in a recoverable Trash,
restore must be deterministic even when the previous parent is gone, and permanent deletion must
be an explicit, confirmed action.

## What Changes

- Soft deletion for to-dos, projects, and areas feeds one Trash view with per-kind sections.
- Restore brings entities back to an active state; to-dos whose previous project/area/heading
  no longer exists fall back to Inbox deterministically, and restored projects drop deleted-area
  assignments.
- `PurgeTodo`/`PurgeProject`/`PurgeArea` permanently delete rows only from Trash, and
  `EmptyTrash` purges every trashed entity for the account; active entities are never touched.
- Replace the placeholder Trash route with a real page: Restore and Delete permanently per row,
  plus an explicit Empty Trash action and an empty state.

## Capabilities

### New Capabilities

- `features/trash`: recoverable soft deletion, deterministic restore, and explicit permanent
  deletion.

### Modified Capabilities

(none)

## Impact

- `crates/app` (trash/restore/purge commands, unit-of-work purge methods, service methods),
  `crates/infra` (hard deletes, deleted-project/area listings), `crates/testkit`, `crates/web`
  (Trash page and routes, navigation).
- No schema change: deleted state and tombstones already exist.
- Non-goals: retention windows/tombstone garbage collection (later sync changes), undo of purges.
