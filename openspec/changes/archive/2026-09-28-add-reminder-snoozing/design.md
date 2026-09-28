# Design — add-reminder-snoozing

## Context

Reminders persist instant, timezone, and delivery state; the scheduler treats `Scheduled` and
`Delivered` and now needs snoozed reminders to fire again on their new instant.

## Requirements Driving the Design

- `Snooze choices`
- `Snooze idempotency`
- `Tomorrow semantics`

## Architecture

- `Reminder::snooze(until)` only transitions from `Delivered`/`Snoozed`; repeating the same target
  is a no-op so repeated clicks cannot create duplicate deliveries. `is_due` and `mark_delivered`
  both accept `Snoozed`, so the existing scheduler redelivers at the new instant and re-delivered
  reminders become `Delivered` again.
- `SnoozeChoice::ALL` defines the configured choices (`ten_minutes`, `one_hour`, `tomorrow`);
  `resolve_snooze` maps a choice plus the reminder and clock into the next instant.
- "Tomorrow": take the reminder's local wall-clock time and the next local calendar date in the
  reminder's timezone via `next_local_day_at`, which retries hour-by-hour across DST gaps so the
  wall-clock intent is preserved (never a fixed 24 hours).
- The web route parses the stable choice identifier, calls the service, and redirects; invalid
  choices or missing reminders produce `?error=snooze` with no state change.

## Data and Persistence

- None: snoozing updates the existing reminder JSON and version.

## Offline and Sync

- Snooze is an ordinary idempotent command; reusing the same target yields the same state and no
  extra change-log churn beyond the first transition.

## UI and Accessibility

- Snooze controls render only for delivered/snoozed reminders; each is a labeled submit button
  (10m, 1h, Tomorrow) inside the row, usable without pointer gestures.

## Error Handling

- Unknown choices and missing reminders redirect with `?error=snooze`; unresolvable timezones map
  to a validation error surfaced the same way.

## Testing Strategy

- Domain unit tests for local-day resolution (normal, DST gap, invalid zone) and snooze
  transitions; testkit tests for idempotent repeats, re-delivery exactly at the snoozed instant,
  and local "tomorrow" not being +24h; web tests for delivered-only controls and routing.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: "tomorrow" keeps the reminder's own wall-clock time on the next local day; a
  configurable preferred snooze hour is out of scope until user preferences exist.
