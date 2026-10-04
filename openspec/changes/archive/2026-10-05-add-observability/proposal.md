# Proposal

## Why

Operations need structured, correlated traces and measurable counters, while diagnostics must
never leak task content, email bodies, tokens, or secrets.

## What Changes

- Add `momentum_app::metrics`: a declared counter registry (HTTP, sync operations/conflicts,
  reminders delivered/failed, recurrence instances, inbound delivered/rejected, calendar
  success/failure, push sent) with a label-free in-process store and Prometheus-style text
  exposition; unknown names are ignored, so metrics output can never carry user data.
- Services accept optional metrics (`with_metrics`) and increment counters: sync
  operations/conflicts, reminder delivered/failed, inbound delivered/rejected; other counters are
  declared for their owners.
- Add `momentum-web::observability`: a request-id middleware that assigns/propagates
  `x-request-id`, traces each request with the id plus method/path only, echoes the id on the
  response, counts served responses, and exposes `GET /metrics`.
- `AppState` carries the metrics registry; the server bootstrap shares one registry across the
  HTTP layer and services.

## Capabilities

### New Capabilities

- `features/observability`: structured tracing, operational metrics, diagnostic privacy.

### Modified Capabilities

(none)

## Impact

- `crates/app` (metrics module + service wiring), `crates/web` (middleware, endpoint, state,
  tests), `crates/testkit` (counter tests). No schema change.
- Non-goals: an external metrics backend/OTLP export, per-route labels (deliberately avoided for
  privacy), distributed tracing propagation beyond the request id.
