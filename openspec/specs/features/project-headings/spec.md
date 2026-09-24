# features/project-headings Specification

## Purpose
Create/edit/reorder/archive headings and move associated child tasks.

## Requirements

### Requirement: Heading lifecycle

Project headings SHALL support create, rename, reorder, archive/restore, and conversion into a
project where supported by the product model.

#### Scenario: Heading lifecycle — primary behavior

- **GIVEN** an open project owned by the account
- **WHEN** a heading command is executed
- **THEN** the containing project preserves deterministic child structure

#### Scenario: Heading lifecycle — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Heading child movement

Moving a heading SHALL move the heading together with the to-dos assigned under it.

#### Scenario: Heading child movement — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a heading is reordered or transferred within project structure
- **THEN** its assigned child group moves as one unit

#### Scenario: Heading child movement — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Heading scope

Headings SHALL exist only within projects and SHALL not become independent global sidebar containers.

#### Scenario: Heading scope — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a heading is referenced outside its project context
- **THEN** the domain rejects invalid standalone placement

#### Scenario: Heading scope — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
