# features/production-deployment Specification

## Purpose

Define production operations: reproducible Docker artifacts and compose topology for the web
server, background worker, database, and ingress-served WASM assets; explicit safe migrations with
liveness/readiness probes so incompatible schema/code states never serve; and database backup,
retention, and restore-verification procedures.

## Requirements

### Requirement: Reproducible deployment

The project SHALL define reproducible production artifacts/topology for web server, background worker, database connectivity, static/WASM assets, and required ingress integrations.

#### Scenario: Reproducible deployment — primary behavior

- **GIVEN** the deployment artifacts
- **WHEN** they are built and composed
- **THEN** server, worker, database, and asset serving are defined reproducibly

#### Scenario: Reproducible deployment — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Safe migrations and health

Deployment SHALL provide readiness/liveness checks and a migration procedure that avoids serving incompatible schema/code states.

#### Scenario: Safe migrations and health — primary behavior

- **GIVEN** a new deployment with pending migrations
- **WHEN** server or worker starts
- **THEN** it refuses to serve until the explicit migration step runs, and probes report readiness truthfully

#### Scenario: Safe migrations and health — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Backup and recovery

Production operations SHALL define database backup, retention, and restore verification procedures.

#### Scenario: Backup and recovery — primary behavior

- **GIVEN** a production database
- **WHEN** backups run and are verified
- **THEN** dumps are retained per policy and a restore verification proves recoverability

#### Scenario: Backup and recovery — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
