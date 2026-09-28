# Proposal

## Why

Navigation by typing is faster than traversing project trees. Quick Find needs deterministic,
transparent ranking over every navigable surface, and recency must boost without hiding matches.

## What Changes

- Add `SearchService` ranking built-in and special lists, settings, areas, projects, headings
  (from project summaries), tags, and to-do titles with confidence tiers: exact, prefix, word
  start, substring; ties break by recency, then kind, then title, and every match is returned.
- Add a `RecentsStore` (memory implementation) recording destinations via
  `GET /search/go?url=...`, which validates internal URLs, records the visit, and redirects.
- Add the Quick Find page (`GET /search?q=`) with a labeled form, ranked result links carrying
  kind/score data attributes, and a Recent badge; add a Quick Find form to the application topbar.

## Capabilities

### New Capabilities

- `features/quick-find`: type-to-open, destination search, recent and exact navigation.

### Modified Capabilities

(none)

## Impact

- `crates/app` (search module + tests), `crates/web` (search page, go route, topbar form, recents
  context, tests). No schema change.
- Non-goals: the type-to-open keyboard gesture itself (hydrated Rust/WASM client attaches to the
  same page and input, recorded as the established hydration gap), fuzzy typo tolerance.
