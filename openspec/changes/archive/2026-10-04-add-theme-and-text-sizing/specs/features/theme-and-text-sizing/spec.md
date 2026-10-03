# theme-and-text-sizing

## Purpose

Light/dark/system themes and accessibility-aware text sizing for the application surface.

## ADDED Requirements

### Requirement: Theme choice

Users SHALL choose Light, Dark, or System theme, with System following browser/OS preference.

#### Scenario: Theme choice — primary behavior

- **GIVEN** a theme preference
- **WHEN** the application renders
- **THEN** explicit light/dark themes apply their token sets and System follows the OS preference

#### Scenario: Theme choice — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Text size

Users SHALL adjust application text size within supported bounds without breaking layout or accessibility.

#### Scenario: Text size — primary behavior

- **GIVEN** a text-size preference within the supported bounds
- **WHEN** it is applied
- **THEN** the root scale changes and rem-based layout tokens scale without structural breakage

#### Scenario: Text size — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Contrast and reduced motion compatibility

Theme/text settings SHALL remain compatible with accessibility contrast and reduced-motion behavior.

#### Scenario: Contrast and reduced motion compatibility — primary behavior

- **GIVEN** a user with high-contrast or reduced-motion preferences
- **WHEN** themes or text size change
- **THEN** contrast strengthening and motion suppression remain in effect

#### Scenario: Contrast and reduced motion compatibility — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
