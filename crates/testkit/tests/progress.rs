//! Derived project and Today progress tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
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

async fn child(
    h: &Harness,
    project: momentum_domain::EntityId,
    title: &str,
) -> momentum_domain::EntityId {
    h.todos
        .create_in(h.account, title, None, ContextRef::Project(project))
        .await
        .expect("create")
        .entity_id
        .expect("id")
}

#[tokio::test]
async fn project_progress_is_derived_from_eligible_children_only() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Progress", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.eligible_count(), 0);
    assert!(
        summary.progress().is_none(),
        "zero items is a neutral state"
    );

    let first = child(&h, project, "First").await;
    let second = child(&h, project, "Second").await;
    h.todos.complete(h.account, second).await.expect("complete");

    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.eligible_count(), 2);
    assert_eq!(summary.completed_count, 1);
    let ratio = summary.progress().expect("ratio");
    assert!((ratio - 0.5).abs() < f32::EPSILON);

    h.todos.complete(h.account, first).await.expect("complete");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.progress(), Some(1.0));

    let canceled = child(&h, project, "Canceled").await;
    h.todos.cancel(h.account, canceled).await.expect("cancel");
    let deleted = child(&h, project, "Deleted").await;
    h.todos.delete(h.account, deleted).await.expect("delete");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(
        summary.eligible_count(),
        2,
        "canceled and deleted children are excluded from progress"
    );
    assert_eq!(summary.completed_count, 2);
    assert_eq!(summary.progress(), Some(1.0));
}

#[tokio::test]
async fn today_progress_tracks_completable_items_and_ignores_excluded_rows() {
    let h = harness();
    let mut open_items = Vec::new();
    for title in ["Open one", "Open two"] {
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
        open_items.push(id);
    }
    let done = h
        .todos
        .create(h.account, "Done today", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, done, ScheduleInput::ThisEvening(date(10)))
        .await
        .expect("schedule");
    h.todos.complete(h.account, done).await.expect("complete");

    let canceled = h
        .todos
        .create(h.account, "Canceled", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, canceled, ScheduleInput::On(date(10)))
        .await
        .expect("schedule");
    h.todos.cancel(h.account, canceled).await.expect("cancel");

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

    let future = h
        .todos
        .create(h.account, "Future", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, future, ScheduleInput::On(date(15)))
        .await
        .expect("schedule");

    let progress = h.todos.today_progress(h.account).await.expect("progress");
    assert_eq!(progress.total, 3, "open two + completed evening one");
    assert_eq!(progress.completed, 1);
    assert_eq!(progress.percent(), Some(33));

    h.todos
        .complete(h.account, open_items[0])
        .await
        .expect("complete");
    let progress = h.todos.today_progress(h.account).await.expect("progress");
    assert_eq!(progress.completed, 2);
    assert_eq!(progress.percent(), Some(67));

    h.todos
        .reopen(h.account, open_items[0])
        .await
        .expect("reopen");
    let progress = h.todos.today_progress(h.account).await.expect("progress");
    assert_eq!(progress.completed, 1, "reopening updates progress again");
}

#[tokio::test]
async fn neutral_today_progress_reports_no_indicator() {
    let h = harness();
    let progress = h.todos.today_progress(h.account).await.expect("progress");
    assert!(progress.is_neutral());
    assert_eq!(progress.percent(), None);
}
