# Proposal

## Why

Users need explicit light/dark/system theming and adjustable text size without breaking layout or
accessibility, and both must coexist with contrast and reduced-motion preferences.

## What Changes

- Extend the settings registry with `appearance.text_size` (normal/large/extra_large) and
  `appearance.reduced_motion` (bool), both browser-local alongside the existing
  `appearance.theme` (system/light/dark).
- Stylesheets gain explicit `[data-theme="dark"]`/`[data-theme="light"]` token sets (with the
  system media query scoped to `:root:not([data-theme="light"])`), rem-based text scaling via
  `html[data-text-size]`, `prefers-contrast: more` strengthening, and an app-level
  `[data-reduced-motion="true"]` override on top of the existing system media queries.
- Settings renders and validates the new preferences through the existing registry machinery;
  the hydrated client applies browser-local values by setting the root attributes.

## Capabilities

### New Capabilities

- `features/theme-and-text-sizing`: theme choice, text size, contrast/reduced motion
  compatibility.

### Modified Capabilities

(none)

## Impact

- `crates/app` (registry entries), `crates/web` (design tokens/CSS, tests). No schema change.
- Non-goals: custom color themes, per-list typography, animation design beyond honoring
  reduced motion.
