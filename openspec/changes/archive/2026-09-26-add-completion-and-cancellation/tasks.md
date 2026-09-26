# Tasks — add-completion-and-cancellation

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Application layer

- [x] 2.1 Keep completion and cancellation distinct with per-status timestamps.
- [x] 2.2 Enforce reopen policy (terminal only; deleted requires restore).
- [x] 2.3 Make project completion safe for direct and heading children.

## 3. Adapters and UI

- [x] 3.1 Expose terminal timestamps in the memory and PostgreSQL read models.
- [x] 3.2 Verify existing web controls end to end.

## 4. Tests and verification

- [x] 4.1 Add distinctness, reopen, and project safety tests.
- [x] 4.2 Add web tests for cancel/reopen and project policies.
- [x] 4.3 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 4.4 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 4.5 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-completion-and-cancellation --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive add-completion-and-cancellation --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
