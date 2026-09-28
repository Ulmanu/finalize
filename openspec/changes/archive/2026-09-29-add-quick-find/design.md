# Design — add-quick-find

## Context

Every navigable surface already has a read path: lists are static, projects carry headings, areas
and tags list from their services, and to-dos list as `AllOpen`. Recents had no storage.

## Requirements Driving the Design

- `Type-to-open Quick Find`
- `Destination search`
- `Recent and exact navigation`

## Architecture

- `SearchService::search` builds candidate `SearchHit`s from static destinations plus the read
  services, scores titles with four confidence tiers, marks recent URLs, and sorts by
  `(score, recency, kind, title, url)`; capped at 60 hits. Empty queries return nothing.
- `RecentsStore` records internal destinations from `/search/go` (rejecting external and
  protocol-relative URLs) with a bounded, deduplicated, newest-first memory implementation.
- The search page renders links through `/search/go` so opening a destination is what records
  recency; the topbar carries the form on every page.

## Data and Persistence

- No schema: recents are process-local navigation state, mirroring the "projection-only UI state"
  rule; a durable recents store can replace the trait implementation later.

## Offline and Sync

- Search is read-only and needs no sync; recording a visit is fire-and-forget.

## UI and Accessibility

- Labeled input with autofocus on the search page, topbar form everywhere, result rows with kind
  and score data attributes, Recent badges; the hydrated client adds the type-to-open gesture.

## Error Handling

- Read failures yield empty results rather than errors on the page; a failing recents store
  degrades to no boost; invalid go targets redirect to the search page.

## Testing Strategy

- App unit test for tier scoring; web tests for tier ordering with mixed entity kinds, static
  destination matches, empty-query behavior, topbar presence, recents recording, invalid-URL
  rejection, and the recency tie-break.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: heading search reuses `ProjectSummary.headings` (archived excluded) instead of a new
  read-store method; recents are in-process because no durable preference store exists yet.
