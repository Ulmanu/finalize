# Tasks — add-undo-redo-engine

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Undo engine

- [x] 2.1 Implement inverse-command derivation for every command variant using pre-state.
- [x] 2.2 Implement `UndoEntry`, `UndoGroup`, and `UndoEngine::{apply_group, undo, redo}` over the dispatcher.
- [x] 2.3 Implement the bounded `UndoStack` with grouped semantics.

## 3. Tests and verification

- [x] 3.1 Add inverse-derivation tests for each command variant.
- [x] 3.2 Add round-trip tests (create/delete/complete/reopen/edit) preserving identity and values.
- [x] 3.3 Add grouped undo/redo tests and history-window eviction tests.
- [x] 3.4 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 3.5 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 3.6 Resolve all critical/actionable verification findings.

## 4. Archive and commit

- [x] 4.1 Re-run `openspec validate add-undo-redo-engine --strict --no-interactive`.
- [x] 4.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 4.3 Archive with `openspec archive add-undo-redo-engine --yes` without bypassing validation.
- [x] 4.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 4.5 Review the full Git diff for accidental/unrelated changes.
- [x] 4.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
