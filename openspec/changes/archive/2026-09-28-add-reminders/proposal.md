# Proposal

## Why

Reminders must fire exactly once, survive restarts, and reach browsers with permission while
deep-linking back to the owning item; delivery state belongs to canonical persistence, not to an
in-process queue.

## What Changes

- Persist delivery state transitions on the existing reminder JSON: `Reminder::is_due` and
  idempotent `mark_delivered`, plus a `MarkTodoReminderDelivered` command so delivery is
  version-guarded and undo-inert.
- Add `ReminderScheduler` with `ReminderSource` (due-reminder read model) and `ReminderNotifier`
  (transport) traits; one pass notifies each due reminder, marks it delivered through the command
  layer, and reports per-item failures that stay scheduled for retry. Restarts cannot re-deliver
  because the persisted state is no longer `Scheduled`.
- Add Web Push plumbing: `PushSubscription{Store}` persistence, `build_push_notification` deep
  links (`/todos/{id}`), a `WebPushNotifier` that fans a JSON payload out to every stored
  subscription through a `PushTransport`, a PostgreSQL `PgReminderSource`, and
  `POST /push/subscriptions` for the hydrated client to register endpoints.

## Capabilities

### New Capabilities

- `features/reminder-delivery`: reminder persistence, reliable scheduler, Web Push delivery.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (reminder transitions), `crates/app` (command + scheduler/push module),
  `crates/infra` (PG source), `crates/web` (subscription route, AppState builder), testkit
  doubles and tests. No schema change (delivery state already lives in `reminder_rule`).
- Non-goals: VAPID key management and the network push adapter (a `PushTransport` implementation
  wired to a push service) plus browser service-worker registration, which belong to the hydrated
  client and deployment configuration; these are recorded as the remaining live gap.
