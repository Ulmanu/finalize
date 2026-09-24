# Design — add-inbox

## Context

Inbox is a projection over open, unfiled to-dos; it is not storage. Ordering is a placement
rank in the `context` scope for the inbox parent. Processing is ordinary commands.

## Requirements Driving the Design

- `Inbox capture`
- `Inbox ordering and badge`
- `Inbox processing`

## Architecture

- `Command::ReorderTodo { id, direction }` (`Up`/`Down`): the dispatcher loads the scope order
  through new unit-of-work operations (`load_context_order`, `set_context_rank`), computes a
  new rank with `ScopeOrder::move_item` (rebalancing when exhausted), and persists the changed
  ranks.
- `UnitOfWork` gains scope-order operations; `PgUnitOfWork` reads/writes `placements` ranks,
  the memory implementation uses `MemoryDatabase` order maps.
- `TodoService` gains `reorder_up`, `reorder_down`, `schedule_quick` (Today/Tomorrow/Someday),
  and `move_to`.
- Web: sidebar badge (count from `TodoService::list(Inbox).len()` via a resource), row
  up/down/schedule buttons, editor schedule/move forms, processing routes
  (`/todos/{id}/move-up`, `/move-down`, `/schedule`, `/move`).

## Data and Persistence

No schema change: ordering uses `placements` rows with `scope = 'context'` and the inbox
parent. Rebalance updates only changed ranks.

## Offline and Sync

Reorders are commands with operation ids; ranks sync like any field.

## UI and Accessibility

- Up/down buttons have accessible names; badge has `aria-label` with the count.
- Schedule form uses a labelled select and optional date input.
- Move form uses a labelled destination select (project/area options are loaded from the
  account's entities; empty until those capabilities render them).

## Error Handling

Reorder at list boundaries is a no-op; processing errors redirect with an `error` parameter.

## Testing Strategy

- Command tests: reorder up/down changes order; boundary no-op; scope independence.
- Service tests: schedule_quick semantics (Tomorrow = next local date).
- Web tests: badge count updates after capture/complete; reorder buttons change order; schedule
  from the list moves the item out of Inbox; move assigns context.

## Rollout / Migration

No migration.

## Open Questions Resolved From the Repository

- **Destination lists**: project/area options are loaded through the read store; until the
  projects/areas capabilities add their lists, the move form accepts an explicit destination
  id (tested) and the UI renders known projects/areas when present.

## Tasks / Definition of Done

See `tasks.md`.
