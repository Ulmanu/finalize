# Proposal

## Why

The Logbook is the completion history. It must derive from terminal state and logging policy,
keep creation/completion/cancellation/logging timestamps queryable, and let users control
whether logging happens automatically or on demand without changing completion state.

## What Changes

- Derive Logbook membership from terminal state plus the account logging policy.
- Keep creation, completion, cancellation, and logging timestamps independently queryable.
- Add the logging policy preference (automatic by default, manual opt-in) and explicit
  `LogTodo`/`LogProject` commands for manual logging.
- Replace the placeholder Logbook page with date-grouped history, a policy control, and pending
  logging with per-entry Log actions.

## Capabilities

### New Capabilities

- `features/logbook`: terminal-state history, timestamps, and logging policy.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (`LoggingPolicy`), `crates/app` (log commands, logbook service/agenda,
  `TodoScope::Logbook { logged_only }`), `crates/infra` (policy store over `preferences`,
  selects), `crates/testkit`, `crates/web` (Logbook page and routes).
- No schema change: `preferences` and `logged_at` already exist.
- Non-goals: sync of preference values (later sync changes), logbook export.
