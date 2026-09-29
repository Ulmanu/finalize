# mobile-gestures

## Purpose

Swipe, pull, long-press, drag and touch multi-selection state machines.

## ADDED Requirements

### Requirement: Swipe actions

Mobile pointer handling SHALL expose configured swipe actions such as When or selection without interfering with ordinary scrolling.

#### Scenario: Swipe actions — primary behavior

- **GIVEN** a horizontal swipe beyond the threshold
- **WHEN** the gesture completes quickly
- **THEN** the configured action for that direction is exposed

#### Scenario: Swipe actions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Long-press drag

Long press SHALL enter reorder/move mode where supported while suppressing accidental click activation.

#### Scenario: Long-press drag — primary behavior

- **GIVEN** a stationary press beyond the duration threshold
- **WHEN** drift stays within the slop
- **THEN** the editor/drag entry is exposed instead of a tap

#### Scenario: Long-press drag — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Pull-to-search

A supported downward gesture at the appropriate scroll boundary SHALL reveal Quick Find/navigation.

#### Scenario: Pull-to-search — primary behavior

- **GIVEN** the list is scrolled to the top
- **WHEN** a fast downward pull beyond the threshold occurs
- **THEN** Quick Find navigation opens

#### Scenario: Pull-to-search — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
