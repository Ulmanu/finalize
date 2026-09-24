# Proposal

## Why

Projects need internal structure: headings split a project into ordered sections and own the
to-dos that follow them. Without headings, long projects are flat lists, and there is no
canonical place to keep section-level grouping that survives reorder and archive.

## What Changes

- Add heading commands scoped to projects: create, rename, reorder, archive, and restore.
- Reordering a heading moves its assigned child to-dos as one unit.
- Extend the unit of work with heading load/save and heading-aware project ordering.
- Extend the project read model with headings and their ordered children.
- Wire the web project detail: heading sections with create/rename/reorder/archive/restore,
  to-dos filed under headings, and heading-scoped to-do creation.

## Capabilities

### New Capabilities

- `features/project-headings`: heading lifecycle inside projects, child movement as a unit, and
  project-only scope.

### Modified Capabilities

(none)

## Impact

- `crates/app` (commands, unit-of-work trait, project service/read model), `crates/infra`
  (heading persistence + migration `0003`), `crates/testkit`, `crates/web`.
- Schema: `entities.archived` column (migration `0003-entity-archived`) to persist heading
  archive state without deleting data.
- Non-goals: heading→project conversion (unsupported by the product model; recorded in design),
  checklists (16), tags (17), drag ordering (32).
