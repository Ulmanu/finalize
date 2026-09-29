# Proposal

## Why

Quick Find only saw titles. Finding a task by a phrase inside its notes, a checklist item, or an
entry completed months ago needs deep content matching with the same deterministic ranking.

## What Changes

- Extend `SearchService` to search notes (to-dos and projects), checklist item text, and logged
  and completed content via the Logbook scope, in addition to the existing surfaces.
- Add `SearchField` (title/checklist/notes) to hits with field priority breaking ties within a
  confidence tier: title matches rank above checklist matches, which rank above notes matches;
  one row per destination keeps the best-matching field.
- Add `normalize` (case folding plus Latin diacritic folding) used for all matching while display
  titles remain the original text; the search page renders `data-matched-in` and a field badge.

## Capabilities

### New Capabilities

- `features/deep-search`: deep content search, normalized matching, offline/server consistency.

### Modified Capabilities

(none)

## Impact

- `crates/app` (deep search + normalization + unit tests), `crates/web` (result metadata/badges,
  tests). No schema change.
- Non-goals: a persisted offline index (the hydrated client reuses this crate's `normalize` and
  scoring, guaranteeing the same tokenization/ranking semantics), full Unicode normalization
  beyond Latin diacritics, fuzzy/typo tolerance.
