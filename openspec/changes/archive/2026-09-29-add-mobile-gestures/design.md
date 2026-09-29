# Design — add-mobile-gestures

## Context

Rows and lists already expose drop targets, selection, and quick actions. Touch recognition is the
missing layer; it must be pure so both the hydrated client and tests agree on thresholds.

## Requirements Driving the Design

- `Swipe actions`
- `Long-press drag`
- `Pull-to-search`

## Architecture

- `classify_touch(TouchSample { dx, dy, duration_ms })` evaluates long press first (>= 500 ms with
  hypot drift <= 10 px → `Open`), then swipes (<= 800 ms, |dx| >= 60, |dx| > |dy| → left/right),
  otherwise `None`. Vertical or slow movement therefore scrolls/navigates normally instead of
  firing row actions.
- `pull_to_search(started_at_top, dx, dy, duration_ms)` requires the top scroll boundary, an
  80 px downward pull, a fast duration, and mostly vertical travel.
- Hydrate wiring: the layout records touchstart state with `std::time::Instant` and navigates to
  `/search` on a qualifying touchend; swipe/long-press actions are exposed through
  `default_swipe_configuration()` for the row client dispatch.

## Data and Persistence

- None; recognition is arithmetic over deltas.

## Offline and Sync

- Client-only navigation; no state changes.

## UI and Accessibility

- Tapping and scrolling are never hijacked (thresholds, slop, and duration limits); pull requires
  the top boundary so scrolling up from mid-list is unaffected; all touch actions have
  server-rendered keyboard/form equivalents.

## Error Handling

- Missing touches, sub-threshold movement, and stale state simply produce no action.

## Testing Strategy

- App unit tests cover the full classification matrix (swipe threshold/direction, slow gestures,
  vertical travel, long-press slop and duration, pull boundary/distance/duration/angle); the wasm
  build validates the hydrate wiring compiles.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: recognition lives in the app crate (shared, testable) and DOM wiring is `hydrate` +
  `wasm32`-gated so native `--all-features` test builds never run client APIs.
