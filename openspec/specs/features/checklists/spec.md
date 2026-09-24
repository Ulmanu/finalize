# features/checklists Specification

## Purpose
Ordered embedded checklist editing, completion, paste expansion and reordering.

## Requirements

### Requirement: Checklist editing

A to-do SHALL support an ordered checklist whose items can be added, edited, completed, uncompleted, reordered, and removed.

#### Scenario: Checklist editing — primary behavior

- **GIVEN** a to-do owned by the account
- **WHEN** a checklist item is changed
- **THEN** the parent to-do remains the same canonical entity and checklist order/state updates

#### Scenario: Checklist editing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Bulk paste expansion

Pasting multi-line checklist content SHALL create one checklist item per non-empty parsed line using deterministic normalization.

#### Scenario: Bulk paste expansion — primary behavior

- **GIVEN** a to-do owned by the account
- **WHEN** multiple lines are pasted into checklist creation
- **THEN** the expected ordered checklist items are produced

#### Scenario: Bulk paste expansion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Template/copy semantics

Checklist state SHALL be compatible with recurrence templates so future copies can inherit structure without inheriting completion state improperly.

#### Scenario: Template/copy semantics — primary behavior

- **GIVEN** a to-do with checklist items in mixed completion states
- **WHEN** a recurring item copy is materialized
- **THEN** it receives checklist content with copy-appropriate completion state

#### Scenario: Template/copy semantics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
