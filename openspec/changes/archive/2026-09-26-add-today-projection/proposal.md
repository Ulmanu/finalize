# Proposal

## Why

Today is the primary working surface. It must be derived from schedule state (never a separate
copy of an item), expose This Evening as its own section, and let users arrange their day
independently of the project/area order the items keep in their contexts.

## What Changes

- Derive Today membership from schedule state (date on or before the local today, overdue
  included, This Evening included) without changing context membership.
- Split Today into a daytime section and a This Evening section with independent ordering
  buckets.
- Add a Today-specific fractional ordering scope with reorder commands, persisted in
  `placements` scope `today`/`today_evening` (migration `0005`), materialized as items enter
  or leave Today.
- Render both Today sections with per-item move controls and keep context ordering untouched.

## Capabilities

### New Capabilities

- `features/today`: derived Today membership, This Evening section, and independent manual
  ordering.

### Modified Capabilities

(none)

## Impact

- `crates/app` (command, ordering scope, unit-of-work methods, service), `crates/infra`
  (projection placements, migration `0005`), `crates/testkit`, `crates/web` (Today page).
- No entity schema change; ordering rows live in the existing `placements` table with a widened
  scope check.
- Non-goals: drag-and-drop pointer interactions (change 32), Upcoming grouping (23), widget
  mirrors of Today.
