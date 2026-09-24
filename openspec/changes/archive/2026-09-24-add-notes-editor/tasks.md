# Tasks — add-notes-editor

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation

- [x] 2.1 Add pure notes utilities (link detection, safe-URL policy, find/replace).
- [x] 2.2 Reuse the existing note commands for persisted replacements.

## 3. Infrastructure and UI

- [x] 3.1 Render the notes panel (links, find traversal, replace-all) on to-do and project pages.
- [x] 3.2 Add replacement routes backed by the existing commands.

## 4. Tests and verification

- [x] 4.1 Add domain unit tests for links, safety, traversal, and replacement.
- [x] 4.2 Add web tests for link rendering, blocked schemes, find, and replace.
- [x] 4.3 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 4.4 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 4.5 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-notes-editor --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive add-notes-editor --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
