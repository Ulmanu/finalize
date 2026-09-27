# drag-and-drop

## Purpose

Reorder/move/date-reschedule behavior across desktop and touch input.

## ADDED Requirements

### Requirement: Pointer drag reorder

Users SHALL reorder supported lists with pointer/touch drag while receiving stable insertion feedback.

#### Scenario: Pointer drag reorder — primary behavior

- **GIVEN** a supported list with multiple items
- **WHEN** an item is dragged within a valid ordering scope
- **THEN** its rank updates to the indicated insertion point

#### Scenario: Pointer drag reorder — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Cross-context move

Dragging to a valid project/area/sidebar destination SHALL invoke the move engine rather than manipulating UI-only state.

#### Scenario: Cross-context move — primary behavior

- **GIVEN** an item and a valid destination
- **WHEN** an item is dropped on a valid destination
- **THEN** canonical context changes atomically

#### Scenario: Cross-context move — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Upcoming reschedule drag

Dragging an eligible task between Upcoming date sections SHALL schedule it to the target date.

#### Scenario: Upcoming reschedule drag — primary behavior

- **GIVEN** an item shown under an Upcoming date
- **WHEN** an item is dropped on a different day
- **THEN** the schedule command runs and contextual membership is preserved

#### Scenario: Upcoming reschedule drag — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
