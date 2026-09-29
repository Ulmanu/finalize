# features/deep-search Specification

## Purpose
Search notes/checklists/logged content with ranking and normalization.

## Requirements

### Requirement: Deep content search

Continue/deep search SHALL include notes, checklist text, and logged/completed content in addition to normal Quick Find scope.

#### Scenario: Deep content search — primary behavior

- **GIVEN** content stored in notes, checklist items, or completed/logged entries
- **WHEN** a matching query is searched
- **THEN** those items are returned with an indication of the matched field

#### Scenario: Deep content search — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Normalized matching

Search SHALL be case-insensitive and diacritic-normalized while preserving original display text.

#### Scenario: Normalized matching — primary behavior

- **GIVEN** content containing diacritics or mixed case
- **WHEN** a folded query is searched
- **THEN** the content matches while the displayed title remains the original text

#### Scenario: Normalized matching — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Offline/server consistency

The local offline search index and server-backed search SHALL use compatible tokenization/ranking semantics sufficient to avoid surprising result loss.

#### Scenario: Offline/server consistency — primary behavior

- **GIVEN** a query evaluated offline and on the server
- **WHEN** both use the shared normalization and scoring semantics
- **THEN** results agree on matches and ordering tiers

#### Scenario: Offline/server consistency — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
