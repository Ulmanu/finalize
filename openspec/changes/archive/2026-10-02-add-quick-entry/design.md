# Design — add-quick-entry

## Context

The keyboard registry (45/46) provides shortcuts; `POST /todos` already creates contextually.
Quick Entry composes these: a capture form that never changes the destination and a route that
applies metadata with compensation.

## Requirements Driving the Design

- `Quick Entry overlay`
- `Full capture metadata`
- `Dismiss/save semantics`

## Architecture

- The overlay is a plain `#quick-entry` container shown via `:target` CSS, so it opens from a
  keyboard-accessible anchor without script; the hydrated client can also open it. It reads
  projects and tags from context to populate destination and tag selects and seeds `return_to`
  from the current pathname.
- `POST /todos/quick` parses schedule kind, dates, destination, tag, and checklist lines up
  front; any invalid value redirects with `?error=quick` before creation. After creation, each
  metadata command runs in order; on failure the route deletes the fresh item (best-effort
  compensation) and redirects with the error, so no half-created item remains.
- Cancel is `href="#"` (clears the target); save redirects to `return_to`.

## Data and Persistence

- No schema change; all writes reuse existing commands.

## Offline and Sync

- Creation is an ordinary command; the outbox/optimistic layer treats it like any capture.

## UI and Accessibility

- The trigger exposes `aria-keyshortcuts` from the registry; the dialog is labeled
  (`role="dialog"`, `aria-label`); every field has a label; the form is usable with keyboard
  only.

## Error Handling

- Invalid inputs are rejected pre-create; post-create failures compensate by deleting the item;
  every failure path redirects with `?error=quick` and leaves the destination intact.

## Testing Strategy

- Web tests for overlay/trigger markup on any page, a full-metadata save verifying title, notes,
  schedule, deadline, project context, tag, and checklist, and two rejection paths proving no
  item is created for invalid input or a tag request without a tag service.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: a CSS `:target` panel gives keyboard/non-JS access to the overlay without handwritten
  JavaScript; the repository's no-handwritten-JS rule excludes a JS dialog controller.
