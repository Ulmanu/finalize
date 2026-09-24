# Proposal

## Why

Checklists are the lightest way to break a to-do into steps. They must be ordered, editable
in place, support bulk entry, and copy correctly when a to-do is duplicated or later repeated.

## What Changes

- Add checklist commands: add, rename, complete/uncomplete, remove, reorder, and bulk expand.
- Deterministic multi-line normalization for pasted checklist content.
- Duplication copies checklist structure while resetting completion state (template semantics).
- Persist checklist items for PostgreSQL and expose them in the to-do read model.
- Render an editable checklist on the to-do editor page.

## Capabilities

### New Capabilities

- `features/checklists`: ordered embedded checklist editing, bulk paste expansion, and
  copy/template semantics.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (line normalization), `crates/app` (commands, to-do read model), `crates/infra`
  (checklist persistence), `crates/testkit`, `crates/web`.
- No schema change: `checklist_items` already exists in the canonical schema.
- Non-goals: recurrence engine (change 34), rich text rendering, tags (17).
