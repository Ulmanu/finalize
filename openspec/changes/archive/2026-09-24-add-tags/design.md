# Design — add-tags

## Context

Tags are `entities.kind = 'tag'` with `tag_assignments` linking them to to-dos. The domain `Tag`
carries an optional parent; ordering and the parent column need persistence, hence migration
`0004`.

## Requirements Driving the Design

- `Tag hierarchy`
- `Tag assignment and filtering`
- `Inherited context tags`

## Architecture

- Commands: `CreateTag`, `RenameTag`, `MoveTag`, `ReorderTag`, `DeleteTag`, `AssignTag`,
  `UnassignTag`.
- `UnitOfWork` gains `load_tag`, `save_tag` (with sibling position), `list_tags`, and
  `delete_tag`; to-dos keep their tag set inside the aggregate and `save_todo` synchronizes
  `tag_assignments` transactionally.
- Cycle prevention walks the proposed parent chain with a bounded depth and rejects moves that
  would reach the moved tag.
- Deleting a tag reparents its children to the deleted tag's parent and removes assignments.
- `TagService`/`TagReadStore` expose `TagSummary { id, title, parent, position, assigned_count }`.
- `TodoSummary` gains direct `tags` and `inherited_tags`; inheritance is derived from context
  (project, heading→project, area, and the project's area) and never written onto children.

## Data and Persistence

- Migration `0004-tag-hierarchy` adds `entities.tag_parent` and `entities.tag_position`.
- Positions are small integers per sibling group; reorder swaps adjacent positions in one
  transaction.

## Offline and Sync

- Tag commands are ordinary idempotent commands with change-log records.

## UI and Accessibility

- `/tags` manages the hierarchy with labeled forms; the to-do editor assigns/unassigns tags; list
  pages filter by selected tags using labeled checkboxes.

## Error Handling

- Unknown tags/to-dos produce `NotFound`; cycles and invalid titles produce `Validation`;
  version mismatches produce `Conflict`.

## Testing Strategy

- Testkit tests for hierarchy (nesting, cycle prevention, delete reparenting), assignment, and
  filter semantics including inheritance.
- Web tests for the tags page, assignment, and filtering.

## Rollout / Migration

- New nullable/defaulted columns apply safely to existing databases; existing tags become
  top-level at position 0.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: sibling order is stored on the entity row (`tag_position`) rather than reusing
  placement scopes, because placements only model contextual parents.
