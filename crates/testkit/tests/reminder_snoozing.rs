//! Reminder snoozing: idempotent choices, local "tomorrow", and scheduler re-delivery.

use std::sync::Arc;

use chrono::{DateTime, Duration, TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, ReminderInput, ScheduleInput};
use momentum_app::reminders::{ReminderScheduler, SnoozeChoice};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::ReminderState;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::reminders::{MemoryReminderSource, RecordingNotifier};
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    todos: Arc<TodoService>,
    scheduler: ReminderScheduler,
    notifier: Arc<RecordingNotifier>,
    account: AccountId,
    now: DateTime<Utc>,
}

fn harness(now: DateTime<Utc>) -> Harness {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(now));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock,
    ));
    let notifier = Arc::new(RecordingNotifier::new());
    let scheduler = ReminderScheduler::new(
        Arc::clone(&todos),
        Arc::new(MemoryReminderSource::new(database)),
        notifier.clone(),
    );
    Harness {
        todos,
        scheduler,
        notifier,
        account: AccountId::new(),
        now,
    }
}

async fn due_reminder(h: &Harness, title: &str, timezone: &str, at: DateTime<Utc>) -> EntityId {
    h.todos
        .create(h.account, title, None)
        .await
        .expect("create");
    let id = h
        .todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == title)
        .expect("created")
        .id;
    h.todos
        .schedule(
            h.account,
            id,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 10).unwrap()),
        )
        .await
        .expect("schedule");
    h.todos
        .set_reminder(
            h.account,
            id,
            Some(ReminderInput {
                at,
                timezone: timezone.to_string(),
            }),
        )
        .await
        .expect("reminder");
    h.scheduler.run_once(h.account, h.now).await.expect("run");
    id
}

#[tokio::test]
async fn short_snoozes_are_idempotent_and_redeliver_once_due() {
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let h = harness(now);
    let id = due_reminder(&h, "Water plants", "UTC", now - Duration::minutes(5)).await;
    assert_eq!(h.notifier.sent().len(), 1);

    h.todos
        .snooze(h.account, id, SnoozeChoice::TenMinutes)
        .await
        .expect("snooze");
    let snoozed = h.todos.get(h.account, id).await.unwrap().unwrap();
    let reminder = snoozed.reminder.expect("reminder");
    assert_eq!(reminder.state, ReminderState::Snoozed);
    assert_eq!(reminder.at, now + Duration::minutes(10));

    // Repeating the same snooze is a no-op: same instant, same version, no extra work.
    h.todos
        .snooze(h.account, id, SnoozeChoice::TenMinutes)
        .await
        .expect("snooze again");
    let repeated = h.todos.get(h.account, id).await.unwrap().unwrap();
    assert_eq!(repeated.reminder.expect("reminder").at, reminder.at);
    assert_eq!(repeated.version, snoozed.version);

    // Before the snoozed instant nothing fires.
    let early = h
        .scheduler
        .run_once(h.account, now + Duration::minutes(5))
        .await
        .expect("run");
    assert!(early.delivered.is_empty());
    assert_eq!(h.notifier.sent().len(), 1);

    // At the snoozed instant it fires exactly once.
    let due = h
        .scheduler
        .run_once(h.account, now + Duration::minutes(10))
        .await
        .expect("run");
    assert_eq!(due.delivered, vec![id]);
    assert_eq!(h.notifier.sent().len(), 2);
    let again = h
        .scheduler
        .run_once(h.account, now + Duration::minutes(11))
        .await
        .expect("run");
    assert!(again.delivered.is_empty());
    assert_eq!(h.notifier.sent().len(), 2);
}

#[tokio::test]
async fn tomorrow_snooze_uses_local_date_not_fixed_24_hours() {
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 22, 30, 0).unwrap();
    let h = harness(now);
    // 18:00Z is 20:00 local in Berlin; now is 00:30 local on June 11.
    let id = due_reminder(
        &h,
        "Call bank",
        "Europe/Berlin",
        Utc.with_ymd_and_hms(2026, 6, 10, 18, 0, 0).unwrap(),
    )
    .await;

    h.todos
        .snooze(h.account, id, SnoozeChoice::Tomorrow)
        .await
        .expect("snooze");
    let reminder = h
        .todos
        .get(h.account, id)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .expect("reminder");
    assert_eq!(reminder.state, ReminderState::Snoozed);
    // June 12 20:00 local == 18:00Z, which is 43.5 hours after now, not a fixed 24.
    assert_eq!(
        reminder.at,
        Utc.with_ymd_and_hms(2026, 6, 12, 18, 0, 0).unwrap()
    );
}

#[tokio::test]
async fn snoozing_requires_a_reminder() {
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let h = harness(now);
    h.todos
        .create(h.account, "No reminder", None)
        .await
        .expect("create");
    let id = h
        .todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;
    assert!(h
        .todos
        .snooze(h.account, id, SnoozeChoice::OneHour)
        .await
        .is_err());
    assert!(h
        .todos
        .get(h.account, id)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .is_none());
}
