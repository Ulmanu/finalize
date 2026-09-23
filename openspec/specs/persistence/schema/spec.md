# persistence/schema Specification

## Purpose

Define Momentum's server-side persistence foundation: the canonical PostgreSQL schema
(accounts, sessions, entities, placements, relationships, recurrence, reminders, preferences,
sync metadata, tombstones), Rust-managed migrations with checksums and rollback, and stable
client-generatable identifiers with optimistic version metadata.

## Requirements

### Requirement: Canonical persistence schema

The system SHALL persist accounts, entities, relationships/placements, recurrence data,
reminders, preferences, sync metadata, and tombstones in PostgreSQL using Rust-managed
migrations.

#### Scenario: Migrations create the canonical schema

- **GIVEN** an empty PostgreSQL database
- **WHEN** the migration engine applies all migrations
- **THEN** the accounts, sessions, entities, placements, tag assignments, checklist items,
  recurrence, reminders, preferences, sync metadata, change log, and tombstone tables exist
- **AND** the applied versions are recorded in the migration table

#### Scenario: Applying migrations is idempotent

- **GIVEN** a database with all migrations applied
- **WHEN** the migration engine runs again
- **THEN** no migration is applied twice
- **AND** the recorded schema state is unchanged

### Requirement: Stable identifiers and versions

Persisted user entities SHALL use stable client-generatable identifiers and optimistic version
metadata suitable for offline creation and synchronization.

#### Scenario: Client-generated identifiers are accepted

- **GIVEN** an entity created offline with a UUIDv7 identifier
- **WHEN** the entity is persisted and later synchronized
- **THEN** the identifier is preserved unchanged
- **AND** the entity carries an incrementing `version` and `updated_at` timestamp

#### Scenario: Deletion is recoverable and sync-visible

- **GIVEN** a persisted entity
- **WHEN** it is deleted through the application
- **THEN** it is marked with deletion state and a tombstone record rather than being
  physically removed immediately

### Requirement: Migration safety

Every persistence change SHALL have a forward migration and an explicit rollback or documented
irreversibility strategy tested against an isolated database.

#### Scenario: Rollback restores the previous schema

- **GIVEN** a database with migration N applied
- **WHEN** the operator rolls back the last migration and a rollback script exists
- **THEN** the schema objects created by that migration are removed
- **AND** the migration record for N is deleted

#### Scenario: Migration definitions are validated before use

- **GIVEN** the compiled migration list
- **WHEN** the validation test inspects it
- **THEN** versions are strictly increasing and unique, every migration has non-empty forward
  SQL, and every migration has a documented rollback or irreversibility note
