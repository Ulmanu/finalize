# Tasks — add-pwa

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation

- [x] 2.1 Implement `PWA installability` through the manifest and shell.
- [x] 2.2 Implement `Offline shell` with the generated service worker.
- [x] 2.3 Implement `Update lifecycle` with skipWaiting/claim and a single reload.

## 3. Infrastructure and UI

- [x] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [x] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [x] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [x] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification

- [x] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `PWA installability`.
- [x] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Offline shell`.
- [x] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Update lifecycle`.
- [x] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [x] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [x] 4.6 Run `/opsx:verify add-pwa` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [x] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-pwa --strict --no-interactive`.
- [x] 5.2 Archive with `openspec archive add-pwa --yes` without bypassing validation.
- [x] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [x] 5.4 Review the full Git diff for accidental/unrelated changes.
- [x] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.
