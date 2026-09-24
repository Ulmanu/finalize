//! Scheduling semantics: states, transitions, and local-date behavior.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::tags::TagService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    todos: TodoService,
    projects: ProjectService,
    tags: TagService,
    account: AccountId,
}

fn harness(now_utc: chrono::DateTime<Utc>) -> Harness {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(now_utc));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = TodoService::new(
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    );
    let projects = ProjectService::new(
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock.clone(),
    );
    let tags = TagService::new(
        dispatcher,
        Arc::new(MemoryTagReadStore::new(database)),
        clock,
    );
    Harness {
        todos,
        projects,
        tags,
        account: AccountId::new(),
    }
}

fn midnight() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 6, 1, 9, 0, 0).unwrap()
}

fn date(year: i32, month: u32, day: u32) -> LocalDate {
    LocalDate::from_ymd(year, month, day).expect("date")
}

#[tokio::test]
async fn schedule_states_define_projection_membership() {
    let h = harness(midnight());
    let today = date(2026, 6, 1);
    let yesterday = date(2026, 5, 31);
    let tomorrow = date(2026, 6, 2);

    let anytime = h
        .todos
        .create(h.account, "Anytime item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let someday = h
        .todos
        .create(h.account, "Someday item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, someday, ScheduleInput::Someday)
        .await
        .expect("someday");
    let scheduled_today = h
        .todos
        .create(h.account, "Due now", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, scheduled_today, ScheduleInput::On(today))
        .await
        .expect("schedule");
    let overdue = h
        .todos
        .create(h.account, "Overdue", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, overdue, ScheduleInput::On(yesterday))
        .await
        .expect("schedule");
    let evening = h
        .todos
        .create(h.account, "This evening", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, evening, ScheduleInput::ThisEvening(today))
        .await
        .expect("schedule");
    let future = h
        .todos
        .create(h.account, "Later", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, future, ScheduleInput::On(tomorrow))
        .await
        .expect("schedule");

    let today_ids: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Today, 0)
        .await
        .expect("today")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert!(today_ids.contains(&scheduled_today));
    assert!(
        today_ids.contains(&overdue),
        "overdue stays visible in Today"
    );
    assert!(today_ids.contains(&evening));
    assert!(!today_ids.contains(&anytime));
    assert!(!today_ids.contains(&someday));
    assert!(!today_ids.contains(&future));

    let upcoming_ids: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Upcoming, 0)
        .await
        .expect("upcoming")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(upcoming_ids, vec![future]);

    let anytime_ids: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Anytime, 0)
        .await
        .expect("anytime")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(anytime_ids, vec![anytime]);

    let someday_ids: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Someday, 0)
        .await
        .expect("someday")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(someday_ids, vec![someday]);
}

#[tokio::test]
async fn schedule_transitions_preserve_unrelated_metadata() {
    let h = harness(midnight());
    let project = h
        .projects
        .create(h.account, "Container", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let tag = h
        .tags
        .create(h.account, "Keep", None)
        .await
        .expect("tag")
        .entity_id
        .expect("id");
    let id = h
        .todos
        .create_in(
            h.account,
            "Travel",
            Some("pack light".to_string()),
            ContextRef::Project(project),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .set_deadline(h.account, id, Some(date(2026, 6, 10)))
        .await
        .expect("deadline");
    h.tags.assign(h.account, id, tag).await.expect("tag");

    for schedule in [
        ScheduleInput::On(date(2026, 6, 5)),
        ScheduleInput::Someday,
        ScheduleInput::ThisEvening(date(2026, 6, 1)),
        ScheduleInput::Anytime,
    ] {
        h.todos
            .schedule(h.account, id, schedule)
            .await
            .expect("reschedule");
        let summary = h
            .todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("todo");
        assert_eq!(summary.context, ContextRef::Project(project));
        assert_eq!(summary.tags, vec![tag]);
        assert_eq!(summary.notes, "pack light");
        assert_eq!(
            summary.deadline.map(|deadline| deadline.date),
            Some(date(2026, 6, 10))
        );
    }
}

#[tokio::test]
async fn local_date_semantics_follow_the_utc_offset() {
    // 16:00 UTC on June 1 is already June 2 in UTC+10 and still June 1 in UTC-5.
    let h = harness(Utc.with_ymd_and_hms(2026, 6, 1, 16, 0, 0).unwrap());
    let id = h
        .todos
        .create(h.account, "Boundary", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(2026, 6, 2)))
        .await
        .expect("schedule");

    let ahead_today: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Today, 600)
        .await
        .expect("today")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert!(
        ahead_today.contains(&id),
        "UTC+10 already sees June 2 as today"
    );
    let ahead_upcoming = h
        .todos
        .list_at(h.account, TodoScope::Upcoming, 600)
        .await
        .expect("upcoming");
    assert!(ahead_upcoming.is_empty());

    let behind_today = h
        .todos
        .list_at(h.account, TodoScope::Today, -300)
        .await
        .expect("today");
    assert!(behind_today.is_empty(), "UTC-5 still sees June 1");
    let behind_upcoming: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Upcoming, -300)
        .await
        .expect("upcoming")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(behind_upcoming, vec![id]);
}
