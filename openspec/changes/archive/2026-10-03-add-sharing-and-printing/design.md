# Design — add-sharing-and-printing

## Context

Read models already exist; sharing/printing must derive their bytes from those models only, so
outputs are deterministic and independent of the interactive view tree.

## Requirements Driving the Design

- `Share render models`
- `Print layouts`
- `Content safety`

## Architecture

- Markdown models escape markdown control characters and emit stable field order (title, notes,
  when/deadline/status, checklist); print documents escape HTML and inline a small stylesheet so
  they render standalone without the app shell.
- Routes read through the same services and a fixed list allow-list; print documents are complete
  HTML documents (`<!DOCTYPE html>`) with escaped titles and content, and open scopes exclude
  trashed data. Sessions are required.
- `@media print` hides navigation, actions, editors, overlays, and footers for interactive pages.

## Data and Persistence

- None; read-only rendering.

## Offline and Sync

- Pure reads; no state changes.

## UI and Accessibility

- Print documents are semantic HTML with readable defaults and print margins; share outputs are
  plain text/markdown safe for any consumer.

## Error Handling

- Unknown lists/entities → 404; missing session → login redirect; storage failures → 500.

## Testing Strategy

- App unit tests for determinism and escaping (markdown controls, HTML entities, script tags);
  web tests for share markdown determinism, list sharing, print HTML escaping and titles, 404s,
  and session enforcement.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: share/print models live in the app crate (pure, testable) and the web layer only maps
  routes to them; anonymous sharing is out of scope so visibility rules stay account-scoped.
