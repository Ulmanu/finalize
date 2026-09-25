# Proposal

## Why

Deadlines are independent of scheduling: an item can be actionable anytime yet due on a
specific day. Deadline state (future, due today, overdue) must be derived from the user's local
date and be visible wherever the item appears, without copying the item into another store.

## What Changes

- Derive and expose deadline states from the local date on the to-do read model.
- Add a Deadlines scope/projection listing open items with deadlines, sorted by due date.
- Show deadline state badges in list rows and provide set/clear deadline forms for to-dos and
  projects through the existing commands.

## Capabilities

### New Capabilities

- `features/deadlines`: independent deadlines, derived states, and deadline projections.

### Modified Capabilities

(none)

## Impact

- `crates/app` (scope, summary helper), `crates/infra`/`crates/testkit` (scope filters/order),
  `crates/web` (badges, forms, `/deadlines` page, navigation).
- No schema change: `entities.deadline` already stores a local `date` and reminders stay
  separate.
- Non-goals: notifications for deadlines (later roadmap), deadline recurrence.
