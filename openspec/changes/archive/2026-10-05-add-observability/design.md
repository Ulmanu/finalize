# Design — add-observability

## Context

The server already uses `tracing` and emits info logs; nothing correlated requests or exposed
counters.

## Requirements Driving the Design

- `Structured tracing`
- `Operational metrics`
- `Diagnostic privacy`

## Architecture

- Request-id middleware: uses the inbound `x-request-id` when present, otherwise a UUIDv7; the
  id is echoed on the response and attached to an `http_request` span containing only method and
  path, so logs can correlate without logging payloads.
- Metrics: counters are compile-time declared with descriptions; increments reject unknown names
  and carry no labels, so no titles, notes, bodies, emails, tokens, or account ids can appear.
  Sync, reminders, and inbound services increment through optional `Arc<Metrics>` builders; the
  HTTP layer counts served responses; `GET /metrics` renders the exposition.
- Privacy: the exposition test asserts no `@` or user strings appear; request spans log only
  method/path/id.

## Data and Persistence

- Counters are in-process and reset on restart (documented); no storage.

## Offline and Sync

- Sync counters are recorded server-side per batch; no payload data.

## UI and Accessibility

- Not applicable.

## Error Handling

- Unknown counter names are ignored; metrics rendering never fails.

## Testing Strategy

- App unit tests for registry integrity, increments, rendering, and unknown-name rejection;
  testkit tests for sync/reminder/inbound counters; web tests for request-id assignment and
  propagation and for privacy-safe metrics output.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: counters are label-free by design (privacy over cardinality); a metrics backend can
  scrape `/metrics` later without code changes.
