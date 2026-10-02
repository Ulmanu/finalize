# Design — add-browser-capture

## Context

Browser share targets navigate to a URL with shared parameters; bookmarklets and extensions can
do the same. All capture logic must stay in the application layer and treat page content as
untrusted.

## Requirements Driving the Design

- `Web share capture`
- `Extension/bookmark adapter boundary`
- `Sanitized external text`

## Architecture

- `build_capture` sanitizes title, URL, and text: URLs must be http/https, controls are stripped,
  lengths are bounded, whitespace is collapsed, and empty titles fall back to "Captured page";
  notes keep the URL first for provenance.
- `/capture` handles GET (share target/bookmarklet) and POST (forms/extensions) identically,
  requiring a session and creating the item in the Inbox; `return_to` lets POST callers land back
  where they came from.
- The manifest declares `share_target` with `action: "/capture"`, `method: GET`, and
  `params {title, text, url}`; the shell links the manifest.

## Data and Persistence

- No schema change; capture creates an ordinary Inbox to-do.

## Offline and Sync

- Capture is a normal command; offline capture flows through the replica/outbox like any create.

## UI and Accessibility

- No new UI: the OS share sheet drives the endpoint. Text is rendered through the existing
  escaped notes rendering; no HTML from captures is ever interpreted.

## Error Handling

- Missing session redirects to login; unavailable services redirect to login; write failures
  redirect with `?error=capture`. Invalid URLs are dropped rather than rejected so the capture
  still succeeds.

## Testing Strategy

- App unit tests for URL/text sanitization and draft composition; web tests for a full share
  capture, malicious-URL dropping with control stripping, the manifest's share target and content
  type, and unauthenticated rejection.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: PWA `share_target` uses GET so no script is involved; POST support is kept for
  extensions and bookmarklets that can submit forms.
