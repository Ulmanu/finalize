# Design — add-natural-language-dates

## Context

Scheduling is command-driven and dates are `LocalDate` values; a text box that resolves to a date
relative to the service's local today fits the existing route without new commands.

## Requirements Driving the Design

- `Deterministic natural-date parsing`
- `Locale-aware parsing`
- `Ambiguity handling`

## Architecture

- `parse_natural_date(expression, base, Option<Locale>)` returns `ParsedNaturalDate { date, time }`
  or a typed `NaturalDateError` (`Empty`, `Unknown`, `Ambiguous { candidates }`, `Invalid`).
- Numeric dates with both parts ≤ 12 are resolved by the locale when provided; without one the
  error carries both candidates (earliest first). Values over 12 disambiguate the order; both
  parts over 12 are invalid.
- Past dates for explicit month/day expressions roll to the next year, matching future intent.
- The web route uses `Locale::EnUs` as the documented default until account preferences exist;
  only the date part is scheduled today.

## Data and Persistence

- None; parsing is pure.

## Offline and Sync

- Pure local computation; resulting commands are ordinary schedule operations.

## UI and Accessibility

- A labeled text input with placeholder examples; parse failures produce the existing schedule
  error state without changing the current schedule.

## Error Handling

- Every failure is recoverable: unknown/invalid/ambiguous input redirects with `?error=schedule`
  and leaves the schedule untouched.

## Testing Strategy

- Domain unit tests per expression family plus ambiguity and time parsing; web tests for
  rendered input, commit paths, and rejection.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: locale defaults to en-US at the call site because account regional preferences do not
  exist yet; the parser API already accepts the locale, so wiring preferences later is a one-line
  change.
