# Design — things-web-parity-hardening

## Context

Every capability 01–73 is archived with canonical specs and automated coverage; this change
performs the final audit and release validation rather than adding features.

## Requirements Driving the Design

- `Parity matrix closure`
- `Zero known implementation gaps`
- `Release-quality validation`

## Architecture

- The matrix is generated from the roadmap headers, archive directories, and canonical spec
  paths, so it cannot drift from the repository: each row names its capability, status, spec, and
  archive entry. Unresolved findings are classified with rationale and evidence instead of being
  left implicit.
- Gap search: zero TODO/FIXME markers in Rust sources, zero `#[ignore]` tests, and every archived
  change carries checked tasks and a passing suite.
- Validation battery: strict `openspec validate` (specs and archived), `scripts/quality-gates.sh`
  (fmt, arch-check, clippy, workspace tests including behavior/visual parity), `scripts/build-web.sh`,
  and `cargo check --release` for the server and worker binaries.

## Data and Persistence

- None.

## Offline and Sync

- Covered by the behavior parity suite.

## UI and Accessibility

- Covered by the accessibility audit and visual parity suite.

## Error Handling

- Any validation failure blocks closure; results are recorded in `OVERNIGHT_REPORT.md`.

## Testing Strategy

- The validation battery itself; the matrix documents how each capability is covered.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: classified gaps are external integrations or the hydration/browser harness, not
  unimplemented in-scope capabilities; they are listed with evidence so closure is auditable.
