# Proposal

## Why

Tags give cross-cutting context that contexts cannot: one to-do can belong to several tags, tags
nest, and a project or area tag should color its children without duplicating assignments.

## What Changes

- Add tag commands: create, rename, move (nest), reorder among siblings, delete, assign, and
  unassign.
- Prevent hierarchy cycles; deleted tags reparent their children deterministically.
- Expose tag reads and per-to-do direct and inherited tags.
- Render a tags management page, tag assignment on the to-do editor, and multi-tag list
  filtering that includes inherited context tags.

## Capabilities

### New Capabilities

- `features/tags`: tag CRUD and hierarchy, assignment/removal, inherited context tags, and
  multi-tag filtering.

### Modified Capabilities

(none)

## Impact

- `crates/app` (commands, tags service, to-do read model), `crates/infra` (tag persistence +
  migration `0004`), `crates/testkit`, `crates/web`.
- Schema: `entities.tag_parent` and `entities.tag_position` (migration `0004-tag-hierarchy`).
- Non-goals: tag-based smart lists beyond filtering (later roadmap), drag ordering (32).
