# Design — add-multi-selection

## Context

Lists already render rows server-side with a hydrated-agnostic option structure (32). Selection
is presentation state and must never reach the change log; bulk operations must reuse the
existing commands so undo, versions, and projections behave identically.

## Requirements Driving the Design

- `Selection state machine`
- `Bulk commands`
- `Selection stability`

## Architecture

- `Selection` in `crates/app` is a `BTreeSet` plus an anchor; `single`, `toggle`, `extend_to`
  (range over the visible order), `select_all`, `clear`, and `retain_visible` are pure and
  deterministic; `ids()` sorts so bulk application order is stable.
- `BulkService` loops the selection through `TodoService`/`TagService` methods, recording
  `BulkOutcome { succeeded, failed }` with the command error message per failed ID. There is no
  new mutation path and no partial silent write: every failure is reported.
- 33 does not add persistence; selection lives in the hydrated client or in browser checkbox
  state for the server-rendered bulk form.

## Data and Persistence

- None; bulk commands reuse existing version-guarded commands and change-log entries.

## Offline and Sync

- Each item is an ordinary idempotent command, so retries behave like single-item retries.
- Selection itself is not synchronized.

## UI and Accessibility

- Each Inbox row carries a labeled checkbox (`name="ids"`, `aria-label="Select <title>"`); the
  bulk form exposes one labeled action select plus optional date/destination/tag selects and a
  submit button, so every bulk operation is keyboard reachable without pointer gestures.
- Existing design-system buttons and selects are reused; no new primitives.

## Error Handling

- Empty selection redirects with `?error=selection`; unknown action or missing parameters with
  `?error=bulk`; per-item failures surface as `?error=bulk` after the eligible items were
  processed and reported by ID.
- Bulk tag assignment without a tag service fails per item with a not-found error.

## Testing Strategy

- App unit tests for the selection machine (toggle/anchor, range replacement, hidden IDs,
  select-all, pruning).
- Testkit tests for per-item bulk results (mixed success/failure) and canonical state after
  schedule, deadline, move, tag, and delete.
- Web tests for checkbox/form markup and bulk POST behavior including empty selection.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: repeated `ids` fields need manual form parsing (`form_urlencoded`) because serde's
  form extractor cannot collect repeated keys; the server-rendered bulk form is the accessible
  parity surface while the hydrated client reuses `Selection`.
