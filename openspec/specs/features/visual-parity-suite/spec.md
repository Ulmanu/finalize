# features/visual-parity-suite Specification

## Purpose
Fixed-viewport screenshot baselines and Rust-driven visual regression checks for the server-rendered surface.

## Requirements

### Requirement: Fixed visual baselines

The test suite SHALL capture approved visual baselines for representative desktop/tablet/mobile states at fixed viewports.

#### Scenario: Fixed visual baselines — primary behavior

- **GIVEN** representative pages and fixed viewports
- **WHEN** the parity suite runs
- **THEN** each fixture is compared against its committed baseline snapshot

#### Scenario: Fixed visual baselines — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deterministic screenshot fixtures

Visual tests SHALL use deterministic seeded data, fonts/assets, time, theme, and animation suppression.

#### Scenario: Deterministic screenshot fixtures — primary behavior

- **GIVEN** seeded data and a fixed clock
- **WHEN** a snapshot is captured twice
- **THEN** the normalized output is byte-identical

#### Scenario: Deterministic screenshot fixtures — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Diff reporting

Visual regression failures SHALL produce inspectable expected/actual/diff artifacts and configurable thresholds.

#### Scenario: Diff reporting — primary behavior

- **GIVEN** a snapshot that differs from its baseline beyond the threshold
- **WHEN** the suite fails
- **THEN** expected/actual/diff artifacts are written and the ratio is reported

#### Scenario: Diff reporting — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
