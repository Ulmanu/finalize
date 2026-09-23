# Design — add-responsive-layout-foundation

## Context

Responsive behavior must be deliberate: breakpoints are part of the design system and layout
mode is observable to later interaction work (gestures, selection). SSR cannot know the
viewport, so the server renders all layout regions and CSS controls visibility, while a
client-side signal tracks the resolved mode for behavior.

## Requirements Driving the Design

- `Desktop layout`
- `Tablet layout`
- `Mobile layout`

## Architecture

- `crates/web/src/responsive.rs`:
  - `LayoutMode { Desktop, Tablet, Mobile }` with `from_width(width_px)`.
  - Breakpoints: `TABLET_MIN = 768`, `DESKTOP_MIN = 1024` (CSS uses `48rem`/`64rem` at a
    16px root; documented as the canonical mapping).
  - `LayoutState` (provided at the app root) holds `mode` and `drawer_open` signals; hydrate
    updates `mode` from `window.innerWidth` and listens for resize.
- `layouts.rs` `AppLayout` renders:
  - toggle button (`aria-expanded`, `aria-controls="app-sidebar"`),
  - sidebar drawer (`id="app-sidebar"`, `data-open`),
  - mobile bottom destination bar (primary lists),
  - one `main` landmark.
- CSS: desktop grid at ≥64rem; tablet/mobile drawer with overlay and `hidden`/visible states;
  mobile bottom bar below 48rem; touch targets use the row-height token.

## Data and Persistence

None. Layout mode and drawer state are UI-only and never synchronized.

## Offline and Sync

Not applicable.

## UI and Accessibility

- The toggle is a real `button` with an accessible name and state.
- Drawer uses `role="dialog"` semantics only when open on mobile/tablet; the overlay is a
  sibling button with `aria-label="Close navigation"`.
- Focus stays visible; Escape closes the drawer.
- Touch targets: minimum `var(--row-height)` (44px) for primary controls.

## Error Handling

Resize handling clamps unknown sizes to `Desktop`; no panics on missing window (SSR).

## Testing Strategy

- Unit tests for `LayoutMode::from_width` boundaries.
- SSR tests: toggle button with `aria-controls`, sidebar id, bottom bar present, single `main`.
- CSS assertions for the documented breakpoints and reduced-motion-safe drawer transition.
- Browser verification of real resize behavior is deferred to the behavior-parity suite.

## Rollout / Migration

Visual/layout only; no data formats affected.

## Open Questions Resolved From the Repository

- **SSR mode**: default `Desktop` because the server has no viewport; hydration corrects it.
- **Hidden regions and a11y**: CSS `display: none` keeps inactive regions out of the
  accessibility tree, so duplicated destinations do not confuse screen readers.

## Tasks / Definition of Done

See `tasks.md`.
