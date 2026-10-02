# Proposal

## Why

Automation (shortcuts, scripts, bookmarklets) needs documented URLs for the core actions, with
strict typed parsing and account-scoped references, while show/search URLs must stay stable and
bookmarkable.

## What Changes

- Add the command URL surface: `GET/POST /add` (full capture via the shared Quick Entry logic),
  `GET /add-project` (title/area/notes), `GET /update` (title/notes/when/date/deadline),
  `GET /show` (todo/project/area/list), and the existing `GET /search` as the documented search
  URL.
- Strict parsing: ids must parse as entity ids, dates must parse, schedule kinds and list names
  come from fixed allow-lists, and `show` accepts exactly one target; entity references resolve
  through account-scoped services so foreign or missing ids redirect to not-found.
- Shareable navigation: `/show` and `/search` never mutate and redirect to canonical stable
  paths.

## Capabilities

### New Capabilities

- `features/command-urls`: command URL surface, safe parameter parsing, shareable navigation
  URLs.

### Modified Capabilities

(none)

## Impact

- `crates/web` (command routes, Quick Entry handler extraction, tests). No schema change.
- Non-goals: URL signing/one-time tokens (sessions authorize), bulk automation APIs beyond the
  documented actions.
