# Design — add-magic-plus

## Context

Creation already routes through `CreateTodo` with a `ContextRef` chosen from hidden form fields,
and change 32 added explicit insertion-position commands (`MoveTodoToPosition`,
`MoveTodayToPosition`). Magic Plus composes these instead of introducing a fourth write path.

## Requirements Driving the Design

- `Contextual creation`
- `Drag insertion`
- `Alternate creation targets`

## Architecture

- `POST /todos` accepts `position` (1-based, like the position forms), `position_scope=today`,
  and `schedule_kind`; after `CreateTodo`, the handler applies `ScheduleTodo` (via
  `QuickSchedule`) and then the positional command for the requested scope, redirecting with
  `?error=position` if reordering fails after creation.
- `MagicPlusTarget` carries position, scope, quick schedule, context hints, return path, and a
  row label; `TodoRow` renders an accessible `<details>` insert form whose hidden fields feed the
  same route, so keyboard users perform exactly the drop operation without pointer input.
- The hydrated client consumes the same hidden-field contract for pointer dragging.

## Data and Persistence

- None; all state changes go through existing commands and change-log entries.

## Offline and Sync

- Each creation is an ordinary idempotent command; position application is a second command and
  failure is surfaced (item still exists) rather than rolled back silently.

## UI and Accessibility

- `<details>`/`<summary>` gives a native, keyboard- and touch-accessible disclosure; labels name
  the row ("Insert above <title>"); the Today page has a plain add form with the schedule hidden
  field so empty lists still offer contextual creation.

## Error Handling

- `?error=create`, `?error=schedule`, and `?error=position` are distinct continuation states;
  invalid context IDs fall back to Inbox as before.

## Testing Strategy

- Web tests for rendered insert controls (Inbox + Today), positioned creation ordering, alternate
  project target, and today-scoped scheduled insertion. Positional command semantics are covered
  by change 32's testkit suite.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: Magic Plus insertion is a create followed by the existing positional command; no new
  command is needed, preserving undo/version semantics.
