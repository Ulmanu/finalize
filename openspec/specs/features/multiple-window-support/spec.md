# features/multiple-window-support Specification

## Purpose
Popout list/project/area windows sharing synchronized local state.

## Requirements

### Requirement: Popout destinations

Lists, projects, and areas SHALL be openable in separate browser windows where browser policy permits.

#### Scenario: Popout destinations — primary behavior

- **GIVEN** a list, project, or area
- **WHEN** it is opened with the popout parameter
- **THEN** it renders without the full application chrome and offers a way back to the full app

#### Scenario: Popout destinations — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Shared state

Multiple app windows SHALL share synchronized local/server state via the same multi-tab coordination mechanisms.

#### Scenario: Shared state — primary behavior

- **GIVEN** two windows on the same account
- **WHEN** one window changes state
- **THEN** the other window reflects it through the shared server state and coordination mechanisms

#### Scenario: Shared state — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Window-safe commands

Commands in a popout SHALL not assume ownership of singleton UI state from the original window.

#### Scenario: Window-safe commands — primary behavior

- **GIVEN** a command issued from a popout window
- **WHEN** it executes
- **THEN** it behaves identically to the main window and does not depend on per-window singleton state

#### Scenario: Window-safe commands — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
