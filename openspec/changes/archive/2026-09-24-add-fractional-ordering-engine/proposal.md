# Proposal

## Why

Ordering is a first-class domain concern: contextual lists (Inbox, project, heading) and
derived lists (Today) must keep independent manual order, and reordering must not rewrite
every sibling. Without a shared engine, later drag-and-drop, move, and multi-select features
would each invent their own ranking.

## What Changes

- Add `momentum-app::ordering`: a gap-based `Rank` type with deterministic midpoint insertion,
  storage encoding that preserves lexicographic order, append/insert/move planning, exhausted
  gap detection, and deterministic rebalancing that preserves visible order.
- Add `OrderingScope` (context parent vs Today) and a `ScopeOrder` container that maintains one
  scope's ordered items independently, so the same entity can hold different positions in its
  context and in Today.
- Define insertion/move plans as data (rank plus any rebalanced neighbors) so the command layer
  can persist them transactionally.

## Capabilities

### New Capabilities

- `domain/ordering`: independent ordering scopes, gap-based insertion without renumbering, and
  deterministic rank maintenance.

### Modified Capabilities

(none)

## Impact

- `crates/app` only (new module and tests); no schema change (the change-05 `rank text` columns
  are reused with a documented encoding).
- Non-goals: no drag-and-drop UI (change 32), no move-destination validation (change 31), no
  projection ordering rules beyond scope separation (changes 22–25).
