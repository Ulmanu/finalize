# features/completion Specification

## Purpose
Distinct completed/canceled semantics, project handling and reopening.

## Requirements

### Requirement: Completed versus canceled

The domain SHALL distinguish completed and canceled terminal states.

#### Scenario: Completed versus canceled — primary behavior

- **GIVEN** an open to-do or project
- **WHEN** a user completes or cancels an item
- **THEN** the corresponding state and timestamp are recorded distinctly

#### Scenario: Completed versus canceled — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reopen

Completed or canceled items SHALL support reopening when allowed, restoring them to an open state without inventing unrelated dates.

#### Scenario: Reopen — primary behavior

- **GIVEN** a completed or canceled item
- **WHEN** a user reopens an item
- **THEN** terminal metadata is cleared according to policy and context is preserved

#### Scenario: Reopen — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Project completion safety

Completing/canceling a project with open children SHALL follow an explicit tested policy rather than silently losing child work.

#### Scenario: Project completion safety — primary behavior

- **GIVEN** a project with open direct and heading children
- **WHEN** a terminal action is requested on a non-empty active project
- **THEN** the defined confirmation/child transition behavior occurs atomically

#### Scenario: Project completion safety — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
