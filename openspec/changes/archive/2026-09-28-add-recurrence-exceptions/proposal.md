# Proposal

## Why

Recurring items diverge in practice: one copy gets rescheduled or completed early while the rule
should keep generating; after-completion rules must anchor to the real completion date. These
behaviors need first-class commands, not UI-only state.

## What Changes

- After-completion anchoring: the materializer gates after-completion templates on the absence of
  an open copy and anchors the next occurrence to the latest completion date (`TemplateState`),
  publishing one copy at a time with count/date/bound end conditions still enforced.
- Early completion: completing the open copy before its scheduled date anchors the next copy to
  the actual completion date — demonstrated by tests moving a copy scheduled for day 3 completed
  on day 1 to the next occurrence on day 4, never day 6.
- One-time versus future edits: edits to a generated copy stay one-time (existing schedule
  commands touch only that copy), while `UpdateRecurrenceRule` replaces the template rule,
  resets persisted progress, and leaves existing copies untouched; the materializer realigns to
  the first rule-matching date after a reset.

## Capabilities

### New Capabilities

- `features/recurrence-exceptions`: early completion, one-time versus future edit,
  after-completion anchor.

### Modified Capabilities

(none)

## Impact

- `crates/app` (materializer gating/anchoring, `TemplateState`, `UpdateRecurrenceRule` command,
  unit-of-work rule persistence), `crates/infra` (PG state/rule SQL), `crates/testkit` (memory
  instance-to-copy mapping and state), tests. No schema change.
- Non-goals: template creation/editing UI, exception history views, time-of-day anchoring for
  after-completion (date-level anchoring only).
