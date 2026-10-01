# offline-local-replica

## Purpose

IndexedDB local replica, optimistic writes, outbox and offline startup.

## ADDED Requirements

### Requirement: IndexedDB local replica

The browser SHALL persist the user's synchronized working set, preferences needed offline, sync cursor, and outbound operations in IndexedDB.

#### Scenario: IndexedDB local replica — primary behavior

- **GIVEN** a configured replica store
- **WHEN** startup runs after a reload
- **THEN** the working set, preferences, cursor, and pending operations are restored

#### Scenario: IndexedDB local replica — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Optimistic local commands

Supported mutations SHALL update the local replica/UI before server acknowledgement and enqueue a durable outbound operation.

#### Scenario: Optimistic local commands — primary behavior

- **GIVEN** an offline or connected client
- **WHEN** a supported mutation is issued
- **THEN** the local replica updates immediately and a durable outbound operation is enqueued

#### Scenario: Optimistic local commands — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Outbox recovery

The outbox SHALL survive browser reloads and replay idempotently after connectivity returns.

#### Scenario: Outbox recovery — primary behavior

- **GIVEN** pending outbound operations
- **WHEN** the connection returns and replay runs
- **THEN** operations are delivered in order exactly once and removed on acknowledgement

#### Scenario: Outbox recovery — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
