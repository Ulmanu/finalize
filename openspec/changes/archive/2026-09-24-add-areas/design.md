# Design — add-areas

## Context

Areas are entities (`kind = 'area'`) with sidebar ordering; loose to-dos point at an area via
their contextual placement; projects reference an area. Deletion must be explicit.

## Requirements Driving the Design

- `Area lifecycle`
- `Area contents`
- `Area deletion policy`

## Architecture

- Commands: `CreateArea { title }`, `RenameArea { id, title }`, `DeleteArea { id, policy }`
  (`Detach` | `TrashContents`), with events and invalidations (`AllProjects`, `Area(id)`,
  `Inbox`, `Trash`).
- `UnitOfWork` additions: `load_area`, `save_area`, `list_area_children` (to-dos and projects).
- `AreaService` mirrors `TodoService`: builds commands and reads through `AreaReadStore`.
- `AreaReadStore` returns `AreaSummary { id, title, project_count, todo_count, rank }`.
- Web: `/areas` list (create form, rename form, delete with policy select) and
  `/areas/:area_id` detail listing projects and loose to-dos; loose to-dos link to the to-do
  editor.

## Data and Persistence

No schema change. Area ordering uses `placements` with `parent_kind = 'inbox'`? No: areas are
ordered in the sidebar scope, stored as a placement with `scope = 'context'` and
`parent_kind = 'inbox'` is wrong; sidebar ordering uses `scope = 'context'` with
`parent_kind = 'inbox'`? The cleanest fit with the existing schema is to store area ordering in
`placements` with `scope = 'context'`, `parent_kind = 'inbox'`, `parent_id = NULL`, keyed by
the area entity — the sidebar scope. This is documented in the design; a dedicated sidebar
scope column can replace it later without changing the command surface.

## Offline and Sync

Area mutations are ordinary commands with operation ids.

## UI and Accessibility

- `main` landmark, labelled forms, destructive delete requires choosing a policy.
- Counts are textual and not color-only.

## Error Handling

Deleting an unknown area is a not-found command error; the UI redirects with an error
parameter.

## Testing Strategy

- Command tests: create/rename/delete with both policies; detach returns to-dos to Inbox and
  clears project area; trash moves to-dos to Trash.
- Web tests: create area, list counts, rename, delete with detach policy; loose to-do remains
  loose.

## Rollout / Migration

No migration.

## Open Questions Resolved From the Repository

- **Sidebar scope storage**: reuse `placements(scope='context', parent_kind='inbox',
  parent_id=NULL)` for area ordering until a dedicated scope is warranted.

## Tasks / Definition of Done

See `tasks.md`.
