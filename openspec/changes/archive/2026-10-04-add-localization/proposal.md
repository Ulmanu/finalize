# Proposal

## Why

Locale, date formatting, week start, hour format, timezone, and parser locale are separate
concerns; date-only values must stay date-only while instants resolve with timezone/DST rules,
and natural-language parsing must select a parser locale independent of storage.

## What Changes

- Extend the settings registry with account-synced `general.locale` (date formatting),
  `general.hour_format` (h24/h12), `general.timezone` (validated IANA text via a new
  `PreferenceDefault::Text`/`PreferenceValue::Text` kind and `momentum_domain::time::is_valid_timezone`),
  and `general.parser_locale` (en-US/en-GB), alongside the existing `general.week_start`.
- Runtime integration: the schedule route's natural-date branch resolves the parser locale from
  the preference; the reminder route uses the configured timezone as its default when the form
  omits one (instants still convert through timezone/DST rules, dates remain date-only).
- Settings renders text preferences as validated text inputs (selects for choices/toggles).

## Capabilities

### New Capabilities

- `features/localization`: locale settings, timezone-correct dates, parser locale integration.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (timezone validation helper), `crates/app` (text preference kind + registry),
  `crates/web` (route integration, settings input, tests). No schema change.
- Non-goals: translated UI strings (only locale metadata today), per-entity timezones, localized
  number formats.
