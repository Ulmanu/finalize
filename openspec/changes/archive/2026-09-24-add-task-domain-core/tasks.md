# Tasks — add-task-domain-core

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain implementation

- [x] 2.1 Add `EntityId` and `ChecklistItemId` identifiers.
- [x] 2.2 Implement `LocalDate` with UTC+offset derivation, day arithmetic, parsing, display, and serde.
- [x] 2.3 Implement `EntityKind`, `Status`, `EntityMeta`, and `Title`.
- [x] 2.4 Implement `ScheduleState`/`Schedule` with evening semantics and actionable/future predicates.
- [x] 2.5 Implement `Deadline`/`DeadlineState` and `Reminder`/`ReminderState` as independent concepts.
- [x] 2.6 Implement `ContextRef`, `Todo`, `Project`, `Area`, `Heading`, `Tag`, `ChecklistItem`, and `ProjectCompletionPolicy`.

## 3. Tests and verification

- [x] 3.1 Add `LocalDate` tests (derivation, arithmetic, serialization round-trip).
- [x] 3.2 Add temporal-independence tests for schedule/deadline/reminder.
- [x] 3.3 Add membership/identity tests (single contextual parent, move preserves fields).
- [x] 3.4 Add status-transition and project-completion-policy tests.
- [x] 3.5 Add checklist-independence and title-validation tests.
- [x] 3.6 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 3.7 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 3.8 Resolve all critical/actionable verification findings.

## 4. Archive and commit

- [x] 4.1 Re-run `openspec validate add-task-domain-core --strict --no-interactive`.
- [x] 4.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 4.3 Archive with `openspec archive add-task-domain-core --yes` without bypassing validation.
- [x] 4.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 4.5 Review the full Git diff for accidental/unrelated changes.
- [x] 4.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
