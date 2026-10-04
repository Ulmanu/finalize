# Proposal

## Why

The deployed web app must enforce secure defaults: session cookies already use HttpOnly +
SameSite=Lax, but responses lack CSP/security headers, mutating requests lack an origin check,
and failed logins are unlimited.

## What Changes

- Add `momentum-web::security`: a headers middleware adding a restrictive CSP (no external
  origins, `frame-ancestors 'none'`, `object-src 'none'`), `nosniff`, referrer policy, frame
  denial, and a permissions policy; an origin guard rejecting mutating requests whose `Origin`
  host differs from the request host (defense-in-depth over SameSite=Lax); and a per-account
  `LoginThrottle` (window + attempt budget) with clear-on-success.
- Wire the layers in the server router and throttle the login handler with
  `?error=throttled`; `AppState` exposes the throttle for tests.
- Add explicit cross-account isolation tests across to-dos, projects, areas, tags, preferences,
  and API tokens (reads empty, mutations rejected, owner state untouched).

## Capabilities

### New Capabilities

- `features/security-hardening`: session/web hardening, authorization isolation tests,
  secret/input/rate safety.

### Modified Capabilities

(none)

## Impact

- `crates/web` (security module, middleware, login throttle, tests), `crates/testkit` (isolation
  tests). No schema change.
- Non-goals: nonce-based CSP (the inline hydration bootstrap is per-render; documented gap),
  distributed throttling (in-process window today), HSTS (belongs to TLS termination config).
