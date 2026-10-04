# Design — add-accessibility

## Context

The server-rendered UI is already form/link based with aria labels added incrementally; this
change closes the known gaps and adds an automated audit so regressions are caught.

## Requirements Driving the Design

- `Keyboard-only operation`
- `Semantic accessibility`
- `Motion/target/contrast requirements`

## Architecture

- The layout gains a skip link as the first focusable element and `id="app-main"` on the main
  landmark; `:focus-visible` outlines use the accent token so keyboard focus is always visible.
- Progress semantics use the native `<progress>` on Today and an explicit progressbar role on the
  companion; the Quick Entry overlay declares `aria-modal`; navigation exposes landmark and
  current-destination semantics.
- Coarse pointers get 44px minimum targets; reduced motion and high contrast rules from earlier
  changes remain in force.
- The audit test parses core pages and asserts: skip link + landmark, labeled inputs (hidden
  exempt), labeled/non-empty buttons, progress/dialog semantics, nav semantics, and the CSS
  contract strings.

## Data and Persistence

- None.

## Offline and Sync

- Not applicable.

## UI and Accessibility

- This change is the accessibility work itself; no new user-facing controls beyond the skip link.

## Error Handling

- Audit failures are test failures, keeping the contract enforced.

## Testing Strategy

- Web audit tests over /inbox, /today, /companion, and /settings plus targeted semantics and CSS
  assertions; the audit helper fails with the offending markup.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the Quick Entry overlay stays a CSS `:target` panel (no scripted focus trap);
  `aria-modal` documents intent and the skip link preserves keyboard reachability, with the
  hydrated client able to add trapping later without markup changes.
