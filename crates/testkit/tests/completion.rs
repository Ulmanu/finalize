//! Completed/canceled semantics, reopen policy, and project completion safety tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, CommandError, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
use momentum_domain::model::{ContextRef, ProjectCompletionPolicy, Status};
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

#[tokio::test]
async fn completed_and_canceled_are_distinct_and_reopen_clears_terminal_metadata() {
    let h = harness();
    let completed = h
        .todos
        .create(h.account, "Completed item", Some("notes".to_string()))
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let canceled = h
        .todos
        .create(h.account, "Canceled item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    h.todos
        .complete(h.account, completed)
        .await
        .expect("complete");
    h.todos.cancel(h.account, canceled).await.expect("cancel");

    let summary = h
        .todos
        .get(h.account, completed)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.status, Status::Completed);
    assert!(
        summary.completed_at.is_some(),
        "completion timestamp recorded"
    );
    assert!(summary.canceled_at.is_none());

    let summary = h
        .todos
        .get(h.account, canceled)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.status, Status::Canceled);
    assert!(
        summary.canceled_at.is_some(),
        "cancellation timestamp recorded"
    );
    assert!(
        summary.completed_at.is_none(),
        "cancel does not set completion"
    );

    h.todos.reopen(h.account, completed).await.expect("reopen");
    h.todos.reopen(h.account, canceled).await.expect("reopen");
    let summary = h
        .todos
        .get(h.account, completed)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.status, Status::Open);
    assert!(summary.completed_at.is_none());
    assert!(summary.canceled_at.is_none());
    assert_eq!(
        summary.notes, "notes",
        "reopen preserves unrelated metadata"
    );
}

#[tokio::test]
async fn reopen_rejects_deleted_items_and_preserves_context() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Container", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let id = h
        .todos
        .create_in(h.account, "Trashed", None, ContextRef::Project(project))
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(
            h.account,
            id,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 20).expect("date")),
        )
        .await
        .expect("schedule");
    h.todos.complete(h.account, id).await.expect("complete");
    h.todos.delete(h.account, id).await.expect("delete");

    let error = h
        .todos
        .reopen(h.account, id)
        .await
        .expect_err("deleted items are restored, not reopened");
    assert!(matches!(error, CommandError::Validation(_)));
    let summary = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.status, Status::Deleted);
    assert_eq!(summary.context, ContextRef::Project(project));

    h.todos.restore(h.account, id).await.expect("restore");
    let summary = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.status, Status::Open);
    assert!(
        summary.completed_at.is_none(),
        "restore clears terminal metadata"
    );
}

#[tokio::test]
async fn project_completion_rejects_open_children_or_cascades_including_headings() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Safety", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let direct = h
        .todos
        .create_in(
            h.account,
            "Direct child",
            None,
            ContextRef::Project(project),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let heading = h
        .projects
        .create_heading(h.account, project, "Phase")
        .await
        .expect("heading")
        .entity_id
        .expect("id");
    let headed = h
        .todos
        .create_in(
            h.account,
            "Headed child",
            None,
            ContextRef::Heading(heading),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let error = h
        .projects
        .complete(h.account, project, ProjectCompletionPolicy::RejectWhileOpen)
        .await
        .expect_err("open children must reject");
    assert!(matches!(error, CommandError::Conflict { .. }));
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(
        summary.status,
        Status::Open,
        "project unchanged after reject"
    );
    for id in [direct, headed] {
        let todo = h
            .todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("todo");
        assert_eq!(todo.status, Status::Open, "no child was touched by reject");
    }

    h.projects
        .complete(
            h.account,
            project,
            ProjectCompletionPolicy::CompleteOpenChildren,
        )
        .await
        .expect("cascade");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.status, Status::Completed);
    assert!(summary.completed_at.is_some());
    for id in [direct, headed] {
        let todo = h
            .todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("todo");
        assert_eq!(
            todo.status,
            Status::Completed,
            "heading children are completed by cascade too"
        );
    }

    h.projects.reopen(h.account, project).await.expect("reopen");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.status, Status::Open);
    assert!(summary.completed_at.is_none());
}

#[tokio::test]
async fn canceling_a_project_preserves_open_children() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Cancel me", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let child = h
        .todos
        .create_in(
            h.account,
            "Living child",
            None,
            ContextRef::Project(project),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");

    h.projects.cancel(h.account, project).await.expect("cancel");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.status, Status::Canceled);
    assert!(summary.canceled_at.is_some());
    assert!(summary.completed_at.is_none());
    let todo = h
        .todos
        .get(h.account, child)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        todo.status,
        Status::Open,
        "canceling a project never silently loses child work"
    );
    assert_eq!(todo.context, ContextRef::Project(project));
}
