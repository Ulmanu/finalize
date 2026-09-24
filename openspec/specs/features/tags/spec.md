# features/tags Specification

## Purpose
Tag CRUD, hierarchy, assignment, inherited context and multi-tag filtering.

## Requirements

### Requirement: Tag hierarchy

Users SHALL create, rename, delete, reorder, and nest tags.

#### Scenario: Tag hierarchy — primary behavior

- **GIVEN** an account with tags
- **WHEN** a nested tag hierarchy is edited
- **THEN** cycles are prevented and deterministic hierarchy is preserved

#### Scenario: Tag hierarchy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Tag assignment and filtering

Open entities SHALL support tag assignment/removal and list filtering by selected tags.

#### Scenario: Tag assignment and filtering — primary behavior

- **GIVEN** an account with tagged and untagged items
- **WHEN** a user filters a list by tags
- **THEN** only items satisfying the defined filter semantics are shown without changing entity membership

#### Scenario: Tag assignment and filtering — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Inherited context tags

Area/project tags SHALL be available as inherited filtering context without duplicating all inherited relationships onto every child entity.

#### Scenario: Inherited context tags — primary behavior

- **GIVEN** a project or area carrying a tag and a child to-do without that tag
- **WHEN** a child item is evaluated for a parent tag filter
- **THEN** the inherited tag contributes to matching without synthetic direct assignment

#### Scenario: Inherited context tags — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
