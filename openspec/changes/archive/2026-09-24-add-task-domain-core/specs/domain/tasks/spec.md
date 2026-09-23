# Spec delta — domain/tasks

## ADDED Requirements

### Requirement: Canonical task model

The domain SHALL model Todo, Project, Area, Heading, ChecklistItem, Tag, schedule, deadline,
reminder, and status concepts without coupling them to UI or database frameworks.

#### Scenario: Entities carry canonical identity and metadata

- **GIVEN** a newly created to-do, project, area, heading, tag, or checklist item
- **WHEN** it is inspected
- **THEN** it has a stable identifier, creation/update instants, and an optimistic version
- **AND** it has no dependency on web, database, or provider types

#### Scenario: Checklist items belong to their to-do

- **GIVEN** a to-do with checklist items
- **WHEN** an item is completed or reordered in the domain model
- **THEN** only that to-do's checklist state changes
- **AND** the to-do's schedule, deadline, and reminder are unchanged

### Requirement: Distinct temporal concepts

Start/schedule state, deadline, and reminder SHALL be represented as separate domain concepts.

#### Scenario: Changing one temporal concept leaves the others intact

- **GIVEN** a to-do with a schedule date, a deadline, and a reminder
- **WHEN** any one of the three is changed
- **THEN** the other two retain their values

#### Scenario: Date-only values are local dates

- **GIVEN** a schedule date or deadline
- **WHEN** it is stored, compared, or serialized
- **THEN** it is a calendar date without a time or UTC-midnight instant
- **AND** deriving "today" from an instant uses the user's offset

#### Scenario: This Evening is a presentation state of a dated schedule

- **GIVEN** a to-do scheduled for a date
- **WHEN** it is marked as This Evening
- **THEN** the schedule state remains date-based with an evening flag
- **AND** clearing the date clears the evening flag

### Requirement: Project/context membership

A to-do SHALL have one canonical identity and a well-defined contextual parent while remaining
eligible for derived list projections.

#### Scenario: A to-do has exactly one contextual parent

- **GIVEN** a to-do in the Inbox
- **WHEN** it is moved to an area, project, or heading
- **THEN** its contextual parent changes to exactly one destination
- **AND** its identity and unrelated fields are preserved

#### Scenario: Status transitions are explicit

- **GIVEN** an open to-do
- **WHEN** it is completed, canceled, or reopened
- **THEN** the status and the corresponding timestamp change together
- **AND** completing a project with open children follows an explicit, tested policy
