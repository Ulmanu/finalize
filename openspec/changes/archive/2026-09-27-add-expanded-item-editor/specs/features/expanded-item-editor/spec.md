# expanded-item-editor

## Purpose

Smooth row-to-expanded editor with title, notes and optional metadata controls.

## ADDED Requirements

### Requirement: Inline expansion

A task row SHALL expand into an editor without losing list position, selection context, or canonical identity.

#### Scenario: Inline expansion — primary behavior

- **GIVEN** a visible task row in a list
- **WHEN** a user opens the row editor
- **THEN** the row transitions into an editable detail surface

#### Scenario: Inline expansion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Progressive fields

Title and notes SHALL remain primary while schedule, deadline, tags, checklist, and recurrence controls are accessible without permanently cluttering the closed row.

#### Scenario: Progressive fields — primary behavior

- **GIVEN** an expanded editor surface
- **WHEN** the user looks for metadata controls
- **THEN** schedule, deadline, tag, checklist, and recurrence affordances are reachable

#### Scenario: Progressive fields — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Commit or cancel editing

Editor changes SHALL have deterministic persistence/optimistic behavior and keyboard/pointer dismissal semantics.

#### Scenario: Commit or cancel editing — primary behavior

- **GIVEN** an editor with pending changes
- **WHEN** the user commits or dismisses the editor
- **THEN** commit persists deterministically and dismissal performs no write

#### Scenario: Commit or cancel editing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
