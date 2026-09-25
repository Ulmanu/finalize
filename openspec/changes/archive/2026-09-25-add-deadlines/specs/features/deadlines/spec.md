# deadlines

## Purpose

Independent deadline model, overdue/due-today/future states and sorting.

## ADDED Requirements

### Requirement: Independent deadline

A deadline SHALL be independent from start/schedule date and reminder.

#### Scenario: Independent deadline — primary behavior

- **GIVEN** a to-do with a schedule and reminder
- **WHEN** a deadline is added or cleared
- **THEN** the start schedule is unchanged

#### Scenario: Independent deadline — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deadline states

The system SHALL derive future, due-today, and overdue states from the user's local date.

#### Scenario: Deadline states — primary behavior

- **GIVEN** an open item with a stored deadline
- **WHEN** the local date advances past an open item's deadline
- **THEN** the item becomes overdue without mutating the stored deadline

#### Scenario: Deadline states — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deadline projections

Deadline metadata SHALL be queryable for deadline-specific views and visible in other applicable views without duplicating the entity.

#### Scenario: Deadline projections — primary behavior

- **GIVEN** an item with a deadline
- **WHEN** an item with a deadline appears in multiple projections
- **THEN** all representations reference the same deadline value

#### Scenario: Deadline projections — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
