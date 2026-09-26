# features/upcoming Specification

## Purpose
Date-grouped future agenda including rescheduling behavior.

## Requirements

### Requirement: Date-grouped Upcoming

Upcoming SHALL group future scheduled items by local calendar date and include future recurrence/deadline/calendar projections as defined by their capabilities.

#### Scenario: Date-grouped Upcoming — primary behavior

- **GIVEN** open items scheduled on two future dates
- **WHEN** a future item exists
- **THEN** it appears under the correct future date

#### Scenario: Date-grouped Upcoming — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Drag-to-reschedule

Moving an eligible item from one Upcoming date to another SHALL change its schedule date through the command layer.

#### Scenario: Drag-to-reschedule — primary behavior

- **GIVEN** an item shown under a future Upcoming date
- **WHEN** a task is dropped on another date
- **THEN** the schedule is updated and the item moves to that day

#### Scenario: Drag-to-reschedule — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Projection-only calendar rows

External calendar events displayed in Upcoming SHALL remain read-only unless an integration explicitly supports editing.

#### Scenario: Projection-only calendar rows — primary behavior

- **GIVEN** an external calendar row in Upcoming
- **WHEN** a user attempts a task-only mutation on an external calendar event
- **THEN** the action is unavailable or rejected

#### Scenario: Projection-only calendar rows — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
