# Design — add-notes-editor

## Context

Notes are already stored as plain text on to-dos and projects and edited through native
textareas, which supply multiline editing, selection, clipboard, and undo/redo on every
target browser. This change adds the parts that must be deterministic in Rust: link policy and
find/replace, plus the UI that exposes them.

## Requirements Driving the Design

- `Rich plain-text notes editor`
- `Find and replace`
- `Safe link handling`

## Architecture

- `momentum_domain::notes` (pure, unit-tested): `NoteLink`, `find_links`, `is_safe_url`,
  `find_matches`, `next_match`, `previous_match`, `replace_all`.
- Safe URL policy: only `http`, `https`, and `mailto` schemes are navigable; `javascript:`,
  `data:`, `vbscript:`, `file:`, control characters, and relative/empty targets are rejected.
- UI: a shared notes panel renders a link list, a find form (`?find=` plus `?from=` traversal
  links), and a replace-all form. Replacement runs on the server through the existing
  `SetTodoNotes`/`SetProjectNotes` commands, so persistence stays authoritative and idempotent.
- Editing itself stays a plain textarea: selection, clipboard, and undo/redo are native
  behaviors, and no handwritten JavaScript is introduced.

## Data and Persistence

- No schema or migration changes; notes remain the existing plain-text column.
- Replacements are optimistic-version-guarded like any other command.

## Offline and Sync

- Replacements are ordinary commands with change-log records; replay is idempotent because the
  command carries the resulting notes text.

## UI and Accessibility

- All controls are labeled buttons/inputs; links use real anchors with `rel="noopener
  noreferrer"`; blocked schemes are rendered as inert text with an explanatory label so screen
  readers do not announce them as links.

## Error Handling

- Empty search terms are rejected as validation; missing entities produce not-found redirects.

## Testing Strategy

- Domain unit tests for link detection, scheme safety, traversal, and replacement.
- Web tests for link rendering, blocked schemes, find counts, traversal, and replacement
  persistence.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: find/replace stays server-driven over existing note commands rather than inventing
  feature-local storage, keeping one authoritative notes path.
