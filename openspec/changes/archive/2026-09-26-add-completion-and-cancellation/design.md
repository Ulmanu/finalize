# Design — add-completion-and-cancellation

## Context

The domain already distinguishes `Status::{Completed, Canceled, Deleted}` with per-status
timestamps and reopen methods. The gaps were: project cascade/reject only counted direct
project-context children (heading children could be left open under a completed project),
reopening a deleted item silently did nothing, and restoring a deleted item left stale terminal
timestamps. Read models did not expose cancellation/completion instants for verification.

## Requirements Driving the Design

- `Completed versus canceled`
- `Reopen`
- `Project completion safety`

## Architecture

- `Todo::complete`/`cancel` and `Project::complete`/`cancel` remain the single source of truth;
  `reopen` only acts on terminal (completed/canceled) states.
- `ReopenTodo`/`ReopenProject` reject `Deleted` with a validation error, directing callers to
  the restore path.
- `RestoreTodo` clears `completed_at`/`canceled_at` together with `deleted_at`.
- `CompleteProject` gathers direct project children plus every child listed under the project's
  headings, so `RejectWhileOpen` cannot be bypassed and `CompleteOpenChildren` completes the
  full open child set before saving the project; it stays one unit of work (transactional).
- `TodoSummary` exposes `canceled_at`; `ProjectSummary` exposes `completed_at`/`canceled_at`.

## Data and Persistence

- None; timestamps already persist as `completed_at`/`canceled_at`.

## Offline and Sync

- Terminal and reopen transitions are ordinary idempotent commands.

## UI and Accessibility

- Existing editor/project controls are exercised end to end by tests; no new widgets.

## Error Handling

- Reopening a deleted item returns validation rather than silently succeeding; the reject policy
  surfaces a conflict that the web maps to an error redirect.

## Testing Strategy

- Testkit tests for distinct timestamps, reopen policy and metadata preservation, project
  reject/cascade including heading children, and cancel preserving children.
- Web tests for the cancel/reopen round trip and both project policies through HTTP.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: heading children are reached through `load_context_order(ContextRef::Heading)`
  rather than a new query port, reusing the existing ordering/placement model.
