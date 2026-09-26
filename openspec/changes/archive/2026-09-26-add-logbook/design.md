# Design — add-logbook

## Context

`logged_at` exists on entities, `preferences` exists for account settings, and `TodoScope::Logbook`
previously included every terminal item with no policy control. This change formalizes the
policy and history semantics.

## Requirements Driving the Design

- `Logbook membership`
- `History timestamps`
- `Logging policy`

## Architecture

- `LoggingPolicy { Automatic, Manual }` (domain) with `as_str`/`parse`; stored per account in
  `preferences` under key `logbook.policy` through a `LoggingPolicyStore` port (memory adapter
  for tests, PostgreSQL adapter for the server).
- `TodoScope::Logbook { logged_only: bool }` lets the logbook service request policy-shaped
  membership without teaching other scopes about preferences.
- `LogTodo`/`LogProject` commands set `logged_at` exactly once for terminal items and reject
  open items; completion state is untouched.
- `LogbookAgenda` groups entries by history date descending, where the history instant is
  `logged_at → completed_at → canceled_at → created_at`; under the manual policy unlogged
  terminal entries surface as `pending` instead of disappearing.

## Data and Persistence

- No schema change; preference rows live in the existing `preferences` table (JSON text value).

## Offline and Sync

- Logging is an ordinary idempotent command; the policy is an account preference not carried in
  the entity change log (preferences sync with their own capability later).

## UI and Accessibility

- Policy form is a labeled select with a current-policy readout; pending entries expose a labeled
  Log button; day groups carry `data-date` hooks and human-readable headings.

## Error Handling

- Logging an open item is a validation error; unknown items are not found; storage failures
  surface as empty agendas.

## Testing Strategy

- Testkit tests for automatic membership and timestamps, manual pending/log transitions with
  validation, and policy switching preserving completion.
- Web tests for automatic date-grouped history and the manual pending → logged flow.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: policy lives in `preferences` (already schema-backed) rather than a new table, and
  manual logging keeps unlogged terminal items visible as `pending` so no completed work becomes
  unreachable.
