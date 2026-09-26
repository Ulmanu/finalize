# Proposal

## Why

Completed and canceled are different outcomes with different timestamps, reopening must be an
explicit, policy-bounded transition, and a project can never silently lose open child work when
it is completed or canceled.

## What Changes

- Prove and expose the distinct terminal semantics: completion records `completed_at` and clears
  cancellation; cancellation records `canceled_at` and clears completion, for to-dos and projects.
- Reopening restores only completed/canceled items, clears terminal metadata, and preserves all
  unrelated state; deleted items must be restored, not reopened (explicit validation error).
- Restoring a deleted item clears stale terminal timestamps so a restored item is genuinely open.
- Project completion safety counts direct and heading children so the reject policy cannot be
  bypassed; the cascade policy completes every open child (direct and under headings)
  transactionally before the project is marked complete; canceling a project leaves open
  children open and reachable.

## Capabilities

### New Capabilities

- `features/completion`: distinct completed/canceled semantics, reopen policy, and project
  completion safety.

### Modified Capabilities

(none)

## Impact

- `crates/app` (command handlers, read models expose terminal timestamps), `crates/infra`
  (selects include `canceled_at`), `crates/testkit`, `crates/web` (no new UI required; existing
  controls now verified end to end).
- No schema change: `completed_at`/`canceled_at` columns already exist.
- Non-goals: logbook history (27), undo/redo extensions (10).
