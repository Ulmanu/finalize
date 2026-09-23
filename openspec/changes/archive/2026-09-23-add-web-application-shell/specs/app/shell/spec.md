# Spec delta — app/shell

## ADDED Requirements

### Requirement: Route-addressable application shell

The system SHALL expose stable routes for Inbox, Today, Upcoming, Anytime, Someday,
Logbook, areas, projects, and individual to-dos.

#### Scenario: Built-in list routes render the same destination on direct load

- **GIVEN** a signed-in account
- **WHEN** the user opens or refreshes `/inbox`, `/today`, `/upcoming`, `/anytime`, `/someday`, or `/logbook`
- **THEN** the same logical list destination renders from server-rendered content
- **AND** the active destination is identified in the primary navigation

#### Scenario: Entity routes are addressable

- **GIVEN** a signed-in account
- **WHEN** the user opens `/areas/:area_id`, `/projects/:project_id`, or `/todos/:todo_id`
- **THEN** the shell renders that destination's route without relying on transient client state

### Requirement: SSR and hydration continuity

The application shell SHALL render server-side and hydrate into interactive Rust/WASM behavior
without replacing canonical route or session state.

#### Scenario: Direct load of an authenticated route

- **GIVEN** a request for an application route
- **WHEN** the server renders the response
- **THEN** the response contains the destination's server-rendered markup and the session state used to render it
- **AND** the Rust/WASM client hydrates the same destination and session state without duplicating or losing it

#### Scenario: Unknown route is handled by the shell

- **WHEN** a request targets a path with no matching destination
- **THEN** the shell renders the not-found boundary
- **AND** the user can navigate back to a known destination

### Requirement: Application boundary handling

The shell SHALL provide explicit loading, not-found, unauthorized, and recoverable error
boundaries.

#### Scenario: Anonymous access to a protected route

- **GIVEN** no authenticated session
- **WHEN** the user requests `/today` or another protected destination
- **THEN** the unauthorized boundary renders with a link to the sign-in route
- **AND** no task data or application chrome is exposed

#### Scenario: Loading and error recovery

- **GIVEN** a route whose data dependency is still pending
- **WHEN** the route renders
- **THEN** the loading boundary exposes a polite status indicator
- **AND** when a rendering error occurs, the error boundary offers a recovery action and keeps navigation usable
