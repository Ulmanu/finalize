# Tasks — add-todo-crud

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Application layer

- [x] 2.1 Add `DuplicateTodo` to the command layer with events and invalidations.
- [x] 2.2 Add `TodoSummary`, `TodoScope`, `TodoReadStore`, and `TodoService` with lifecycle methods.

## 3. Adapters and UI

- [x] 3.1 Add `PgTodoReadStore` and the testkit `MemoryTodoReadStore`.
- [x] 3.2 Wire `TodoService` into `AppState` and add command-backed POST routes.
- [x] 3.3 Render the Inbox list with an add form and the to-do editor page with lifecycle actions.

## 4. Tests and verification

- [x] 4.1 Add duplicate and lifecycle service tests (metadata preservation).
- [x] 4.2 Add web tests: create/list/complete/delete/duplicate/editor update and unauthenticated rejection.
- [x] 4.3 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 4.4 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 4.5 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-todo-crud --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive add-todo-crud --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
