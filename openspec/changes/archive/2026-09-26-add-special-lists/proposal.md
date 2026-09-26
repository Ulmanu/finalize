# Proposal

## Why

Things-style navigation includes special destinations beyond the five built-in projections:
Tomorrow, Deadlines, Repeating, All Projects, and Logged Projects. They must be real read paths
over canonical state, not placeholder pages.

## What Changes

- Add a Tomorrow scope and page for items scheduled on the next local calendar date.
- Keep the Deadlines special view and add a Repeating special view that lists recurrence
  templates through a provider-neutral read port over the existing `recurrence_templates` table.
- Add Logged Projects (completed/canceled) alongside the existing All Projects page, with a link
  from Projects rather than permanent sidebar membership.

## Capabilities

### New Capabilities

- `features/special-lists`: Tomorrow, Deadlines/Repeating, and project special views.

### Modified Capabilities

(none)

## Impact

- `crates/app` (`TodoScope::Tomorrow`, logged-project query, recurrence read service), `crates/infra`
  (scope filter, recurrence query over `recurrence_templates`), `crates/testkit`, `crates/web`
  (three special pages plus navigation links).
- No schema change: `recurrence_templates` already exists and Tomorrow derives from schedules.
- Non-goals: the recurrence command/engine itself (later roadmap), calendar integration.
