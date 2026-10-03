# Design — add-multiple-window-support

## Context

The shell already renders all destinations inside one layout. Popouts are the same routes with
chrome omitted, so no page needs to know it is windowed.

## Requirements Driving the Design

- `Popout destinations`
- `Shared state`
- `Window-safe commands`

## Architecture

- `AppLayout` reads `?popout=1` from the query map; when set it renders a minimal exit header and
  omits the topbar, sidebar, scrim, bottom navigation, and footer (real omission, not CSS
  hiding), and the root carries `data-window="popout"` for tests. The full layout adds an
  "Open in window" anchor using the current pathname.
- Popouts are tabs: server reads are authoritative and the multi-tab coordinator (52) plus the
  realtime hint hub keep windows synchronized; commands are stateless HTTP forms.

## Data and Persistence

- None; no per-window state exists.

## Offline and Sync

- Popouts use the same PWA shell, replica, and sync paths as any tab.

## UI and Accessibility

- The exit link is keyboard reachable and labeled; chrome omission improves small-window focus;
  no window-specific controls are introduced.

## Error Handling

- Invalid destinations behave exactly as in the full layout (existing redirects).

## Testing Strategy

- Web tests: popout rendering omits chrome and offers the exit link while showing shared data;
  the full layout carries `data-window="full"` and the open-in-window link; a project popout
  renders; a command issued from a popout is reflected on the next read in every window.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: popout detection uses the query map rather than a separate path, so every destination
  (lists, projects, areas, search) supports windows with one change.
