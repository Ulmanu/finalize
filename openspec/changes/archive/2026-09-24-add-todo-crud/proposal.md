# Proposal

## Why

The domain, ordering, and command layers exist, but users cannot create or manage a to-do in
the application. This change delivers the first end-to-end feature: the to-do lifecycle
through the command layer with a real list UI.

## What Changes

- Add `DuplicateTodo` to the command layer (copies title/notes/schedule/deadline/reminder/
  context into a new canonical identity; checklist/tags arrive with their capabilities).
- Add `momentum-app::todos`: a read-store port plus a `TodoService` that builds and dispatches
  lifecycle commands (create, edit title/notes, duplicate, move, schedule, deadline, reminder,
  complete, cancel, reopen, delete, restore) and lists to-dos for a scope.
- Add read-store adapters: PostgreSQL (`PgTodoReadStore`) and in-memory (testkit).
- Wire the web application: `/inbox` renders real to-dos with an add form, row completion,
  and navigation to `/todos/:id`, which renders an editor with lifecycle actions; all
  mutations post to command-backed routes and redirect back.
- Every mutation is account-scoped, goes through the command layer, and is covered by tests.

## Capabilities

### New Capabilities

- `features/todo-crud`: the to-do create/edit/duplicate/move/complete/cancel/reopen/delete
  lifecycle with list and editor UI.

### Modified Capabilities

(none)

## Impact

- `crates/app` (commands + todos), `crates/infra` (read store), `crates/testkit` (read store +
  tests), `crates/web` (inbox list, to-do editor, command routes, AppState wiring).
- No schema change.
- Non-goals: projections beyond a basic Inbox listing (change 12), drag-and-drop (change 32),
  the expanded editor (change 35), checklists/tags (changes 16/17).
