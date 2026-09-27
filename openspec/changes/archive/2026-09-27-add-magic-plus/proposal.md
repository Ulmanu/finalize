# Proposal

## Why

The creation control should be context-aware: the same control appears on every list and creates
into the currently visible logical context, and dropping/positioning it between rows must insert
at that rank rather than appending.

## What Changes

- Extend `POST /todos` with optional `position` (1-based), `position_scope` (`today` for
  projection buckets), `schedule_kind` (quick schedule), and existing `context_kind`/`context_id`
  alternate targets; creation composes the existing create, schedule, and positional-ordering
  commands instead of a new path.
- Add a Magic Plus insertion control per row (`MagicPlusTarget` + `<details>` insert form) on the
  Inbox and Today lists, carrying the target position and context hints through hidden fields.
- Add the today-scoped Magic Plus add form on the Today page so the currently visible context is
  the creation default even when the list is empty; project/heading add forms already carry
  their context.

## Capabilities

### New Capabilities

- `features/magic-plus`: contextual creation, drag insertion positioning, alternate targets.

### Modified Capabilities

(none)

## Impact

- `crates/web` (create handler, TodoRow Magic Plus target, Inbox/Today controls) and tests. No
  schema change; no new commands.
- Non-goals: pointer-gesture dragging of the control itself (hydrated client attaches to the
  same hidden-field/position hooks); visual art for the control (branding change later).
