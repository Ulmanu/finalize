# features/localization Specification

## Purpose

Define localization infrastructure: independent locale, date-format, week-start, hour-format, and
timezone settings with typed validation; date-only scheduling values that stay date-only while
reminder instants resolve through timezone/DST rules; and a parser locale selected independently
of storage representation.

## Requirements

### Requirement: Locale settings

The system SHALL separate UI language/locale, date formatting locale, first day of week, 12/24-hour format, and timezone configuration.

#### Scenario: Locale settings — primary behavior

- **GIVEN** the settings surface
- **WHEN** locale preferences are viewed or changed
- **THEN** each concern is configured independently with validation and defaults

#### Scenario: Locale settings — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Timezone-correct dates

Date-only scheduling/deadline values SHALL remain date-only while reminders/instants SHALL resolve with timezone/DST rules.

#### Scenario: Timezone-correct dates — primary behavior

- **GIVEN** a configured timezone
- **WHEN** reminders are created without an explicit timezone
- **THEN** the configured timezone is used and the instant is converted with DST rules while dates stay date-only

#### Scenario: Timezone-correct dates — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Parser locale integration

Natural-language date parsing SHALL select the configured parser locale independently from storage representation.

#### Scenario: Parser locale integration — primary behavior

- **GIVEN** a configured parser locale
- **WHEN** an ambiguous natural date is parsed
- **THEN** the configured locale determines the reading while stored dates remain locale-independent

#### Scenario: Parser locale integration — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
