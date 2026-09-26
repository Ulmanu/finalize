# Design — add-upcoming-projection

## Context

`TodoScope::Upcoming` already selects open items scheduled on future local dates, ordered by
date. This change turns that list into a date-grouped agenda and specifies how rescheduling and
projection-only rows behave.

## Requirements Driving the Design

- `Date-grouped Upcoming`
- `Drag-to-reschedule`
- `Projection-only calendar rows`

## Architecture

- `UpcomingAgenda { days, calendar }` groups `TodoSummary` values by `schedule.date()` with
  `UpcomingAgenda::from_todos`; grouping is pure and preserves the canonical (date-ordered)
  input order within each day.
- `TodoService::upcoming_agenda` composes the Upcoming scope read with grouping and leaves the
  calendar list empty until an integration exists.
- Rescheduling uses the existing `ScheduleTodo` command (`when=date&date=...`), so movement
  between day groups is optimistic-version-guarded and idempotent; scheduling onto today moves
  the item into the Today projection automatically.
- `CalendarRow { date, title }` deliberately has no `EntityId`, so the type system prevents
  calendar rows from being addressed by task commands; a mutation against a non-entity id is
  rejected with not-found.

## Data and Persistence

- None; the agenda is a derived read model over stored local schedule dates.

## Offline and Sync

- Rescheduling is an ordinary command with change-log records.

## UI and Accessibility

- Each day is a labeled section (`data-date` hook) with a human-readable heading; each row keeps
  its standard actions plus a labeled "Move to date" form. Calendar rows render as inert text
  with a badge and no action forms.

## Error Handling

- Invalid dates leave the schedule unchanged with an error redirect; unknown entities are
  not found; calendar rows cannot be mutated by construction.

## Testing Strategy

- Testkit tests for grouping (future only, ascending, order-preserving), rescheduling across
  days including onto today, and the read-only/unaddressable calendar semantics.
- Web tests for day sections, move-to-date rescheduling, empty state, and the absence of
  calendar rows without an integration.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: external calendar rows are modeled without entity identity rather than as
  pseudo-entities, so no speculative provider schema is introduced before an integration exists.
