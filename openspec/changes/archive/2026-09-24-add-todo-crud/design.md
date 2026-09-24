# Design — add-todo-crud

## Context

Change 09 provides the command boundary; this change adds the missing duplicate command, read
queries, and the first real UI so the lifecycle is user-visible and tested end to end.

## Requirements Driving the Design

- `Todo lifecycle`
- `Metadata preservation`
- `Optimistic concurrency`

## Architecture

- `crates/app/src/commands.rs`: new `DuplicateTodo { id }` — loads the source, creates a new
  to-do with a fresh identity copying title/notes/schedule/deadline/reminder/context/status
  reset to open; emits `TodoCreated`.
- `crates/app/src/todos.rs`:
  - `TodoSummary` read model and `TodoScope` (`Inbox`, `AllOpen`).
  - `TodoReadStore` port (`list_todos`, `get_todo`).
  - `TodoService { dispatcher, reader }` with lifecycle methods that construct envelopes and
    dispatch them, returning outcomes; reads go through the port.
- `crates/infra/src/todos.rs`: `PgTodoReadStore` (account-scoped selects, context join).
- `crates/testkit/src/todos.rs`: `MemoryTodoReadStore` reading from `MemoryDatabase`.
- `crates/web`:
  - `AppState.todos: Option<Arc<TodoService>>`.
  - `/inbox` lists inbox to-dos as `TaskRow`s with an add form; `/todos/:id` renders an editor
    with title/notes forms and lifecycle buttons.
  - Command routes: `POST /todos`, `POST /todos/:id`, `POST /todos/:id/{complete,cancel,
    reopen,delete,restore,duplicate}`; all redirect back (303) and require an authenticated
    session.

## Data and Persistence

No schema change. The read store queries `entities` + `placements` with `account_id` scoping;
writes stay in `PgUnitOfWork`.

## Offline and Sync

All mutations are commands with operation ids, so offline capture can reuse them; the read
store is replaceable by the offline replica in change 49.

## UI and Accessibility

- Native forms and buttons; labels for inputs; `aria-label`s on row actions.
- Completed rows show the completed state; delete/restore are explicit actions.
- The list uses the design-system `TaskRow`; errors render as status messages.

## Error Handling

Command errors map to redirects with an `error` query parameter; not-found/conflict never
render raw errors. Unauthenticated posts redirect to `/login`.

## Testing Strategy

- App tests: duplicate copies metadata and creates a new identity; lifecycle service methods.
- Web tests over in-memory adapters: create → appears in Inbox; complete → completed state;
  delete → removed; duplicate → second row; editor update → title/notes persist; metadata
  preserved across title edit and completion.
- Concurrency: command-layer conflict test already covers stale versions; service test asserts
  the error surfaces.

## Rollout / Migration

No data migration; the UI is additive.

## Open Questions Resolved From the Repository

- **Form posts vs server functions**: native posts keep the feature testable and
  progressively enhanced; server functions can wrap the same service later.
- **Duplicate title**: duplicated to-dos keep the same title (Things-like), distinguished by
  identity; a "(copy)" suffix was rejected to avoid inventing behavior.

## Tasks / Definition of Done

See `tasks.md`.
