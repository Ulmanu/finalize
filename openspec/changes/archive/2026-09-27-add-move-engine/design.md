# Design — add-move-engine

## Context

To-dos moved through `MoveTodo` with an inline heading check; projects moved to areas without
destination validation; headings were permanently bound to their creating project.

## Requirements Driving the Design

- `Authoritative destination validation`
- `Move preservation`
- `Adapter reuse`

## Architecture

- `movement::{Destination, Movable}` encode the destination matrix in one pure function:
  to-dos accept Inbox/Area/Project/Heading; projects accept Inbox/Area; headings accept Project.
- `movement::ensure_destination` centralizes existence, ownership, and alive checks (areas must
  not be trashed, projects must be open, headings must exist) inside the unit of work.
- `MoveTodo` and `MoveProject` handlers call the engine; `MoveHeading` uses it too and updates
  the heading's project placement, which the read model interprets as the heading (and its
  children, which reference the heading) living in the new project.
- All moves remain ordinary version-guarded commands, so every adapter (service, HTTP form,
  later drag/automation) shares identical semantics.

## Data and Persistence

- None; heading membership is the existing placement row, updated by `save_heading`.

## Offline and Sync

- Moves are idempotent commands with change-log records and projection invalidations.

## UI and Accessibility

- The heading row gains a labeled "Move heading" form with a destination project input; the
  to-do Move dialog keeps its existing labeled controls and now benefits from shared validation.

## Error Handling

- Invalid destination kinds are validation errors; unknown or trashed destinations are
  not-found; failed moves leave the entity untouched (no partial writes).

## Testing Strategy

- Testkit tests for the destination matrix, preservation across moves (tags/notes/deadline/
  schedule/checklist/identity), heading moves with children, area validation, and adapter
  equivalence between service and direct command dispatch.
- A web test drives the todo Move dialog and the heading move form through HTTP.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: heading transfer is implemented by changing the heading's project field (its
  placement), letting children follow by reference instead of rewriting child rows.
