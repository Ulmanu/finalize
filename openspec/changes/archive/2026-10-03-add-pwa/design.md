# Design — add-pwa

## Context

The manifest already existed for share capture (54). Service workers must be JavaScript, and the
mission forbids handwritten JavaScript application logic, so the worker and its registration are
generated from Rust (`momentum-web::pwa`) as versioned build artifacts.

## Requirements Driving the Design

- `PWA installability`
- `Offline shell`
- `Update lifecycle`

## Architecture

- `service_worker_script(version)` emits a worker whose cache name embeds `SHELL_VERSION`;
  precache covers `/`, `/today`, styles, the WASM package, manifest, and icon. Static assets are
  cache-first (refresh-on-miss), navigations are network-first with a cache fallback to `/today`,
  and `/sync` plus `/api/` are excluded so live data is never served stale.
- `REGISTRATION_SNIPPET` registers the worker after load and reloads once on
  `controllerchange`; the worker calls `skipWaiting`/`clients.claim` and deletes non-current
  caches on activation — a new deployment replaces old code without trapping clients.
- The icon is an inline SVG served from Rust; the manifest references it with
  `sizes: any, purpose: any maskable`.

## Data and Persistence

- Cache storage only; no schema change. Offline writes remain the replica/outbox's job.

## Offline and Sync

- The shell starts offline; data reads/writes rely on the existing replica and sync protocol.

## UI and Accessibility

- No new UI; shortcuts deep-link to documented command URLs.

## Error Handling

- Registration failures are ignored; fetch failures fall back to cache or the cached shell;
  a missing network never blocks a precached start.

## Testing Strategy

- Rust unit tests for the versioned worker script and registration reload guard; web tests for
  manifest installability fields and shortcuts, worker headers/content (version, skipWaiting,
  claim, precache, exclusions), icon content type, and shell manifest/icon/registration links.

## Rollout / Migration

- Bumping `SHELL_VERSION` deploys new assets; old caches are removed on activation.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: a generated-from-Rust worker respects the no-handwritten-JS rule while still giving a
  real offline shell; registration lives in the Rust shell head like the hydration bootstrap.
