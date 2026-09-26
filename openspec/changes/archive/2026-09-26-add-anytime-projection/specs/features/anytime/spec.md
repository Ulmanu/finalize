# anytime

## Purpose

Correct currently-actionable projection and exclusions.

## ADDED Requirements

### Requirement: Actionable membership

Anytime SHALL include currently actionable open items that are not Someday and are not blocked by a future start date.

#### Scenario: Actionable membership — primary behavior

- **GIVEN** an open item with no future block
- **WHEN** an open item has no future block
- **THEN** it is eligible for Anytime

#### Scenario: Actionable membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Exclusions

Future-scheduled, Someday, completed, canceled, deleted, and otherwise inactive items SHALL be excluded from Anytime.

#### Scenario: Exclusions — primary behavior

- **GIVEN** items in active and inactive states
- **WHEN** an item's state becomes inactive
- **THEN** it disappears from Anytime without losing context

#### Scenario: Exclusions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Context grouping

Anytime MAY group by areas/projects but SHALL not create new parent relationships as a side effect of projection rendering.

#### Scenario: Context grouping — primary behavior

- **GIVEN** actionable items in the Inbox, an area, and a project
- **WHEN** Anytime is rendered
- **THEN** grouping reflects existing context only

#### Scenario: Context grouping — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
