# logbook

## Purpose

Completion history, date grouping, canceled items, projects and logging behavior.

## ADDED Requirements

### Requirement: Logbook membership

Logbook SHALL derive completed/canceled entities from terminal state and logging policy.

#### Scenario: Logbook membership — primary behavior

- **GIVEN** an account with terminal to-dos and projects
- **WHEN** an eligible item is logged
- **THEN** it appears in Logbook grouped according to completion/cancellation date

#### Scenario: Logbook membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: History timestamps

Creation, completion, cancellation, and logging timestamps SHALL remain queryable after logging.

#### Scenario: History timestamps — primary behavior

- **GIVEN** a logged item
- **WHEN** a logged item is inspected
- **THEN** historical timestamps are available without reconstructing them from display order

#### Scenario: History timestamps — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Logging policy

Automatic versus manual logging behavior SHALL be controlled by preference while preserving completed state independently.

#### Scenario: Logging policy — primary behavior

- **GIVEN** an account logging policy
- **WHEN** the logging preference changes
- **THEN** completed items move to Logbook according to policy without changing completion timestamps

#### Scenario: Logging policy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
