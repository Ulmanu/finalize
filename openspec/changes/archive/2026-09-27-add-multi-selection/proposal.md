# Proposal

## Why

Batch work (complete a handful of errands, reschedule a set, tag several captures) is a core
Things interaction, and it needs one deterministic selection model plus bulk application of the
existing single-item commands rather than a parallel write path.

## What Changes

- Add a deterministic selection state machine (single, toggle, range, select-all, clear) with
  anchor semantics and pruning against the visible row set.
- Add `BulkService`, which applies complete, cancel, schedule, deadline, move, tag, and delete
  to every selected item through the same version-guarded commands as single-item paths and
  returns explicit per-item results.
- Surface selection in the Inbox list: a row checkbox per item, a bulk action form posting to
  `/todos/bulk` (server-rendered, keyboard reachable), and deterministic redirects that flag
  partial failures.

## Capabilities

### New Capabilities

- `features/multi-selection`: selection state machine, bulk commands, selection stability.

### Modified Capabilities

(none)

## Impact

- `crates/app` (selection module, bulk module, `dispatch` visibility), `crates/web` (bulk form,
  row checkbox, bulk route, `form_urlencoded` dependency), `crates/testkit`/`crates/web` tests.
  No schema change; selection never persists.
- Non-goals: future capabilities; cross-list selection persistence; the hydrated gesture layer
  (selection survives rerenders by pruning, documented like changes 32/71/72).
