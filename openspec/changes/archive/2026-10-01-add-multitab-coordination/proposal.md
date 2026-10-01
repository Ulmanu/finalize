# Proposal

## Why

Several tabs share one outbox and one server cursor. They need one owner for sync work and a way
to tell each other when data changed — through browser-native channels, with deterministic
leadership that survives a tab closing.

## What Changes

- Add `momentum_app::tabs`: `TabId`, `TabMessage` (Committed, LeadershipClaimed,
  OutboxChanged), the `TabBroadcaster` port, and `TabCoordinator` with a lease-based
  `LeadershipState` (lowest live tab leads), `claim_or_renew`, `on_message` adoption, and
  propagation helpers.
- Highlight local pulls: a sibling `Committed` message is delivered to this tab's hint
  subscribers via `HintHub::notify`, so other tabs refresh promptly through the canonical sync
  protocol.
- Leader recovery: leases expire deterministically; the next-lowest tab claims without touching
  the shared replica store, so outbox work survives the transition.

## Capabilities

### New Capabilities

- `features/multitab-coordination`: multi-tab coordination, cross-tab state propagation, leader
  recovery.

### Modified Capabilities

(none)

## Impact

- `crates/app` (tabs module + unit tests, hint `notify` helper), `crates/testkit` (integration
  tests). No schema change.
- Non-goals: the concrete BroadcastChannel/Web Locks adapter (hydrated-client work; the port and
  deterministic logic are complete) and UI for leadership state.
