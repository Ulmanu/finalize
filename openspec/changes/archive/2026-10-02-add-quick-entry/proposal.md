# Proposal

## Why

Capturing a thought should never require navigating away from the current list. Quick Entry is a
keyboard-invoked overlay with full capture metadata and deterministic save/cancel semantics that
never leave half-created items.

## What Changes

- Add the Quick Entry overlay to the authenticated layout: a `#quick-entry` target-driven panel
  (CSS-only open/close, no handwritten JS) with a topbar trigger carrying the registry shortcut
  (`quick_entry` added to the keyboard registry as `q`), title, notes, when picker
  (none/today/evening/tomorrow/someday/anytime/on-date), schedule date, deadline, destination
  (Inbox or project), tag, and a checklist textarea.
- Add `POST /todos/quick`: validates every field before creating anything, creates the item in the
  chosen context, applies schedule/deadline/tag/checklist, and compensates on any post-create
  failure by deleting the fresh item before redirecting with `?error=quick`.
- Cancel is an anchor that clears the `#quick-entry` target; save redirects back to `return_to`
  (the current destination).

## Capabilities

### New Capabilities

- `features/quick-entry`: overlay, full capture metadata, dismiss/save semantics.

### Modified Capabilities

(none)

## Impact

- `crates/app` (registry command), `crates/web` (overlay component, layout trigger, quick route,
  CSS, tests). No schema change.
- Non-goals: the hydrated client's shortcut handling (registry + target overlay are ready for it),
  attachment/location metadata beyond the supported set.
