# Design — add-date-pickers

## Context

Scheduling and deadline commands already exist and accept dates; reminders had a command but no
UI and no validation that a start date exists. Timezone handling existed only for deriving the
local "today".

## Requirements Driving the Design

- `When picker`
- `Deadline picker`
- `Reminder time picker`

## Architecture

- Domain: `parse_local_datetime` (accepts `YYYY-MM-DDTHH:MM[:SS]`) and `local_datetime_to_utc`
  using `chrono-tz`; invalid zones and nonexistent local times are typed errors, ambiguous times
  resolve to the earliest instant.
- Application: `SetTodoReminder` rejects attaching a reminder when the schedule has no date;
  clearing remains allowed.
- Web: `POST /todos/{id}/reminder` maps the form's local time plus timezone → UTC
  `ReminderInput`; the row editor renders When/deadline/reminder controls and a reminder badge
  showing the stored UTC instant.

## Data and Persistence

- Reminders already persist with instant + timezone; no migration.

## Offline and Sync

- Reminder set/clear are ordinary commands with change-log entries, so retries are idempotent.

## UI and Accessibility

- Labeled date, datetime-local, and timezone inputs inside the expanded editor; clear actions are
  separate labeled submits so each picker can be emptied independently.

## Error Handling

- `?error=reminder` for parse/conversion failures and schedule-gate violations; deadline and
  schedule failures keep their existing `?error=` states.

## Testing Strategy

- Domain unit tests for timezone conversion (including DST gap and invalid zone); testkit tests
  for the reminder schedule gate, clearing, and deadline independence; web tests for picker
  markup and commit paths.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the account timezone preference does not exist yet, so the reminder form carries an
  explicit IANA timezone (defaulting to UTC) and stores it with the reminder; when preferences
  land, the default value comes from the account.
