# Proposal

## Why

Projects are the second half of the Things information architecture: ordered containers of
to-dos (and headings) with their own schedule, deadline, notes, and completion semantics.
Without them, work cannot be grouped beyond areas and loose items.

## What Changes

- Add project commands: create, rename, set notes, schedule, set deadline, complete (with an
  explicit policy for open children), cancel, reopen, move to/from an area, and delete (with an
  explicit contents policy).
- Extend the unit of work with project load/save and direct-child listing.
- Add `ProjectSummary`, `ProjectReadStore`, and `ProjectService`.
- Wire the web application: `/projects` list with create controls and derived progress, and
  `/projects/:project_id` detail with notes editor, lifecycle actions, and its direct to-dos;
  creating a to-do inside a project files it in that project.
- Project temporal values participate in projections without being copied to children.

## Capabilities

### New Capabilities

- `features/projects`: project lifecycle, membership/ordering of direct children, notes, and
  temporal participation.

### Modified Capabilities

(none)

## Impact

- `crates/app` (commands, unit-of-work trait, projects service), `crates/infra`, `crates/testkit`,
  `crates/web`.
- No schema change (projects are `entities.kind = 'project'`; membership via placements).
- Non-goals: headings (change 15), checklists (16), tags (17), progress rings are derived in
  change 30, drag ordering (32).
