# recurrence-model

## Purpose

Regular and after-completion recurrence domain including end conditions.

## ADDED Requirements

### Requirement: Recurrence patterns

The domain SHALL support regular daily/weekly/monthly/yearly schedules and after-completion schedules for to-dos and projects.

#### Scenario: Recurrence patterns — primary behavior

- **GIVEN** a recurrence pattern and a base date
- **WHEN** the next occurrence is computed
- **THEN** regular patterns advance from the scheduled date and after-completion patterns advance from completion

#### Scenario: Recurrence patterns — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Pattern detail

Regular recurrence SHALL support selected weekdays/dates and optional end conditions where defined.

#### Scenario: Pattern detail — primary behavior

- **GIVEN** a weekly or monthly rule with selected days and an end condition
- **WHEN** occurrences are computed repeatedly
- **THEN** only selected days occur and the rule stops deterministically at its bound

#### Scenario: Pattern detail — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Template inheritance

A recurrence template SHALL carry the future-copy title, notes, tags, checklist/project structure, reminder/deadline rules, and context without sharing mutable copy state.

#### Scenario: Template inheritance — primary behavior

- **GIVEN** a recurrence template
- **WHEN** a copy is materialized for an occurrence
- **THEN** the copy carries the full structure as independent owned values

#### Scenario: Template inheritance — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
