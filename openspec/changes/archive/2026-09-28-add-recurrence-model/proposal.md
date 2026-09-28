# Proposal

## Why

Recurrence needs a pure domain model before any copy-on-complete behavior: patterns, end
conditions, and an owned template describing what future copies inherit.

## What Changes

- Add `momentum_domain::recurrence` with regular patterns (daily, weekly with selected weekdays,
  monthly with selected dates, yearly) and after-completion delays, each advancing from a base
  date.
- End conditions (`Never`, inclusive `On(date)`, `After(count)`) evaluated against the occurrence
  count so rules stop deterministically.
- `RecurrenceTemplate`: fully owned snapshot carrying title, notes, tags, checklist, heading
  structure, deadline/reminder offsets, and context; `deadline_for` and `copy_for` materialize
  per-occurrence values without sharing mutable state.

## Capabilities

### New Capabilities

- `features/recurrence-model`: recurrence patterns, pattern detail, template inheritance.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (new module + unit tests) and a testkit integration test. No schema, app, or
  web changes yet; copy-on-complete consumes this model in the next change.
- Non-goals: scheduling/copy commands, UI, persistence of rules (later changes).
