# Proposal

## Why

Capturing by forwarding mail needs an unguessable per-account address that can be rotated or
disabled, deterministic subject/body mapping, and ingress protections against abuse and
duplicates.

## What Changes

- Add `momentum_app::inbound`: `InboundAlias` (high-entropy token, `address()`), the `AliasStore`
  and `MessageLedger` ports, `TokenSource` (CSPRNG UUIDv4 tokens rendered as 32 hex characters),
  `InboundLimits`, deterministic `map_mail` (subject→title with fallback, quoted reply lines
  dropped, whitespace collapsed, bounded), and `InboundService` with `ensure_alias`/`rotate`/
  `disable` and `ingest` enforcing size limits, enabled-state, a persisted message-id ledger, and
  a sliding-window hourly rate limit.
- Migration 0006 adds `inbound_aliases` and `inbound_messages`; `PgInboundStore` implements both
  ports.
- Web: `POST /inbound/mail` (provider webhook guarded by `x-inbound-secret`) maps outcomes to
  deterministic HTTP statuses; Settings shows the private address with rotate/disable actions;
  `AppState::with_inbound` wires the service and secret.

## Capabilities

### New Capabilities

- `features/mail-to-inbox`: private inbound address, mail-to-task mapping, abuse and duplicate
  protection.

### Modified Capabilities

(none)

## Impact

- `crates/app` (inbound module + unit tests), `crates/infra` (migration + PG store),
  `crates/web` (webhook, settings surface, tests), `crates/testkit` (memory stores + tests).
  Workspace uuid gains the `v4` feature.
- Non-goals: running an actual mail transfer agent (the provider webhook is the boundary),
  HTML mail rendering (plain text only), per-sender allow lists.
