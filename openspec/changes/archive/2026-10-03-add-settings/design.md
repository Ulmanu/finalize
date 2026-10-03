# Design — add-settings

## Context

The `preferences` table already declares `scope in ('account','device','browser')`; the Logbook
policy is one such preference. This change generalizes that into a typed registry.

## Requirements Driving the Design

- `Settings domains`
- `Preference scope`
- `Validation/defaults`

## Architecture

- `PreferenceDefinition` carries key, title, domain, scope, typed default, and choices; tests
  enforce unique keys, valid defaults, and coverage of every domain. `parse_value` rejects
  unknown keys and invalid values; `sanitize` falls back to the default so persisted garbage is
  inert.
- `effective` reads account-scoped values from `PreferenceStore` (PG `preferences` table; memory
  in tests) and returns sanitized defaults for device/browser scopes.
- The Settings section renders the registry grouped by domain with scope badges and select-based
  forms; `POST /settings/preferences` validates, persists account-scoped values, and acknowledges
  device/browser values for client-side storage.

## Data and Persistence

- Uses the existing `preferences` table (no migration); values are stored as JSON strings with
  the declared scope.

## Offline and Sync

- Account-scoped preferences sync like any account data; device/browser scopes stay local.

## UI and Accessibility

- Labeled selects per preference with visible scope badges; invalid submissions redirect with a
  typed error state.

## Error Handling

- Unknown keys/values → `?error=preference` without writes; storage failures → same error state;
  missing store → error state; unknown persisted values never surface.

## Testing Strategy

- App unit tests for registry integrity, parsing rejection, and sanitize fallbacks; testkit store
  tests; web tests for the rendered domains/scopes, account persistence and display, invalid and
  unknown rejection, and browser-scope acknowledgement without server storage.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: device/browser scopes are validated server-side but stored by the hydrated client;
  the server only persists account-scoped values, matching the declared semantics.
