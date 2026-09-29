# calendar-integration

## Purpose

Provider-neutral calendar interface plus initial web calendar adapters.

## ADDED Requirements

### Requirement: Provider-neutral calendar API

Core application code SHALL depend on a CalendarProvider contract rather than Google/Microsoft/CalDAV-specific types.

#### Scenario: Provider-neutral calendar API — primary behavior

- **GIVEN** any implemented provider adapter
- **WHEN** core code reads events for a range
- **THEN** it receives provider-neutral events without provider-specific types

#### Scenario: Provider-neutral calendar API — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Read-only projection

Connected calendar events SHALL appear in Today/Upcoming as external read-only rows by default.

#### Scenario: Read-only projection — primary behavior

- **GIVEN** connected calendar events in the visible range
- **WHEN** Today or Upcoming renders
- **THEN** the events appear as external read-only rows with no task actions

#### Scenario: Read-only projection — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Provider resilience

Calendar failures/auth expiry SHALL degrade the calendar projection without blocking task data or corrupting cached calendar state.

#### Scenario: Provider resilience — primary behavior

- **GIVEN** a provider failure or expired authorization
- **WHEN** Today or Upcoming renders
- **THEN** task rows remain complete and calendar rows degrade with a clear reason

#### Scenario: Provider resilience — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
