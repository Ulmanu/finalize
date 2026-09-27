# date-pickers

## Purpose

When/deadline/reminder date interfaces and quick destinations.

## ADDED Requirements

### Requirement: When picker

The date UI SHALL provide Today, This Evening, Tomorrow, Anytime, Someday, explicit date, and clear actions where valid.

#### Scenario: When picker — primary behavior

- **GIVEN** an item without a schedule
- **WHEN** the user picks a quick destination or an explicit date
- **THEN** the schedule state updates accordingly

#### Scenario: When picker — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deadline picker

The deadline UI SHALL select or clear an explicit deadline independently from schedule/reminder.

#### Scenario: Deadline picker — primary behavior

- **GIVEN** an item with any schedule
- **WHEN** the user sets or clears a deadline
- **THEN** only the deadline changes

#### Scenario: Deadline picker — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reminder time picker

A reminder time SHALL be attachable only under valid start-date semantics and represented in the user's timezone.

#### Scenario: Reminder time picker — primary behavior

- **GIVEN** an item with a date-based schedule and a selected local time/timezone
- **WHEN** the reminder is committed
- **THEN** the stored reminder carries the converted instant plus the timezone,
- **AND** items without a date-based schedule reject attachment

#### Scenario: Reminder time picker — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
