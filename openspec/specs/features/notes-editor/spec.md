# features/notes-editor Specification

## Purpose
Multiline notes, links, selection, find/replace and editing behavior.

## Requirements

### Requirement: Rich plain-text notes editor

A to-do/project note SHALL support multiline editing, selection, clipboard operations, undo/redo, and link interaction.

#### Scenario: Rich plain-text notes editor — primary behavior

- **GIVEN** a to-do or project owned by the account
- **WHEN** a user edits a note
- **THEN** text changes persist without corrupting surrounding task metadata

#### Scenario: Rich plain-text notes editor — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Find and replace

The note editor SHALL support in-note find, next/previous, and replace operations for long notes.

#### Scenario: Find and replace — primary behavior

- **GIVEN** a note containing a search term
- **WHEN** a search term exists in the open note
- **THEN** matching ranges can be traversed and replaced deterministically

#### Scenario: Find and replace — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Safe link handling

Detected links SHALL open through safe browser navigation rules and SHALL not execute untrusted script URLs.

#### Scenario: Safe link handling — primary behavior

- **GIVEN** a note containing links
- **WHEN** a note contains a link
- **THEN** supported URLs are navigable and unsafe schemes are rejected

#### Scenario: Safe link handling — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
