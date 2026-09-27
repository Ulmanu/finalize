# multi-selection

## Purpose

Range/toggle/select-all selection state plus bulk commands and multi-item movement.

## ADDED Requirements

### Requirement: Selection state machine

The UI SHALL support single, toggle, range, select-all, and clear selection semantics over selectable list rows.

#### Scenario: Selection state machine — primary behavior

- **GIVEN** a selectable list with an established anchor
- **WHEN** a user extends or toggles selection
- **THEN** the selected ID set and focus/anchor behave deterministically

#### Scenario: Selection state machine — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Bulk commands

Supported commands such as complete, cancel, schedule, deadline, move, tag, and delete SHALL operate on the selected set transactionally or with explicit per-item results.

#### Scenario: Bulk commands — primary behavior

- **GIVEN** a non-empty selection
- **WHEN** a bulk command runs
- **THEN** all eligible selected items are processed and failures are reported deterministically

#### Scenario: Bulk commands — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Selection stability

Selection SHALL survive non-destructive list rerenders where selected entities remain visible and valid.

#### Scenario: Selection stability — primary behavior

- **GIVEN** selected items that remain visible
- **WHEN** projection data refreshes
- **THEN** still-valid selected items remain selected

#### Scenario: Selection stability — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
