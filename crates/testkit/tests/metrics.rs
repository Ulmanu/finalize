//! Operational metrics: sync, reminder, and inbound counters.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{Command, CommandDispatcher};
use momentum_app::inbound::{InboundLimits, InboundMail, InboundService, IngestOutcome};
use momentum_app::metrics::Metrics;
use momentum_app::reminders::ReminderScheduler;
use momentum_app::sync::{SyncOperation, SyncRequest, SyncService};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::inbound::{DeterministicTokenSource, MemoryInboundStore};
use momentum_testkit::reminders::{MemoryReminderSource, RecordingNotifier};
use momentum_testkit::sync::MemoryChangeLogReader;
use momentum_testkit::todos::MemoryTodoReadStore;
use uuid::Uuid;

fn now() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap()
}

#[tokio::test]
async fn sync_counters_track_operations_and_conflicts() {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(now()));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    ));
    let metrics = Arc::new(Metrics::new());
    let sync = SyncService::new(
        dispatcher,
        todos.clone(),
        Arc::new(MemoryChangeLogReader::new(database)),
        clock,
    )
    .with_metrics(metrics.clone());
    let account = AccountId::new();
    todos.create(account, "Task", None).await.expect("create");
    let id = todos
        .list(account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;

    let response = sync
        .sync(
            account,
            SyncRequest {
                cursor: 0,
                operations: vec![
                    SyncOperation {
                        operation_id: Uuid::now_v7(),
                        command: Command::SetTodoTitle {
                            id,
                            title: "Renamed".to_string(),
                        },
                    },
                    SyncOperation {
                        operation_id: Uuid::now_v7(),
                        command: Command::SetTodoTitle {
                            id: momentum_domain::EntityId::new(),
                            title: "Missing".to_string(),
                        },
                    },
                ],
            },
        )
        .await
        .expect("sync");
    assert_eq!(response.acknowledged.len(), 1);
    assert_eq!(response.conflicts.len(), 1);
    assert_eq!(metrics.value("momentum_sync_operations_total"), 1);
    assert_eq!(metrics.value("momentum_sync_conflicts_total"), 1);
}

#[tokio::test]
async fn reminder_and_inbound_counters_track_outcomes() {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(now()));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    ));
    let metrics = Arc::new(Metrics::new());
    let account = AccountId::new();

    // Reminder: one due reminder delivered, one failing notifier pass counted.
    todos
        .create(account, "Reminder", None)
        .await
        .expect("create");
    let todo = todos
        .list(account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;
    todos
        .schedule(
            account,
            todo,
            momentum_app::commands::ScheduleInput::On(
                momentum_domain::time::LocalDate::from_ymd(2026, 6, 10).unwrap(),
            ),
        )
        .await
        .expect("schedule");
    todos
        .set_reminder(
            account,
            todo,
            Some(momentum_app::commands::ReminderInput {
                at: now() - chrono::Duration::minutes(5),
                timezone: "UTC".to_string(),
            }),
        )
        .await
        .expect("reminder");
    let notifier = Arc::new(RecordingNotifier::new());
    notifier.fail_for(todo);
    let scheduler = ReminderScheduler::new(
        todos.clone(),
        Arc::new(MemoryReminderSource::new(database)),
        notifier.clone(),
    )
    .with_metrics(metrics.clone());
    let run = scheduler.run_once(account, now()).await.expect("run");
    assert_eq!(run.failed.len(), 1);
    assert_eq!(metrics.value("momentum_reminders_failed_total"), 1);
    notifier.clear_failures();
    scheduler.run_once(account, now()).await.expect("run");
    assert_eq!(metrics.value("momentum_reminders_delivered_total"), 1);

    // Inbound: one delivered, one rejected.
    let inbound = InboundService::new(
        Arc::new(MemoryInboundStore::new()),
        Arc::new(MemoryInboundStore::new()),
        todos.clone(),
        Arc::new(DeterministicTokenSource::new()),
        InboundLimits::default(),
    )
    .with_metrics(metrics.clone());
    let alias = inbound.ensure_alias(account).await.expect("alias");
    let mail = InboundMail {
        message_id: "m1".to_string(),
        subject: "Hello".to_string(),
        body: String::new(),
    };
    assert!(matches!(
        inbound
            .ingest(&alias.token, &mail, now())
            .await
            .expect("ingest"),
        IngestOutcome::Delivered { .. }
    ));
    assert_eq!(
        inbound
            .ingest("unknown", &mail, now())
            .await
            .expect("ingest"),
        IngestOutcome::UnknownToken
    );
    assert_eq!(metrics.value("momentum_inbound_delivered_total"), 1);
    assert_eq!(metrics.value("momentum_inbound_rejected_total"), 1);
}
