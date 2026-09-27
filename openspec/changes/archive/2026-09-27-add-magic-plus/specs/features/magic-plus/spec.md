# magic-plus

## Purpose

Context-aware Things-style creation control and insertion positioning.

## ADDED Requirements

### Requirement: Contextual creation

Magic Plus SHALL create a to-do in the currently visible logical context by default.

#### Scenario: Contextual creation — primary behavior

- **GIVEN** a visible list context (Inbox, Today, project, or heading)
- **WHEN** the add control is activated normally
- **THEN** a new editable item appears at the expected insertion point/context

#### Scenario: Contextual creation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Drag insertion

Dragging the add control between rows SHALL create at the indicated rank/location.

#### Scenario: Drag insertion — primary behavior

- **GIVEN** a list with existing rows
- **WHEN** the control is dropped between existing items
- **THEN** the new item receives a rank between those neighbors

#### Scenario: Drag insertion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Alternate creation targets

Where supported, dragging the creation control toward heading/Inbox affordances SHALL create the appropriate entity/context through normal commands.

#### Scenario: Alternate creation targets — primary behavior

- **GIVEN** an alternate creation affordance (project/heading/Inbox)
- **WHEN** the control is dropped on a special creation target
- **THEN** the corresponding create command executes with deterministic context

#### Scenario: Alternate creation targets — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
