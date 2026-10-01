# multitab-coordination

## Purpose

BroadcastChannel/local leadership and synchronized browser-tab replicas.

## ADDED Requirements

### Requirement: Multi-tab coordination

Tabs SHALL coordinate sync/outbox ownership through browser-native coordination so duplicate workers are minimized.

#### Scenario: Multi-tab coordination — primary behavior

- **GIVEN** several tabs of the same account
- **WHEN** leadership is claimed and messages are exchanged
- **THEN** exactly one tab owns sync/outbox work at a time

#### Scenario: Multi-tab coordination — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Cross-tab state propagation

Committed local/server changes SHALL be broadcast so other tabs refresh their replica/view promptly.

#### Scenario: Cross-tab state propagation — primary behavior

- **GIVEN** one tab commits or receives new data
- **WHEN** the change is broadcast
- **THEN** sibling tabs receive a pull hint and refresh through the canonical protocol

#### Scenario: Cross-tab state propagation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Leader recovery

If the coordinating tab closes, another tab SHALL take over without losing outbox work.

#### Scenario: Leader recovery — primary behavior

- **GIVEN** a leader whose lease expires
- **WHEN** another tab claims leadership
- **THEN** it takes over and the shared outbox remains intact

#### Scenario: Leader recovery — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
