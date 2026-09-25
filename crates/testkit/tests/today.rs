//! Today projection membership, This Evening section, and manual ordering tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, CommandError, ReorderDirection, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    todos: TodoService,
    projects: ProjectService,
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
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    );
    let projects = ProjectService::new(
        dispatcher,
        Arc::new(MemoryProjectReadStore::new(database)),
        clock,
    );
    Harness {
        todos,
        projects,
        account: AccountId::new(),
    }
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

async fn today_ids(h: &Harness) -> Vec<momentum_domain::EntityId> {
    h.todos
        .list_at(h.account, TodoScope::Today, 0)
        .await
        .expect("today")
        .into_iter()
        .map(|todo| todo.id)
        .collect()
}

#[tokio::test]
async fn today_membership_is_derived_without_moving_items() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Container", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let in_project = h
        .todos
        .create_in(
            h.account,
            "Project item",
            None,
            ContextRef::Project(project),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, in_project, ScheduleInput::On(date(10)))
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
        .schedule(h.account, overdue, ScheduleInput::On(date(1)))
        .await
        .expect("schedule");

    let evening = h
        .todos
        .create(h.account, "Evening", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, evening, ScheduleInput::ThisEvening(date(10)))
        .await
        .expect("schedule");

    let someday = h
        .todos
        .create(h.account, "Someday", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, someday, ScheduleInput::Someday)
        .await
        .expect("schedule");

    let ids = today_ids(&h).await;
    assert!(ids.contains(&in_project), "project item joins Today");
    assert!(ids.contains(&overdue), "overdue stays in Today");
    assert!(ids.contains(&evening), "evening remains a Today item");
    assert!(!ids.contains(&someday));

    let summary = h
        .todos
        .get(h.account, in_project)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        summary.context,
        ContextRef::Project(project),
        "Today membership never moves the item out of its project"
    );
    let project_summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(project_summary.open_count, 1);
}

#[tokio::test]
async fn this_evening_orders_independently_from_the_day_section() {
    let h = harness();
    let mut day = Vec::new();
    let mut evening = Vec::new();
    for (title, schedule) in [
        ("Day A", ScheduleInput::On(date(10))),
        ("Day B", ScheduleInput::On(date(10))),
        ("Eve A", ScheduleInput::ThisEvening(date(10))),
        ("Eve B", ScheduleInput::ThisEvening(date(10))),
    ] {
        let id = h
            .todos
            .create(h.account, title, None)
            .await
            .expect("create")
            .entity_id
            .expect("id");
        h.todos
            .schedule(h.account, id, schedule)
            .await
            .expect("schedule");
        if title.starts_with("Eve") {
            evening.push(id);
        } else {
            day.push(id);
        }
    }

    h.todos
        .reorder_today(h.account, day[0], ReorderDirection::Down)
        .await
        .expect("move day item");
    let ids = today_ids(&h).await;
    let day_order: Vec<_> = ids.iter().filter(|id| day.contains(id)).copied().collect();
    let evening_order: Vec<_> = ids
        .iter()
        .filter(|id| evening.contains(id))
        .copied()
        .collect();
    assert_eq!(day_order, vec![day[1], day[0]], "day section reordered");
    assert_eq!(
        evening_order,
        vec![evening[0], evening[1]],
        "evening order untouched"
    );

    h.todos
        .reorder_today(h.account, evening[0], ReorderDirection::Down)
        .await
        .expect("move evening item");
    let ids = today_ids(&h).await;
    let day_order: Vec<_> = ids.iter().filter(|id| day.contains(id)).copied().collect();
    let evening_order: Vec<_> = ids
        .iter()
        .filter(|id| evening.contains(id))
        .copied()
        .collect();
    assert_eq!(day_order, vec![day[1], day[0]]);
    assert_eq!(evening_order, vec![evening[1], evening[0]]);
}

#[tokio::test]
async fn today_manual_order_is_independent_from_context_order() {
    let h = harness();
    let mut ids = Vec::new();
    for title in ["One", "Two", "Three"] {
        let id = h
            .todos
            .create(h.account, title, None)
            .await
            .expect("create")
            .entity_id
            .expect("id");
        h.todos
            .schedule(h.account, id, ScheduleInput::On(date(10)))
            .await
            .expect("schedule");
        ids.push(id);
    }

    let inbox_before: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Inbox, 0)
        .await
        .expect("inbox")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(inbox_before, ids, "context order is creation order");

    h.todos
        .reorder_today(h.account, ids[2], ReorderDirection::Up)
        .await
        .expect("move in Today");
    let ids_in_today = today_ids(&h).await;
    let today_order: Vec<_> = ids_in_today
        .iter()
        .filter(|id| ids.contains(id))
        .copied()
        .collect();
    assert_eq!(
        today_order,
        vec![ids[0], ids[2], ids[1]],
        "Today shows the manual order"
    );

    let inbox_after: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Inbox, 0)
        .await
        .expect("inbox")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(
        inbox_after, ids,
        "context ordering is unchanged by Today reordering"
    );

    let later = h
        .todos
        .create(h.account, "Tomorrow", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, later, ScheduleInput::On(date(11)))
        .await
        .expect("schedule");
    let error = h
        .todos
        .reorder_today(h.account, later, ReorderDirection::Up)
        .await
        .expect_err("only Today items can be reordered in Today");
    assert!(matches!(error, CommandError::Validation(_)));
}
