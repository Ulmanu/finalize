# Design — add-scheduling-model

## Context

`Schedule { state: Anytime | Someday | On(LocalDate), evening: bool }` already exists, as do the
schedule commands and the Inbox scope. This change adds the remaining projection scopes and
makes the local-date boundary explicit and testable.

## Requirements Driving the Design

- `Schedule states`
- `Schedule transitions`
- `Local-date semantics`

## Architecture

- `TodoScope::{Today, Upcoming, Anytime, Someday}` are evaluated by the read stores against a
  `today: LocalDate` parameter supplied by the application service.
- `TodoService::list_at(account, scope, offset_minutes)` converts the service clock instant to
  a local date through `LocalDate::today_from_utc`; `list` keeps UTC until localization.
- Web schedule handlers derive Today/This Evening/Tomorrow from `TodoService::local_today()`
  (`ProjectService::local_today()` for projects), never from `Utc::now()` directly.
- Projection pages render `TodoScope` lists through one shared component.

## Data and Persistence

- No schema change: `schedule_kind`, `schedule_date` (SQL `date`), and `schedule_evening`
  already persist the canonical schedule without UTC-midnight timestamps.

## Offline and Sync

- Rescheduling stays an ordinary idempotent command with a change-log record.

## UI and Accessibility

- Projection pages reuse the shared list markup and loading/empty states with page-specific
  `data-testid` hooks.

## Error Handling

- Invalid evening-without-date schedules are rejected by the domain; invalid dates in forms
  redirect with an error marker.

## Testing Strategy

- Testkit tests for scope membership, transition metadata preservation, and offset-based
  local-date boundaries; a web test moves an item across projection pages.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: until the localization capability stores a per-account timezone, the service clock
  (UTC) defines the local date; the offset-aware entry point keeps boundary semantics testable
  and ready for change 65.
