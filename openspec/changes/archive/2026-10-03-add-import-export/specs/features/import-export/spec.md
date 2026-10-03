# import-export

## Purpose

Versioned JSON backups plus CSV/Markdown/import adapters.

## ADDED Requirements

### Requirement: Versioned backup export

The system SHALL export a versioned, documented JSON backup independent of raw database table layout.

#### Scenario: Versioned backup export — primary behavior

- **GIVEN** an account with areas, projects, tags, and to-dos
- **WHEN** the backup endpoint is requested
- **THEN** a versioned document with plain records is returned

#### Scenario: Versioned backup export — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Human-readable export

Supported lists/projects/items SHALL export to CSV/Markdown formats with deterministic field mapping.

#### Scenario: Human-readable export — primary behavior

- **GIVEN** existing to-dos
- **WHEN** CSV or Markdown export is requested
- **THEN** the output uses stable columns/rows and quoted fields

#### Scenario: Human-readable export — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Import validation

Imports SHALL validate schema/content, report unsupported records, and avoid partial destructive replacement unless explicitly chosen.

#### Scenario: Import validation — primary behavior

- **GIVEN** a backup document
- **WHEN** it is imported
- **THEN** the version and every record are validated first, unsupported records are reported, and existing data is never replaced

#### Scenario: Import validation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
