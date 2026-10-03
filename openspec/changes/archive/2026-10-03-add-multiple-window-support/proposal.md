# Proposal

## Why

Users want a list, project, or area in its own browser window without the full navigation chrome;
those windows must share synchronized state and issue commands safely.

## What Changes

- Popout destinations: any page opened with `?popout=1` renders without the topbar, sidebar,
  drawer scrim, bottom navigation, and footer, leaving a minimal exit link ("Open full app");
  the full layout offers an "Open in window" anchor (`target="_blank"`, noopener) that appends the
  parameter to the current path.
- Shared state: popout windows are ordinary tabs — they read the same server state and coordinate
  through the existing hint/replica/multi-tab mechanisms (52); no separate state is introduced.
- Window-safe commands: forms and routes are stateless and per-request; commands issued from a
  popout behave identically and are visible to every window on its next read.

## Capabilities

### New Capabilities

- `features/multiple-window-support`: popout destinations, shared state, window-safe commands.

### Modified Capabilities

(none)

## Impact

- `crates/web` (layout popout mode, CSS, tests). No schema change.
- Non-goals: `window.open` scripting (links with `target="_blank"` are native), per-window
  preferences, detached native windows.
