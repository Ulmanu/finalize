# Spec delta — features/todo-crud

## ADDED Requirements

### Requirement: Todo lifecycle

Users SHALL be able to create, edit title/notes, duplicate, move, complete, cancel, reopen,
and delete a to-do.

#### Scenario: Create from the list

- **GIVEN** an authenticated account viewing a list
- **WHEN** the user submits a new to-do
- **THEN** a canonical to-do is created through the command layer and appears in the list

#### Scenario: Lifecycle actions from the list and editor

- **GIVEN** an existing to-do
- **WHEN** the user completes, cancels, reopens, deletes, or restores it
- **THEN** the action executes through the command layer and the list reflects the new state

#### Scenario: Duplicate creates a new canonical identity

- **GIVEN** an existing to-do with schedule, deadline, reminder, and notes
- **WHEN** the user duplicates it
- **THEN** a new to-do with a new identity carries the same metadata
- **AND** the original is unchanged

### Requirement: Metadata preservation

Operations that do not semantically alter schedule, deadline, tags, checklist, or context SHALL
preserve those fields.

#### Scenario: Title edit preserves metadata

- **GIVEN** a to-do with schedule, deadline, and reminder
- **WHEN** the title is edited
- **THEN** schedule, deadline, reminder, context, and notes are unchanged

#### Scenario: Completion preserves metadata

- **GIVEN** a to-do with schedule and deadline
- **WHEN** it is completed and later reopened
- **THEN** schedule and deadline are unchanged

### Requirement: Optimistic concurrency

Todo updates SHALL detect stale versions instead of silently overwriting newer state.

#### Scenario: Stale update is rejected

- **GIVEN** a to-do whose stored version advanced
- **WHEN** an update carries the stale version
- **THEN** the command fails with a conflict error
- **AND** the stored to-do is unchanged
