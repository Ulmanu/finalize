# Design — add-large-list-performance

## Context

Read services return full lists; the API returned everything; the schema lacked composite indexes
matching the common list order.

## Requirements Driving the Design

- `Performance budgets`
- `List virtualization`
- `Query/index tuning`

## Architecture

- Budgets live in one registry with operation ids, targets, dataset sizes, and a server-measured
  flag; benchmarks assert against the target with a debug factor (release runs meet the raw
  target). Client-only budgets (scroll, replica startup) are declared and documented for the
  hydrated client.
- Windowed reads: `window` slices materialized lists with a clamped limit (1..=500, default
  100); the API exposes `limit`/`offset` so responses stay bounded. True scroll virtualization
  belongs to the client and the page contract stays unchanged.
- Indexes: migration 0008 adds the composite list-order and change-log recency indexes.

## Data and Persistence

- Migration 0008 is additive and reversible; no data changes.

## Offline and Sync

- Sync apply is measured against its budget; windowing does not affect sync semantics.

## UI and Accessibility

- Windowing is an API concern; pages are unchanged, so accessibility and keyboard behavior are
  unaffected.

## Error Handling

- Invalid `limit`/`offset` values are rejected by typed query parsing (400); oversized limits
  clamp instead of failing.

## Testing Strategy

- App unit tests for budget registry integrity and window clamping/slicing; testkit benchmarks
  for list/search/reorder/sync-apply at 2,000 items in both debug and release; web test for
  windowed API reads and invalid-parameter rejection.

## Rollout / Migration

- Migration 0008 applies additively.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: PG queries still materialize before windowing (acceptable at the current query
  shapes and covered by the new indexes); row-level pagination can replace it later without API
  changes.
