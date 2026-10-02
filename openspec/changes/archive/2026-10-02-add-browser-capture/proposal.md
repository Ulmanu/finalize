# Proposal

## Why

Capturing a page or a selection from the browser must land in the inbox without leaving the page
or trusting third-party content, and optional extensions/bookmarklets should use one documented
command URL rather than reimplementing capture logic.

## What Changes

- Add `momentum_app::capture`: `sanitize_url` (http/https only, control-stripped, capped),
  `sanitize_text` (controls removed, whitespace collapsed, capped), and `build_capture` (title
  fallback plus URL-first notes).
- Add `GET/POST /capture` accepting `title`, `url`, `text`, and optional `return_to`; it
  authenticates, sanitizes, creates the to-do in the Inbox, and redirects to the destination —
  this is the documented command URL for browser adapters.
- Add a PWA manifest served at `/manifest.webmanifest` with a `share_target` action of
  `/capture` (GET) and link it from the shell, enabling native share capture without script.

## Capabilities

### New Capabilities

- `features/browser-capture`: web share capture, extension/bookmark adapter boundary, sanitized
  external text.

### Modified Capabilities

(none)

## Impact

- `crates/app` (capture module + unit tests), `crates/web` (capture routes, manifest, shell link,
  tests). No schema change.
- Non-goals: shipping a browser extension (the documented command URL is the boundary), OAuth or
  native share APIs beyond the manifest share target.
