# Proposal

## Why

Recurring templates must produce exactly one copy per logical occurrence, keep upcoming
projections and reminders meaningful without generating forever, and survive restarts by reading
progress from persistence rather than an in-memory cursor.

## What Changes

- Add an idempotent materialization command: `MaterializeRecurrenceOccurrence` loads the persisted
  template, checks the instance guard, creates the copy (title, notes, checklist, deadline
  offset, context, occurrence schedule) and records the occurrence in the same transaction; the
  PostgreSQL `(template_id, occurrence_key)` primary key plus `on conflict do nothing` makes a
  concurrent loser roll back rather than duplicate.
- Add `RecurrenceMaterializer` with `MaterializerStore` (templates due, progress advance) and a
  bounded horizon (`HORIZON_DAYS = 30`, `MAX_OCCURRENCES_PER_RUN = 90`); repeated runs and
  restarts skip existing occurrences because both instance rows and template progress are
  persisted.
- Persist progress on `recurrence_templates.next_occurrence`/`horizon_date`; ended rules stop
  generating. Tags from the template are applied to copies through the existing tag command.

## Capabilities

### New Capabilities

- `features/recurrence-materializer`: idempotent materialization, horizon generation, restart
  safety.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (serde on recurrence types), `crates/app` (unit-of-work methods, command,
  materializer module, service method), `crates/infra` (PG unit-of-work methods +`
  `PgMaterializerStore`), `crates/testkit` (memory store selection state + materializer store),
  tests. No migration: `recurrence_templates`/`recurrence_instances` already exist.
- Non-goals: template creation/editing UI and commands (later changes), reminder-rule
  materialization beyond deadline offsets, background loop scheduling.
