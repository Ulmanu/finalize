# Proposal

## Why

Production needs reproducible artifacts and topology, safe schema rollout, probes that reflect
real readiness, and a verified backup/restore path.

## What Changes

- Reproducible deployment: a multi-stage Dockerfile (release server + worker binaries, WASM
  client artifacts, assets) and a docker-compose topology (PostgreSQL, one-shot migrate step,
  server, worker) with documented ingress requirements for `/pkg/*`; `DEPLOYMENT.md` covers
  environment variables, procedures, and observability.
- Worker topology: a `momentum-worker` binary running reminder delivery scheduling and
  recurrence materialization per account every 60s, refusing to start with pending migrations,
  shutting down gracefully, and logging only ids/timestamps.
- Safe migrations: `momentum-server migrate` applies migrations as an explicit deployment step;
  the server and worker refuse to start while migrations are pending, so incompatible schema/code
  states never serve traffic. Probes: `/healthz` liveness and `/readyz` readiness (database
  reachability and authentication state, degraded payload on failure).
- Backup/recovery: `scripts/backup.sh` writes custom-format dumps with retention and a `verify`
  mode that restores into a scratch database, checks counts, and cleans up.

## Capabilities

### New Capabilities

- `features/production-deployment`: reproducible deployment, safe migrations and health,
  backup/recovery.

### Modified Capabilities

(none)

## Impact

- `crates/web` (migrate subcommand, startup guard, worker binary, deployment tests),
  `crates/infra` (account enumeration for worker passes), repository-level Dockerfile,
  compose, backup script, and `DEPLOYMENT.md`. No schema change.
- Non-goals: Kubernetes manifests, hosted CI/CD pipelines, a managed push transport in the worker
  (the notifier logs deliveries until a transport is configured).
