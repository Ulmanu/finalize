# Tasks — add-things-design-system

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Tokens and assets

- [x] 2.1 Add `design/tokens.rs` with typography, spacing, radius, surface, shadow, icon, row, sidebar, and motion tokens.
- [x] 2.2 Add `assets/design.css` custom properties matching the Rust tokens; add the token/CSS consistency test.
- [x] 2.3 Add original Rust inline-SVG icon components.

## 3. Primitives

- [x] 3.1 Implement `CheckControl`, `ProgressRing`, and `SectionHeading`.
- [x] 3.2 Implement `TaskRow`, `ProjectRow`, and `SidebarRow` using tokens and icons.
- [x] 3.3 Implement `Popover`, `ContextMenu`, `DateControl`, `AddControl`, and `Motion`.
- [x] 3.4 Add the public `/styleguide` catalog route and serve `/static/design.css`.
- [x] 3.5 Migrate existing shell/layout/boundary styles to the tokens.

## 4. Tests and verification

- [x] 4.1 Add SSR tests for every primitive (roles, ARIA, token classes, original icons).
- [x] 4.2 Add the token/CSS consistency test and the no-proprietary-assets test.
- [x] 4.3 Add the `/styleguide` integration test.
- [x] 4.4 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 4.5 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 4.6 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-things-design-system --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive add-things-design-system --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
