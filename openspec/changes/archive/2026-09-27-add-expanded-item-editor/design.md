# Design — add-expanded-item-editor

## Context

Rows already carry identity (`data-todo-id`), selection checkboxes, and per-row action forms
from earlier changes. The expanded editor is layered inside the same `<li>`, so expansion cannot
lose list position or identity, and every mutation keeps using the existing command routes.

## Requirements Driving the Design

- `Inline expansion`
- `Progressive fields`
- `Commit/cancel editing`

## Architecture

- `RowEditorContext { return_to, tags }` is a `TodoRow` prop; when present the row renders a
  `<details data-testid="row-editor">` with a title/notes form posting to
  `POST /todos/{id}`, quick-schedule forms posting to `POST /todos/{id}/schedule`, a deadline
  form posting to `POST /todos/{id}/deadline`, tag assign/unassign forms per available tag, and
  a link to the detail page for checklist and recurrence.
- Commits are ordinary commands followed by redirects to `return_to`; cancel is an anchor to
  `return_to`, so dismissal performs no write and reloads the list deterministically.
- `UpdateTodoForm` gained an optional `return_to`, preserving the detail-page default when
  absent.

## Data and Persistence

- None; all writes reuse existing version-guarded commands.

## Offline and Sync

- Commit semantics equal the standalone detail editor; failures redirect with typed error query
  states rather than partially applying editor-local state.

## UI and Accessibility

- Native `<details>`/`<summary>` provides keyboard/pointer expansion without script; labels name
  the task; all inputs are labeled; the closed row is not made noisier (progressive fields live
  inside the disclosure).

## Error Handling

- Title/notes failures redirect to the detail error state; schedule/deadline/tag failures use the
  existing per-route error redirects.

## Testing Strategy

- Web tests for editor markup (fields, quick controls, tag buttons, cancel), commit behavior
  including the `?saved=1` return target, quick schedule/deadline/reschedule commits, and tag
  assign/remove affordance switching.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: no new editor command or draft table is needed; each control posts to an existing
  route, keeping optimistic behavior and undo identical to the detail editor.
