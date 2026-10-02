# mail-to-inbox

## Purpose

Secure inbound email address translating mail to Inbox tasks.

## ADDED Requirements

### Requirement: Private inbound address

Each enabled account SHALL receive a high-entropy inbound email alias/token that can be rotated or disabled.

#### Scenario: Private inbound address — primary behavior

- **GIVEN** an account
- **WHEN** the alias is requested, rotated, or disabled
- **THEN** a high-entropy token is issued, rotation invalidates the previous token, and disabled aliases reject mail

#### Scenario: Private inbound address — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Mail-to-task mapping

Inbound mail SHALL map subject to title and supported body text to notes using deterministic normalization.

#### Scenario: Mail-to-task mapping — primary behavior

- **GIVEN** an inbound message
- **WHEN** it is ingested
- **THEN** the subject becomes the title (with a deterministic fallback) and normalized body text becomes the notes

#### Scenario: Mail-to-task mapping — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Abuse and duplicate protection

Email ingress SHALL enforce size/rate limits and message-id/idempotency handling.

#### Scenario: Abuse and duplicate protection — primary behavior

- **GIVEN** repeated, oversized, or duplicate inbound messages
- **WHEN** they are ingested
- **THEN** duplicates are acknowledged without new items, oversized mail is rejected, and rate limits stop floods

#### Scenario: Abuse and duplicate protection — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
