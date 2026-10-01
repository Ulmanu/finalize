# features/realtime-sync Specification

## Purpose
WebSocket/SSE invalidation hints feeding the canonical sync protocol.

## Requirements

### Requirement: Realtime invalidation hints

Realtime transport SHALL notify connected clients that newer server sequence data exists without introducing a second mutation protocol.

#### Scenario: Realtime invalidation hints — primary behavior

- **GIVEN** a connected client and a successful server mutation
- **WHEN** the change is committed
- **THEN** the client receives a hint that newer data exists and pulls through the canonical sync protocol

#### Scenario: Realtime invalidation hints — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reconnect behavior

Realtime disconnect/reconnect SHALL not lose correctness because cursor-based sync remains authoritative.

#### Scenario: Reconnect behavior — primary behavior

- **GIVEN** a client that was disconnected during changes
- **WHEN** it reconnects
- **THEN** it is told to pull and reconciles through its cursor with no data loss

#### Scenario: Reconnect behavior — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Backpressure and coalescing

Repeated realtime hints SHALL be coalesced so a busy account does not create unbounded simultaneous pull loops.

#### Scenario: Backpressure and coalescing — primary behavior

- **GIVEN** a burst of mutations while a hint is pending delivery
- **WHEN** hints are published
- **THEN** at most one pending hint exists per account and it carries the newest sequence

#### Scenario: Backpressure and coalescing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
