# Proposal

## Why

A delivered reminder needs a way back into the schedule. Snooze choices must be configured,
repeating a snooze must not stack duplicate deliveries, and "tomorrow" must follow the user's
local calendar rather than a naive 24-hour offset.

## What Changes

- Domain transitions: snoozing a delivered (or already snoozed) reminder sets a new instant and
  the `Snoozed` state idempotently; snoozed reminders are due again when their instant passes.
- Application: `SnoozeChoice` (ten minutes, one hour, tomorrow) with `resolve_snooze`, plus a
  `SnoozeTodoReminder` command and `TodoService::snooze` that loads the reminder, resolves the
  instant, and dispatches the version-guarded command.
- Local tomorrow semantics: `momentum_domain::time::next_local_day_at` computes the next local
  calendar day at the reminder's wall-clock time in its IANA timezone, stepping over
  daylight-saving gaps instead of adding fixed hours.
- UI: delivered/snoozed reminder rows show labeled snooze controls posting to
  `POST /todos/{id}/snooze` with `?error=snooze` continuation.

## Capabilities

### New Capabilities

- `features/reminder-snoozing`: snooze choices, idempotency, tomorrow semantics.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (time helper + transitions), `crates/app` (choices/resolution, command,
  service), `crates/web` (route and controls), tests. No schema or migration change.
- Non-goals: user-configurable snooze presets (only the configured defaults), notification action
  buttons (hydrated client), custom snooze times.
