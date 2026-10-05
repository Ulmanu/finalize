# Design — add-behavior-parity-suite

## Context

All primitives exist (capture, scheduling, movement, search, recurrence, sync, replica, tabs);
this change composes them into scenario tests that encode the product's GIVEN/WHEN/THEN behavior.

## Requirements Driving the Design

- `Behavior parity scenarios`
- `Cross-view identity assertions`
- `Offline/multi-device assertions`

## Architecture

- One harness builds the canonical services over the memory database with a fixed clock; scenario
  tests exercise only commands/reads (never UI-only state) and assert projection membership plus
  canonical `get` state.
- `assert_unique` verifies every projection contains no duplicate rows and that each listed id
  resolves to its canonical entity; move/scheduling scenarios assert stale rows disappear from
  the source projection.
- Offline scenarios use the outbox + sync protocol (replay once, replayed-flag idempotence,
  delete-beats-edit classification); multi-tab scenarios use the tab coordinator and hint hub
  (commit propagation, lease-expiry recovery with outbox intact).

## Data and Persistence

- Memory database per scenario; no schema change.

## Offline and Sync

- Replica/outbox and sync semantics are asserted end-to-end rather than per-unit.

## UI and Accessibility

- Projection reads mirror what the UI renders; the structural visual suite covers markup.

## Error Handling

- Scenario failures name the GIVEN/WHEN/THEN contract in the test name and assertions.

## Testing Strategy

- The suite itself is the strategy: seven scenario tests spanning capture, scheduling, move,
  search, offline/sync, multi-tab, and cross-view identity.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: Inbox intentionally retains today-scheduled items (projection semantics from change
  22), so parity assertions encode that behavior rather than assuming exclusivity.
