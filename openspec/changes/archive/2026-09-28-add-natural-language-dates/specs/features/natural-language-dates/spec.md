# natural-language-dates

## Purpose

Deterministic Rust date parser with tested supported expressions.

## ADDED Requirements

### Requirement: Deterministic natural-date parsing

The Rust parser SHALL recognize supported relative dates, weekdays, month/day expressions, offsets, and reminder times without network or LLM dependence.

#### Scenario: Deterministic natural-date parsing — primary behavior

- **GIVEN** a supported expression and a base date
- **WHEN** the parser runs
- **THEN** it returns the same calendar date (and reminder time where present) every time

#### Scenario: Deterministic natural-date parsing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Locale-aware parsing

Parsing SHALL be modular by locale and SHALL respect configured regional date conventions where ambiguity exists.

#### Scenario: Locale-aware parsing — primary behavior

- **GIVEN** an ambiguous numeric date
- **WHEN** a locale is configured
- **THEN** the day/month order follows that locale's convention

#### Scenario: Locale-aware parsing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Ambiguity handling

Ambiguous input SHALL produce explicit candidates or a recoverable parse failure rather than silently choosing an unsafe date.

#### Scenario: Ambiguity handling — primary behavior

- **GIVEN** an ambiguous numeric expression and no locale
- **WHEN** the parser runs
- **THEN** it reports both candidate dates as an error and schedules nothing

#### Scenario: Ambiguity handling — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
