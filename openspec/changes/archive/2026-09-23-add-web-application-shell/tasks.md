# Tasks — add-web-application-shell

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation

- [x] 2.1 Add `AccountId` identity newtype in `momentum-domain`.
- [x] 2.2 Add `momentum-app::session` with `SessionState`, `AccountSession`, and serde round-trip tests.
- [x] 2.3 Define the `SessionResolver` contract and anonymous default resolver in the web crate.

## 3. UI and routing

- [x] 3.1 Add `PublicLayout` and `AppLayout` with accessible landmarks and primary navigation.
- [x] 3.2 Add `Loading`, `NotFound`, `Unauthorized`, `ErrorView`, and `AppBoundary` components.
- [x] 3.3 Add the route table covering built-in lists, All Projects/Areas, entity routes, `/login`, `/settings`, and `/*any`.
- [x] 3.4 Add `Protected` session boundary and session bootstrap JSON element for hydration.
- [x] 3.5 Wire `generate_route_list` + `leptos_routes_with_context` + `file_and_error_handler` in the Axum server.

## 4. Tests and verification

- [x] 4.1 Add integration tests: authenticated built-in list routes render their destination.
- [x] 4.2 Add integration tests: anonymous protected route renders the unauthorized boundary without task UI.
- [x] 4.3 Add integration tests: unknown route renders the not-found boundary; SSR contains session bootstrap JSON.
- [x] 4.4 Add component tests for loading, not-found, unauthorized, and error boundaries.
- [x] 4.5 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 4.6 Perform explicit artifact-to-implementation verification (no `/opsx:verify` skill installed) and record it in `OVERNIGHT_REPORT.md`.
- [x] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit

- [x] 5.1 Re-run `openspec validate add-web-application-shell --strict --no-interactive`.
- [x] 5.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 5.3 Archive with `openspec archive add-web-application-shell --yes` without bypassing validation.
- [x] 5.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 5.5 Review the full Git diff for accidental/unrelated changes.
- [x] 5.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
