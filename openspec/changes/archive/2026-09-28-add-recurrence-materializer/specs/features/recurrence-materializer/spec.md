# recurrence-materializer

## Purpose

Concurrent-safe, idempotent generation of recurrence instances.

## ADDED Requirements

### Requirement: Idempotent materialization

For a given template and logical occurrence, at most one generated copy SHALL exist.

#### Scenario: Idempotent materialization — primary behavior

- **GIVEN** a template and an occurrence
- **WHEN** materialization runs once or repeatedly, including concurrently
- **THEN** exactly one copy exists for that occurrence and later attempts are no-ops

#### Scenario: Idempotent materialization — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Horizon generation

The materializer SHALL generate the required upcoming occurrences far enough for Upcoming/reminder behavior without unbounded eager generation.

#### Scenario: Horizon generation — primary behavior

- **GIVEN** an active template
- **WHEN** the materializer runs
- **THEN** occurrences up to the bounded horizon are generated and no further

#### Scenario: Horizon generation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Restart safety

Materialization progress SHALL be recoverable from persisted recurrence/template state rather than in-memory scheduler state.

#### Scenario: Restart safety — primary behavior

- **GIVEN** completed materialization
- **WHEN** the process restarts and runs again
- **THEN** progress resumes from persisted state without duplicates or replays

#### Scenario: Restart safety — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
