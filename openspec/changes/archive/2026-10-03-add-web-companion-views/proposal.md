# Proposal

## Why

Small surfaces (phones, companion windows, kiosk views) need a compact presentation of Today,
progress, and quick add that still uses the canonical commands and the same progress derivation
as the main app.

## What Changes

- Add `/companion`: a compact page with Today items (each carrying the canonical completion
  action), derived Today progress (`today_progress` — identical to the main app), a quick-add
  form creating Inbox items through the normal create command, and the read-only calendar
  section for today.
- Compact CSS tuned for small screens with large tap targets; the page links back to the full
  app.

## Capabilities

### New Capabilities

- `features/web-companion-views`: compact Today view, progress companion, quick add companion.

### Modified Capabilities

(none)

## Impact

- `crates/web` (companion route/component, compact CSS, tests). No schema change.
- Non-goals: a separate installable companion app, widget/OS integration, offline-specific
  behavior beyond the existing PWA shell.
