# Proposal

## Why

Users need portable backups independent of database layout, human-readable exports, and imports
that validate everything up front and never destroy data.

## What Changes

- Add `momentum_app::backup`: `BACKUP_VERSION`, `BackupDocument` (areas, projects, tags, to-dos
  with status/schedule/deadline/checklist), `build_backup`, deterministic `todos_to_csv`
  (RFC 4180 quoting) and `todos_to_markdown` (checkbox rows with when/deadline and nested
  checklist), `validate_backup` (version check plus per-record validation), `apply_import`
  (additive creation through the normal commands, restoring schedules, deadlines, checklists, and
  terminal statuses), and skip reporting for unsupported or invalid records.
- Add routes: `GET /export/backup.json`, `GET /export/todos.csv`, `GET /export/todos.md`, and
  `POST /import/backup.json` (typed JSON summary; unsupported versions rejected with a typed
  `unsupported_version` error before anything is applied).

## Capabilities

### New Capabilities

- `features/import-export`: versioned backup export, human-readable export, import validation.

### Modified Capabilities

(none)

## Impact

- `crates/app` (backup module + unit tests), `crates/web` (export/import routes, tests),
  `crates/testkit` (import/round-trip tests). No schema change.
- Non-goals: destructive replace-mode imports (additive only, by design), binary attachments,
  provider-specific backup formats.
