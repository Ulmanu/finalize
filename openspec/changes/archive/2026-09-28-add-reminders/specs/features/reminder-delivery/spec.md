# reminder-delivery

## Purpose

Reminder persistence, scheduler, browser Web Push and routing.

## ADDED Requirements

### Requirement: Reminder persistence

Reminder configuration SHALL persist local date/time, timezone context, owning entity, and delivery state separately from deadlines.

#### Scenario: Reminder persistence — primary behavior

- **GIVEN** an item with a date-based schedule
- **WHEN** a reminder is attached, cleared, or delivered
- **THEN** the stored reminder carries instant, timezone, owning entity, and delivery state independently of the deadline

#### Scenario: Reminder persistence — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reliable scheduler

The backend scheduler SHALL deliver due reminders idempotently and tolerate restarts without duplicate delivery storms.

#### Scenario: Reliable scheduler — primary behavior

- **GIVEN** a due reminder and a scheduler pass
- **WHEN** passes run again or the process restarts
- **THEN** the reminder is delivered exactly once and failures remain scheduled for retry

#### Scenario: Reliable scheduler — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Web Push delivery

Browsers with granted permission SHALL receive Web Push notifications that deep-link to the owning item.

#### Scenario: Web Push delivery — primary behavior

- **GIVEN** a registered push subscription for the owning account
- **WHEN** the scheduler delivers a due reminder
- **THEN** the notification payload deep-links to the owning item and is sent to the subscription endpoints

#### Scenario: Web Push delivery — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
