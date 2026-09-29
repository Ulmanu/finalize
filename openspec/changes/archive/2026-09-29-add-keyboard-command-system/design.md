# Design — add-keyboard-command-system

## Context

Rows already answer to form controls and the layout handles Escape; nothing centralized the
bindings, and reserved shortcuts were undocumented.

## Requirements Driving the Design

- `Central shortcut registry`
- `Command coverage`
- `Conflict handling`

## Architecture

- A static `REGISTRY` of `CommandDefinition`s holds `(id, title, category, keys, reserved_by,
  alternative, surfaces)`. Components never hard-code bindings: `keys_for` returns the effective
  binding (the web alternative when the primary is reserved), which row controls use for
  `aria-keyshortcuts`, and the Settings page renders every entry with its conflict note.
- Coverage is enforced by tests: unique non-empty ids and primary bindings, every category
  present, surfaces filter correctly, and every reserved binding documents an alternative.
- Conflict mappings: Cmd+K → `/` (address bar), Cmd+Backspace → Shift+Delete (macOS text
  editing), Cmd+E → F2 (browser search bar), Cmd+A → Shift+A (browser select-all).

## Data and Persistence

- None; the registry is compile-time data.

## Offline and Sync

- Pure data shared by server rendering and the hydrated client.

## UI and Accessibility

- Settings lists commands grouped by category with `<kbd>` keys and reserved badges; buttons
  expose `aria-keyshortcuts` populated from `keys_for`, so assistive technology sees the same
  bindings as the reference.

## Error Handling

- Unknown identifiers return `None`; `keys_for` falls back gracefully in markup.

## Testing Strategy

- App unit tests for uniqueness, coverage, surface filtering, and conflict alternatives; web tests
  for the Settings reference and the `aria-keyshortcuts` hints on row controls.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: reserved bindings keep their conventional primary key for documentation while the
  effective binding is always the safe alternative; the hydrated client should dispatch on
  `keys_for`.
