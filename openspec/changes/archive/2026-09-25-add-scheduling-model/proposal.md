# Proposal

## Why

Schedule state decides which built-in list an item belongs to: Anytime, Someday, a specific
local date, Today, and This Evening. Membership must be derived from one canonical schedule so
no list ever duplicates an entity, and dates must behave as user-local calendar days.

## What Changes

- Add schedule-driven scopes: Today (date on or before the local today, This Evening included),
  Upcoming (future dates), Anytime, and Someday, in both read stores.
- Evaluate membership at an explicit UTC-offset local date so date boundaries are correct.
- Derive quick scheduling (Today/Tomorrow/This Evening) from the application clock instead of
  the wall clock so behavior is deterministic and consistent with reads.
- Replace the placeholder Today/Upcoming/Anytime/Someday pages with real projection lists.

## Capabilities

### New Capabilities

- `features/scheduling`: schedule states, transitions, and local-date semantics.

### Modified Capabilities

(none)

## Impact

- `crates/app` (to-do scopes, service), `crates/infra` (scope filters), `crates/testkit`,
  `crates/web` (projection pages, scheduling handlers).
- No schema change: schedules already persist as local `date` columns plus an evening flag.
- Non-goals: timezone preferences and localization (change 65), recurrence (34), deadlines (21).
