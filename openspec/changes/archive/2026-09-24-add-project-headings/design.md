# Design — add-project-headings

## Context

Headings already exist in the domain (`Heading { meta, title, project, archived }`) and in the
canonical schema (`entities.kind = 'heading'`, placement `parent_kind = 'project'`). Project
ordering shares `ContextRef::Project` ranks between headings and project-scope to-dos. This
change turns those primitives into a complete, testable capability.

## Requirements Driving the Design

- `Heading lifecycle`
- `Heading child movement`
- `Heading scope`

## Architecture

- Commands: `CreateHeading { project, title }`, `RenameHeading { id, title }`,
  `ReorderHeading { id, direction }`, `ArchiveHeading { id }`, `RestoreHeading { id }`.
- Events: `HeadingCreated`, `HeadingChanged`, `HeadingReordered`, `HeadingArchived`,
  `HeadingRestored`, each invalidating project projections.
- `UnitOfWork`: `load_heading` and `save_heading`; `list_project_children` already returns
  heading ids; `load_context_order` includes headings for project scopes.
- `ProjectSummary` gains `headings: Vec<HeadingSummary>` (ordered, with ordered child to-do ids)
  and `order: Vec<ProjectOrderItem>` describing how headings and unheaded to-dos interleave.
- `ProjectService` gains heading lifecycle methods; heading reads go through the project read
  store.
- Web: `/projects/:project_id` renders heading sections with lifecycle controls and heading
  scoped to-do creation; to-dos filed in a heading are created with `ContextRef::Heading`.

## Data and Persistence

- Migration `0003-entity-archived` adds `entities.archived boolean not null default false`.
- Headings persist as `entities` rows with `kind = 'heading'` and a context placement with
  `parent_kind = 'project'`; archive is a flag, not a deletion, so children keep their identity.
- Reordering rewrites context ranks in the same transaction as the command.

## Offline and Sync

- Heading mutations are ordinary commands with change-log records and are idempotent by
  operation id, consistent with to-dos, projects, and areas.
- Projection-only grouping (which to-dos render under which heading) is derived from canonical
  context and rank data and is never synchronized separately.

## UI and Accessibility

- Heading controls are native forms/buttons with accessible names, keyboard reachable, using
  existing design-system classes; reorder uses explicit move up/down controls.
- Archived headings are hidden from the default project view but restorable, so no destructive
  action is required to declutter.

## Error Handling

- Unknown or deleted projects/headings produce `NotFound`; invalid titles produce `Validation`;
  optimistic version mismatches produce `Conflict`.
- A to-do can only reference an existing heading (`ContextRef::Heading` requires a live heading);
  headings cannot be placed in Inbox or areas because no such command exists and the domain
  constructor requires a project.

## Testing Strategy

- In-memory command/service tests: lifecycle, child movement as a unit, archive/restore,
  ordering determinism, scope rejection for unknown headings, and authorization isolation.
- Web tests: heading sections render, create/rename/archive/restore/reorder endpoints work, and
  heading-scoped to-do creation files the to-do under the heading.
- PostgreSQL paths are exercised by the existing conditional live suite (skipped without
  `DATABASE_URL`).

## Rollout / Migration

- The new column has a safe default and is applied by the existing migration runner before
  serving traffic; pre-existing rows read as not archived.
- No data-format change for existing to-dos or projects.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: heading→project conversion is not supported by the product model (no command, no
  UI), so the lifecycle requirement's conversion clause is documented as a non-goal rather than
  invented.
- Resolved: heading archive persists via a new `archived` column because the canonical schema had
  no equivalent flag.
