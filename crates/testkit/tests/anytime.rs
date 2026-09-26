//! Anytime actionable membership, exclusions, and context grouping tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::areas::AreaService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::areas::MemoryAreaReadStore;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    todos: TodoService,
    projects: ProjectService,
    areas: AreaService,
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
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock.clone(),
    );
    let areas = AreaService::new(
        dispatcher,
        Arc::new(MemoryAreaReadStore::new(database)),
        clock,
    );
    Harness {
        todos,
        projects,
        areas,
        account: AccountId::new(),
    }
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

#[tokio::test]
async fn anytime_includes_actionable_items_and_excludes_inactive_ones() {
    let h = harness();
    let anytime = h
        .todos
        .create(h.account, "Plain any time", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let future = h
        .todos
        .create(h.account, "Future", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, future, ScheduleInput::On(date(20)))
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

    let completed = h
        .todos
        .create(h.account, "Completed", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .complete(h.account, completed)
        .await
        .expect("complete");

    let canceled = h
        .todos
        .create(h.account, "Canceled", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.cancel(h.account, canceled).await.expect("cancel");

    let deleted = h
        .todos
        .create(h.account, "Deleted", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.delete(h.account, deleted).await.expect("delete");

    let agenda = h.todos.anytime_agenda(h.account).await.expect("agenda");
    let ids: Vec<_> = agenda
        .groups
        .iter()
        .flat_map(|group| group.todos.iter().map(|todo| todo.id))
        .collect();
    assert!(ids.contains(&anytime));
    for excluded in [future, someday, completed, canceled, deleted] {
        assert!(!ids.contains(&excluded), "excluded {excluded}");
    }
}

#[tokio::test]
async fn anytime_groups_by_existing_context_without_creating_parents() {
    let h = harness();
    let area = h
        .areas
        .create(h.account, "Home area")
        .await
        .expect("area")
        .entity_id
        .expect("id");
    let project = h
        .projects
        .create(h.account, "Project", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");

    let inbox_item = h
        .todos
        .create(h.account, "Loose item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let area_item = h
        .todos
        .create_in(h.account, "Area item", None, ContextRef::Area(area))
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let project_item = h
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

    let agenda = h.todos.anytime_agenda(h.account).await.expect("agenda");
    assert_eq!(agenda.groups.len(), 3);
    assert_eq!(agenda.groups[0].context, ContextRef::Inbox);
    assert_eq!(agenda.groups[1].context, ContextRef::Area(area));
    assert_eq!(agenda.groups[2].context, ContextRef::Project(project));
    assert_eq!(agenda.groups[0].todos[0].id, inbox_item);
    assert_eq!(agenda.groups[1].todos[0].id, area_item);
    assert_eq!(agenda.groups[2].todos[0].id, project_item);

    for (id, context) in [
        (inbox_item, ContextRef::Inbox),
        (area_item, ContextRef::Area(area)),
        (project_item, ContextRef::Project(project)),
    ] {
        let summary = h
            .todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("todo");
        assert_eq!(
            summary.context, context,
            "rendering the projection never rewrites the context"
        );
    }
}
