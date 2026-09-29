# features/type-travel Specification

## Purpose
Typing outside editors immediately invokes navigation/search behavior.

## Requirements

### Requirement: Type travel

Printable typing outside editable controls SHALL transition immediately into destination/search navigation.

#### Scenario: Type travel — primary behavior

- **GIVEN** focus outside any editable control
- **WHEN** the user types a printable character
- **THEN** Quick Find opens seeded with that character

#### Scenario: Type travel — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Editor exemption

Typing inside note/title/checklist/date text inputs SHALL never trigger Type Travel.

#### Scenario: Editor exemption — primary behavior

- **GIVEN** focus inside a text input, textarea, select, or editable element
- **WHEN** the user types a printable character
- **THEN** the character is entered normally and no navigation occurs

#### Scenario: Editor exemption — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Escape/restore focus

Leaving Type Travel SHALL restore a sensible prior navigation focus target.

#### Scenario: Escape/restore focus — primary behavior

- **GIVEN** the Quick Find surface opened by typing
- **WHEN** the user presses Escape
- **THEN** the prior navigation destination is restored

#### Scenario: Escape/restore focus — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
