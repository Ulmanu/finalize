# Proposal

## Why

Someday is the deliberate "not now" state. Deferred items must leave every actionable view and
return only when the user explicitly reschedules them, even when their parent project is active.

## What Changes

- Keep Someday membership for to-dos and add deferred projects to the Someday view.
- Reactivation happens only through an explicit schedule change (Anyday/Today/Tomorrow/date)
  which immediately moves the item into the matching projection.
- An active project never implicitly reactivates a Someday child; children keep their Someday
  schedule and stay out of actionable views.
- Replace the flat Someday list with To-Dos and Projects sections plus reactivation controls.

## Capabilities

### New Capabilities

- `features/someday`: Someday membership, explicit reactivation, and parent/child independence.

### Modified Capabilities

(none)

## Impact

- `crates/app` (project Someday query), `crates/web` (Someday page and project rows).
- No schema change; Someday is an existing schedule state.
- Non-goals: scheduled smart lists, recurrence (34), drag ordering (32).
