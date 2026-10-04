# Design — add-animation-system

## Context

Design tokens already define `--motion-fast/base/slow` and easings; scattered transitions existed
without a registry. Reduced-motion rules from 64/66 already cover global suppression.

## Requirements Driving the Design

- `Motion primitives`
- `State-preserving animation`
- `Reduced motion`

## Architecture

- `MotionPrimitive` entries declare id, class, duration token, and essential flag; tests enforce
  unique ids/classes, token prefixes, and that the reduced-motion suppression set equals all
  nonessential primitives.
- CSS keyframes/transitions live once in app.css and are applied through the declared classes;
  the markup carries insert/drag/editor/popover/nav classes today, and the client applies the
  removal class while a row leaves.
- Animations are `both`-fill CSS effects with no JS gating, so interrupted animations cannot
  affect domain ordering or block form submission; reduced motion disables every nonessential
  primitive through both the media query and the explicit override.

## Data and Persistence

- None; motion is presentation-only.

## Offline and Sync

- Not applicable.

## UI and Accessibility

- Essential focus visibility is unaffected by motion suppression; all primitives are nonessential
  today and therefore removed under reduced motion.

## Error Handling

- Registry lookups return `None` for unknown ids; CSS classes without matching rules are inert.

## Testing Strategy

- App unit tests for registry integrity and suppression coverage; web tests assert the markup
  carries every primitive class (except client-only removal) and the stylesheet declares all
  rules plus reduced-motion suppression for both the media query and the explicit override.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: removal motion is declared centrally but applied by the hydrated client (server
  markup has no leaving rows); the registry remains the single source for its class and token.
