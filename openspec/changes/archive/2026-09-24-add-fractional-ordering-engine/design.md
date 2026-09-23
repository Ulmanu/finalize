# Design — add-fractional-ordering-engine

## Context

Ordering must be cheap (drag/drop and multi-select depend on it), deterministic (tests and
sync), and independent per scope (context vs Today). Ranks cross the offline/sync boundary, so
they must be stable values, not UI indices.

## Requirements Driving the Design

- `Independent ordering scopes`
- `Fractional insertion`
- `Rank maintenance`

## Architecture

- `crates/app/src/ordering.rs`:
  - `Rank(i64)` with `STEP = 1024` and storage encoding as a zero-padded 12-digit decimal
    string, so lexicographic `text` comparison equals numeric order without a schema change.
  - `Rank::between(lower, upper) -> Result<Rank, OrderingError>`: midpoint when a gap exists;
    `OrderingError::Exhausted` when neighbors are adjacent or the range would overflow.
  - `Rank::append/prepend/start`, `rebalance(count)` producing evenly spaced ranks.
  - `OrderedItem { entity_id, rank }`, `OrderingScope` (`Context(ContextRef)` | `Today`).
  - `ScopeOrder { scope, items }` with `insert_at`, `move_item`, `plan_append`, `rebalance`,
    `ranks()`; every mutation returns an `OrderingChange` describing the new/changed ranks so
    the command layer can persist them transactionally.
  - `plan_insert(items, index)` / `plan_move(items, from, to)` are pure functions usable by
    drag-and-drop, Move dialogs, keyboard commands, and automation.
- Contextual vs Today scopes are separate `ScopeOrder` values; nothing in the engine ties one
  scope to another, which is the mechanical guarantee behind ordering independence.

## Data and Persistence

`placements.rank`/`checklist_items.rank` stay `text`; the zero-padded encoding keeps
lexicographic comparisons correct. No migration is required. A scope's `placements` row set is
identified by `scope` + `parent_kind` + `parent_id`.

## Offline and Sync

Ranks are ordinary values: offline reorders produce new ranks that sync like any field, and
concurrent reorders converge because ranks are compared, not rewritten globally.

## UI and Accessibility

No UI; the engine's insertion indices are the contract drag-and-drop will use.

## Error Handling

`OrderingError::Exhausted { scope, position }` is a recoverable signal: the caller rebalances
the scope (returning the updated ranks) and retries the insertion. `OutOfBounds` reports
invalid indices.

## Testing Strategy

- Rank encoding/comparison and midpoint math.
- Insert/append/prepend/move plans, including first/last positions.
- Exhaustion detection and rebalance order preservation (property-style loop over many inserts).
- Scope independence: same entity different ranks in two scopes; mutations in one scope leave
  the other untouched.

## Rollout / Migration

No schema or data migration; new module only.

## Open Questions Resolved From the Repository

- **Fractional vs integer ranks**: integer gap ranks with a zero-padded text encoding were
  chosen over LexoRank-style strings because they are simpler to test, deterministic, and
  sufficient for tens of thousands of items; the encoding preserves text ordering.
- **Where rebalancing happens**: the engine plans; the command layer persists, keeping
  persistence atomic (change 09).

## Tasks / Definition of Done

See `tasks.md`.
