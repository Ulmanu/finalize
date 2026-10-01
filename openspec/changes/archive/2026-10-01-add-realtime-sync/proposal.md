# Proposal

## Why

Other sessions and devices should learn that newer server data exists without a second mutation
protocol; hints must coalesce so busy accounts cannot stampede clients into pull loops, and
reconnects must stay correct because the cursor protocol is authoritative.

## What Changes

- Add `momentum_app::realtime`: `InvalidationHint`, the `HintSink` connection contract, and a
  coalescing `HintHub` (account-scoped subscriptions, one pending hint per account until
  delivered, newest sequence retained during bursts, no accumulation without subscribers).
- `SyncService::with_hints` publishes a hint after each successful pushed operation; the stream
  never carries mutations, so clients still pull through `/sync` with their cursor.
- Add `GET /sync/events` SSE: subscribes the connection, emits an immediate connect hint (so a
  reconnected client pulls), streams coalesced invalidation events with keepalive, and
  unsubscribes on disconnect; `AppState::with_hints` wires the hub and the server bootstrap
  constructs it.

## Capabilities

### New Capabilities

- `features/realtime-sync`: realtime invalidation hints, reconnect behavior,
  backpressure/coalescing.

### Modified Capabilities

(none)

## Impact

- `crates/app` (hub + unit tests), `crates/web` (SSE route, state wiring, tests); workspace
  dependencies gain the tokio `sync` feature and `tokio-stream`. No schema change.
- Non-goals: WebSocket transport (SSE is the initial adapter and the hint contract is
  transport-agnostic), client-side pull scheduling loops.
