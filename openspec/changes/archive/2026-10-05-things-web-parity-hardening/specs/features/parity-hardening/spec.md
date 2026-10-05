# parity-hardening

## Purpose

Full audit against the parity matrix and elimination of remaining implementation, specification, and test gaps.

## ADDED Requirements

### Requirement: Parity matrix closure

The final hardening change SHALL audit every roadmap capability against the frozen parity matrix and classify unresolved items before closure.

#### Scenario: Parity matrix closure — primary behavior

- **GIVEN** the completed roadmap
- **WHEN** the parity matrix is generated
- **THEN** every capability lists status, canonical spec, and archived change, and every unresolved item is classified with rationale

#### Scenario: Parity matrix closure — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Zero known implementation gaps

The change SHALL resolve all known BUG/MISSING findings that are in scope for the web product before completion.

#### Scenario: Zero known implementation gaps — primary behavior

- **GIVEN** the audited repository
- **WHEN** markers, ignored tests, and spec coverage are checked
- **THEN** no in-scope capability lacks implementation or tests and no unresolved finding remains unclassified

#### Scenario: Zero known implementation gaps — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Release-quality validation

The complete workspace SHALL pass strict OpenSpec validation, Rust quality gates, behavior parity, visual parity, accessibility/security checks, and production build/deployment verification.

#### Scenario: Release-quality validation — primary behavior

- **GIVEN** the final workspace state
- **WHEN** the validation battery runs
- **THEN** strict spec/archive validation, quality gates, parity suites, the WASM build, and release binaries all pass

#### Scenario: Release-quality validation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption
