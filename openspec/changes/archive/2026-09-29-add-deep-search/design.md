# Design — add-deep-search

## Context

Quick Find (43) matched titles only. Notes, checklist titles, and completed/logged entries are all
available through existing read paths (`TodoSummary.notes`, `TodoSummary.checklist`,
`TodoScope::Logbook { logged_only: false }`, `ProjectSummary.notes`).

## Requirements Driving the Design

- `Deep content search`
- `Normalized matching`
- `Offline/server consistency`

## Architecture

- `todo_hit` considers title, every checklist item, and notes, keeping the best
  `(score, field_rank)`; `SearchField` ranks title < checklist < notes so a checklist hit beats a
  notes hit at the same confidence. Logbook entries are searched with the same helper, so
  completed and canceled content is findable.
- `normalize` folds case and common Latin diacritics before every comparison; display strings are
  never rewritten. Because both the server search and the future hydrated client call this
  function, tokenization and ranking semantics cannot drift.
- Deduplication by URL keeps the single best field row per destination; the sort adds field
  priority between score and recency.

## Data and Persistence

- None; all deep content comes from existing reads.

## Offline and Sync

- Read-only; the normalization function is the shared contract for local ranking parity.

## UI and Accessibility

- Result rows expose `data-matched-in` and a "Notes"/"Checklist" badge next to Recent badges.

## Error Handling

- Read failures degrade to empty results as in Quick Find; no new error states.

## Testing Strategy

- App unit tests for tier scoring and normalization (case and diacritics); web tests for notes
  matches (diacritic query), checklist matches with badges, logged/completed content, and title
  matches ranking above notes matches.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: normalization is a hand-rolled Latin fold because the workspace has no
  unicode-normalization dependency and the supported product languages are Latin-script.
