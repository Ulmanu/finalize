# Design — add-someday-projection

## Context

`ScheduleState::Someday` and `TodoScope::Someday` already exist; projects can be scheduled
Someday through `ScheduleProject`. This change completes the projection: projects join the
view, and tests pin the "explicit reactivation only" invariant.

## Requirements Driving the Design

- `Someday membership`
- `Someday reactivation`
- `Parent/child independence`

## Architecture

- To-do membership remains `status = open && schedule.is_someday()`.
- `ProjectService::someday` filters the project read model to open, Someday-scheduled projects.
- Reactivation reuses the existing schedule commands; no projection writes schedule state.
- Child independence falls out of per-entity schedule state: parents never propagate schedules
  to children, verified by tests.

## Data and Persistence

- None.

## Offline and Sync

- Reactivation is an ordinary idempotent command with change-log records.

## UI and Accessibility

- Two labeled sections (To-Dos, Projects) with standard rows; project rows carry a labeled
  "Reactivate project" schedule form and an Edit link; empty states are explicit.

## Error Handling

- Storage failures surface as empty sections; invalid schedule values redirect with error
  markers on the command routes.

## Testing Strategy

- Testkit tests for membership/reactivation transitions and parent/child independence.
- Web tests for section rendering, reactivation flows, and the empty state.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: projects participate as their own section rather than being merged into the to-do
  scope, because project schedules are a separate aggregate.
