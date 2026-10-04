# Proposal

## Why

Motion should be one system, not per-component one-offs: insertion, removal, editor expansion,
popovers, drag, and navigation need centralized primitives that never touch domain state and
always honor reduced motion.

## What Changes

- Add `momentum_app::motion`: a registry of primitives (insert, remove, editor, popover, drag,
  navigation) with stable ids, CSS class names, duration tokens, and essential flags, plus
  lookups (`primitive`, `class_for`, `nonessential`).
- Add the CSS primitives using the existing motion/easing tokens; wire classes into markup (rows
  carry insert+drag, the expanded editor carries editor motion, the Quick Entry panel carries
  popover motion, the main content carries navigation motion; removal motion is applied by the
  client as a row leaves).
- Reduced motion: every nonessential primitive is suppressed under
  `prefers-reduced-motion: reduce` and the explicit `[data-reduced-motion="true"]` override;
  motion is presentation-only, so interruptions can never block command completion or reorder
  domain state.

## Capabilities

### New Capabilities

- `features/animation-system`: motion primitives, state-preserving animation, reduced motion.

### Modified Capabilities

(none)

## Impact

- `crates/app` (motion module + unit tests), `crates/web` (markup classes, CSS, tests). No schema
  change.
- Non-goals: JS-driven layout animations, spring physics, per-theme motion variants.
