# scheduling

## Purpose

Anytime/Someday/on-date/Today/This Evening scheduling semantics.

## ADDED Requirements

### Requirement: Schedule states

A to-do/project SHALL support Anytime, Someday, Today/date-based scheduling, and This Evening as a Today presentation state.

#### Scenario: Schedule states — primary behavior

- **GIVEN** an account with scheduled and unscheduled items
- **WHEN** a schedule command is applied
- **THEN** projection membership changes according to schedule semantics

#### Scenario: Schedule states — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Schedule transitions

Moving among Anytime, Someday, Today, evening, and future dates SHALL preserve unrelated context and metadata.

#### Scenario: Schedule transitions — primary behavior

- **GIVEN** a to-do with context, tags, notes, and a deadline
- **WHEN** an item is rescheduled
- **THEN** parent project/area, tags, notes, and deadline are preserved

#### Scenario: Schedule transitions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Local-date semantics

Schedule dates SHALL be stored and compared as user-local dates rather than UTC midnight timestamps.

#### Scenario: Local-date semantics — primary behavior

- **GIVEN** an item scheduled for a calendar date
- **WHEN** the user's timezone crosses a UTC date boundary
- **THEN** the intended calendar date remains unchanged

#### Scenario: Local-date semantics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
