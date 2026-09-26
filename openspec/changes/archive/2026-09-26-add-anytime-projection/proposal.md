# Proposal

## Why

Anytime is the "what can I do now" list: every open item that is actionable today, regardless of
whether it lives in the Inbox or a project, while future-scheduled and deferred work stays out.
Rendering it must never manufacture context relationships.

## What Changes

- Refine Anytime membership to schedule-actionable open items: Anytime schedules plus date
  schedules on or before the local today (overdue and This Evening included).
- Exclude future-scheduled, Someday, completed, canceled, and deleted items.
- Group Anytime by the item's existing context (Inbox, area, project) without writing any
  context changes.
- Replace the flat Anytime list with grouped sections that label groups from existing titles.

## Capabilities

### New Capabilities

- `features/anytime`: actionable membership, exclusions, and context grouping.

### Modified Capabilities

(none; the scheduling capability defines states generically and this capability refines the
Anytime projection membership)

## Impact

- `crates/app` (agenda read model, service), `crates/infra` (Anytime scope filter), `crates/testkit`
  (scope predicate and tests), `crates/web` (Anytime page).
- No schema change.
- Non-goals: Someday split view (25), drag ordering (32), recurrence (34).
