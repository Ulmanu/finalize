# Proposal

## Why

Progress in Things is never a stored percentage: it is derived from eligible child completion,
and Today shows how much of the day is done. The edge cases (zero items, canceled rows) need
explicit, tested rules.

## What Changes

- Derive project progress from eligible direct children (open + completed); canceled and
  deleted children are excluded from both numerator and denominator, and zero eligible
  children render a neutral state.
- Add Today completion progress based on completable Today items (open + completed to-dos
  scheduled on or before the local today; canceled, deleted, future, and Someday rows excluded).
- Render progress indicators on project rows, project detail, and the Today header using native
  accessible `<progress>` elements plus text.

## Capabilities

### New Capabilities

- `features/progress`: derived project progress and Today completion progress with explicit
  edge-case policy.

### Modified Capabilities

(none)

## Impact

- `crates/app` (progress model, `TodoScope::TodayAll`, `today_progress`, eligible counts),
  `crates/infra`/`crates/testkit` (count semantics, scope filters), `crates/web` (indicators).
- No schema change and no stored percentages.
- Non-goals: project progress in widgets (later), animated rings (design follow-ups).
