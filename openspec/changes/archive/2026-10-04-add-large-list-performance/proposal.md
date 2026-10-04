# Proposal

## Why

Accounts with thousands of items need measurable budgets and tuned reads; unbounded list
responses and missing indexes turn into user-visible latency.

## What Changes

- Add `momentum_app::performance`: declared budgets for initial render, search, reorder, sync
  apply, scroll, and replica startup (with dataset sizes and server-measured flags), page-limit
  constants, and deterministic `window` offset/limit slicing.
- Testkit benchmarks seed 2,000 items and enforce the server-measured budgets (with a documented
  debug factor) for list, deep search, 100 positional reorders, and a 100-operation sync apply.
- API list reads become windowed: `/api/v1/todos` accepts `limit`/`offset` (default 100, clamped
  to 500), keeping large accounts bounded; virtualization for scroll remains the hydrated
  client's concern with the documented scroll budget.
- Migration 0008 adds large-account indexes (`entities(account_id, kind, created_at desc)` and
  `change_log(account_id, created_at desc)`).

## Capabilities

### New Capabilities

- `features/large-list-performance`: performance budgets, list virtualization, query/index
  tuning.

### Modified Capabilities

(none)

## Impact

- `crates/app` (performance module + unit tests), `crates/infra` (migration 0008),
  `crates/web` (windowed API reads, tests), `crates/testkit` (benchmarks). No behavioral change
  to list contents beyond windowing defaults on the API.
- Non-goals: a WASM virtualization component (hydrated client work), query plan automation.
