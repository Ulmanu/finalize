//! Deadline independence, state derivation, and projection tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::DeadlineState;
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

fn date(month: u32, day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, month, day).expect("date")
}

#[tokio::test]
async fn deadlines_are_independent_from_schedule_and_metadata() {
    let h = harness();
    let id = h
        .todos
        .create(h.account, "Independent", Some("notes stay".to_string()))
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(6, 12)))
        .await
        .expect("schedule");
    h.todos
        .set_reminder(
            h.account,
            id,
            Some(momentum_app::commands::ReminderInput {
                at: Utc.with_ymd_and_hms(2026, 6, 12, 8, 0, 0).unwrap(),
                timezone: "UTC".to_string(),
            }),
        )
        .await
        .expect("reminder");

    h.todos
        .set_deadline(h.account, id, Some(date(6, 20)))
        .await
        .expect("deadline");

    let summary = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.schedule.date(), Some(date(6, 12)));
    assert_eq!(
        summary.deadline.map(|deadline| deadline.date),
        Some(date(6, 20))
    );
    assert_eq!(summary.notes, "notes stay");
    assert!(summary.reminder.is_some());

    h.todos
        .set_deadline(h.account, id, None)
        .await
        .expect("clear deadline");
    let summary = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    assert!(summary.deadline.is_none());
    assert_eq!(
        summary.schedule.date(),
        Some(date(6, 12)),
        "schedule unchanged"
    );
    assert_eq!(summary.notes, "notes stay");
}

#[tokio::test]
async fn deadline_states_derive_from_the_local_date() {
    let h = harness();
    let id = h
        .todos
        .create(h.account, "Stateful", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .set_deadline(h.account, id, Some(date(6, 10)))
        .await
        .expect("deadline");
    let summary = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");

    assert_eq!(
        summary.deadline_state(date(6, 9)),
        Some(DeadlineState::Future)
    );
    assert_eq!(
        summary.deadline_state(date(6, 10)),
        Some(DeadlineState::DueToday)
    );
    assert_eq!(
        summary.deadline_state(date(6, 11)),
        Some(DeadlineState::Overdue)
    );

    let without = h
        .todos
        .create(h.account, "No deadline", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let summary = h
        .todos
        .get(h.account, without)
        .await
        .expect("get")
        .expect("todo");
    assert!(summary.deadline_state(date(6, 11)).is_none());
}

#[tokio::test]
async fn deadline_projection_lists_sorted_without_duplicating_identity() {
    let h = harness();
    let later = h
        .todos
        .create(h.account, "Later deadline", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let sooner = h
        .todos
        .create(h.account, "Sooner deadline", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let none = h
        .todos
        .create(h.account, "No deadline", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .set_deadline(h.account, later, Some(date(6, 30)))
        .await
        .expect("deadline");
    h.todos
        .set_deadline(h.account, sooner, Some(date(6, 15)))
        .await
        .expect("deadline");

    let listed: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Deadlines, 0)
        .await
        .expect("deadlines")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(listed, vec![sooner, later], "sorted by deadline date");
    assert!(!listed.contains(&none));

    let anytime: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Anytime, 0)
        .await
        .expect("anytime")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert!(
        anytime.contains(&sooner),
        "deadline items stay in their schedule projection"
    );
    let summary = h
        .todos
        .get(h.account, sooner)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        summary.deadline.map(|deadline| deadline.date),
        Some(date(6, 15)),
        "both projections reference the same deadline value"
    );
}
