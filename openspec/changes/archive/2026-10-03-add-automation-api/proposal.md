# Proposal

## Why

Automation needs durable, scoped credentials and a versioned REST surface that behaves exactly
like the UI. Personal tokens must be view-once, revocable, rotatable, and scope-limited, and the
API must delegate to the same commands with typed errors.

## What Changes

- Add `momentum_app::api_tokens`: `ApiScope` (read; write implies read), `ApiTokenSummary`,
  view-once `CreatedApiToken`, the `ApiTokenStore` port, `ApiPrincipal`, and `ApiTokenService`
  (create, list, revoke, rotate, authenticate) reusing the authentication token generator for
  generation and deterministic hashing (plaintext never stored).
- Migration 0007 adds `api_tokens` (id, account, name, hash, scopes, created/used/revoked
  timestamps); `PgApiTokenStore` implements the port.
- Add the versioned REST surface `/api/v1/todos` (list with scope allow-list, create, show,
  patch, complete/cancel/reopen/delete actions) authenticated by `Authorization: Bearer`, with
  typed JSON errors (`{"error":{"code","message"}}`) and every mutation delegated to the
  application services.
- Settings gains token management: create (name + scopes) with view-once display via redirect,
  list with revoke/rotate actions.

## Capabilities

### New Capabilities

- `features/automation-api`: scoped API tokens, command-layer API, stable API contract.

### Modified Capabilities

(none)

## Impact

- `crates/app` (token module + unit tests), `crates/infra` (migration + PG store), `crates/web`
  (API routes, settings surface, tests), `crates/testkit` (memory store + tests).
- Non-goals: OAuth client registration, webhook subscriptions, API versioning beyond v1.
