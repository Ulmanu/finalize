//! Someday membership, reactivation, and parent/child independence tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
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

#[tokio::test]
async fn someday_items_leave_actionable_views_and_reactivate_explicitly() {
    let h = harness();
    let id = h
        .todos
        .create(h.account, "Deferred", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    h.todos
        .schedule(h.account, id, ScheduleInput::Someday)
        .await
        .expect("defer");
    let someday: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Someday, 0)
        .await
        .expect("someday")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(someday, vec![id]);
    let anytime = h
        .todos
        .list_at(h.account, TodoScope::Anytime, 0)
        .await
        .expect("anytime");
    assert!(!anytime.iter().any(|todo| todo.id == id));
    let today = h
        .todos
        .list_at(h.account, TodoScope::Today, 0)
        .await
        .expect("today");
    assert!(!today.iter().any(|todo| todo.id == id));

    h.todos
        .schedule(h.account, id, ScheduleInput::Anytime)
        .await
        .expect("reactivate");
    let someday = h
        .todos
        .list_at(h.account, TodoScope::Someday, 0)
        .await
        .expect("someday");
    assert!(someday.is_empty());
    let anytime: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Anytime, 0)
        .await
        .expect("anytime")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert!(anytime.contains(&id));

    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(15)))
        .await
        .expect("schedule future");
    let upcoming: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Upcoming, 0)
        .await
        .expect("upcoming")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert!(upcoming.contains(&id));
    assert_eq!(
        h.todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("todo")
            .schedule
            .date(),
        Some(date(15))
    );
}

#[tokio::test]
async fn someday_projects_are_members_and_reactivate_explicitly() {
    let h = harness();
    let id = h
        .projects
        .create(h.account, "Deferred project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .schedule(h.account, id, ScheduleInput::Someday)
        .await
        .expect("defer");

    let deferred = h.projects.someday(h.account).await.expect("someday");
    assert_eq!(deferred.len(), 1);
    assert_eq!(deferred[0].id, id);
    assert!(deferred[0].schedule.is_someday());

    h.projects
        .schedule(h.account, id, ScheduleInput::Anytime)
        .await
        .expect("reactivate");
    let deferred = h.projects.someday(h.account).await.expect("someday");
    assert!(deferred.is_empty());

    h.projects
        .schedule(h.account, id, ScheduleInput::On(date(12)))
        .await
        .expect("schedule future");
    let deferred = h.projects.someday(h.account).await.expect("someday");
    assert!(deferred.is_empty(), "date schedules leave Someday");
}

#[tokio::test]
async fn an_active_parent_does_not_reactivate_a_someday_child() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Active project", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let child = h
        .todos
        .create_in(
            h.account,
            "Deferred child",
            None,
            ContextRef::Project(project),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, child, ScheduleInput::Someday)
        .await
        .expect("defer child");

    let anytime = h
        .todos
        .list_at(h.account, TodoScope::Anytime, 0)
        .await
        .expect("anytime");
    assert!(
        !anytime.iter().any(|todo| todo.id == child),
        "an active project must not pull a Someday child into Anytime"
    );
    let someday: Vec<_> = h
        .todos
        .list_at(h.account, TodoScope::Someday, 0)
        .await
        .expect("someday")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(someday, vec![child]);

    let summary = h
        .todos
        .get(h.account, child)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.context, ContextRef::Project(project));
}
