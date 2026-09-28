# Design — add-recurrence-exceptions

## Context

Materialization (41) generates regular occurrences ahead of time and guards duplicates with
instance rows. After-completion rules cannot generate ahead: the next copy only makes sense once
the current one is completed.

## Requirements Driving the Design

- `Early completion`
- `One-time versus future edit`
- `After-completion anchor`

## Architecture

- `TemplateState { open_copy, last_completion, copies }` (from a join over instances and their
  entities) gates after-completion templates: an open copy pauses generation; otherwise the next
  occurrence is `completion_date + delay` (or today for the first copy). End conditions are
  evaluated against copy counts and date bounds before generating.
- Regular rules realign to rule-matching dates (`next_after(date - 1) == date`) so a rule change
  or reset cannot emit an off-pattern first copy.
- One-time edits use existing per-copy commands (schedule/deadline/tags); future edits use
  `UpdateRecurrenceRule`, which persists the new rule and resets `next_occurrence` to today with
  `horizon_date = null` so the materializer regenerates under the new rule.

## Data and Persistence

- Uses existing recurrence tables; no migration. Instances now retain their copy identity in
  memory for state computation (PG joins the entities table).

## Offline and Sync

- Both commands are ordinary idempotent writes; rule replacement is version-free by nature
  (template row), instance guards keep copies unique.

## Error Handling

- Unknown templates return not-found; per-occurrence failures stay reported and retryable.

## Testing Strategy

- Testkit tests: after-completion pausing while open, completion-anchored next occurrence, early
  completion anchoring (day 4, not day 6), one-time copy edit leaving the template progress
  untouched, and rule replacement regenerating on-pattern occurrences while existing copies stay
  unchanged.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: anchoring is date-level using the completion timestamp's UTC date; time-of-day
  anchoring would need a policy preference and is deferred.
