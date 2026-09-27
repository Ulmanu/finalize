//! Project-heading lifecycle, child movement, and scope tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, CommandError, ReorderDirection};
use momentum_app::projects::{ProjectOrderItem, ProjectService};
use momentum_app::todos::TodoService;
use momentum_domain::model::ContextRef;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    projects: ProjectService,
    todos: TodoService,
    account: AccountId,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 1, 9, 0, 0).unwrap(),
    ));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let projects = ProjectService::new(
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock.clone(),
    );
    let todos = TodoService::new(
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    );
    Harness {
        projects,
        todos,
        account: AccountId::new(),
    }
}

async fn create_project(h: &Harness, title: &str) -> EntityId {
    h.projects
        .create(h.account, title, None)
        .await
        .expect("create project")
        .entity_id
        .expect("project id")
}

async fn create_heading(h: &Harness, project: EntityId, title: &str) -> EntityId {
    h.projects
        .create_heading(h.account, project, title)
        .await
        .expect("create heading")
        .entity_id
        .expect("heading id")
}

async fn create_todo_in(h: &Harness, context: ContextRef, title: &str) -> EntityId {
    h.todos
        .create_in(h.account, title, None, context)
        .await
        .expect("create todo")
        .entity_id
        .expect("todo id")
}

async fn order_titles(h: &Harness, project: EntityId) -> Vec<String> {
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get project")
        .expect("project");
    summary
        .order
        .into_iter()
        .map(|item| match item {
            ProjectOrderItem::Heading(id) => summary
                .headings
                .iter()
                .find(|heading| heading.id == id)
                .map_or_else(|| "?".to_string(), |heading| heading.title.clone()),
            ProjectOrderItem::Todo(_) => "(todo)".to_string(),
        })
        .collect()
}

#[tokio::test]
async fn heading_lifecycle_create_rename_archive_restore() {
    let h = harness();
    let project = create_project(&h, "Roadmap").await;
    let heading = create_heading(&h, project, "Phase one").await;

    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.headings.len(), 1);
    assert_eq!(summary.headings[0].title, "Phase one");
    assert!(!summary.headings[0].archived);
    assert_eq!(summary.order, vec![ProjectOrderItem::Heading(heading)]);

    h.projects
        .rename_heading(h.account, heading, "Phase 1")
        .await
        .expect("rename");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.headings[0].title, "Phase 1");

    let child = create_todo_in(&h, ContextRef::Heading(heading), "Child stays").await;
    h.projects
        .archive_heading(h.account, heading)
        .await
        .expect("archive");

    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert!(summary.headings[0].archived);
    assert!(
        !summary.order.contains(&ProjectOrderItem::Heading(heading)),
        "archived headings leave the active order"
    );
    let child_summary = h
        .todos
        .get(h.account, child)
        .await
        .expect("get todo")
        .expect("todo");
    assert_eq!(child_summary.status, momentum_domain::model::Status::Open);
    assert_eq!(child_summary.context, ContextRef::Heading(heading));

    h.projects
        .restore_heading(h.account, heading)
        .await
        .expect("restore");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert!(!summary.headings[0].archived);
    assert_eq!(summary.order, vec![ProjectOrderItem::Heading(heading)]);
}

#[tokio::test]
async fn reorder_heading_moves_children_as_a_unit() {
    let h = harness();
    let project = create_project(&h, "Sections").await;
    let first = create_heading(&h, project, "First").await;
    let second = create_heading(&h, project, "Second").await;
    let first_child_a = create_todo_in(&h, ContextRef::Heading(first), "A1").await;
    let first_child_b = create_todo_in(&h, ContextRef::Heading(first), "A2").await;
    let second_child = create_todo_in(&h, ContextRef::Heading(second), "B1").await;

    assert_eq!(order_titles(&h, project).await, vec!["First", "Second"]);

    h.projects
        .reorder_heading(h.account, first, ReorderDirection::Down)
        .await
        .expect("move down");
    assert_eq!(order_titles(&h, project).await, vec!["Second", "First"]);

    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    let first_summary = summary
        .headings
        .iter()
        .find(|heading| heading.id == first)
        .expect("first heading");
    assert_eq!(
        first_summary.todo_ids,
        vec![first_child_a, first_child_b],
        "children keep order under the moved heading"
    );
    let second_summary = summary
        .headings
        .iter()
        .find(|heading| heading.id == second)
        .expect("second heading");
    assert_eq!(second_summary.todo_ids, vec![second_child]);

    h.projects
        .reorder_heading(h.account, first, ReorderDirection::Up)
        .await
        .expect("move up");
    assert_eq!(order_titles(&h, project).await, vec!["First", "Second"]);

    h.projects
        .reorder_heading(h.account, second, ReorderDirection::Down)
        .await
        .expect("move past end is a no-op");
    assert_eq!(order_titles(&h, project).await, vec!["First", "Second"]);
}

#[tokio::test]
async fn headings_are_scoped_to_their_project() {
    let h = harness();
    let project = create_project(&h, "Scoped").await;
    let other = create_project(&h, "Other").await;
    let heading = create_heading(&h, project, "Only here").await;

    let summary = h
        .projects
        .get(h.account, other)
        .await
        .expect("get")
        .expect("project");
    assert!(summary.headings.is_empty());
    assert!(summary.order.is_empty());

    let missing = EntityId::new();
    let error = h
        .todos
        .create_in(h.account, "Nowhere", None, ContextRef::Heading(missing))
        .await
        .expect_err("unknown heading context must be rejected");
    assert!(matches!(error, CommandError::Validation(_)));

    let todo = create_todo_in(&h, ContextRef::Inbox, "Wanderer").await;
    let error = h
        .todos
        .move_to(h.account, todo, ContextRef::Heading(missing))
        .await
        .expect_err("moving to an unknown heading must be rejected");
    assert!(
        matches!(error, CommandError::NotFound),
        "the move engine reports unknown destinations as not-found"
    );

    let stranger = AccountId::new();
    let error = h
        .projects
        .rename_heading(stranger, heading, "Hijack")
        .await
        .expect_err("foreign account must not see the heading");
    assert!(matches!(error, CommandError::NotFound));
}

#[tokio::test]
async fn create_heading_requires_a_live_project() {
    let h = harness();
    let missing = EntityId::new();
    let error = h
        .projects
        .create_heading(h.account, missing, "Orphan")
        .await
        .expect_err("unknown project must be rejected");
    assert!(matches!(error, CommandError::NotFound));

    let project = create_project(&h, "Disposable").await;
    h.projects
        .delete(
            h.account,
            project,
            momentum_app::commands::ProjectDeletionPolicy::DetachChildren,
        )
        .await
        .expect("delete");
    let error = h
        .projects
        .create_heading(h.account, project, "After delete")
        .await
        .expect_err("deleted project must be rejected");
    assert!(matches!(error, CommandError::NotFound));
}
