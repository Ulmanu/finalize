# Design — add-special-lists

## Context

Deadlines already existed as a special view. Tomorrow needed a scope; project special views
needed a logged listing; Repeating needed a read path even though the recurrence engine lands
later, and the canonical schema already stores `recurrence_templates`.

## Requirements Driving the Design

- `Tomorrow view`
- `Deadlines and Repeating views`
- `Project special views`

## Architecture

- `TodoScope::Tomorrow` = open date schedules equal to `today + 1`, evaluated against the
  service's local today in memory and as `schedule_date = ($2::date + 1)` in PostgreSQL.
- `ProjectService::logged` filters terminal projects and orders by logged/completed/canceled
  instants.
- `RecurringService`/`RecurrenceReadStore` expose `RecurrenceTemplateSummary { id, title,
  next_occurrence }`, joining `recurrence_templates` with entities so the view is real before
  the recurrence engine exists; memory and PostgreSQL adapters implement it.
- Special destinations are linked from existing pages (Today → Tomorrow, Projects → Logged
  Projects) instead of being added to the permanent sidebar.

## Data and Persistence

- None; all views are derived reads.

## Offline and Sync

- Read-only change; no new commands.

## UI and Accessibility

- Standard list markup with per-page `data-testid` hooks and explicit empty states.

## Error Handling

- Storage failures surface as empty lists (existing behavior).

## Testing Strategy

- Testkit tests for Tomorrow membership, logged-project filtering, and account-scoped template
  reads.
- Web tests for all three special pages and the link affordances.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the Repeating view reads the existing `recurrence_templates` table through a new
  port rather than inventing template storage; an empty view is data-correct until the
  recurrence engine creates templates.
