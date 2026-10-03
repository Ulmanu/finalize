# Proposal

## Why

Settings need one typed registry covering every domain, with explicit scope per preference and
sanitized defaults so bad persisted data can never break startup.

## What Changes

- Add `momentum_app::preferences`: `PreferenceScope` (account/device/browser), `PreferenceDomain`
  (general, appearance, today, calendar, notifications, quick entry, logbook, account/sync, data),
  typed `PreferenceValue`/`PreferenceDefinition` registry with defaults and choice allow-lists,
  strict `parse_value`, and `sanitize` (invalid or unknown persisted values fall back to the
  typed default).
- Add a `PreferenceStore` port with a PostgreSQL implementation over the existing `preferences`
  table and a memory implementation for tests; `effective` resolves account-synced values.
- Extend Settings with a Preferences section rendering every domain, each row's scope, its
  current/default value, and a validated submission form; account-scoped values persist
  server-side, device/browser scopes are validated and acknowledged for the hydrated client to
  store locally.

## Capabilities

### New Capabilities

- `features/settings`: settings domains, preference scope, validation/defaults.

### Modified Capabilities

(none)

## Impact

- `crates/app` (preferences module + unit tests), `crates/infra` (PG store), `crates/web`
  (settings section, submission route, tests), `crates/testkit` (memory store). No schema change
  (`preferences` already exists with account/device/browser scope).
- Non-goals: live theme/density application in the hydrated client, per-device storage adapters.
