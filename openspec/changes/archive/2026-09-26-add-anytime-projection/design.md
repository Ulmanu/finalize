# Design — add-anytime-projection

## Context

`TodoScope::Anytime` previously matched only `schedule_kind = 'anytime'`. The refined semantics
use the domain's existing `Schedule::is_actionable(today)` predicate so overdue and today items
are actionable while future and Someday items are not.

## Requirements Driving the Design

- `Actionable membership`
- `Exclusions`
- `Context grouping`

## Architecture

- `TodoScope::Anytime` = `status = open && schedule.is_actionable(today)` in memory and
  PostgreSQL; terminal and deleted statuses are excluded by the status clause.
- `AnytimeAgenda::from_todos` groups by `todo.context`, preserving input order and ordering
  groups Inbox first, then areas, then projects, then headings, deterministically by identity.
- `TodoService::anytime_agenda` composes the scope read with grouping; the web page resolves
  group labels from `AreaService`/`ProjectService` lists and falls back to generic labels.
- Grouping is a read-only projection: no context writes occur, verified by tests that re-read
  each item's context after rendering.

## Data and Persistence

- None.

## Offline and Sync

- Read-only change; no new commands.

## UI and Accessibility

- Sections with labeled headings and `data-context` hooks; standard row markup preserved.

## Error Handling

- Storage failures surface as empty projections (existing behavior); no new failure modes.

## Testing Strategy

- Testkit tests for actionable membership/exclusions and grouping stability of contexts, plus
  the updated scheduling expectations.
- Web tests for grouped rendering, titles, exclusions, and the empty state.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: Anytime reuses `Schedule::is_actionable` rather than duplicating date logic, keeping
  local-date semantics in the domain.
