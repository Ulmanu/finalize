# Spec delta — app/undo

## ADDED Requirements

### Requirement: Reversible mutation records

User-visible destructive or state-changing commands SHALL produce enough inverse information to
support undo within the active history window.

#### Scenario: Undoing a completion reopens the item

- **GIVEN** an open to-do completed through the command layer
- **WHEN** the completion is undone
- **THEN** the to-do returns to the open state through the reopen command
- **AND** no unrelated fields change

#### Scenario: Undoing a creation removes the item without losing identity

- **GIVEN** a to-do created through the command layer
- **WHEN** the creation is undone and then redone
- **THEN** the to-do ends up restored with its original identity
- **AND** the operations reuse command-layer validation rather than raw persistence writes

#### Scenario: Undoing an edit restores the previous value

- **GIVEN** a to-do whose title or schedule was changed
- **WHEN** the change is undone
- **THEN** the previous value is restored exactly

### Requirement: Grouped undo

Related multi-entity operations SHALL be undoable as a single group.

#### Scenario: A grouped operation undoes as one step

- **GIVEN** a group of commands affecting several entities
- **WHEN** the group is undone once
- **THEN** every command in the group is reverted in reverse order
- **AND** the history stack records a single undo entry for the group

#### Scenario: History is bounded

- **GIVEN** a history window of N groups
- **WHEN** more than N groups are recorded
- **THEN** the oldest groups are evicted and cannot be undone

### Requirement: Redo consistency

Undo and redo SHALL reuse domain commands/invariants rather than bypassing validation with raw
database snapshots.

#### Scenario: Redo restores the exact post-operation state

- **GIVEN** an operation that was undone
- **WHEN** it is redone
- **THEN** the state matches the original post-operation state
- **AND** every step passes through command validation and emits events
