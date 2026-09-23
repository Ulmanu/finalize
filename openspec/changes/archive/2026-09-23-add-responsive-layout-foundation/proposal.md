# Proposal

## Why

The shell currently stacks on narrow screens by hiding the sidebar, which loses feature
reachability on tablet and mobile. Things-style parity requires deliberate desktop, tablet,
and mobile navigation models with touch-sized targets, and later gesture/multi-selection work
needs a known layout mode to key off.

## What Changes

- Add an explicit layout model with documented breakpoints and a pure `LayoutMode::from_width`
  mapping (desktop/tablet/mobile) plus a client-side layout state signal.
- Desktop keeps persistent navigation; tablet collapses navigation into an overlay drawer
  opened from a toggle; mobile uses a single-column model with a bottom destination bar and an
  overlay drawer for secondary destinations.
- Add touch-sized targets (row/touch tokens) and make every destination reachable in all three
  modes.
- Extend the shell markup with the toggle button (`aria-expanded`/`aria-controls`), drawer
  sidebar, and mobile bottom navigation; keep one primary landmark per mode.

## Capabilities

### New Capabilities

- `ui/responsive-layout`: desktop/tablet/mobile application shells with parity-oriented
  navigation, explicit breakpoints, and touch-sized controls.

### Modified Capabilities

(none)

## Impact

- `crates/web/src/responsive.rs`, `layouts.rs`, `assets/app.css` (breakpoints/drawer/bottom
  nav), shell tests.
- No persistence, no domain behavior change.
- Non-goals: gestures/swipe (change 47), multi-window (change 60), no handwritten JS/TS.
