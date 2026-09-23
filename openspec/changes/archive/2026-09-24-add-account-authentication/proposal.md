# Proposal

## Why

The shell treats every request as anonymous, so no user data can exist. Authentication is the
gate for all later capabilities: accounts, credentials, sessions, and account-scoped
authorization must exist before tasks, projects, and sync can be built.

## What Changes

- Add domain value objects: normalized email and password policy.
- Add `momentum-app::auth`: account lifecycle commands (register, verify, login, logout,
  password reset/change, session revocation, account deletion) over account/session/token
  store ports, plus a `PasswordHasher` port.
- Add PostgreSQL adapters (`PgAuthStore`, `Argon2Hasher`) and migration 0002 for verification
  and reset tokens.
- Replace the anonymous-only session boundary with a real cookie-backed session resolver:
  session tokens are random, stored hashed, sent in an `HttpOnly`, `SameSite=Lax` (and
  `Secure` when configured) cookie, validated per request, and revoked on logout/password
  change/account deletion.
- Add auth HTTP routes (`/auth/register`, `/auth/verify`, `/auth/login`, `/auth/logout`,
  `/auth/password-reset`, `/auth/password-reset/confirm`, `/auth/password-change`) and real
  sign-in/registration forms; protected routes now require a valid session.
- Account isolation: every auth and (later) user-data operation is scoped to the
  authenticated account; unauthenticated or foreign-account access is rejected.

## Capabilities

### New Capabilities

- `auth/accounts`: account lifecycle, Argon2id password storage, secure server-validated
  sessions, and account-scoped authorization.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (email/password), `crates/app` (auth ports/service), `crates/infra`
  (Argon2, PostgreSQL stores, migration 0002), `crates/testkit` (in-memory auth stores and
  lifecycle tests), `crates/web` (session middleware/resolver, auth routes, pages).
- New dependencies: `argon2`, `sha2`, `rand`, `async-trait`.
- Non-goals: OAuth/social login, MFA, email delivery provider (verification/reset tokens are
  surfaced through the service and recorded as a live-integration gap), CSRF/CSP hardening
  (change 69), no handwritten JS/TS.
