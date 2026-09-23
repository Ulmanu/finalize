# Spec delta — app/commands

## ADDED Requirements

### Requirement: Typed application commands

All authoritative mutations SHALL enter through typed application commands rather than direct
UI or persistence writes.

#### Scenario: A mutation produces domain events and invalidations

- **GIVEN** a valid typed command for an account-scoped entity
- **WHEN** the dispatcher executes it
- **THEN** the domain entity is updated according to domain rules
- **AND** the outcome contains the emitted domain events and the projections to invalidate

#### Scenario: Commands are account-scoped

- **GIVEN** an entity owned by account B
- **WHEN** account A issues a command for that entity
- **THEN** the command fails with a not-found error
- **AND** no state changes are persisted for either account

### Requirement: Atomic command transaction

A command SHALL update domain state, persistence, change-log metadata, and projection
invalidation atomically where persistence is involved.

#### Scenario: A failed command leaves no partial state

- **GIVEN** a command whose persistence step fails
- **WHEN** the dispatcher executes it
- **THEN** the entity, change log, and sync sequence are unchanged
- **AND** the failure is reported as a typed error

#### Scenario: Successful commands append ordered change records

- **GIVEN** two successful commands for one account
- **WHEN** both commit
- **THEN** the change log contains two records with strictly increasing sequences
- **AND** each record carries the command's operation id

### Requirement: Deterministic command errors

Validation, authorization, conflict, and not-found failures SHALL be represented as typed
application errors consumable by web, sync, and automation adapters.

#### Scenario: Stale version is a conflict

- **GIVEN** an entity updated since the caller last read it
- **WHEN** a command persists an update with the stale version
- **THEN** the command fails with a conflict error carrying expected and actual versions

#### Scenario: Replayed operations are idempotent

- **GIVEN** a command that already committed with a given operation id
- **WHEN** the same operation id is submitted again
- **THEN** the recorded outcome is returned and marked as replayed
- **AND** no additional change-log record or state mutation is produced
