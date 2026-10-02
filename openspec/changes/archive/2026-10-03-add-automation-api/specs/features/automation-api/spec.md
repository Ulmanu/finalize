# automation-api

## Purpose

Scoped personal API tokens and REST automation over application commands.

## ADDED Requirements

### Requirement: Scoped API tokens

Users SHALL create, view-once, scope, revoke, and rotate personal automation API tokens.

#### Scenario: Scoped API tokens — primary behavior

- **GIVEN** an account
- **WHEN** tokens are created, listed, rotated, or revoked
- **THEN** plaintext is shown once, only hashes persist, scopes constrain use, and revoked or rotated tokens stop working

#### Scenario: Scoped API tokens — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Command-layer API

REST automation mutations SHALL delegate to the same application commands as the first-party UI.

#### Scenario: Command-layer API — primary behavior

- **GIVEN** an authenticated API request
- **WHEN** it creates, updates, or acts on a to-do
- **THEN** the mutation runs through the same command layer with the same validation and effects

#### Scenario: Command-layer API — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Stable API contract

The API SHALL use versioned routes and typed error responses suitable for automation.

#### Scenario: Stable API contract — primary behavior

- **GIVEN** a request with an error condition
- **WHEN** the API responds
- **THEN** it uses `/api/v1` routes and a typed JSON error with a stable code

#### Scenario: Stable API contract — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
