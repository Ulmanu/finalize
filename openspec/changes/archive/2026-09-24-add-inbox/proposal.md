# Proposal

## Why

Capture exists, but the Inbox is not yet a first-class working list: it has no badge, no manual
ordering, and no processing controls, so captured items cannot be triaged the way Things-style
workflows require.

## What Changes

- Add Inbox manual ordering through the command layer: `ReorderTodo { id, direction }` moves an
  item within its contextual scope using the ordering engine's gap ranks.
- Extend the unit of work with scope-order read/write operations (PostgreSQL + in-memory).
- Add an accurate Inbox badge (open inbox count) to the primary navigation.
- Add Inbox processing controls: schedule (Anytime/Today/Tomorrow/Someday/explicit date) and
  move to a project/area destination, in addition to the existing complete/cancel/delete.
- Extend the inbox list rows and to-do editor with the new processing actions.

## Capabilities

### New Capabilities

- `features/inbox`: capture into Inbox, accurate badge, manual ordering, and processing actions
  through the shared command system.

### Modified Capabilities

(none)

## Impact

- `crates/app` (commands + todos service + unit-of-work trait), `crates/infra`, `crates/testkit`,
  `crates/web` (sidebar badge, processing routes, row/editor controls).
- No schema change (`placements.rank` reused).
- Non-goals: drag-and-drop (change 32), Magic Plus (change 34), tag assignment (change 17),
  Quick Entry overlay (change 53).
