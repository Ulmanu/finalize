# markdown

## Purpose

Things-style Markdown subset with syntax-visible rich rendering.

## ADDED Requirements

### Requirement: Supported Markdown subset

Notes SHALL render the selected Things-style Markdown subset including emphasis, strong text, highlight, headings, lists, quotes, code, and links.

#### Scenario: Supported Markdown subset — primary behavior

- **GIVEN** a note containing supported syntax
- **WHEN** a note contains supported syntax
- **THEN** the corresponding visual formatting is rendered

#### Scenario: Supported Markdown subset — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Syntax-visible editing

Markdown punctuation SHALL remain part of the editable text while formatting is visually represented.

#### Scenario: Syntax-visible editing — primary behavior

- **GIVEN** a note with formatted content
- **WHEN** a formatted note is edited
- **THEN** the raw Markdown syntax remains directly editable

#### Scenario: Syntax-visible editing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Lossless storage

Markdown rendering SHALL be derived from stored source text rather than storing a destructive rich-text representation.

#### Scenario: Lossless storage — primary behavior

- **GIVEN** a saved note
- **WHEN** the same note is loaded after save
- **THEN** the original Markdown source is preserved

#### Scenario: Lossless storage — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
