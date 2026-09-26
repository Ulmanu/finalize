//! Tomorrow, logged projects, and repeating-view read tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::recurring::RecurringService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::recurring::MemoryRecurrenceReadStore;
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
async fn tomorrow_exposes_only_next_day_items() {
    let h = harness();
    let mut scheduled = Vec::new();
    for (title, schedule, expected) in [
        ("Tomorrow day", ScheduleInput::On(date(11)), true),
        (
            "Tomorrow evening",
            ScheduleInput::ThisEvening(date(11)),
            true,
        ),
        ("Today", ScheduleInput::On(date(10)), false),
        ("Day after", ScheduleInput::On(date(12)), false),
        ("Someday", ScheduleInput::Someday, false),
        ("Anytime", ScheduleInput::Anytime, false),
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
        scheduled.push((id, expected));
    }

    let tomorrow: Vec<EntityId> = h
        .todos
        .list_at(h.account, TodoScope::Tomorrow, 0)
        .await
        .expect("tomorrow")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    for (id, expected) in scheduled {
        assert_eq!(tomorrow.contains(&id), expected, "item {id}");
    }
    assert_eq!(tomorrow.len(), 2);
}

#[tokio::test]
async fn logged_projects_returns_terminal_projects_only() {
    let h = harness();
    let open = h
        .projects
        .create(h.account, "Open", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let completed = h
        .projects
        .create(h.account, "Completed", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .complete(
            h.account,
            completed,
            momentum_domain::model::ProjectCompletionPolicy::CompleteOpenChildren,
        )
        .await
        .expect("complete");
    let canceled = h
        .projects
        .create(h.account, "Canceled", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .cancel(h.account, canceled)
        .await
        .expect("cancel");

    let logged = h.projects.logged(h.account).await.expect("logged");
    let ids: Vec<EntityId> = logged.iter().map(|project| project.id).collect();
    assert!(ids.contains(&completed));
    assert!(ids.contains(&canceled));
    assert!(
        !ids.contains(&open),
        "open projects stay out of Logged Projects"
    );
    assert_eq!(ids.len(), 2);
}

#[tokio::test]
async fn repeating_view_lists_account_templates_with_next_occurrence() {
    let store = Arc::new(MemoryRecurrenceReadStore::new());
    let account = AccountId::new();
    let other = AccountId::new();
    let service = RecurringService::new(store.clone());
    store.add(account, EntityId::new(), "Weekly review", Some(date(17)));
    store.add(account, EntityId::new(), "Monthly report", None);
    store.add(other, EntityId::new(), "Other account", Some(date(20)));

    let listed = service.list(account).await.expect("list");
    assert_eq!(listed.len(), 2);
    assert!(listed
        .iter()
        .any(|template| template.title == "Weekly review"
            && template.next_occurrence == Some(date(17))));
    assert!(listed
        .iter()
        .any(|template| template.title == "Monthly report" && template.next_occurrence.is_none()));
    assert!(
        !listed
            .iter()
            .any(|template| template.title == "Other account"),
        "templates are account-scoped"
    );
}
