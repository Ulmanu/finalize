# Design — add-production-deployment

## Context

The server already exposed `/healthz`/`/readyz` with database health checks; startup previously
auto-applied migrations, which can serve mixed schema/code states during rollout.

## Requirements Driving the Design

- `Reproducible deployment`
- `Safe migrations and health`
- `Backup and recovery`

## Architecture

- One image builds release binaries and the WASM client; compose wires PostgreSQL → migrate →
  server/worker with health-gated dependencies. Ingress serves `/pkg/*` (documented) while the
  server serves static assets, the manifest, the service worker, and all app/API routes.
- Startup guard: both binaries call `migrations::pending` and refuse to start when non-empty;
  `momentum-server migrate` applies migrations explicitly. Existing readiness reports
  persistence/authentication state and degrades on database failure.
- Worker passes enumerate accounts (`Database::account_ids`), run the reminder scheduler and
  recurrence materializer, and log privacy-safe counters; graceful shutdown via signal.
- Backup script uses `pg_dump --format=custom` with retention and a restore verification that
  checks account and migration counts in a scratch database.

## Data and Persistence

- No schema change; migration procedure and rollback/restore paths are documented.

## Offline and Sync

- Worker passes use the same command layer as the server, so sync semantics are unchanged.

## UI and Accessibility

- Not applicable.

## Error Handling

- Pending migrations fail startup with an actionable message; readiness degrades on database
  failure; backup verification fails loudly on restore errors.

## Testing Strategy

- Deployment tests for liveness/readiness payloads without persistence and without a session;
  gates compile the worker binary and validate compose/Dockerfile syntax indirectly through
  formatting review; backup script is exercised manually in environments with PostgreSQL.

## Rollout / Migration

- Deployments run the migrate step before rolling server/worker; rollback via restore is
  documented.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the server already had probes with a different payload schema; the change keeps those
  endpoints (avoiding duplicate routes) and documents their fields, adding only the migration
  guard, worker, and deployment artifacts.
