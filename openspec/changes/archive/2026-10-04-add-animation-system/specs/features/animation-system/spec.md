# animation-system

## Purpose

Centralized insertion/removal/editor/popover/drag/navigation motion primitives.

## ADDED Requirements

### Requirement: Motion primitives

Insertion, removal/completion, editor expansion, popovers, drag lift/drop, and navigation transitions SHALL use centralized motion primitives.

#### Scenario: Motion primitives — primary behavior

- **GIVEN** the motion registry
- **WHEN** a component animates
- **THEN** it uses the declared primitive class and its design-token duration rather than ad-hoc values

#### Scenario: Motion primitives — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: State-preserving animation

Animations SHALL not alter domain state ordering or block command completion if interrupted.

#### Scenario: State-preserving animation — primary behavior

- **GIVEN** an interrupted or skipped animation
- **WHEN** commands run
- **THEN** domain state and ordering are unaffected and completion is never gated on motion

#### Scenario: State-preserving animation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reduced motion

Nonessential transitions SHALL be removed or shortened when reduced-motion preference is active.

#### Scenario: Reduced motion — primary behavior

- **GIVEN** reduced motion is active via the system or the explicit preference
- **WHEN** the UI renders
- **THEN** every nonessential primitive is suppressed

#### Scenario: Reduced motion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
