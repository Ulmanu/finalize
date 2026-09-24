# Proposal

## Why

Areas are the top level of the Things information architecture: ongoing responsibilities that
group projects and loose to-dos. Nothing can file work into an area yet, and deleting an area
has no explicit policy, which risks silent orphaning.

## What Changes

- Add area commands: `CreateArea`, `RenameArea`, `DeleteArea` with a required explicit
  `AreaDeletionPolicy` (`Detach` — projects lose the area and loose to-dos return to Inbox;
  `TrashContents` — contained projects/to-dos go to Trash).
- Extend the unit of work with area load/save and contained-child listing (PostgreSQL +
  in-memory).
- Add `AreaSummary`, `AreaReadStore`, and `AreaService` (create, rename, delete, list with
  child counts).
- Wire the web application: `/areas` list with create/rename/delete controls and counts, and a
  `/areas/:area_id` detail page listing the area's projects and loose to-dos.
- Loose to-dos remain directly under the area (no synthetic projects).

## Capabilities

### New Capabilities

- `features/areas`: area lifecycle, area contents (projects and loose to-dos), and explicit
  deletion policy.

### Modified Capabilities

(none)

## Impact

- `crates/app` (commands, unit-of-work trait, areas service), `crates/infra`, `crates/testkit`,
  `crates/web`.
- No schema change (`entities.kind = 'area'`, `placements`, `projects.area` via placements all
  exist).
- Non-goals: area tagging UI (change 17), sidebar drag ordering (change 32), project lifecycle
  detail (change 14).
