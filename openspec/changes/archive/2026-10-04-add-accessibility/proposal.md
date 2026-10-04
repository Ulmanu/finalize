# Proposal

## Why

Keyboard-only operation, screen-reader semantics, visible focus, and motion/target/contrast
respect are product requirements that must be verified, not assumed.

## What Changes

- Keyboard access: a skip link to `#app-main`, `:focus-visible` outlines, logical DOM focus
  order, and the existing registry-driven `aria-keyshortcuts` hints; all workflows already have
  keyboard equivalents (forms, buttons, links).
- Semantics: the companion progress exposes `role="progressbar"` with aria-valuemin/max/now;
  Today keeps its native `<progress>` with an accessible label; the Quick Entry overlay is
  `role="dialog"` with `aria-modal="true"`; navigation carries `aria-label="Primary"` and
  `aria-current` on the active destination; controls are labeled.
- Motion/target/contrast: reduced-motion rules (system + explicit override), 44px minimum touch
  targets under `pointer: coarse`, focus outlines using the accent token, and high-contrast
  strengthening (from 64).
- Add an accessibility audit test suite that scans core pages for skip link/main landmarks,
  labeled inputs/selects/textareas/buttons, progress/dialog semantics, and the CSS contract.

## Capabilities

### New Capabilities

- `features/accessibility`: keyboard-only operation, semantic accessibility,
  motion/target/contrast requirements.

### Modified Capabilities

(none)

## Impact

- `crates/web` (layout skip link/main id, progress/dialog semantics, CSS, audit tests). No schema
  change.
- Non-goals: full WCAG certification, screen-reader lab testing, custom focus traps (native
  `:target` dialog has no scripted trap).
