# features/reminder-snoozing Specification

## Purpose
Snooze durations, tomorrow behavior and scheduler integration.

## Requirements

### Requirement: Snooze choices

Delivered reminders SHALL support configured snooze choices including short durations and next-day behavior.

#### Scenario: Snooze choices — primary behavior

- **GIVEN** a delivered reminder
- **WHEN** the user picks a configured snooze choice
- **THEN** the reminder is rescheduled to the resolved instant

#### Scenario: Snooze choices — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Snooze idempotency

Repeated clicks/retries on the same snooze action SHALL not create duplicate scheduled deliveries.

#### Scenario: Snooze idempotency — primary behavior

- **GIVEN** a reminder snoozed to an instant
- **WHEN** the same snooze action is repeated
- **THEN** the reminder keeps one delivery at that instant

#### Scenario: Snooze idempotency — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Tomorrow semantics

Snoozing to tomorrow SHALL resolve using the user's local timezone/date policy rather than adding a fixed 24 hours.

#### Scenario: Tomorrow semantics — primary behavior

- **GIVEN** a delivered reminder with a timezone
- **WHEN** the user snoozes to tomorrow
- **THEN** the new instant is the next local calendar date at the reminder's wall-clock time

#### Scenario: Tomorrow semantics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
