# Proposal

## Why

The shell currently styles destinations with ad-hoc CSS and inline class names. Every later
feature (task rows, projections, editors, drag-and-drop) needs one shared visual language so
that behavior and appearance stay consistent and so accessibility/motion decisions are made
once. This change introduces centralized tokens and reusable primitives before feature UI is
built.

## What Changes

- Add centralized design tokens for typography, spacing, radius, surface, shadow, icon sizing,
  row metrics, sidebar metrics, and motion durations/easing, exposed as Rust constants and CSS
  custom properties with a single source of truth per value.
- Add reusable Leptos primitives: `TaskRow`, `ProjectRow`, `SidebarRow`, `CheckControl`,
  `ProgressRing`, `SectionHeading`, `Popover`, `ContextMenu`, `DateControl`, `AddControl`,
  `Icon`, and motion wrappers.
- Add an original icon set rendered as Rust-defined inline SVG (no Cultured Code assets).
- Add a public `/styleguide` route that renders every primitive, used by tests and later by
  visual-parity fixtures.
- Migrate the existing shell/layout/boundary styles onto the tokens.

## Capabilities

### New Capabilities

- `ui/design-system`: centralized visual tokens, reusable Things-style primitives, original
  asset policy, and motion primitives with reduced-motion support.

### Modified Capabilities

(none)

## Impact

- `crates/web/src/design/` (tokens, icons, primitives, motion), `assets/design.css`,
  `/styleguide` route, shell/layout/boundary styles updated to tokens.
- No persistence, no migrations, no domain behavior change.
- Non-goals: no task/projection logic; no drag-and-drop or gesture behavior (later changes);
  no handwritten JS/TS; no Cultured Code assets.
