# recurrence-exceptions

## Purpose

Early completion, one-time schedule overrides and future-rule editing.

## ADDED Requirements

### Requirement: Early completion

A scheduled recurrence copy/template SHALL support early completion according to defined recurrence semantics.

#### Scenario: Early completion — primary behavior

- **GIVEN** an open recurrence copy scheduled for a later date
- **WHEN** the copy is completed early
- **THEN** the next occurrence is anchored to the actual completion date

#### Scenario: Early completion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: One-time versus future edit

Rescheduling/editing a generated occurrence SHALL distinguish a one-time exception from modifying the recurrence template/future rule.

#### Scenario: One-time versus future edit — primary behavior

- **GIVEN** a generated occurrence
- **WHEN** the user edits that occurrence versus updating the rule
- **THEN** the occurrence edit changes only that copy while the rule update changes future generation

#### Scenario: One-time versus future edit — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: After-completion anchor

For after-completion recurrence, the next occurrence SHALL be anchored to the actual completion date/time according to the rule.

#### Scenario: After-completion anchor — primary behavior

- **GIVEN** an after-completion rule with an open copy
- **WHEN** that copy completes
- **THEN** the next occurrence is generated relative to the completion, one copy at a time

#### Scenario: After-completion anchor — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
