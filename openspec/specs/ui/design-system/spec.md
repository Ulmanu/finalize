# ui/design-system Specification

## Purpose

Define Momentum's shared visual language: centralized design tokens (typography, spacing,
radius, surface, shadow, icon, row, sidebar, motion), reusable Things-style primitives, an
original inline-SVG icon set, and reduced-motion-aware motion wrappers.

## Requirements

### Requirement: Central visual tokens

The UI SHALL define centralized typography, spacing, radius, surface, shadow, icon, row,
sidebar, and motion tokens instead of duplicating literal values across features.

#### Scenario: Primitives consume tokens

- **GIVEN** a rendered design-system primitive
- **WHEN** its markup and styles are inspected
- **THEN** spacing, radius, typography, surface, and motion values come from the central token
  set rather than feature-local literals

#### Scenario: Token changes propagate

- **GIVEN** a token value changed in the central token source
- **WHEN** the application is rebuilt
- **THEN** every primitive that consumes that token reflects the change without edits in
  feature code

### Requirement: Reusable Things-style primitives

The UI SHALL provide reusable task rows, project rows, area/sidebar rows, check controls,
progress rings, section headings, popovers, context menus, date controls, and add controls.

#### Scenario: Primitive catalog renders

- **WHEN** the style-guide route renders
- **THEN** every primitive is displayed with representative states
- **AND** each interactive primitive exposes accessible names, roles, and states

#### Scenario: Primitives are presentation-only

- **GIVEN** a primitive rendered with representative data
- **WHEN** the user activates an interactive control
- **THEN** the primitive reports the intent through its callback instead of mutating domain
  state or storage directly

### Requirement: Original asset policy

The product SHALL reproduce the intended visual structure with original branding and original
icons/assets rather than redistributing Cultured Code proprietary assets.

#### Scenario: Icons are original and local

- **WHEN** the repository and rendered pages are inspected
- **THEN** every icon is defined in Rust source in this repository
- **AND** no Cultured Code trademark, logo, marketing artwork, or proprietary icon file is present

#### Scenario: Motion respects reduced-motion preference

- **GIVEN** a user whose environment requests reduced motion
- **WHEN** primitives and motion wrappers render
- **THEN** non-essential transitions are removed or shortened without changing domain state
