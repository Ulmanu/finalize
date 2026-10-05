# Design — add-visual-parity-suite

## Context

No browser automation exists in the workspace; the server-rendered markup is the deterministic
artifact available to CI. Visual parity therefore captures canonicalized structural snapshots
that are byte-stable for seeded data.

## Requirements Driving the Design

- `Fixed visual baselines`
- `Deterministic screenshot fixtures`
- `Diff reporting`

## Architecture

- Normalization removes the only nondeterministic parts of SSR output: hydration/resource scripts,
  UUIDs, emails, 32-hex tokens, nonce/crossorigin values; attributes are sorted with a
  quote-aware scanner and whitespace is collapsed. Tests assert idempotence on every render.
- Fixtures pair pages with fixed viewports; the baseline is a versioned JSON map of fixture name
  to snapshot, committed for review. Missing baselines or `UPDATE_VISUAL_BASELINES=1` regenerate.
- Diffs compute a differing-line ratio against a threshold (`VISUAL_DIFF_THRESHOLD`, default
  exact) and write expected/actual/diff files for inspection.

## Data and Persistence

- One committed baseline JSON file; artifacts are transient build outputs.

## Offline and Sync

- Not applicable.

## UI and Accessibility

- Snapshots include the accessibility markup (skip link, landmarks, labels), so structural
  regressions in those contracts are caught here too.

## Error Handling

- Missing fixture baselines fail with the fixture name; diffs fail with ratio and artifact path.

## Testing Strategy

- Unit tests for normalization determinism, placeholder replacement, diff/artifact writing, and
  baseline round-trips; the parity suite itself renders 12 fixtures and compares against the
  committed baseline.

## Rollout / Migration

- Regenerating baselines is an explicit reviewed action.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: pixel screenshots require a browser harness the workspace lacks; structural snapshots
  are the deterministic equivalent today and the browser layer remains the documented gap.
