# features/quick-entry Specification

## Purpose
Keyboard-invoked capture overlay that saves without leaving current context.

## Requirements

### Requirement: Quick Entry overlay

A keyboard-accessible overlay SHALL capture a new to-do without navigating away from the current destination.

#### Scenario: Quick Entry overlay — primary behavior

- **GIVEN** any destination in the application
- **WHEN** Quick Entry is invoked
- **THEN** an overlay capture form opens with the destination retained as the save target

#### Scenario: Quick Entry overlay — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Full capture metadata

Quick Entry SHALL support title, notes, schedule, deadline, tags, destination, checklist, and other supported capture metadata.

#### Scenario: Full capture metadata — primary behavior

- **GIVEN** a completed Quick Entry form
- **WHEN** it is saved
- **THEN** every provided metadata field is applied to the created item

#### Scenario: Full capture metadata — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Dismiss/save semantics

Save and cancel SHALL have deterministic keyboard behavior and never leave a half-created canonical item.

#### Scenario: Dismiss/save semantics — primary behavior

- **GIVEN** an open Quick Entry overlay
- **WHEN** the user saves or cancels, or input is invalid
- **THEN** save commits deterministically, cancel performs no write, and failures leave no half-created item

#### Scenario: Dismiss/save semantics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
