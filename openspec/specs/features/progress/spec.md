# features/progress Specification

## Purpose
Derived project progress rings and Today completion progress.

## Requirements

### Requirement: Project progress

Project progress SHALL be derived from eligible child completion rather than stored as an independently editable percentage.

#### Scenario: Project progress — primary behavior

- **GIVEN** a project with open and completed children
- **WHEN** a child task completion changes
- **THEN** the project progress indicator recalculates

#### Scenario: Project progress — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Today progress

Today SHALL expose a completion progress indicator based on the defined set of completable Today items.

#### Scenario: Today progress — primary behavior

- **GIVEN** completable Today items
- **WHEN** a Today item is completed or reopened
- **THEN** the Today progress indicator updates

#### Scenario: Today progress — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Edge-case policy

Zero-item, canceled-item, and non-completable projected rows SHALL have explicit inclusion/exclusion rules for progress calculations.

#### Scenario: Edge-case policy — primary behavior

- **GIVEN** a project or list whose rows are all excluded or absent
- **WHEN** a project/list has only excluded rows
- **THEN** the progress UI renders the defined neutral state

#### Scenario: Edge-case policy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
