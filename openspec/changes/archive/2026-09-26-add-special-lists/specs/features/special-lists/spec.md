# special-lists

## Purpose

Tomorrow, Deadlines, Repeating, All Projects and Logged Projects.

## ADDED Requirements

### Requirement: Tomorrow view

The system SHALL expose a Tomorrow special view for items scheduled for the next local calendar date.

#### Scenario: Tomorrow view — primary behavior

- **GIVEN** items scheduled for today, tomorrow, and later dates
- **WHEN** Tomorrow is opened
- **THEN** eligible items for the next day are shown

#### Scenario: Tomorrow view — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deadlines and Repeating views

The system SHALL expose special views for open deadline-bearing items and recurrence templates.

#### Scenario: Deadlines and Repeating views — primary behavior

- **GIVEN** an account with deadline items and recurrence templates
- **WHEN** those special destinations are opened
- **THEN** items are ordered/grouped according to their capability semantics

#### Scenario: Deadlines and Repeating views — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Project special views

All Projects and Logged Projects SHALL expose open and logged/completed project sets respectively.

#### Scenario: Project special views — primary behavior

- **GIVEN** an account with open and completed projects
- **WHEN** a user opens either project special view
- **THEN** the correct project population appears without adding permanent sidebar membership

#### Scenario: Project special views — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
