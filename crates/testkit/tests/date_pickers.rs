//! When/deadline/reminder semantics: schedule-gated reminders and independent deadline.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, CommandError, ReminderInput, ScheduleInput};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::Schedule;
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    todos: TodoService,
    account: AccountId,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
    ));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = TodoService::new(
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    );
    Harness {
        todos,
        account: AccountId::new(),
    }
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

async fn create(h: &Harness, title: &str) -> momentum_domain::EntityId {
    h.todos
        .create(h.account, title, None)
        .await
        .expect("create");
    h.todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == title)
        .expect("created todo")
        .id
}

fn reminder() -> ReminderInput {
    ReminderInput {
        at: Utc.with_ymd_and_hms(2026, 6, 20, 7, 0, 0).unwrap(),
        timezone: "Europe/Berlin".to_string(),
    }
}

#[tokio::test]
async fn reminders_require_a_scheduled_date_and_clear_independently() {
    let h = harness();
    let id = create(&h, "A").await;

    let error = h
        .todos
        .set_reminder(h.account, id, Some(reminder()))
        .await
        .expect_err("reminder without schedule");
    assert!(matches!(error, CommandError::Validation(_)));

    h.todos
        .schedule(h.account, id, ScheduleInput::Someday)
        .await
        .expect("someday");
    let error = h
        .todos
        .set_reminder(h.account, id, Some(reminder()))
        .await
        .expect_err("reminder on someday");
    assert!(matches!(error, CommandError::Validation(_)));

    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(20)))
        .await
        .expect("schedule");
    h.todos
        .set_reminder(h.account, id, Some(reminder()))
        .await
        .expect("reminder");
    let summary = h.todos.get(h.account, id).await.unwrap().unwrap();
    let stored = summary.reminder.expect("reminder");
    assert_eq!(stored.at, reminder().at);
    assert_eq!(stored.timezone, "Europe/Berlin");
    assert_eq!(summary.schedule, Schedule::on(date(20)));

    h.todos
        .set_reminder(h.account, id, None)
        .await
        .expect("clear reminder");
    let cleared = h.todos.get(h.account, id).await.unwrap().unwrap();
    assert!(cleared.reminder.is_none());
    assert_eq!(cleared.schedule, Schedule::on(date(20)));
}

#[tokio::test]
async fn deadline_selection_and_clearing_leave_schedule_untouched() {
    let h = harness();
    let id = create(&h, "B").await;

    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(20)))
        .await
        .expect("schedule");
    h.todos
        .set_deadline(h.account, id, Some(date(18)))
        .await
        .expect("deadline");
    let dated = h.todos.get(h.account, id).await.unwrap().unwrap();
    assert_eq!(dated.deadline.unwrap().date, date(18));
    assert_eq!(dated.schedule, Schedule::on(date(20)));

    h.todos
        .set_deadline(h.account, id, None)
        .await
        .expect("clear deadline");
    let cleared = h.todos.get(h.account, id).await.unwrap().unwrap();
    assert!(cleared.deadline.is_none());
    assert_eq!(cleared.schedule, Schedule::on(date(20)));
}
