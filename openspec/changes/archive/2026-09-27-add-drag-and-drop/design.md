# Design — add-drag-and-drop

## Context

The move engine (31) already single-sources cross-context moves, and the ordering engine
provides insertion between neighbors. What was missing was an explicit *insertion position*
command (drops are positions, not just up/down steps), Today-bucket positional moves, and
server-rendered drop-target metadata for the gesture layer.

## Requirements Driving the Design

- `Pointer drag reorder`
- `Cross-context move`
- `Upcoming reschedule drag`

## Architecture

- `MoveTodoToPosition { id, position }` and `MoveTodayToPosition { id, position }` clamp the
  requested zero-based index to the scope/bucket and call `ScopeOrder::move_item`, updating only
  the ranks of the moved item and any rebalanced neighbors. Context ranks and Today ranks stay
  independent.
- Cross-context drops call `MoveTodo` (move engine); Upcoming day drops call `ScheduleTodo` with
  the target date. No membership state exists outside canonical ranks/context/schedule.
- Rows render `draggable` plus `data-drag-index`/`data-drop-position`; Upcoming day sections
  render `data-drop-date`; every row also carries a labeled position form (1-based display,
  converted to zero-based index server-side) so the same drop operation is keyboard-reachable
  and testable without a browser.
- The hydrated Rust client is the intended pointer-gesture consumer of these hooks; it is built
  by `scripts/build-web.sh` and will attach handlers when the hydration glue lands.

## Data and Persistence

- None; positional moves reuse the existing rank columns and placements.

## Offline and Sync

- Positional moves are ordinary idempotent commands with change-log records and projection
  invalidations (`TodoReordered`, `TodayReordered`).

## UI and Accessibility

- Drop metadata plus an explicit, labeled position input and Move-here button; up/down buttons
  remain for stepwise keyboard moves; no pointer-only affordance is required to complete any
  operation.

## Error Handling

- Non-today items dropped in Today are validation errors; unknown items are not-found; invalid
  positions clamp; failed drops leave ordering untouched.

## Testing Strategy

- Testkit tests for positional context moves (including clamping and no-ops), independent Today
  bucket ordering, day-drop rescheduling with preserved context, and cross-context drops.
- Web tests for drop-target metadata, the position forms, Today drops, and Upcoming day targets.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: drops are modeled as explicit insertion positions and destinations (not pointer
  events) so the authoritative behavior is server-testable; the pointer gesture is a thin
  hydrated layer over the same commands. This is recorded as the remaining live-browser gap.
