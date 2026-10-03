# Design — add-web-companion-views

## Context

Today, progress, and add already exist; the companion is a presentation layer over the same
reads and commands, not a new domain surface.

## Requirements Driving the Design

- `Compact Today view`
- `Progress companion`
- `Quick add companion`

## Architecture

- `CompanionView` reuses `TodoService::list(Today)` for rows, `TodoService::today_progress` for
  the progress line (rendered with `data-completed`/`data-total` for tests and assistive
  consumers), and the standard create/complete form posts, so actions and calculations cannot
  drift from the main app.
- The layout is a single narrow column with large controls; the calendar section is read-only as
  everywhere else.

## Data and Persistence

- None; all reads/writes go through existing services and commands.

## Offline and Sync

- The companion is covered by the PWA shell and the replica/outbox like any other page.

## UI and Accessibility

- Large tap targets, labeled inputs and buttons, compact typography; the page remains fully
  usable with keyboard only.

## Error Handling

- Empty state explains there is nothing scheduled; command failures use the existing redirect
  error behavior.

## Testing Strategy

- Web tests for compact markup, progress values matching the derived calculation after a
  completion, canonical completion posting, quick-add creating an Inbox item, and the empty
  state.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: completion redirects to the item page (existing lifecycle behavior) rather than the
  companion; the compact surface preserves the canonical action rather than inventing a
  companion-only response.
