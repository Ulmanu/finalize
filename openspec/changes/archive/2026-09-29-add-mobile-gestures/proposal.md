# Proposal

## Why

Touch interactions need the same determinism as keyboard commands: recognition must not fight
scrolling, long press must not double-fire as a click, and pull-to-search must only trigger at the
top boundary.

## What Changes

- Add `momentum_app::gestures`: pure classification of touch samples into row gestures
  (long press opens the editor only within slop; fast horizontal swipes beyond the threshold map
  to configured left/right actions; everything else is scroll/tap) plus `pull_to_search`
  (top boundary, downward, quick, mostly vertical).
- Add a default swipe configuration (left selects, right opens When controls) so swipe actions are
  declared rather than hard-coded.
- Wire pull-to-search in the authenticated layout for `hydrate` + `wasm32` builds: touchstart
  records position/timestamp/top-boundary state, touchend navigates to Quick Find when the
  gesture qualifies; non-wasm builds compile the handlers as no-ops.

## Capabilities

### New Capabilities

- `features/mobile-gestures`: swipe actions, long-press drag entry, pull-to-search.

### Modified Capabilities

(none)

## Impact

- `crates/app` (gesture machines + unit tests), `crates/web` (layout pull wiring). No schema
  change; SSR output unchanged.
- Non-goals: full drag-and-drop reorder gesture rewiring (the durable drop-target operations and
  selection machine exist from changes 32/33), gesture animation tuning, haptics.
