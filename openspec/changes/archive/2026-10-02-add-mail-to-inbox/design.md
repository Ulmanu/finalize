# Design — add-mail-to-inbox

## Context

The capture sanitizers (54) provide untrusted-text handling. Inbound needs durable aliases and a
durable dedupe ledger, so migration 0006 adds both tables with a unique `(account, message_id)`
primary key.

## Requirements Driving the Design

- `Private inbound address`
- `Mail-to-task mapping`
- `Abuse and duplicate protection`

## Architecture

- Aliases are 128-bit random tokens (`Uuid::new_v4` as 32 hex characters); `rotate` replaces the
  account's alias atomically (transaction in PG), `disable` flips the enabled flag, and ingress
  resolves `(account, enabled)` from the token so disabled aliases are distinguished from unknown
  ones.
- `map_mail` strips controls, drops quoted reply lines, collapses whitespace, applies the "Email"
  title fallback, and bounds lengths; notes reuse the capture sanitizer.
- `ingest` checks size first, then token/enabled, then the persisted ledger (duplicates are free
  of rate charges), then the sliding hourly window, then creates the Inbox to-do and records the
  message id. Webhook responses are deterministic: 200 delivered/duplicate, 404 unknown, 403
  disabled, 429 rate limited, 413 too large.

## Data and Persistence

- Migration 0006: `inbound_aliases(token pk, account_id, enabled, created_at)` with an account
  index; `inbound_messages(account_id, message_id, received_at, pk(account_id, message_id))`.

## Offline and Sync

- Inbound creates are ordinary commands; the webhook is server-side only.

## UI and Accessibility

- Settings renders the address as escaped code text with labeled rotate/disable buttons; the
  secret is deployment configuration and never rendered.

## Error Handling

- Unconfigured service or secret answers 503; a mismatched secret answers 401 without touching
  data; storage failures answer 500.

## Testing Strategy

- App unit tests for mapping; testkit tests for alias lifecycle (rotation invalidates the old
  token, disable blocks), mapping and dedupe with persisted ledger, size and rate limits with
  window recovery; web tests for webhook delivery/duplicate/disabled, secret enforcement, and the
  settings address plus rotate/disable flows.

## Rollout / Migration

- Migration 0006 is additive and reversible.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the webhook trusts only the shared secret header and the unguessable alias token; no
  mail-provider signature scheme is assumed because the provider is deployment-specific.
