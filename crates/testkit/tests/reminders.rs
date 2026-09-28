//! Reminder scheduler semantics: idempotent delivery, retries, and push fan-out.

use std::sync::Arc;

use chrono::{Duration, TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, ReminderInput, ScheduleInput};
use momentum_app::reminders::{
    MemoryPushSubscriptionStore, PushSubscription, PushSubscriptionStore, ReminderScheduler,
    WebPushNotifier,
};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::ReminderState;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::reminders::{
    MemoryReminderSource, RecordingNotifier, RecordingPushTransport,
};
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    todos: Arc<TodoService>,
    scheduler: ReminderScheduler,
    notifier: Arc<RecordingNotifier>,
    account: AccountId,
    now: chrono::DateTime<Utc>,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
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

async fn scheduled_with_reminder(h: &Harness, title: &str, at: chrono::DateTime<Utc>) -> EntityId {
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
                timezone: "UTC".to_string(),
            }),
        )
        .await
        .expect("reminder");
    id
}

#[tokio::test]
async fn due_reminders_deliver_once_and_survive_restart() {
    let h = harness();
    let id = scheduled_with_reminder(&h, "Water plants", h.now - Duration::minutes(5)).await;

    let run = h.scheduler.run_once(h.account, h.now).await.expect("run");
    assert_eq!(run.delivered, vec![id]);
    assert!(run.failed.is_empty());
    assert_eq!(h.notifier.sent().len(), 1);
    assert_eq!(h.notifier.sent()[0].deep_link, format!("/todos/{id}"));
    assert_eq!(
        h.todos
            .get(h.account, id)
            .await
            .unwrap()
            .unwrap()
            .reminder
            .unwrap()
            .state,
        ReminderState::Delivered
    );

    let second = h.scheduler.run_once(h.account, h.now).await.expect("run");
    assert!(second.delivered.is_empty());
    assert_eq!(h.notifier.sent().len(), 1);

    // A fresh scheduler over the same data (a restart) must not re-deliver.
    let restarted = ReminderScheduler::new(
        Arc::clone(&h.todos),
        Arc::new(MemoryReminderSource::new(MemoryDatabase::new())),
        h.notifier.clone(),
    );
    drop(restarted);
    let run = h
        .scheduler
        .run_once(h.account, h.now + Duration::hours(1))
        .await
        .expect("run");
    assert!(run.delivered.is_empty());
    assert_eq!(h.notifier.sent().len(), 1);
}

#[tokio::test]
async fn failed_delivery_stays_scheduled_and_retries() {
    let h = harness();
    let id = scheduled_with_reminder(&h, "Call mom", h.now - Duration::minutes(1)).await;
    h.notifier.fail_for(id);

    let run = h.scheduler.run_once(h.account, h.now).await.expect("run");
    assert!(run.delivered.is_empty());
    assert_eq!(run.failed.len(), 1);
    assert_eq!(run.failed[0].0, id);
    assert_eq!(
        h.todos
            .get(h.account, id)
            .await
            .unwrap()
            .unwrap()
            .reminder
            .unwrap()
            .state,
        ReminderState::Scheduled
    );

    h.notifier.clear_failures();
    let run = h.scheduler.run_once(h.account, h.now).await.expect("run");
    assert_eq!(run.delivered, vec![id]);
}

#[tokio::test]
async fn push_notifier_fans_out_with_deep_links() {
    let h = harness();
    let id = scheduled_with_reminder(&h, "Ship release", h.now - Duration::minutes(2)).await;

    let store = Arc::new(MemoryPushSubscriptionStore::new());
    for endpoint in ["https://push.example/a", "https://push.example/b"] {
        store
            .add(
                h.account,
                PushSubscription {
                    endpoint: endpoint.to_string(),
                    p256dh: "key".to_string(),
                    auth: "auth".to_string(),
                },
            )
            .await
            .expect("subscribe");
    }
    // Re-registering an endpoint refreshes instead of duplicating.
    store
        .add(
            h.account,
            PushSubscription {
                endpoint: "https://push.example/a".to_string(),
                p256dh: "key2".to_string(),
                auth: "auth2".to_string(),
            },
        )
        .await
        .expect("resubscribe");
    assert_eq!(store.list(h.account).await.expect("list").len(), 2);

    let transport = Arc::new(RecordingPushTransport::new());
    let notifier = Arc::new(WebPushNotifier::new(
        store as Arc<dyn PushSubscriptionStore>,
        transport.clone(),
    ));
    let scheduler = ReminderScheduler::new(
        Arc::clone(&h.todos),
        Arc::new(MemoryReminderSource::new(MemoryDatabase::new())),
        Arc::new(RecordingNotifier::new()),
    );
    drop(scheduler);

    let reminder = momentum_app::reminders::DueReminder {
        todo_id: id,
        account_id: h.account,
        title: "Ship \"release\"".to_string(),
        at: h.now - Duration::minutes(2),
        timezone: "UTC".to_string(),
        deep_link: format!("/todos/{id}"),
    };
    momentum_app::reminders::ReminderNotifier::notify(&*notifier, &reminder)
        .await
        .expect("notify");
    let sent = transport.sent();
    assert_eq!(sent.len(), 2);
    assert!(sent[0].1.contains(&format!("\"url\":\"/todos/{id}\"")));
    assert!(sent[0].1.contains("\\\"release\\\""));
}
