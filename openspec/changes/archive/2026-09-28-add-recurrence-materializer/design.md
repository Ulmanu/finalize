# Design — add-recurrence-materializer

## Context

The 0001 schema already defines `recurrence_templates` (rule/template JSON, next_occurrence,
horizon_date) and `recurrence_instances` with a composite primary key on
`(template_id, occurrence_key)`, but no application code wrote or read them for generation.

## Requirements Driving the Design

- `Idempotent materialization`
- `Horizon generation`
- `Restart safety`

## Architecture

- `MaterializeRecurrenceOccurrence { template_id, occurrence }` runs inside the dispatcher
  transaction: it returns a no-op when the occurrence exists; otherwise loads the template via
  `UnitOfWork::load_recurrence_template`, builds the copy (`Todo::new`, notes, context,
  `Schedule::on(occurrence)`, `deadline_for(occurrence)`, checklist items), appends the context
  rank, then `record_recurrence_instance`; a conflicting insert returns an error that rolls the
  copy back, so at most one copy survives a race.
- `RecurrenceMaterializer::run(account, today)` walks each due template from its persisted
  `next_occurrence` to `today + HORIZON_DAYS`, bounded by `MAX_OCCURRENCES_PER_RUN`, applying
  template tags through the tag service, then persists `next_occurrence`/`horizon` via
  `MaterializerStore::advance`.
- Restart safety: instances and progress live in persistence; a fresh materializer re-reads them.

## Data and Persistence

- Uses existing tables; no migration. `PgUnitOfWork` gained load/exists/record methods;
  `PgMaterializerStore` reads due templates and updates progress.

## Offline and Sync

- Copies are ordinary commands with change-log entries; the guard makes retries and replays
  no-ops.

## Error Handling

- Occurrence failures are reported per item and left for a later run; a duplicate claim rolls the
  copy back rather than surfacing partial state.

## Testing Strategy

- Testkit tests: bounded horizon generation with template inheritance, repeat-run and
  restart no-ops with persisted progress, end conditions stopping generation, and pre-existing
  occurrence claims being skipped without duplicates.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: occurrences are keyed by ISO date (`occurrence_key`) matching the schema; the unique
  primary key is the concurrency guard, so no advisory locks are needed.
