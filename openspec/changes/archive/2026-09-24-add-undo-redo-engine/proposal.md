# Proposal

## Why

Users expect undo for destructive and state-changing actions, and grouped operations (bulk
completion, moves, reschedules) must undo as one step. Snapshot-based undo would bypass domain
rules, so undo/redo must reuse the command layer.

## What Changes

- Add `momentum-app::undo`: inverse-command derivation, grouped undo records, an undo/redo
  engine that executes inverse commands through the command dispatcher, and a bounded history
  stack with grouped semantics.
- Undo of a create is a delete; redo of that undo is a restore, so identities are preserved and
  no raw database snapshots are used.
- Related multi-entity commands execute as one labeled group that undoes/redoes as a unit.

## Capabilities

### New Capabilities

- `app/undo`: reversible mutation records and grouped undo/redo semantics built on the command
  layer.

### Modified Capabilities

(none)

## Impact

- `crates/app` (undo module), `crates/testkit` (undo tests over the in-memory unit of work).
- No schema change (history is in-memory for the active session; persistence is a later
  concern).
- Non-goals: no UI affordances (later changes), no cross-device history sync, no recurrence
  undo semantics (change 42).
