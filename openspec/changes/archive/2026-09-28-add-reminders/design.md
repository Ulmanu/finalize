# Design — add-reminders

## Context

Reminders already persist as JSON on the entity (`instant`, `timezone`, `state`) and commands
exist to set/clear them. Nothing consumed due reminders, and no push registration existed.

## Requirements Driving the Design

- `Reminder persistence`
- `Reliable scheduler`
- `Web Push delivery`

## Architecture

- Domain: `Reminder::is_due(now)` (scheduled and at-or-before) and `mark_delivered()` (idempotent
  Scheduled→Delivered); `Todo::mark_reminder_delivered` touches metadata only on a real change.
- Command: `MarkTodoReminderDelivered { id }` emits `TodoReminderChanged`, is version-guarded, and
  has no undo inverse (delivery is a system transition).
- Scheduler: `ReminderScheduler::run_once(account, now)` iterates `ReminderSource::due_reminders`,
  calls `ReminderNotifier::notify`, then marks delivered; notify or mark failures are recorded per
  item and remain retryable. Second run and restarted schedulers observe `Delivered` state and are
  no-ops.
- Push: `PushSubscriptionStore` (memory implementation in-process; PG persistence via the same
  pattern when deployment needs it), `build_push_notification` producing
  `{title, body, url:/todos/{id}}` JSON, `WebPushNotifier` fanning out through `PushTransport`.
- PG source: `PgReminderSource` selects to-do rows with non-null `reminder_rule` and filters by
  `is_due` in Rust, avoiding JSON SQL portability risk.

## Data and Persistence

- No migration: delivery state is part of the stored reminder JSON; subscriptions are client
  registrations documented above.

## Offline and Sync

- Delivery marking is an ordinary idempotent command with change-log entries; notification failure
  leaves the reminder scheduled, so retries are safe.

## Error Handling

- Source read errors propagate; notify/mark failures surface per item as `failed` entries without
  stopping other deliveries; subscription errors redirect with `?error=push`.

## Testing Strategy

- Domain unit test for due/idempotent delivery; testkit tests for deliver-once, restart no-op,
  failure retry, and push fan-out with deep-link payloads; web tests for subscription registration,
  endpoint refresh, and missing-store error.

## Rollout / Migration

- None required; existing stored reminders become due when their instant passes.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the browser cannot register push subscriptions without JavaScript, and no VAPID/HTTP
  push adapter exists offline, so this change delivers the persistence, scheduler, deep-link
  payloads, subscription API, and transports behind traits; the network adapter and service
  worker glue are the documented hydration/deployment gap.
