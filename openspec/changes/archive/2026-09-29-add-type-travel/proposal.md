# Proposal

## Why

The registry (45) declared Quick Find's binding, but printable typing outside editors should
itself transition immediately into search — the type-travel gesture — with editors exempt and a
sensible way back.

## What Changes

- Add `type_travel_seed` to the keyboard registry: it returns the seed character only for
  printable single-character keys pressed without Ctrl/Meta/Alt while focus is outside editable
  controls, so typing inside notes, titles, checklists, and date inputs never triggers it.
- Wire the gesture in the authenticated layout as hydrate-only Rust: a document-level keydown
  listener checks the active element, resolves the seed, and navigates to the seeded Quick Find
  page; the SSR build omits the listener.
- Escape restoration: the Quick Find input's Escape handler steps back in history (hydrate-only),
  returning the user to the prior destination.

## Capabilities

### New Capabilities

- `features/type-travel`: type travel, editor exemption, escape/restore focus.

### Modified Capabilities

(none)

## Impact

- `crates/app` (seed decision + unit tests), `crates/web` (layout listener, Quick Find escape,
  shared percent encoding). No schema change; the SSR response is unchanged.
- Non-goals: customizing the gesture, chording; non-Latin IME composition handling beyond the
  printable-character rule.
