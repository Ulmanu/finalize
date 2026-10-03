# features/web-companion-views Specification

## Purpose
Compact Today/progress/quick-add views equivalent to native companion surfaces.

## Requirements

### Requirement: Compact Today view

The web app SHALL expose a compact Today presentation suitable for small surfaces while preserving canonical completion actions.

#### Scenario: Compact Today view — primary behavior

- **GIVEN** items scheduled for today
- **WHEN** the companion surface renders
- **THEN** each item is shown compactly with the canonical completion action available

#### Scenario: Compact Today view — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Progress companion

A compact progress presentation SHALL show derived Today completion state using the same progress calculation as the main app.

#### Scenario: Progress companion — primary behavior

- **GIVEN** a mix of completed and open items scheduled today
- **WHEN** the companion surface renders
- **THEN** the progress shown equals the main app's derived calculation

#### Scenario: Progress companion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Quick add companion

A compact quick-add surface SHALL create Inbox/current-context items through the normal command layer.

#### Scenario: Quick add companion — primary behavior

- **GIVEN** the companion surface
- **WHEN** a title is submitted
- **THEN** an Inbox item is created through the standard create command

#### Scenario: Quick add companion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
