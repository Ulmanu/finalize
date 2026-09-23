# Tasks — add-responsive-layout-foundation

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Layout model

- [x] 2.1 Add `LayoutMode` with documented breakpoints and `from_width` mapping plus unit tests.
- [x] 2.2 Add `LayoutState` (mode + drawer open) provided at the application root, with hydrate resize handling.

## 3. Shell and styles

- [x] 3.1 Add the navigation toggle, overlay drawer, and mobile bottom destination bar to `AppLayout`.
- [x] 3.2 Add desktop/tablet/mobile CSS with touch-sized targets and drawer transitions that respect reduced motion.
- [x] 3.3 Ensure every destination remains reachable in all three modes.

## 4. Tests and verification

- [x] 4.1 Add SSR tests for toggle state, drawer id, bottom bar, and single main landmark.
- [x] 4.2 Add CSS breakpoint assertions and layout-mode unit tests.
- [x] 4.3 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 4.4 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 4.5 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-responsive-layout-foundation --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive add-responsive-layout-foundation --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
