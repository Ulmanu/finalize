# Spec delta — features/projects

## ADDED Requirements

### Requirement: Project lifecycle

Users SHALL create, edit, schedule, deadline, complete, cancel, reopen, move, and delete
projects.

#### Scenario: Create, rename, and edit notes

- **GIVEN** an authenticated account
- **WHEN** the user creates a project, renames it, and edits its notes
- **THEN** the project keeps one canonical identity with the new title and notes

#### Scenario: Scheduling and deadlines are project-level

- **GIVEN** a project with direct child to-dos
- **WHEN** the project is scheduled for a date or given a deadline
- **THEN** the project's schedule/deadline change
- **AND** no child to-do values are copied or modified

#### Scenario: Completing with open children follows an explicit policy

- **GIVEN** a project with open direct children
- **WHEN** completion is requested with the reject policy
- **THEN** the command fails with a conflict and the project stays open
- **AND** when completion is requested with the cascade policy, children are completed and the
  project is completed

### Requirement: Project child ordering

Projects SHALL own an ordered sequence of headings and/or direct to-dos while keeping each
child's canonical identity.

#### Scenario: Direct children are ordered

- **GIVEN** a project with several direct to-dos
- **WHEN** the project detail renders
- **THEN** children appear in their project order
- **AND** each child keeps the same identity it has in its other projections

### Requirement: Project temporal participation

Projects SHALL participate in schedule/deadline/logbook projections without copying those
values to every child.

#### Scenario: Logbook and deadlines include projects

- **GIVEN** a completed project and a project with a deadline
- **WHEN** the corresponding projections are derived
- **THEN** the project appears in them once, by its own identity
- **AND** its children are not duplicated into the project's projection rows

### Requirement: Project deletion policy

Deleting a project SHALL require an explicit policy for direct children and SHALL never
silently orphan or destroy them.

#### Scenario: Detach policy preserves children

- **GIVEN** a project with open direct to-dos
- **WHEN** the project is deleted with the detach policy
- **THEN** children return to the Inbox and the project is soft-deleted

#### Scenario: Trash policy moves children to Trash

- **GIVEN** a project with open direct to-dos
- **WHEN** the project is deleted with the trash policy
- **THEN** the children are soft-deleted and the project is soft-deleted
