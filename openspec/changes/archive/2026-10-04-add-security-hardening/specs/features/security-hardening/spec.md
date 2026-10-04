# security-hardening

## Purpose

Argon2id, secure sessions, CSRF/CSP, rate limiting, isolation and API authorization.

## ADDED Requirements

### Requirement: Session and web hardening

The deployed web app SHALL enforce secure cookies, CSRF defenses, CSP, secure headers, and authentication throttling appropriate to its architecture.

#### Scenario: Session and web hardening — primary behavior

- **GIVEN** any web response or mutating request
- **WHEN** it is served or submitted
- **THEN** secure cookies, CSP and companion headers apply, foreign-origin mutations are rejected, and failed logins are throttled

#### Scenario: Session and web hardening — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Authorization isolation tests

Every user-owned API/query family SHALL include cross-account isolation tests.

#### Scenario: Authorization isolation tests — primary behavior

- **GIVEN** two accounts with data
- **WHEN** one account queries or mutates the other's entities
- **THEN** reads are empty or not-found, mutations are rejected, and owner state is unchanged

#### Scenario: Authorization isolation tests — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Secret, input and rate safety

Secrets SHALL remain server-side, inputs SHALL be bounded/validated, and abuse-prone endpoints SHALL enforce rate/resource limits.

#### Scenario: Secret, input and rate safety — primary behavior

- **GIVEN** secrets, oversized inputs, or abusive request patterns
- **WHEN** they are handled
- **THEN** secrets never render, inputs are bounded/rejected, and rate limits apply

#### Scenario: Secret, input and rate safety — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
