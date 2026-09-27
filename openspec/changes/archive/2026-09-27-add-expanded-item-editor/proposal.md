# Proposal

## Why

Editing a task should happen in place: expanding a row into an editor keeps the user's list
position and selection context, while progressive metadata stays out of the closed row.

## What Changes

- Add an inline expanded editor to Inbox rows as a native `<details>` disclosure containing the
  title/notes commit form, quick schedule controls, a deadline control, tag assign/remove
  controls, and a link to checklist and recurrence surfaces.
- Extend `POST /todos/{id}` with an optional `return_to` so editor commits land back on the list
  with the deterministic `?saved=1` state; cancellation is a plain navigation to the list without
  submitting, so no partial writes occur.
- Progressive disclosure: schedule/deadline/tag/checklist/recurrence stay accessible from the
  expanded surface through existing command routes and the detail page.

## Capabilities

### New Capabilities

- `features/expanded-item-editor`: inline expansion, progressive fields, commit/cancel semantics.

### Modified Capabilities

(none)

## Impact

- `crates/web` (UpdateTodoForm return_to, `RowEditorContext`, TodoRow editor markup, Inbox
  wiring) and tests. No schema change; no new commands.
- Non-goals: pointer/animations for expansion (hydrated client), moving existing closed-row
  controls into the editor.
