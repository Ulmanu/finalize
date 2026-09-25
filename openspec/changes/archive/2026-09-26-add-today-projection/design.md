# Design — add-today-projection

## Context

The ordering engine already declares `OrderingScope::Today`, and the canonical `placements`
scope check already allowed `'today'`. This change activates that bucket, adds a separate
`today_evening` bucket, and teaches commands and reads to use them.

## Requirements Driving the Design

- `Today membership`
- `This Evening section`
- `Today manual ordering`

## Architecture

- Membership stays derived: `TodoScope::Today` selects open items whose schedule is `On(date)`
  with `date <= today`; nothing is copied and context placement is untouched.
- `ProjectionScope { Today, ThisEvening }` maps to placement scopes `today`/`today_evening`;
  a `CASE` join in PostgreSQL and a rank lookup in memory order each section independently.
- `ReorderToday` command swaps the item with its bucket neighbor using the existing
  `ScopeOrder` engine; identities and context ranks are never rewritten.
- Ranks are materialized when an item becomes Today-eligible (create/schedule/reopen) and
  cleared when it stops being eligible; items without a rank fall back to creation order until
  first moved.
- The Today page renders the daytime and evening sections with labeled move buttons.

## Data and Persistence

- Migration `0005-projection-scope-evening` widens `placements.scope` to include
  `today_evening` (reversible, deletes only the new bucket on rollback).

## Offline and Sync

- `ReorderToday` is an ordinary command with a change-log record and projection invalidation
  (`Today`), so sync/outbox semantics match other reorders.

## UI and Accessibility

- Two labeled sections with headings, per-item `Move up`/`Move down` buttons that name their
  intent, and empty states; no pointer-only affordance is required.

## Error Handling

- Reordering a non-Today or non-open item returns a validation error; unknown items return
  `NotFound`; version conflicts behave like other commands.

## Testing Strategy

- Testkit tests for membership (including overdue and project items staying in their project),
  evening/day ordering independence, and context-order stability.
- A web test proves the sections render, manual order applies, and context order is unchanged.

## Rollout / Migration

- Existing Today items have no projection rows and keep creation order until first moved; the
  new placement rows are additive.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: Today uses the existing `placements` table (scope `today`/`today_evening`) instead
  of a new table, matching the ordering engine's prepared `OrderingScope::Today`.
