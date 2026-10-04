# features/observability Specification

## Purpose

Define observability: structured request traces with assigned/propagated correlation ids;
privacy-safe, label-free operational counters for requests, sync, reminders, recurrence,
calendar, inbound mail, and push with a text exposition endpoint; and diagnostic privacy so logs
and metrics never carry task content, email bodies, tokens, or secrets.

## Requirements

### Requirement: Structured tracing

Server, worker, sync, scheduler, and integration paths SHALL emit structured traces with correlation/request identifiers.

#### Scenario: Structured tracing — primary behavior

- **GIVEN** an incoming request
- **WHEN** it is handled
- **THEN** a request identifier is assigned or propagated and attached to structured request spans

#### Scenario: Structured tracing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Operational metrics

The system SHALL expose privacy-safe health/performance counters for requests, sync, reminders, recurrence, database pool, and external integrations.

#### Scenario: Operational metrics — primary behavior

- **GIVEN** served requests and background work
- **WHEN** metrics are scraped
- **THEN** declared counters reflect requests, sync, reminders, recurrence, and integration outcomes

#### Scenario: Operational metrics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Diagnostic privacy

Logs/metrics SHALL avoid task titles, notes, email bodies, tokens, and secrets unless explicitly redacted/test-safe.

#### Scenario: Diagnostic privacy — primary behavior

- **GIVEN** user data flowing through the system
- **WHEN** logs or metrics are emitted
- **THEN** they contain identifiers, names, and counters only — never content, tokens, or secrets

#### Scenario: Diagnostic privacy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
