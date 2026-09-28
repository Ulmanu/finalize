# quick-find

## Purpose

Type-to-search navigation across entities, tags, lists and settings.

## ADDED Requirements

### Requirement: Type-to-open Quick Find

Typing outside an editor SHALL open Quick Find and seed it with the typed query.

#### Scenario: Type-to-open Quick Find — primary behavior

- **GIVEN** focus outside an editor
- **WHEN** the user starts typing
- **THEN** Quick Find opens seeded with the typed characters

#### Scenario: Type-to-open Quick Find — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Destination search

Quick Find SHALL search built-in lists, special lists, areas, projects, headings, tags, and to-do titles for navigation.

#### Scenario: Destination search — primary behavior

- **GIVEN** an account with lists, projects, headings, tags, and to-dos
- **WHEN** a query is searched
- **THEN** matching destinations from every surface are returned as navigable results

#### Scenario: Destination search — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Recent and exact navigation

Exact/high-confidence destinations and recent destinations SHALL receive deterministic ranking without hiding other matches.

#### Scenario: Recent and exact navigation — primary behavior

- **GIVEN** a query and previously opened destinations
- **WHEN** results are ranked
- **THEN** exact matches rank highest, recency breaks ties, and all other matches remain listed

#### Scenario: Recent and exact navigation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
