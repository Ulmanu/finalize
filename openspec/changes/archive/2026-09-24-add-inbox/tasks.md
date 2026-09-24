# Tasks — add-inbox

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Application layer

- [x] 2.1 Add `ReorderTodo { id, direction }` to the command layer with scope-order operations on the unit of work.
- [x] 2.2 Implement scope-order read/write in the PostgreSQL and in-memory adapters.
- [x] 2.3 Add `TodoService::{reorder_up, reorder_down, schedule_quick}`.

## 3. UI

- [x] 3.1 Add the Inbox badge to the primary navigation.
- [x] 3.2 Add reorder, schedule, and move controls to inbox rows and the to-do editor.
- [x] 3.3 Add processing routes (`move-up`, `move-down`, `schedule`, `move`).

## 4. Tests and verification

- [x] 4.1 Add command tests for reordering (order change, boundary no-op, scope independence).
- [x] 4.2 Add web tests: badge count, reorder buttons, scheduling out of Inbox, move to a destination.
- [x] 4.3 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 4.4 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 4.5 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-inbox --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive add-inbox --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
