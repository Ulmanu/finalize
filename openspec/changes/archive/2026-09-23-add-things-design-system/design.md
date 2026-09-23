# Design — add-things-design-system

## Context

Feature UI begins in change 11. Establishing tokens and primitives now prevents per-feature
styling drift and keeps accessibility/motion policy centralized. The app must remain
server-renderable and WASM-hydratable, so primitives are pure Leptos components with no
browser-only APIs at render time.

## Requirements Driving the Design

- `Central visual tokens`
- `Reusable Things-style primitives`
- `Original asset policy`

## Architecture

- `crates/web/src/design/tokens.rs` — the single Rust source of token values and semantic
  class names:
  - `space` (4/8/12/16/24/32), `radius` (sm/md/lg/pill), `typography` (display/title/body/
    caption/mono), `surface` (canvas/raised/sunken/overlay), `shadow` (row/popover/modal),
    `icon` sizes, `row` metrics (height/indent/inset), `sidebar` width, `motion`
    (fast/base/slow durations + standard/decelerate easings).
  - Tokens are emitted as CSS custom properties in `assets/design.css`; Rust constants hold the
    same values for components that need inline geometry (e.g. progress-ring radius).
  - A unit test asserts Rust token values match the CSS custom properties so the two sources
    cannot drift.
- `crates/web/src/design/icons.rs` — original inline-SVG icon components (`CheckIcon`,
  `PlusIcon`, `CalendarIcon`, `FlagIcon`, `TagIcon`, `NoteIcon`, `RepeatIcon`, `TrashIcon`,
  `ChevronIcon`), each `aria-hidden` by default and sized by token.
- `crates/web/src/design/primitives.rs`:
  - `CheckControl` — native `<input type="checkbox">` visually styled, or `role="checkbox"`
    button with `aria-checked`; disabled/checked/mixed states; callback prop.
  - `TaskRow` — checkbox + title + note indicator + tag chips + date/deadline badges; hover
    actions region; selected/editing states; row height/indent from tokens.
  - `ProjectRow`, `SidebarRow` — progress ring/icon + title + counts; sidebar indentation.
  - `ProgressRing` — SVG ring with `role="img"` + `aria-label` (percentage), token radius.
  - `SectionHeading` — list section titles (Today/This Evening style).
  - `Popover` — button + panel with `aria-expanded`, `aria-controls`, Escape dismissal; panel
    positioning via CSS; no portal needed yet.
  - `ContextMenu` — `role="menu"`/`menuitem` list with keyboard traversal contract.
  - `DateControl` — button showing a date/badge with accessible label.
  - `AddControl` — the add button used by Magic Plus later; reports activation intent.
  - `Motion` — wrapper applying a motion class and `data-motion` attribute; CSS honors
    `prefers-reduced-motion`.
- `/styleguide` (public) renders a catalog of every primitive in representative states.

## Data and Persistence

None. Primitives are stateless; callbacks are the extension point that later features connect
to commands.

## Offline and Sync

Not applicable. No state is persisted by primitives.

## UI and Accessibility

- Native semantics first (`button`, `input`, `nav`, headings); ARIA only where native elements
  cannot express the state.
- Focus-visible rings use the accent token with sufficient contrast in light and dark themes.
- Minimum pointer target size is a token (2.75rem row / 2.25rem compact).
- Motion tokens are centralized; reduced motion removes non-essential transitions.
- Contrast: token pairs are chosen to meet WCAG AA for text on their surfaces.

## Error Handling

Primitives do not fail: invalid data renders an empty/placeholder state; callbacks are
`Option`-based so presentation never panics on missing handlers.

## Testing Strategy

- SSR render tests for every primitive: expected roles/ARIA, token classes, original icon
  markup.
- Token/CSS consistency test (Rust constants vs `:root` custom properties).
- No-asset test: no binary icon/font files outside the original CSS/SVG sources; icons are
  inline Rust SVG.
- `/styleguide` integration test through the Axum router.

## Rollout / Migration

Visual-only change; no data formats affected.

## Open Questions Resolved From the Repository

- **CSS delivery**: CSS is embedded with `include_str!` and served from Axum (`/static/*.css`),
  keeping styling in Rust-managed assets without a JS/CSS build pipeline.
- **Checkbox semantics**: `CheckControl` uses a native checkbox input wrapped in a label so
  keyboard behavior and screen-reader state come from the platform.

## Tasks / Definition of Done

See `tasks.md`.
