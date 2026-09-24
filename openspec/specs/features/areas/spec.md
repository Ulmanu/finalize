# features/areas Specification

## Purpose

Define area management: creating, renaming, reordering, and deleting ongoing responsibilities
that group projects and loose to-dos, with an explicit, non-destructive-by-default deletion
policy for contained work.

## Requirements

### Requirement: Area lifecycle

Users SHALL create, rename, reorder, tag, and delete Areas representing ongoing
responsibilities.

#### Scenario: Creating and renaming an area

- **GIVEN** an authenticated account
- **WHEN** the user creates an area and later renames it
- **THEN** the area exists with a canonical identity and the new title
- **AND** its ordering rank is preserved

#### Scenario: Listing areas with counts

- **GIVEN** areas containing projects and loose to-dos
- **WHEN** the areas list renders
- **THEN** each area shows its project and loose to-do counts

### Requirement: Area contents

An Area SHALL contain projects and loose to-dos without forcing loose items into synthetic
projects.

#### Scenario: Loose to-dos stay loose

- **GIVEN** a to-do moved directly to an area
- **WHEN** the area detail renders
- **THEN** the to-do is listed as a loose to-do with its canonical identity
- **AND** no project entity is created

#### Scenario: Projects group under the area

- **GIVEN** a project assigned to an area
- **WHEN** the area detail renders
- **THEN** the project is listed under the area

### Requirement: Area deletion policy

Deleting an Area SHALL require an explicit deterministic policy for contained projects/to-dos
and SHALL never silently orphan or destroy them.

#### Scenario: Detach policy preserves contents

- **GIVEN** an area with projects and loose to-dos
- **WHEN** the user deletes the area with the detach policy
- **THEN** contained projects lose the area assignment
- **AND** contained loose to-dos return to the Inbox
- **AND** no contained entity is deleted

#### Scenario: Trash policy moves contents to Trash

- **GIVEN** an area with open contents
- **WHEN** the user deletes the area with the trash-contents policy
- **THEN** contained to-dos are moved to Trash
- **AND** contained projects are canceled/trashed according to their lifecycle
