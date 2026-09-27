# Proposal

## Why

When, deadline, and reminder need dedicated picker interfaces with clear semantics: deadline is
independent of schedule, quick destinations cover the common choices, and reminders can only
attach to date-scheduled items and are interpreted in a named timezone.

## What Changes

- Extend the expanded row editor with a When picker (quick Today/This Evening/Tomorrow/Someday/
  Anytime plus explicit date and clear-via-Anytime), a deadline picker with explicit set/clear,
  and a reminder picker (local datetime + IANA timezone) with set and clear.
- Add `POST /todos/{id}/reminder` with `at`/`timezone` parsing, converting wall-clock local times
  to UTC through a domain helper (`local_datetime_to_utc`), rejecting invalid zones and
  daylight-saving gaps with `?error=reminder`.
- Enforce schedule-gated reminders in the command layer: attaching a reminder requires a
  date-based schedule; clearing is always allowed.

## Capabilities

### New Capabilities

- `features/date-pickers`: when picker, deadline picker, reminder time picker.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (local datetime parsing/conversion helpers and errors), `crates/app` (reminder
  validation), `crates/web` (reminder route, editor picker controls, reminder row badge) and
  tests. No schema change.
- Non-goals: user timezone preference storage (later localization change); timezone pickers
  beyond a labeled input; reminder delivery/snooze.
