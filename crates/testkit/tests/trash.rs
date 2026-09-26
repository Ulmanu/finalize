//! Soft deletion, restore fallback, and permanent deletion tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::areas::AreaService;
use momentum_app::commands::{CommandDispatcher, CommandError, ProjectDeletionPolicy};
use momentum_app::projects::ProjectService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::{ContextRef, Status};
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

#[tokio::test]
async fn deleting_supported_entities_moves_them_to_trash() {
    let h = harness();
    let todo = h
        .todos
        .create(h.account, "Trash me", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.delete(h.account, todo).await.expect("delete");
    let trash: Vec<_> = h
        .todos
        .list(h.account, TodoScope::Trash)
        .await
        .expect("trash")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(trash, vec![todo]);
    let inbox = h
        .todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox");
    assert!(!inbox.iter().any(|entry| entry.id == todo));

    let project = h
        .projects
        .create(h.account, "Trashed project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .delete(h.account, project, ProjectDeletionPolicy::DetachChildren)
        .await
        .expect("delete project");
    let trashed_projects = h.projects.trash(h.account).await.expect("trash");
    assert_eq!(trashed_projects.len(), 1);
    assert_eq!(trashed_projects[0].id, project);
    let active = h.projects.list(h.account).await.expect("list");
    assert!(active.is_empty());

    let area = h
        .areas
        .create(h.account, "Trashed area")
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.areas
        .delete(
            h.account,
            area,
            momentum_app::commands::AreaDeletionPolicy::Detach,
        )
        .await
        .expect("delete area");
    let trashed_areas = h.areas.trash(h.account).await.expect("trash");
    assert_eq!(trashed_areas.len(), 1);
    assert_eq!(trashed_areas[0].id, area);
}

#[tokio::test]
async fn restore_recovers_items_and_falls_back_when_context_is_gone() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Doomed", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let child = h
        .todos
        .create_in(h.account, "Orphan", None, ContextRef::Project(project))
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .delete(h.account, project, ProjectDeletionPolicy::TrashChildren)
        .await
        .expect("delete project");
    h.projects
        .purge(h.account, project)
        .await
        .expect("purge project");

    h.todos
        .restore(h.account, child)
        .await
        .expect("restore child");
    let summary = h
        .todos
        .get(h.account, child)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.status, Status::Open);
    assert_eq!(
        summary.context,
        ContextRef::Inbox,
        "restore falls back to Inbox when the prior context is gone"
    );
    assert!(summary.completed_at.is_none());

    let restored_project = h
        .projects
        .create(h.account, "Restored project", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    h.projects
        .delete(
            h.account,
            restored_project,
            ProjectDeletionPolicy::DetachChildren,
        )
        .await
        .expect("delete");
    h.projects
        .restore(h.account, restored_project)
        .await
        .expect("restore");
    let summary = h
        .projects
        .get(h.account, restored_project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.status, Status::Open);
}

#[tokio::test]
async fn permanent_deletion_requires_trash_and_empty_trash_purges_everything() {
    let h = harness();
    let active = h
        .todos
        .create(h.account, "Active", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let error = h
        .todos
        .purge(h.account, active)
        .await
        .expect_err("active items cannot be purged");
    assert!(matches!(error, CommandError::Validation(_)));

    let doomed = h
        .todos
        .create(h.account, "Doomed", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.delete(h.account, doomed).await.expect("delete");
    h.todos.purge(h.account, doomed).await.expect("purge");
    assert!(
        h.todos.get(h.account, doomed).await.expect("get").is_none(),
        "purged rows are gone from reads"
    );

    let second = h
        .todos
        .create(h.account, "Second", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.delete(h.account, second).await.expect("delete");
    let project = h
        .projects
        .create(h.account, "Project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .delete(h.account, project, ProjectDeletionPolicy::DetachChildren)
        .await
        .expect("delete");
    h.todos.empty_trash(h.account).await.expect("empty");
    assert!(h.todos.get(h.account, second).await.expect("get").is_none());
    assert!(h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .is_none());
    let trash = h
        .todos
        .list(h.account, TodoScope::Trash)
        .await
        .expect("trash");
    assert!(trash.is_empty());
    assert!(h.projects.trash(h.account).await.expect("trash").is_empty());
    assert!(
        !h.todos.get(h.account, active).await.expect("get").is_none(),
        "empty trash never touches active items"
    );
}
