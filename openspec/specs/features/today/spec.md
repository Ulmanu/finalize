# features/today Specification

## Purpose
Derived Today view, This Evening, manual ordering and correct membership.

## Requirements

### Requirement: Today membership

Today SHALL be a derived projection over eligible open items scheduled for today or otherwise required by documented Today semantics.

#### Scenario: Today membership — primary behavior

- **GIVEN** open items scheduled for today, overdue, or this evening and items in projects
- **WHEN** the local day begins or an item is scheduled for today
- **THEN** the item is included without being moved out of its project/context

#### Scenario: Today membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: This Evening section

Today SHALL support a distinct This Evening section with independent presentation ordering.

#### Scenario: This Evening section — primary behavior

- **GIVEN** a Today item
- **WHEN** a Today item is moved to This Evening
- **THEN** it remains a Today item while rendering in the evening section

#### Scenario: This Evening section — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Today manual ordering

Users SHALL be able to reorder Today independently from project/area ordering.

#### Scenario: Today manual ordering — primary behavior

- **GIVEN** a Today item also present in a contextual list
- **WHEN** a Today item is dragged
- **THEN** Today rank changes while contextual rank remains stable

#### Scenario: Today manual ordering — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
