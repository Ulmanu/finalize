# cloud-sync-protocol

## Purpose

Incremental push/pull, cursors, operation IDs, versions and conflict rules.

## ADDED Requirements

### Requirement: Incremental push/pull protocol

Clients SHALL push idempotent operations and pull server changes using monotonic per-account cursor/sequence semantics.

#### Scenario: Incremental push/pull protocol — primary behavior

- **GIVEN** a client with a cursor and pending operations
- **WHEN** it syncs
- **THEN** operations are acknowledged with assigned sequences and only changes after the cursor are returned

#### Scenario: Incremental push/pull protocol — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Idempotent operation IDs

The server SHALL deduplicate retried client operations by stable client operation identifier.

#### Scenario: Idempotent operation IDs — primary behavior

- **GIVEN** an operation already applied under its identifier
- **WHEN** the client retries it
- **THEN** the server acknowledges a replay with the original sequence and creates no new change

#### Scenario: Idempotent operation IDs — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Conflict policy

The sync layer SHALL implement explicit field/domain conflict rules for concurrent edits, deletes, completion, and ordering rather than generic destructive last-write-wins.

#### Scenario: Conflict policy — primary behavior

- **GIVEN** conflicting concurrent changes (stale versions, deletes, completions, ordering)
- **WHEN** operations are pushed
- **THEN** each conflict is classified under a named rule with a deterministic resolution and no destructive overwrite

#### Scenario: Conflict policy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
