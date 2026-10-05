# Proposal

## Why

The roadmap is complete; closure requires a full audit against the frozen parity matrix,
classification of every unresolved item, and a release-quality validation run across the whole
workspace.

## What Changes

- Parity matrix closure: `PARITY_MATRIX.md` audits all 74 roadmap capabilities with status,
  canonical spec, and archived change, and classifies every unresolved finding
  (hydration-harness, browser-adapter, deployment-integration, out-of-web-scope, environment)
  with rationale and evidence.
- Zero known implementation gaps: the workspace contains no TODO/FIXME markers, no ignored
  tests, and no in-scope capability without implemented, tested requirement scenarios; the audit
  found no BUG/MISSING findings requiring code changes.
- Release-quality validation: strict OpenSpec validation for canonical specs and archived
  changes, the full Rust quality gates (fmt, architecture check, clippy `-D warnings`, workspace
  tests including behavior and visual parity suites), the WASM client build, and a release build
  of both production binaries.

## Capabilities

### New Capabilities

- `features/parity-hardening`: parity matrix closure, zero known implementation gaps,
  release-quality validation.

### Modified Capabilities

(none)

## Impact

- `PARITY_MATRIX.md`, `OVERNIGHT_REPORT.md`, and this change's artifacts. No product code changes;
  validation evidence is recorded in the report.
- Non-goals: implementing the classified external/hydration items (they are deployment or
  browser-harness work outside the web product's in-scope surface).
