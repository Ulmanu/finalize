# features/command-urls Specification

## Purpose
Web equivalents for add/update/show/search automation URLs.

## Requirements

### Requirement: Command URL surface

The app SHALL expose documented URL actions for add, add-project, update, show, and search equivalents.

#### Scenario: Command URL surface — primary behavior

- **GIVEN** a documented command URL with valid parameters
- **WHEN** it is invoked by automation or a person
- **THEN** the corresponding action executes through the standard application services

#### Scenario: Command URL surface — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Safe parameter parsing

Command URL parameters SHALL use strict typed parsing and reject unauthorized entity references or unsupported values.

#### Scenario: Safe parameter parsing — primary behavior

- **GIVEN** malformed or unsupported command URL parameters
- **WHEN** the URL is invoked
- **THEN** the request is rejected deterministically and no partial write occurs

#### Scenario: Safe parameter parsing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Shareable navigation URLs

Non-mutating show/search URLs SHALL remain bookmarkable and stable across sessions.

#### Scenario: Shareable navigation URLs — primary behavior

- **GIVEN** a show or search URL
- **WHEN** it is revisited in another session
- **THEN** it resolves to the same canonical destination without mutating data

#### Scenario: Shareable navigation URLs — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
