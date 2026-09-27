# features/movement Specification

## Purpose
One authoritative movement engine for tasks/projects/headings and destinations.

## Requirements

### Requirement: Authoritative destination validation

The move engine SHALL define valid destinations for to-dos, projects, and headings in one domain/application service.

#### Scenario: Authoritative destination validation — primary behavior

- **GIVEN** any move request
- **WHEN** any adapter requests a move
- **THEN** destination validation uses the same authoritative rules

#### Scenario: Authoritative destination validation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Move preservation

Moving an entity SHALL preserve data unrelated to membership/order/schedule unless the destination semantics require a change.

#### Scenario: Move preservation — primary behavior

- **GIVEN** a to-do with notes, tags, deadline, checklist, and a schedule
- **WHEN** a to-do moves between projects
- **THEN** title, notes, tags, deadline, checklist, and identity remain unchanged

#### Scenario: Move preservation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Adapter reuse

Drag-and-drop, Move dialog, keyboard commands, and automation API SHALL call the same move operation.

#### Scenario: Adapter reuse — primary behavior

- **GIVEN** the same move initiated through different surfaces
- **WHEN** the same move is initiated through different UI/API surfaces
- **THEN** the resulting domain state is equivalent

#### Scenario: Adapter reuse — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
