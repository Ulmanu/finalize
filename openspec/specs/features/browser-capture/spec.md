# features/browser-capture Specification

## Purpose
PWA/share/browser capture of title, URL and selected text.

## Requirements

### Requirement: Web share capture

The PWA/browser integration SHALL accept shared page title, URL, and selected text where browser capabilities provide them.

#### Scenario: Web share capture — primary behavior

- **GIVEN** a share target invocation with title, URL, and/or text
- **WHEN** the capture endpoint receives it
- **THEN** an Inbox to-do is created carrying the shared data

#### Scenario: Web share capture — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Extension/bookmark adapter boundary

Optional browser-extension/bookmark capture SHALL call documented command URLs/API rather than duplicate domain logic.

#### Scenario: Extension/bookmark adapter boundary — primary behavior

- **GIVEN** an optional adapter
- **WHEN** it captures a page
- **THEN** it calls the documented capture URL and all validation/sanitization/creation happens server-side

#### Scenario: Extension/bookmark adapter boundary — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Sanitized external text

Captured third-party page content SHALL be treated as untrusted text and sanitized for display/navigation.

#### Scenario: Sanitized external text — primary behavior

- **GIVEN** shared content containing control characters or non-http(s) URLs
- **WHEN** the capture is stored
- **THEN** controls are stripped, unsafe URLs are dropped, lengths are bounded, and content is only ever displayed as escaped text

#### Scenario: Sanitized external text — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
