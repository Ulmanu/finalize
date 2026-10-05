# Proposal

## Why

Visual regressions need fixed, reviewable baselines: deterministic seeded renders at fixed
viewports compared against committed snapshots, with inspectable diffs when they change.

## What Changes

- Add `momentum_testkit::visual`: fixed `Viewport` definitions (desktop/tablet/mobile),
  `VisualFixture` naming, deterministic `normalize_html` (hydration scripts removed; UUIDs,
  emails, 32-hex tokens, nonces, and crossorigin values replaced; attributes sorted; whitespace
  collapsed), `VisualBaseline` JSON load/save, `compare` with a configurable threshold, and
  expected/actual/diff artifact writing.
- Add the web visual-parity suite: seeded pages (/inbox, /today, /companion, /settings) rendered
  at three fixed viewports each, normalized, and compared against
  `crates/web/tests/visual-baselines.json`; `UPDATE_VISUAL_BASELINES=1` regenerates and
  `VISUAL_DIFF_THRESHOLD` configures tolerance; failures write artifacts under
  `target/visual-artifacts`.
- Determinism comes from fixed test clocks, deterministic auth/token generators, seeded titles,
  script stripping, and normalization; animation classes are suppressed via the motion system and
  the snapshots are structural.

## Capabilities

### New Capabilities

- `features/visual-parity-suite`: fixed visual baselines, deterministic screenshot fixtures,
  diff reporting.

### Modified Capabilities

(none)

## Impact

- `crates/testkit` (visual primitives + unit tests), `crates/web` (parity suite + committed
  baseline file). No schema or product behavior change.
- Non-goals: pixel-level browser screenshots (no browser harness in the workspace; the hydrated
  visual layer remains a documented gap), perceptual image diffing.
