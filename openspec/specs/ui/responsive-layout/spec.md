# ui/responsive-layout Specification

## Purpose

Define Momentum's explicit responsive application shells: persistent desktop navigation, a
collapsible overlay drawer for tablet, and a single-column mobile model with a bottom
destination bar, using documented breakpoints and touch-sized controls.

## Requirements

### Requirement: Desktop layout

The application SHALL provide a desktop layout with persistent navigation and content suitable
for wide screens.

#### Scenario: Wide viewport shows persistent navigation

- **GIVEN** a viewport at or above the desktop breakpoint
- **WHEN** an authenticated destination renders
- **THEN** the primary navigation is visible without a toggle
- **AND** the content region renders beside it without overlap

### Requirement: Tablet layout

The application SHALL provide a tablet layout that preserves full functionality while allowing
navigation to collapse or overlay.

#### Scenario: Tablet viewport uses an overlay drawer

- **GIVEN** a viewport between the tablet and desktop breakpoints
- **WHEN** the user activates the navigation toggle
- **THEN** the navigation drawer opens as an overlay with `aria-expanded` reflecting its state
- **AND** every destination remains reachable and the content region stays usable when closed

### Requirement: Mobile layout

The application SHALL provide a single-column mobile navigation model with touch-sized controls
and full feature reachability.

#### Scenario: Mobile viewport exposes a bottom destination bar

- **GIVEN** a viewport below the tablet breakpoint
- **WHEN** an authenticated destination renders
- **THEN** a single-column layout renders with a bottom destination bar
- **AND** secondary destinations are reachable through the drawer
- **AND** interactive controls meet the touch target token
