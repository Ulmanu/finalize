# Tasks — add-account-authentication

## 1. Plan and validate

- [x] 1.1 Read current canonical specs and all archived dependencies.
- [x] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [x] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [x] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain and application

- [x] 2.1 Add `Email` and `Password` value objects with normalization, policy, and unit tests.
- [x] 2.2 Add auth records, ports (`AccountStore`, `SessionStore`, `AuthTokenStore`, `PasswordHasher`, `Clock`), typed `AuthError`, and `AuthService` implementing the full lifecycle.
- [x] 2.3 Add token generation/hashing with expiry and single-use semantics.

## 3. Infrastructure

- [x] 3.1 Add `Argon2Hasher` (Argon2id) with unit tests.
- [x] 3.2 Add migration 0002 (`auth_tokens`) with rollback and tests.
- [x] 3.3 Add `PgAuthStore` implementing the ports with runtime queries and account scoping.

## 4. Web

- [x] 4.1 Convert `SessionResolver` to async, add session middleware inserting `SessionState` into request extensions, and read it from reactive context.
- [x] 4.2 Add `CookieSessionResolver` validating the session cookie (HttpOnly, SameSite=Lax, Secure when configured) with expiry/revocation checks.
- [x] 4.3 Add auth routes and accessible sign-in/registration/reset forms with uniform failure behavior.
- [x] 4.4 Add testkit in-memory auth stores and deterministic hasher.

## 5. Tests and verification

- [x] 5.1 Add application lifecycle tests (register/verify/login/logout/reset/change/revoke/delete, duplicate email, wrong password, single-use and expired tokens).
- [x] 5.2 Add web tests for forms, login success/failure, cookie session access, and logout.
- [x] 5.3 Add account-isolation test proving another account's data is inaccessible.
- [x] 5.4 Run `cargo fmt --all -- --check`, Clippy `-D warnings`, `cargo test --workspace --all-features`, and the WASM client build.
- [x] 5.5 Perform explicit artifact-to-implementation verification and record it in `OVERNIGHT_REPORT.md` (including email-delivery and live-PostgreSQL gaps).
- [x] 5.6 Resolve all critical/actionable verification findings.

## 6. Archive and commit

- [x] 6.1 Re-run `openspec validate add-account-authentication --strict --no-interactive`.
- [x] 6.2 Confirm every task in this file is checked with acceptance evidence.
- [x] 6.3 Archive with `openspec archive add-account-authentication --yes` without bypassing validation.
- [x] 6.4 Run strict validation for canonical specs and archived-task completeness; complete the new spec's Purpose section.
- [x] 6.5 Review the full Git diff for accidental/unrelated changes.
- [x] 6.6 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this change.
