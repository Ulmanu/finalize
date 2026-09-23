# Spec delta — auth/accounts

## ADDED Requirements

### Requirement: Account lifecycle

The system SHALL support registration, verification, login, logout, password reset/change,
session revocation, and account deletion.

#### Scenario: Registration then verification then login

- **GIVEN** an unregistered email address
- **WHEN** the user registers with a valid password
- **THEN** an unverified account is created with an Argon2id password hash
- **AND** a verification token is issued
- **AND** after verification, valid credentials create a session and redirect to an application destination

#### Scenario: Password reset revokes existing sessions

- **GIVEN** an account with an active session
- **WHEN** the user completes a password reset
- **THEN** the password hash is replaced and every existing session for that account is revoked
- **AND** the user can sign in with the new password only

#### Scenario: Account deletion removes access

- **GIVEN** an authenticated account
- **WHEN** the user deletes the account
- **THEN** the account is marked deleted, sessions are revoked, and subsequent authentication fails

### Requirement: Secure password and session handling

Passwords SHALL be stored with Argon2id and authenticated browser sessions SHALL use secure
server-validated session state.

#### Scenario: Session tokens are opaque and stored hashed

- **GIVEN** a successful login
- **WHEN** the session is created
- **THEN** the client receives a high-entropy opaque token in an `HttpOnly`, `SameSite=Lax` cookie
- **AND** only the token hash is persisted
- **AND** a request presenting an unknown, expired, or revoked token is treated as anonymous

#### Scenario: Credential failures do not leak account existence

- **WHEN** login or password-reset is attempted with an unknown email
- **THEN** the response is indistinguishable from the wrong-password case

### Requirement: Account isolation

All authenticated application queries and commands SHALL be scoped to the owning account.

#### Scenario: Foreign account data is inaccessible

- **GIVEN** account A is authenticated
- **WHEN** a request references an entity belonging to account B
- **THEN** the operation fails with a not-found/authorization error
- **AND** no account B data is returned or modified

#### Scenario: Unauthenticated access is rejected

- **GIVEN** no valid session
- **WHEN** a protected route or authenticated command is requested
- **THEN** the unauthorized boundary or an authorization error is returned
