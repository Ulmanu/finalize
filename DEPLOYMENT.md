# Deployment

## Docker for local testing (no login)

```bash
docker compose up --build -d
```

Open **http://localhost:3000**. The default Compose configuration starts only the local web
server and explicitly clears `DATABASE_URL`. You can create and complete tasks immediately;
there is no account registration or login. Tasks are in memory and reset on server restart.
The image includes the generated browser JavaScript/WASM assets and serves them at `/pkg/*`.

Stop the local app with `docker compose stop`.

## Production topology

The database-backed deployment is opt-in:

```bash
export POSTGRES_PASSWORD=your-database-password
docker compose -f docker-compose.production.yml up --build -d
```

Use `-f docker-compose.production.yml` for the production migration, backup, and lifecycle
commands below. Stop the local server before using production on the same port.

- **PostgreSQL 16** — canonical storage; the only stateful service.
- **`momentum-server migrate`** — one-shot migration step; runs before server/worker start.
- **`momentum-server`** — the SSR web server (probes: `GET /healthz` liveness, `GET /readyz`
  readiness with database reachability). It refuses to start when migrations are pending.
- **`momentum-worker`** — background passes every 60s: reminder delivery scheduling and
  recurrence materialization per account; graceful shutdown on SIGINT/SIGTERM; logs only ids and
  timestamps.
- **Ingress** — routes requests to the server, which serves `/pkg/*` from the packaged
  `target/site/pkg` browser artifacts. The server also serves `/static/*` (styles, icon), `/manifest.webmanifest`,
  `/service-worker.js`, and all application/API routes.

## Local mode (no database, no sign-in)

With `DATABASE_URL` unset, the server runs in **local mode**: in-memory storage, a fixed local
account, and no sign-in — useful for trying the app or developing. A warning is logged and data
is not persisted. The `local` feature (enabled by default in `crates/web`) provides the in-memory
services; disable it with `--no-default-features --features ssr` for a build that requires a
database. Production always sets `DATABASE_URL`, so local mode never applies there.

## Environment

| Variable | Purpose |
| --- | --- |
| `DATABASE_URL` | PostgreSQL connection string (required in production). |
| `MOMENTUM_BIND_ADDR` | Server bind address (default `127.0.0.1:3000`). |
| `MOMENTUM_SECURE_COOKIES` | `true` behind HTTPS so session cookies carry `Secure`. |
| `MOMENTUM_INBOUND_SECRET` | Shared secret expected from the mail provider webhook. |

## Migrations

Migrations are Rust-managed, versioned, and checksum-verified (`crates/infra/src/db`). Procedure:

1. Build the new image.
2. Run `momentum-server migrate` (compose: the `migrate` service) and wait for success.
3. Roll out `server` and `worker`; both refuse to start with pending migrations, so incompatible
   schema/code states cannot serve traffic.

Rollback of a migration is an explicit operator action (`rollback_last` in the migrations module);
restore from backup is the general recovery path.

## Backup and recovery

`scripts/backup.sh backup` writes compressed custom-format dumps to `$BACKUP_DIR` and prunes to
`$BACKUP_RETENTION` (default 14). `scripts/backup.sh verify <dump>` restores into a scratch
database, checks account and migration counts, and drops the scratch database — run verification
on every scheduled backup. Recovery: stop server/worker, restore the dump into the production
database, run `momentum-server migrate`, start server/worker.

## Health and observability

- `/healthz` reports process liveness; `/readyz` reports database reachability (503 when
  unreachable).
- `/metrics` exposes privacy-safe counters (requests, sync, reminders, recurrence, inbound,
  calendar, push); scrape it from the ingress and keep it internal.
- Request logs carry an `x-request-id` correlation id with method/path only — never task content.
