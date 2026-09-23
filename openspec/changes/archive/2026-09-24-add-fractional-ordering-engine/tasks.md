# Tasks — add-fractional-ordering-engine

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Ordering engine

- [x] 2.1 Implement `Rank` with gap-based midpoint insertion, append/prepend, storage encoding, and parsing.
- [x] 2.2 Implement `OrderingScope`, `OrderedItem`, and `ScopeOrder` with insert/move/append/rebalance plans.
- [x] 2.3 Implement exhaustion detection and deterministic rebalance preserving visible order.

## 3. Tests and verification

- [x] 3.1 Add rank encoding/comparison and midpoint tests.
- [x] 3.2 Add insert/append/prepend/move plan tests including boundaries.
- [x] 3.3 Add exhaustion + rebalance tests (including a long insertion sequence).
- [x] 3.4 Add scope-independence tests (same entity in context and Today; cross-scope isolation).
- [x] 3.5 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 3.6 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md`.
- [x] 3.7 Resolve all critical/actionable verification findings.

## 4. Archive and commit

- [x] 4.1 Re-run `openspec validate add-fractional-ordering-engine --strict --no-interactive`.
- [x] 4.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 4.3 Archive with `openspec archive add-fractional-ordering-engine --yes` without bypassing validation.
- [x] 4.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 4.5 Review the full Git diff for accidental/unrelated changes.
- [x] 4.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
