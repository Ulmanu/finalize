# Proposal

## Why

Shortcuts scattered across components drift and collide. A single registry with availability
predicates, coverage across the required operations, and documented conflict mappings is the
foundation the hydrated client and every component share.

## What Changes

- Add `momentum_app::keyboard`: `CommandDefinition` entries declaring id, title, category,
  primary keys, surfaces, and conflict metadata, with lookup helpers (`command`, `keys_for`,
  `commands_for`, `conflicts`).
- Cover creation, completion/cancellation/deletion, scheduling and deadlines, navigation and
  search, movement, tags, selection, and reordering (24 commands across seven categories).
- Map browser/OS-reserved bindings to documented safe web equivalents (Cmd+K → `/`,
  Cmd+Backspace → Shift+Delete, Cmd+E → F2, Cmd+A → Shift+A) and render the whole registry on the
  Settings page; row controls carry `aria-keyshortcuts` resolved through `keys_for`.

## Capabilities

### New Capabilities

- `features/keyboard-command-system`: central registry, command coverage, conflict handling.

### Modified Capabilities

(none)

## Impact

- `crates/app` (registry + unit tests), `crates/web` (Settings shortcut reference, aria hints,
  tests). No schema change.
- Non-goals: the actual key-event dispatch loop (hydrated Rust/WASM client consumes this
  registry), user-customizable bindings.
