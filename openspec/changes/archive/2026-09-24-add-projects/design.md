# Design — add-projects

## Context

Projects share the entity table and the command/read-store patterns established for to-dos and
areas. Membership is a placement: direct to-dos carry `ContextRef::Project(id)`; project→area
assignment is a placement with `parent_kind = 'area'`.

## Requirements Driving the Design

- `Project lifecycle`
- `Project child ordering`
- `Project temporal participation`
- `Project deletion policy`

## Architecture

- Commands: `CreateProject`, `RenameProject`, `SetProjectNotes`, `ScheduleProject`,
  `SetProjectDeadline`, `CompleteProject { policy }`, `CancelProject`, `ReopenProject`,
  `MoveProject { area }`, `DeleteProject { policy }`.
  - `ProjectCompletionPolicy` (`RejectWhileOpen` | `CompleteOpenChildren`) is reused from the
    domain; `ProjectDeletionPolicy` mirrors the area policy for direct children.
- `UnitOfWork`: `load_project`, `save_project`, `list_project_children` (direct to-do ids +
  heading ids).
- `ProjectService` + `ProjectReadStore` (`ProjectSummary { id, title, notes, status, schedule,
  deadline, area, open_count, completed_count, version }`).
- Web: `/projects` list and `/projects/:project_id` detail; the detail page lists direct
  to-dos through `TodoService` filtered by project context and offers lifecycle actions.

## Data and Persistence

No schema change. Project rows store title/notes/status/schedule/deadline; membership and area
assignment are placements; child ordering uses placement ranks in the project context.

## Offline and Sync

Project mutations are ordinary commands with operation ids.

## UI and Accessibility

Forms use native controls; completion with open children surfaces the conflict as a status
message; the detail page shows counts and metadata textually.

## Error Handling

Completion with open children under the reject policy returns a typed conflict; deletion always
requires a policy.

## Testing Strategy

- Command/service tests: create/rename/notes, schedule/deadline independence from children,
  complete reject vs cascade, cancel/reopen, delete detach vs trash, move to area.
- Web tests: create project, list with counts, detail with child to-dos, complete cascade,
  delete detach.

## Rollout / Migration

No migration.

## Open Questions Resolved From the Repository

- **Completion policy default**: the command requires an explicit policy so callers cannot
  silently destroy child work.
- **Child ordering**: direct to-dos are ordered by their placement rank in the project context,
  reusing the ordering engine.

## Tasks / Definition of Done

See `tasks.md`.
