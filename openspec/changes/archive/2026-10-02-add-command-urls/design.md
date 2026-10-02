# Design — add-command-urls

## Context

`/capture` (54) established the automation boundary for browser capture; `/todos/quick` already
implements full capture. Command URLs extend that boundary to add/update/show/search.

## Requirements Driving the Design

- `Command URL surface`
- `Safe parameter parsing`
- `Shareable navigation URLs`

## Architecture

- `/add` reuses the Quick Entry handler (`apply_quick_entry`) via `Query` (GET) and `Form`
  (POST), so validation/compensation semantics are identical to the UI.
- `/add-project` parses an optional area id, creates the project, applies notes, and compensates
  by deleting the fresh project if notes fail. `/update` applies only provided fields in a fixed
  order (title, notes, schedule, deadline) with parse-before-write and error redirects.
- `/show` requires exactly one of `todo|project|area|list`; entities are verified through
  account-scoped services (missing/foreign → not-found redirect), lists map through a fixed
  allow-list, and both show and search are non-mutating and stable.

## Data and Persistence

- No schema change; all mutations go through existing commands.

## Offline and Sync

- Command URLs are server-side conveniences; offline automation flows through the replica/outbox
  like any other command.

## UI and Accessibility

- No new UI; the URLs are documented and bookmarkable.

## Error Handling

- Unsupported values, malformed ids, and conflicting parameters redirect with deterministic
  `?error=` markers; write failures compensate where a fresh entity was created.

## Testing Strategy

- Web tests for add with metadata, add-project with notes, update applying typed fields and
  rejecting unsupported values without partial writes, show redirects for lists/entities plus
  rejection of malformed and unsupported references, and search remaining a stable page.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: GET command URLs are intentionally non-idempotent for automation parity with
  `/capture`; POST variants exist for add and Quick Entry where forms are available.
