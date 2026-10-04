# features/large-list-performance Specification

## Purpose

Define large-account performance work: declared measurable budgets for initial render, scroll,
search, reorder, sync apply, and replica startup; windowed/bounded list reads that preserve
keyboard, selection, and ordering semantics; and composite indexes matching list and change-log
query order so thousands to tens of thousands of items stay within budget.

## Requirements

### Requirement: Performance budgets

The project SHALL define measurable budgets for initial render, scroll responsiveness, search, reorder, sync apply, and local-replica startup across large datasets.

#### Scenario: Performance budgets — primary behavior

- **GIVEN** a declared budget and dataset size
- **WHEN** the corresponding operation is benchmarked
- **THEN** it meets the target (with a documented debug factor in debug builds)

#### Scenario: Performance budgets — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: List virtualization

Large projections SHALL virtualize rendering where needed without breaking keyboard navigation, selection, or drag/reorder semantics.

#### Scenario: List virtualization — primary behavior

- **GIVEN** a large projection
- **WHEN** it is read or rendered
- **THEN** reads are windowed/bounded while keyboard, selection, and ordering semantics are unchanged

#### Scenario: List virtualization — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Query and index tuning

Server and local queries SHALL use indexes/batching appropriate for large accounts.

#### Scenario: Query and index tuning — primary behavior

- **GIVEN** a large account
- **WHEN** list and change-log queries run
- **THEN** composite indexes matching their order exist and queries stay within budgets

#### Scenario: Query and index tuning — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
