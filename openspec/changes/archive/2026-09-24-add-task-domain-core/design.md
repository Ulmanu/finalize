# Design — add-task-domain-core

## Context

The domain crate is pure Rust with no UI/DB/provider dependencies. This change defines the
canonical model that commands, projections, ordering, and recurrence all share, so it must
encode the roadmap invariants directly in types and transitions.

## Requirements Driving the Design

- `Canonical task model`
- `Distinct temporal concepts`
- `Project/context membership`

## Architecture

- `momentum-domain::time::LocalDate` — `chrono::NaiveDate` newtype with:
  - `today_from_utc(now, offset_minutes)`, `from_ymd`, `next_day`, `previous_day`,
    `days_until`, `Display`/`FromStr` as `YYYY-MM-DD`, serde as a date string.
- `momentum-domain::model`:
  - `EntityKind`, `Status` (`Open | Completed | Canceled | Deleted`), `EntityMeta`.
  - `ScheduleState` (`Anytime | Someday | On(LocalDate)`), `Schedule { state, evening }` with
    `anytime`, `someday`, `on(date)`, `this_evening(date)`, `with_state` (clears evening when
    the state is not date-based), `is_actionable(today)`, `is_future(today)`, `is_someday()`.
  - `Deadline { date }` with `DeadlineState` (`Future | DueToday | Overdue`) computed against a
    local date.
  - `Reminder { at: DateTime<Utc>, timezone, state }` with `ReminderState`
    (`Scheduled | Delivered | Snoozed | Canceled`) — an instant plus timezone, separate from
    the date-only deadline.
  - `Title` (trimmed, length-bounded, control characters rejected).
  - `Todo`, `Project`, `Area`, `Heading`, `Tag`, `ChecklistItem` sharing `EntityMeta`.
  - `ContextRef` (`Inbox | Area(EntityId) | Project(EntityId) | Heading(EntityId)`) as the
    single contextual parent of a to-do.
  - `ProjectCompletionPolicy` (`RejectWhileOpen | CompleteOpenChildren`) with a pure
    `completion_decision(open_children, policy)` helper.
- Identifiers: `EntityId` and `ChecklistItemId` join `AccountId`/`SessionId` in
  `momentum-domain::ids`.

## Data and Persistence

No persistence change; the change-05 schema already matches this model (date columns,
`schedule_evening`, `recurrence_rule`/`reminder_rule` JSONB, placements for ordering).

## Offline and Sync

Entities carry `version`, `updated_at`, and optional `deleted_at` so offline creation and
conflict handling can reuse them. The model itself performs no I/O.

## UI and Accessibility

No UI in this change; the model is what UI renders.

## Error Handling

Typed errors (`TitleError`, `ScheduleError`) for invalid construction; transitions are
infallible where the domain allows them (complete/cancel/reopen) and return typed errors where
a precondition fails (evening without a date).

## Testing Strategy

- Unit tests for `LocalDate` derivation/arithmetic/serialization.
- Invariant tests: independent temporal fields, evening semantics, single contextual parent,
  identity preservation on move, status transitions and timestamps, project completion policy,
  checklist independence, tag sets, title validation.

## Rollout / Migration

Pure domain addition; no runtime impact.

## Open Questions Resolved From the Repository

- **Identity type**: a single `EntityId` with `EntityKind` matches the canonical `entities`
  table and the "one canonical identity" invariant; separate ID types per kind would not match
  the persistence model or allow future item/type conversion.
- **Evening**: modeled as a flag on a dated schedule (not a separate schedule state) so Today
  presentation can split the list without duplicating identity.

## Tasks / Definition of Done

See `tasks.md`.
