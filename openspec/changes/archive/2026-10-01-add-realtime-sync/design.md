# Design — add-realtime-sync

## Context

The cursor-based sync protocol (50) is authoritative. Realtime only needs to say "there is newer
data for your account"; no command is ever delivered over the hint channel.

## Requirements Driving the Design

- `Realtime invalidation hints`
- `Reconnect behavior`
- `Backpressure/coalescing`

## Architecture

- `HintHub` holds account-scoped subscriptions and one pending hint per account. `publish` sends
  only when a subscriber exists and no hint is pending; while pending, the sequence is merged
  upward, so bursts collapse into a single latest hint. The SSE stream calls `delivered` after
  writing a hint (when it next publishes) — in practice the transport writes the hint and the
  next push emits again only once the previous one left the hub.
- `GET /sync/events` subscribes, immediately emits a connect hint, streams
  `event: invalidation` frames with keepalive, and unsubscribes on drop via a RAII guard.
- `SyncService::with_hints` publishes after acknowledged pushes, so any session pushing through
  `/sync` wakes others; the clients then pull with their own cursors.

## Data and Persistence

- No storage: hints are ephemeral; correctness lives in the change log and cursors.

## Offline and Sync

- A disconnected client misses nothing: reconnect emits a connect hint and the cursor pull
  reconciles.

## UI and Accessibility

- No visible UI; the hydrated client uses EventSource (or any transport implementing the hint
  contract) and schedules one pull per hint.

## Error Handling

- Publishing without subscribers is a no-op; stream send failures drop the hint; a missing hub
  answers 503; missing sessions answer 401.

## Testing Strategy

- App unit tests for coalescing until delivered, account scoping, unsubscribe, and no-subscriber
  behavior; testkit sync test asserts a pushed operation emits exactly one coalesced hint; web
  tests assert SSE headers plus the initial connect frame and the absent-hub 503.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: hinted delivery uses a bounded-in-practice coalescing rule rather than an explicit
  backlog because hints are idempotent "pull now" signals, not data.
