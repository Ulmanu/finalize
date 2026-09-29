# Proposal

## Why

External calendars belong in Today and Upcoming, but only through a provider-neutral contract so
no Google/Microsoft/CalDAV type leaks into core code, and calendar outages must never block task
data.

## What Changes

- Add `momentum_app::calendar`: `CalendarEvent` (read-only), `CalendarRange`, `CalendarProvider`
  trait, typed `CalendarError` (auth expiry, network, provider), and `CalendarService` projecting
  provider events into a cached, failure-tolerant `CalendarProjection { events, degraded, stale }`.
- Wire the projection into Today (new "Calendar" section with external read-only rows) and
  Upcoming (`UpcomingAgenda.calendar` plus a degraded banner), both rendering `data-external`
  rows with no edit controls.
- Provide `FlakyCalendarProvider` (failure injection wrapper) plus a testkit
  `FakeCalendarProvider`, and expose the service through `AppState::with_calendar`/context.

## Capabilities

### New Capabilities

- `features/calendar-integration`: provider-neutral API, read-only projection, provider
  resilience.

### Modified Capabilities

(none)

## Impact

- `crates/app` (calendar module, `CalendarRow::from_event`, agenda `with_calendar_rows`),
  `crates/web` (context/service, Today/Upcoming surfaces, tests), `crates/testkit` (fake +
  tests). No schema change.
- Non-goals: shipping a live Google/Microsoft/CalDAV network adapter or OAuth flow (the provider
  contract, resilience, and UI are complete and the adapters are deployment work), calendar
  write-back (events are read-only by design).
