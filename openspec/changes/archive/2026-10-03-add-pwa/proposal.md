# Proposal

## Why

The app should install like a native app, start offline from previously loaded assets, and update
safely across deployments without trapping running clients on incompatible code.

## What Changes

- Installability: the manifest gains `theme_color`, `background_color`, an SVG icon
  (`sizes: any`, maskable), and app shortcuts over command URLs (New to-do → `/add`, Quick Find →
  `/search`, Today → `/today`); the shell links the manifest and icon.
- Offline shell: a Rust-generated service worker (`momentum-web::pwa`) precaches the shell
  (pages, styles, WASM package, manifest, icon) with a versioned cache, serves static assets
  cache-first and navigations network-first with an offline fallback, and never caches
  `/sync` or `/api/` endpoints.
- Update lifecycle: `install` skips waiting, `activate` deletes older versioned caches and claims
  clients, and a Rust-generated registration snippet reloads the page exactly once on
  `controllerchange`, so running clients never stay on incompatible code.

## Capabilities

### New Capabilities

- `features/pwa`: installability, offline shell, update lifecycle.

### Modified Capabilities

(none)

## Impact

- `crates/web` (pwa module + unit tests, server routes for the worker/icon, extended manifest,
  shell head, web tests). No schema change.
- Non-goals: push notifications via service workers (Web Push plumbing exists server-side),
  background sync (the replica/outbox already covers offline writes), app-store packaging.
