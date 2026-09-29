# Design — add-type-travel

## Context

Prior changes documented type-to-open as a hydration gap. The keyboard registry now provides the
central command data; this change implements the gesture as real Rust in the hydrate build.

## Requirements Driving the Design

- `Type travel`
- `Editor exemption`
- `Escape/restore focus`

## Architecture

- `keyboard::type_travel_seed(key, ctrl, meta, alt, editing)` is pure and unit-tested: editable
  focus, modifier chords, multi-character keys, control keys, and whitespace all return `None`;
  printable single characters return the seed.
- The layout installs a document-level keydown listener inside `#[cfg(feature = "hydrate")]`
  using a leaked `Closure` (the layout lives for the app session), inspects
  `document.active_element()` for `input`/`textarea`/`select`/`contenteditable`, and navigates to
  `/search?q=<seed>`; the SSR build compiles a `let _ = navigate;` rather than the listener.
- Escape on the Quick Find input calls `window.history().back()` in hydrate builds, restoring the
  previous navigation target.

## Data and Persistence

- None; the gesture is pure client navigation.

## Offline and Sync

- No data effects; navigation only.

## UI and Accessibility

- The gesture never fires while an editor is focused, so typing in any form is safe; Escape
  restores the prior page; assistive technology sees the standard search form.

## Error Handling

- History/back failures are ignored; navigation falls back to normal routing.

## Testing Strategy

- App unit tests cover the seed decision matrix (printable, accented, editing, modifiers,
  multi-char, control, whitespace, empty). The hydrate listener and history restore are compiled
  and validated by the WASM build; SSR behavior is unchanged.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the listener is attached to the document (not the layout div) because key events with
  body focus do not bubble through the layout; hydrate-only cfg keeps the SSR build dependency-free.
