# Proposal

## Why

Upcoming is the agenda of what comes next. Future-scheduled items must be grouped by the user's
local calendar date, rescheduling must run through the command layer, and any external calendar
rows must stay read-only unless an integration explicitly supports editing.

## What Changes

- Add an Upcoming agenda read model that groups future-scheduled items by local date, ascending,
  preserving canonical order within a day.
- Reschedule eligible items to another date through the existing schedule command, including
  moving them onto today (which leaves Upcoming for Today).
- Model external calendar contributions as read-only rows that carry no entity identity, so task
  mutations cannot target them until a provider-neutral integration exists.
- Replace the flat Upcoming list with date-grouped sections and per-item move-to-date controls.

## Capabilities

### New Capabilities

- `features/upcoming`: date-grouped future agenda with command-layer rescheduling and read-only
  calendar rows.

### Modified Capabilities

(none)

## Impact

- `crates/app` (agenda read model and service query), `crates/web` (Upcoming page, row control).
- No schema change: grouping is derived from stored schedule dates.
- Non-goals: external calendar provider integration (later roadmap), recurrence materialization
  (change 34), drag-and-drop pointer interactions (change 32).
