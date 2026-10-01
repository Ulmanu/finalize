# Design — add-multitab-coordination

## Context

Outboxes and cursors are shared through the replica store (49); hints flow through the hub (51).
Browser tabs coordinate via BroadcastChannel/Web Locks, which cannot run in the SSR harness, so
the deterministic logic is provider-neutral and the channel is a port.

## Requirements Driving the Design

- `Multi-tab coordination`
- `Cross-tab state propagation`
- `Leader recovery`

## Architecture

- `LeadershipState` tracks per-tab lease expiries; the leader is the lowest-id tab with a live
  lease. `claim_or_renew` prunes expired leases, claims when none is live (or when this tab is
  already lowest), extends the lease, and broadcasts the claim. Deterministic min-id election
  means two tabs converge without a central authority.
- `TabCoordinator::on_message` adopts sibling claims and turns `Committed` messages into local
  hints via `HintHub::notify` (bypassing local coalescing bookkeeping so a remote commit always
  triggers a pull for this tab). `propagate_committed`/`propagate_outbox` broadcast local events.
- Leader recovery: no leadership state is stored outside the tab; an expired lease simply lets the
  next tab claim, and the outbox remains in the shared store untouched.

## Data and Persistence

- No storage: leases are in-memory views of broadcast claims; outbox durability is the replica
  store's responsibility.

## Offline and Sync

- A non-leader tab still renders from the shared replica; when it becomes leader it resumes
  outbox replay through the existing coordinator.

## UI and Accessibility

- No visible UI; tabs refresh through the existing hint/pull pipeline.

## Error Handling

- Messages from unknown tabs are harmless; expired leases are pruned on every claim; a tab
  without leadership simply defers.

## Testing Strategy

- App unit tests for election, claim adoption, takeover after expiry, and hint delivery; testkit
  tests for leader recovery preserving outbox work and for sibling commit propagation.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: BroadcastChannel ports messages and the lease lives in tab memory; Web Locks can
  replace the lease later without changing the coordinator's observable behavior.
