# Design — add-deadlines

## Context

`Deadline { date }` and `Deadline::state(today)` already exist, and to-dos/projects persist
`deadline` as a SQL `date` independent from `schedule_date` and reminder rules. What was
missing is derivation on reads, a deadline projection, and user-facing controls.

## Requirements Driving the Design

- `Independent deadline`
- `Deadline states`
- `Deadline projections`

## Architecture

- `TodoSummary::deadline_state(today)` derives `Future | DueToday | Overdue` from the summary's
  canonical deadline and a supplied local date; the stored deadline is never mutated by reading.
- `TodoScope::Deadlines` lists open items with deadlines, ordered by due date ascending.
- List rows render a state badge (`Overdue:`, `Due today:`, `Due:`) when the surrounding view
  supplies the service's local date.
- Set/clear mutations reuse `SetTodoDeadline`/`SetProjectDeadline` commands through form routes.

## Data and Persistence

- No schema change; deadlines persist as local dates.

## Offline and Sync

- Deadline changes remain idempotent commands with change-log records.

## UI and Accessibility

- Deadline forms use labeled `date` inputs; badges use text, not color alone, and expose a
  `data-testid` hook.

## Error Handling

- Unparseable dates are ignored (treated as clear) with an error redirect on command failure;
  missing entities produce not-found redirects.

## Testing Strategy

- Testkit tests for independence, derived states, and projection ordering/identity.
- Web tests for badge states across views and the deadlines page.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: state derivation lives on the read model (not stored), so every projection shows the
  same canonical deadline value and no duplication occurs.
