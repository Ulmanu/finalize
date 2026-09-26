# trash

## Purpose

Recoverable deletion, restore, permanent deletion and empty-trash flow.

## ADDED Requirements

### Requirement: Soft deletion

Deleting a supported entity SHALL move it into recoverable Trash via tombstone/deleted state rather than immediate physical loss.

#### Scenario: Soft deletion — primary behavior

- **GIVEN** an active to-do, project, or area
- **WHEN** a user deletes an eligible entity
- **THEN** it disappears from active projections and appears in Trash

#### Scenario: Soft deletion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Restore

Trash items SHALL be restorable with deterministic recovery of their prior or valid fallback context.

#### Scenario: Restore — primary behavior

- **GIVEN** a trashed entity, possibly with a deleted parent
- **WHEN** a deleted entity is restored
- **THEN** it becomes active in a valid context with its user data preserved

#### Scenario: Restore — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Permanent deletion

Permanent deletion/empty Trash SHALL require an explicit action and remove or anonymize data according to retention policy.

#### Scenario: Permanent deletion — primary behavior

- **GIVEN** trashed entities
- **WHEN** a user confirms permanent deletion
- **THEN** the data is irrecoverably removed from active storage and sync tombstones remain only as long as required

#### Scenario: Permanent deletion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
