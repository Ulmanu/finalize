# Design — add-import-export

## Context

Read services already expose everything a backup needs; imports must flow through the command
layer so validation, projections, and the change log behave exactly like user actions.

## Requirements Driving the Design

- `Versioned backup export`
- `Human-readable export`
- `Import validation`

## Architecture

- `BackupDocument` is a documented, versioned, layout-independent schema. `build_backup` reads
  open, logged, and trashed to-dos plus areas/projects/tags supplied by the web layer.
- CSV uses deterministic columns with RFC 4180 quoting; Markdown uses checkbox rows with
  when/deadline metadata and nested checklist items.
- `validate_backup` rejects unknown versions wholesale and otherwise produces an additive plan:
  invalid or unsupported records (including areas/projects/tags in this version) are reported with
  kind/index/reason. `apply_import` creates each planned to-do through `create_in`, applies
  schedule/deadline/checklist, and restores terminal statuses; nothing is deleted or replaced, so
  a failed import can never destroy existing data.

## Data and Persistence

- No schema change; imports are ordinary commands recorded in the change log.

## Offline and Sync

- Imported items sync like any created item; export is a read-only snapshot.

## UI and Accessibility

- Endpoints are documented URLs; JSON errors are typed for automation.

## Error Handling

- Unsupported versions → 400 `unsupported_version`; empty/invalid records → reported in the
  summary and skipped; storage failures → 500.

## Testing Strategy

- App unit tests for version rejection, plan validation, and CSV/Markdown mapping; testkit tests
  for additive imports preserving existing data and restoring terminal states, plus an
  export→import round trip preserving schedule/deadline/checklist; web tests for all export
  endpoints and the import summary/typed version error.

## Rollout / Migration

- None required; version 1 is the initial schema.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: this version imports to-dos only and explicitly reports areas/projects/tags as
  unsupported records; replace-mode imports are deliberately absent so imports are never
  destructive.
