# features/settings Specification

## Purpose

Define the typed settings registry: preferences across general, appearance, Today, calendar,
notifications, Quick Entry/keyboard, Logbook, account/sync/integrations, and data domains, each
declaring an account-synced, device-local, or browser-local scope, with typed defaults and
validation so unknown or invalid persisted values can never break startup.

## Requirements

### Requirement: Settings domains

The app SHALL provide settings for general behavior, appearance, Today, calendar, notifications, Quick Entry/keyboard, Logbook, account/sync/integrations, and data controls as capabilities become available.

#### Scenario: Settings domains — primary behavior

- **GIVEN** the settings surface
- **WHEN** preferences are rendered
- **THEN** every declared domain appears with its preferences and current/default values

#### Scenario: Settings domains — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Preference scope

Each preference SHALL declare whether it is account-synced, device-local, or browser-local.

#### Scenario: Preference scope — primary behavior

- **GIVEN** a preference definition
- **WHEN** it is displayed or persisted
- **THEN** its declared scope determines where it is stored and shown

#### Scenario: Preference scope — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Validation and defaults

Settings SHALL have typed defaults and validation so unknown/invalid persisted values cannot break app startup.

#### Scenario: Validation and defaults — primary behavior

- **GIVEN** a missing, unknown, or invalid stored value
- **WHEN** the effective value is resolved
- **THEN** the typed default is used and invalid submissions are rejected without writes

#### Scenario: Validation and defaults — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
