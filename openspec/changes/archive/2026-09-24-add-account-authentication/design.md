# Design — add-account-authentication

## Context

Authentication must be real before any user data exists. Ports keep crypto and SQL out of the
application layer; deterministic in-memory stores keep lifecycle tests fast, while the real
Argon2id and PostgreSQL adapters are unit/integration tested separately.

## Requirements Driving the Design

- `Account lifecycle`
- `Secure password and session handling`
- `Account isolation`

## Architecture

- `momentum-domain::email` — `Email` with normalization (trim + lowercase) and validation;
  display form preserved.
- `momentum-domain::password` — `Password` policy (minimum length, maximum length, rejects
  passwords equal to the email).
- `momentum-app::auth`:
  - Records: `AccountRecord`, `SessionRecord`, `AuthTokenRecord`.
  - Ports: `AccountStore`, `SessionStore`, `AuthTokenStore`, `PasswordHasher`, `Clock`.
  - `AuthService` implements register/verify/login/logout/reset/change/revoke/delete; tokens
    are random 32-byte hex, stored as SHA-256 hashes with expiry and single use.
  - Errors are typed (`AuthError`) and never reveal whether an email exists.
- `momentum-infra::auth`:
  - `Argon2Hasher` (Argon2id, per-password salt) implementing `PasswordHasher`.
  - `PgAuthStore` implementing all three stores with runtime sqlx queries.
  - Migration 0002 adds `auth_tokens(kind, token_hash unique, account_id, expires_at,
    consumed_at)`.
- `crates/web`:
  - `SessionResolver` becomes async and is run by an Axum middleware that inserts
    `SessionState` into request extensions; the reactive context reads it from
    `http::request::Parts.extensions`.
  - `CookieSessionResolver` validates the `momentum_session` cookie against `SessionStore`.
  - Auth routes are plain Axum handlers with HTML forms (progressive enhancement): register,
    verify, login, logout, password reset/confirm, password change; successful login sets the
    cookie and redirects with 303.
  - Login/registration pages are server-rendered forms; failure renders the page with an
    error message (no user enumeration).
- `momentum-testkit::auth` — `MemoryAuthStore` and `DeterministicHasher` plus lifecycle tests.

## Data and Persistence

Migration 0002 creates `auth_tokens`. Sessions reuse the change-05 `sessions` table. Password
hashes live on `accounts.password_hash`; `email_normalized` is unique. Account deletion is
soft (`deleted_at`) with session revocation and an audit-friendly tombstone path later.

## Offline and Sync

Auth is server-authoritative; offline clients authenticate when online and hold the session
cookie. Session revocation is checked on every request, so a revoked offline client cannot
mutate after reconnecting.

## UI and Accessibility

Forms use native labels, `autocomplete` attributes (`email`, `current-password`,
`new-password`), visible focus, and error text tied to inputs via `aria-describedby`. The
login page is reachable from the landing page and unauthorized boundary.

## Error Handling

- Uniform failure responses for credential and reset flows (no enumeration).
- Expired/revoked sessions resolve to `Anonymous` (boundary, not error page).
- Storage errors are logged and surfaced as generic failures.

## Testing Strategy

- Domain unit tests: email normalization/validation, password policy.
- Application lifecycle tests (testkit in-memory): register → verify → login → logout, wrong
  password, duplicate email, reset revokes sessions, change password, revoke session, delete
  account, token single-use and expiry.
- Infra unit tests: Argon2 hash/verify; conditional PostgreSQL auth store test.
- Web tests: login/register pages render accessible forms; login with valid/invalid
  credentials; protected route becomes accessible with a valid cookie; logout clears it.
- Live email delivery is out of scope and recorded.

## Rollout / Migration

Migration 0002 is additive. Existing development databases upgrade in place.

## Open Questions Resolved From the Repository

- **Async session resolution**: cookie validation needs I/O, so resolution moved from the
  reactive context into Axum middleware; the reactive context now reads request extensions.
- **Server functions vs routes**: plain Axum form routes were chosen for progressive
  enhancement and direct integration testing; Leptos server functions can wrap them later.
- **Token hashing**: SHA-256 (not Argon2) for high-entropy random tokens; Argon2id is reserved
  for low-entropy human passwords.

## Tasks / Definition of Done

See `tasks.md`.
