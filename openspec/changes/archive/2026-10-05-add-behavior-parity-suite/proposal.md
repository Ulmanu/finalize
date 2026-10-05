# Proposal

## Why

Feature-level tests prove individual capabilities; parity requires end-to-end GIVEN/WHEN/THEN
scenarios proving the workflows agree across capture paths, projections, offline replay, and
multiple tabs without duplicating or corrupting canonical state.

## What Changes

- Add the behavior parity suite: named scenarios covering capture parity (Quick Entry, browser
  capture, inbound mail all landing once in the Inbox), scheduling parity (projections stay
  exact; moves leave no stale rows), move parity (context identity across views), search parity
  (deep search ranking/dedup), recurrence and offline parity (optimistic enqueue, replay once,
  idempotent retries, delete-beats-edit conflicts), multi-tab parity (hint propagation, leader
  recovery preserving outbox work), and cross-view identity (every projection lists each
  canonical entity at most once with matching state).
- Assertions compare projections against canonical `get` reads, so a projection can never
  duplicate, omit, or contradict the entity state.

## Capabilities

### New Capabilities

- `features/behavior-parity-suite`: behavior parity scenarios, cross-view identity assertions,
  offline/multi-device assertions.

### Modified Capabilities

(none)

## Impact

- `crates/testkit` (one scenario suite composed from existing services/commands). No product
  behavior change, no schema change.
- Non-goals: browser-level interaction tests (documented hydration gap), load testing beyond the
  performance budgets.
