# Design — add-recurrence-model

## Context

The Repeating special view already reads recurrence-flagged items, but there is no rule model.
This change provides pure types so later copy-on-complete work stays deterministic and testable.

## Requirements Driving the Design

- `Recurrence patterns`
- `Pattern detail`
- `Template inheritance`

## Architecture

- `RecurrencePattern` enumerates daily, weekly (selected weekdays as days from Monday), monthly
  (selected day numbers), yearly, and after-completion delays. `next_after(base)` computes the
  next date: weekly searches the following seven days, monthly searches forward up to 400 days,
  yearly clamps leap days to the month length.
- `Recurrence::next_occurrence(base, occurrences)` applies end conditions: `After(n)` stops once
  the count is reached, `On(date)` stops when the next date would pass the bound.
- `RecurrenceTemplate` owns every field; `copy_for` returns a deep clone and `deadline_for`
  derives the occurrence deadline from the offset, so copies never alias the original.

## Data and Persistence

- None; the module is pure and infallible apart from calendar construction.

## Offline and Sync

- Pure computation; no I/O.

## Testing Strategy

- Domain unit tests for every pattern, leap-year clamping, end conditions, and copy independence;
  a testkit integration test exercises the public API from outside the domain crate.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: weekdays are represented as `u32` days from Monday because `chrono::Weekday` does not
  implement `Ord` needed by `BTreeSet`; this keeps the set deterministic and serialization-ready.
