# Design — add-security-hardening

## Context

Sessions are HttpOnly + SameSite=Lax cookies (Argon2id hashing already protects passwords);
secrets live server-side (token hashes, env webhook secret); inputs are bounded (title caps,
capture/backup caps, size-limited inbound mail).

## Requirements Driving the Design

- `Session/web hardening`
- `Authorization isolation tests`
- `Secret/input/rate safety`

## Architecture

- Headers middleware adds the CSP and companion headers to every response. CSP permits inline
  scripts because the hydration bootstrap is emitted inline with a per-render nonce that is not
  yet propagated to headers (documented gap); no external origins are allowed.
- Origin guard: mutating methods with an `Origin` header must match `Host`; absent origins pass
  (same-origin forms, automation, provider webhooks). SameSite=Lax already prevents cross-site
  cookies, so the guard is defense-in-depth.
- Login throttle records attempts per lowercased email in a sliding window and clears on success;
  blocked attempts redirect with `?error=throttled` before touching the auth service.
- Isolation tests exercise every user-owned family with two accounts: empty reads, not-found
  gets, rejected mutations, and unchanged owner state.

## Data and Persistence

- None; throttle state is in-process.

## Offline and Sync

- Not applicable.

## UI and Accessibility

- Throttled logins surface as the standard login error state.

## Error Handling

- Foreign-origin mutations → 403; throttled logins → redirect with error; header/guard failures
  never leak secrets.

## Testing Strategy

- Web tests for headers on responses, foreign/matching/absent origins, and throttling with a
  small budget; testkit isolation tests across to-dos, projects, areas, tags, preferences, and
  API tokens; a unit test for the throttle window.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: HSTS and nonce-CSP depend on deployment/hydration wiring that does not exist yet and
  are recorded as gaps rather than half-configured.
