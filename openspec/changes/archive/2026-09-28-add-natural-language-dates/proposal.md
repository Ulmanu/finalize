# Proposal

## Why

Typing "tomorrow", "friday", or "jun 20" is faster than any picker. The parsing must be
deterministic, offline, locale-aware where conventions differ, and honest about ambiguity.

## What Changes

- Add a domain natural-date parser: relative keywords, offsets (`in 3 days`, `next week`,
  `next month`), weekdays (`friday`, `next friday`), named month/day forms (`june 20`, `20 jun`,
  `jun 20th`), numeric dates resolved by locale, and reminder times (`tomorrow at 9:30am`,
  `21:00`, `tonight`).
- Locale module: `Locale::EnUs` (month/day) and `Locale::EnGb` (day/month) resolve numeric
  ambiguity; parsing without a locale returns `Ambiguous` with both candidate dates.
- Wire the parser into the schedule route as `when=natural&text=...`, with the row editor gaining
  a Natural date input next to the When picker; failures redirect with `?error=schedule`.

## Capabilities

### New Capabilities

- `features/natural-language-dates`: deterministic parsing, locale awareness, ambiguity handling.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (new `natural_date` module with unit tests), `crates/web` (schedule route
  branch, editor input, tests). No schema change; no network or model dependency.
- Non-goals: languages beyond the English conventions; year-qualified expressions; wiring parsed
  reminder times into reminder creation (the parser returns them; a later change can consume).
