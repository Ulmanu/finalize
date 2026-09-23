# Design — add-undo-redo-engine

## Context

Undo must be consistent with domain rules. The cleanest approach on top of change 09 is
inverse commands: undo executes a command that reverses the effect; redo executes the inverse
of the inverse. This keeps validation, events, versioning, and sync hooks intact.

## Requirements Driving the Design

- `Reversible mutation records`
- `Grouped undo`
- `Redo consistency`

## Architecture

- `crates/app/src/undo.rs`:
  - `UndoEntry { command: Command, inverse: Vec<Command> }` — executed command plus the
    commands that revert it.
  - `UndoGroup { id, label, entries, created_at }`.
  - `UndoEngine { factory, dispatcher, clock }`:
    - `apply_group(account, label, commands)`: for each command, load the target's pre-state
      (via the unit of work), derive inverse commands, dispatch the command, collect entries.
    - `undo(group)`: apply the group's inverse commands in reverse order (returns the redo
      group).
    - `redo(group)`: apply the group's stored inverse commands in order (returns a new undo
      group).
  - `UndoStack` with a bounded window (`DEFAULT_WINDOW = 100`) and grouped entries.
  - Inverse derivation (`inverse_commands(pre_state, command)`):
    - create → delete, delete → restore, complete/cancel → reopen, reopen → complete or cancel
      (whichever it was, from pre-state),
    - title/notes/schedule/deadline/reminder → set to the previous value,
    - move → move back to the previous context.
- Reuses `CommandDispatcher` so undo/redo pass through validation, events, and change-log
  records.

## Data and Persistence

No persistence change. The history stack lives for the active session; a durable history can
be layered on later without changing the inverse-command contract.

## Offline and Sync

Because undo/redo are ordinary commands, offline clients can undo optimistically and sync the
resulting inverse operations like any other mutation.

## UI and Accessibility

No UI; later changes bind keyboard shortcuts and menu items to the stack.

## Error Handling

Undo/redo return typed `CommandError`s. A failed undo leaves the stack unchanged so the user
can retry after resolving the conflict.

## Testing Strategy

- Inverse derivation tests for every command variant (including reopen-after-cancel).
- Round-trip tests: create → undo → redo preserves identity; edit → undo restores value.
- Group tests: multi-entity group undoes in reverse and redoes exactly.
- Window tests: oldest groups evicted; empty undo returns `None`.

## Rollout / Migration

No schema or data migration.

## Open Questions Resolved From the Repository

- **Identity on redo**: undo of a create is a soft delete, so redo is a restore and the
  identity is stable without storing snapshots.
- **Where pre-state comes from**: the undo engine reads through the unit of work, not through
  UI state, so it is correct for offline/sync scenarios too.

## Tasks / Definition of Done

See `tasks.md`.
