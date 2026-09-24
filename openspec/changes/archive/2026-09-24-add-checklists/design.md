# Design — add-checklists

## Context

The domain already models `Todo.checklist: Vec<ChecklistItem>` and the canonical schema already
has `checklist_items`. This change routes checklist mutations through typed commands on the
to-do aggregate and makes adapters and UI use the existing structures.

## Requirements Driving the Design

- `Checklist editing`
- `Bulk paste expansion`
- `Template/copy semantics`

## Architecture

- Commands: `AddChecklistItem`, `SetChecklistItemTitle`, `SetChecklistItemCompleted`,
  `RemoveChecklistItem`, `ReorderChecklistItem`, `ExpandChecklist`.
- All mutations load/save the parent to-do, so the to-do remains the single canonical entity
  and the checklist is a value inside its aggregate.
- `DuplicateTodo` copies checklist structure with completion reset, which is also the copy
  semantics recurrence materialization will reuse.
- `domain::model::parse_checklist_lines` performs deterministic normalization: trim, drop empty
  lines, strip bullet/checkbox prefixes, preserve order.
- `TodoSummary` gains `checklist: Vec<ChecklistItemSummary>` for reads.

## Data and Persistence

- `checklist_items` rows are synchronized transactionally with the parent to-do; item ids are
  stable across edits, and removals delete the row.
- Rank text preserves order deterministically.

## Offline and Sync

- Checklist commands are ordinary idempotent commands recorded in the change log.

## UI and Accessibility

- The to-do editor renders each item with a completion checkbox, rename field, move up/down and
  remove controls, plus add-item and bulk-paste forms; all controls are labeled and keyboard
  reachable using existing primitives.

## Error Handling

- Unknown to-dos/items produce `NotFound`; invalid titles or empty bulk input produce
  `Validation`; version mismatches produce `Conflict`.

## Testing Strategy

- Testkit tests for editing, ordering, bulk normalization, and duplicate semantics.
- Web tests for editor rendering and endpoints.

## Rollout / Migration

- No migration required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the checklist lives inside the to-do aggregate (not an independent entity), so
  completion state reset on copy is a pure function of the aggregate.
