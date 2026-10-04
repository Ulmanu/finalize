# Design — add-localization

## Context

Dates are stored as `LocalDate` (date-only) and reminders as UTC instants with a timezone; the
natural-date parser accepts a `Locale`. The missing piece was configuration: typed preferences
plus route integration.

## Requirements Driving the Design

- `Locale settings`
- `Timezone-correct dates`
- `Parser locale integration`

## Architecture

- The registry gains a validated text kind: `PreferenceDefault::Text` + `PreferenceValue::Text`
  with per-key validation (`general.timezone` uses `is_valid_timezone`, backed by chrono-tz in the
  domain crate). Choice preferences cover date locale, hour format, parser locale, and week start.
- The schedule route resolves `general.parser_locale` through `effective` and passes it to
  `parse_natural_date`; storage stays date-only regardless of locale.
- The reminder route defaults an omitted timezone to `general.timezone`; conversion still runs
  through `local_datetime_to_utc` (DST-safe).
- Settings renders text preferences as labeled text inputs and choices/toggles as selects.

## Data and Persistence

- Account-synced preferences use the existing `preferences` table; no schema change.

## Offline and Sync

- Preferences sync as account data; parsing and conversion are pure functions.

## UI and Accessibility

- Labeled inputs/selects with the same scope badges; invalid timezones are rejected with the
  standard preference error state.

## Error Handling

- Unknown keys/values and invalid timezones are rejected without writes; missing preferences fall
  back to en-US parsing and UTC reminders.

## Testing Strategy

- Domain unit test for timezone validation; registry tests cover text defaults; web tests for the
  rendered locale preferences and persistence/rejection, en-GB parser behavior on an ambiguous
  date, and the timezone preference as the reminder default.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: timezone validation lives in the domain crate (which already depends on chrono-tz)
  rather than duplicating a zone list in the app layer.
