# someday

## Purpose

Deferred/inactive tasks and projects with correct transitions.

## ADDED Requirements

### Requirement: Someday membership

Someday SHALL contain open items/projects intentionally deferred into the Someday schedule state.

#### Scenario: Someday membership — primary behavior

- **GIVEN** an item deferred to the Someday schedule state
- **WHEN** an item is deferred to Someday
- **THEN** it leaves actionable date views and appears in Someday

#### Scenario: Someday membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Someday reactivation

A Someday item SHALL become actionable only through an explicit schedule/context action that changes its schedule state.

#### Scenario: Someday reactivation — primary behavior

- **GIVEN** a Someday item
- **WHEN** a Someday item is assigned Today/Anytime/future date
- **THEN** projection membership updates accordingly

#### Scenario: Someday reactivation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Parent/child independence

A parent being active SHALL not implicitly reactivate a child explicitly scheduled Someday.

#### Scenario: Parent/child independence — primary behavior

- **GIVEN** an active project containing a Someday to-do
- **WHEN** an active project contains a Someday to-do
- **THEN** the child remains excluded from actionable projections

#### Scenario: Parent/child independence — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
