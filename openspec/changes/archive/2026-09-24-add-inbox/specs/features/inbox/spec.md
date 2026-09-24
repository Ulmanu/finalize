# Spec delta — features/inbox

## ADDED Requirements

### Requirement: Inbox capture

A newly captured unfiled to-do SHALL appear in Inbox until it is assigned to another context or
otherwise processed.

#### Scenario: Capture lands in Inbox

- **GIVEN** an authenticated account
- **WHEN** a to-do is created without a destination
- **THEN** it appears in the Inbox list and in the Inbox badge count

#### Scenario: Processing removes it from Inbox

- **GIVEN** an Inbox to-do
- **WHEN** it is scheduled for a future date, moved to a project or area, completed, or deleted
- **THEN** it leaves the Inbox projection while its canonical identity is preserved

### Requirement: Inbox ordering and badge

Inbox SHALL support manual ordering and an accurate open-item count/badge.

#### Scenario: Manual reorder changes only this scope

- **GIVEN** several Inbox to-dos in a known order
- **WHEN** one is moved up or down
- **THEN** the Inbox order reflects the change
- **AND** Today ordering and other scopes are unaffected

#### Scenario: Badge tracks the open count

- **GIVEN** open Inbox to-dos
- **WHEN** items are captured, processed, completed, or deleted
- **THEN** the badge equals the number of open Inbox to-dos

### Requirement: Inbox processing

Inbox items SHALL support scheduling, tagging, assignment to project/area, completion, and
deletion through the shared command system.

#### Scenario: Scheduling from the list

- **GIVEN** an Inbox to-do
- **WHEN** the user schedules it for today, tomorrow, someday, or an explicit date
- **THEN** the command layer applies the schedule and the item leaves Inbox as appropriate

#### Scenario: Moving to a destination

- **GIVEN** an Inbox to-do
- **WHEN** the user assigns it to a project or area
- **THEN** its contextual parent changes and it leaves the Inbox
