# Proposal

## Why

No task model exists yet. Every later capability (ordering, commands, projections, recurrence)
needs one canonical, framework-independent definition of to-dos, projects, areas, headings,
checklists, tags, schedule, deadline, and reminders — with the temporal concepts kept separate
and date-only values kept as local dates.

## What Changes

- Add `momentum-domain::time::LocalDate` — a real local calendar date (never a fake UTC-midnight
  instant) with UTC+offset derivation and day arithmetic.
- Add `momentum-domain::model`: entity kinds and statuses, `ScheduleState`/`Schedule` (Anytime,
  Someday, On(date), This Evening as a Today presentation state), `Deadline` with
  future/due-today/overdue states, `Reminder` with delivery state, `Title`, and the canonical
  `Todo`, `Project`, `Area`, `Heading`, `Tag`, and `ChecklistItem` types.
- Enforce domain invariants in pure Rust: one canonical identity per entity, exactly one
  contextual parent, independent schedule/deadline/reminder, evening only with a date, and
  explicit status transitions (complete/cancel/reopen) with a project-completion policy.
- Add stable entity identifiers (`EntityId`, `ChecklistItemId`) alongside `AccountId` and
  `SessionId`.

## Capabilities

### New Capabilities

- `domain/tasks`: canonical task/context model with distinct temporal concepts, canonical
  identity, and pure domain invariants.

### Modified Capabilities

(none)

## Impact

- `crates/domain` only (new modules and tests). No UI, persistence, or web changes.
- Non-goals: no projections or list membership rules (changes 20–29), no ordering engine
  (change 08), no recurrence (change 40), no persistence mapping (change 09+), no UI.
