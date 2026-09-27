# Proposal

## Why

Reordering and rescheduling by dragging is the primary Things interaction, but a drop is really
an insertion point or a destination: the authoritative behavior must be one version-guarded
command per gesture, identical to keyboard and dialog paths.

## What Changes

- Add positional ordering commands: `MoveTodoToPosition` (contextual scope) and
  `MoveTodayToPosition` (Today/This Evening buckets) so a drop lands at an explicit index.
- Cross-context drops use the existing move engine (`MoveTodo`), and day drops use the existing
  schedule command, so no UI-only membership state exists.
- Emit drop-target metadata (`draggable`, `data-drag-index`, `data-drop-position`,
  `data-drop-date`) in server-rendered rows/sections and provide accessible position controls
  (number input + Move here) that perform exactly the drop operation for keyboard users.

## Capabilities

### New Capabilities

- `features/drag-and-drop`: pointer drag reorder, cross-context move, and Upcoming day
  rescheduling.

### Modified Capabilities

(none)

## Impact

- `crates/app` (positional commands, service methods), `crates/web` (row drop hooks, position
  forms, routes), `crates/testkit`/`crates/web` tests. No schema change.
- The pointer gesture itself is delivered by the hydrated Rust/WASM client; drop targets,
  insertion feedback data, and the authoritative operations are all server-rendered and tested
  now, and the gesture layer attaches to these hooks when the hydration harness is wired
  (documented as the remaining live-browser gap alongside parity changes 71/72).
- Non-goals: multi-select drag (33), visual snapshots (71).
