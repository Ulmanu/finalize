//! Tag hierarchy, assignment, and inherited-context tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::areas::AreaService;
use momentum_app::commands::{CommandDispatcher, CommandError, ReorderDirection};
use momentum_app::projects::ProjectService;
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
use momentum_domain::model::ContextRef;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::areas::MemoryAreaReadStore;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    tags: TagService,
    todos: TodoService,
    projects: ProjectService,
    areas: AreaService,
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
    let tags = TagService::new(
        dispatcher.clone(),
        Arc::new(MemoryTagReadStore::new(database.clone())),
        clock.clone(),
    );
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
        tags,
        todos,
        projects,
        areas,
        account: AccountId::new(),
    }
}

async fn create_tag(h: &Harness, title: &str, parent: Option<EntityId>) -> EntityId {
    h.tags
        .create(h.account, title, parent)
        .await
        .expect("create tag")
        .entity_id
        .expect("tag id")
}

#[tokio::test]
async fn hierarchy_nests_reorders_and_prevents_cycles() {
    let h = harness();
    let work = create_tag(&h, "Work", None).await;
    let deep = create_tag(&h, "Deep", Some(work)).await;
    let personal = create_tag(&h, "Personal", None).await;

    let listed = h.tags.list(h.account).await.expect("list");
    assert_eq!(listed.len(), 3);
    let deep_summary = listed.iter().find(|tag| tag.id == deep).expect("deep tag");
    assert_eq!(deep_summary.parent, Some(work));

    let error = h
        .tags
        .move_to_parent(h.account, work, Some(deep))
        .await
        .expect_err("cycle must be rejected");
    assert!(matches!(error, CommandError::Validation(_)));
    let error = h
        .tags
        .move_to_parent(h.account, work, Some(work))
        .await
        .expect_err("self-parent must be rejected");
    assert!(matches!(error, CommandError::Validation(_)));

    h.tags
        .reorder(h.account, personal, ReorderDirection::Up)
        .await
        .expect("reorder");
    let listed = h.tags.list(h.account).await.expect("list");
    let top_level: Vec<&str> = listed
        .iter()
        .filter(|tag| tag.parent.is_none())
        .map(|tag| tag.title.as_str())
        .collect();
    assert_eq!(top_level, vec!["Personal", "Work"]);

    h.tags.delete(h.account, work).await.expect("delete parent");
    let listed = h.tags.list(h.account).await.expect("list");
    assert!(listed.iter().all(|tag| tag.id != work));
    let deep_summary = listed
        .iter()
        .find(|tag| tag.id == deep)
        .expect("deep survives");
    assert_eq!(
        deep_summary.parent, None,
        "children are reparented to top level"
    );
}

#[tokio::test]
async fn assignment_covers_todos_projects_and_areas() {
    let h = harness();
    let tag = create_tag(&h, "Focus", None).await;
    let todo = h
        .todos
        .create(h.account, "Tag me", None)
        .await
        .expect("todo")
        .entity_id
        .expect("id");
    let project = h
        .projects
        .create(h.account, "Tagged project", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let area = h
        .projects
        .create(h.account, "Placeholder", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");

    h.tags
        .assign(h.account, todo, tag)
        .await
        .expect("assign todo");
    h.tags
        .assign(h.account, project, tag)
        .await
        .expect("assign project");
    h.tags
        .assign(h.account, area, tag)
        .await
        .expect("assign area");

    let listed = h.tags.list(h.account).await.expect("list");
    assert_eq!(listed[0].assigned_count, 3);

    h.tags
        .unassign(h.account, todo, tag)
        .await
        .expect("unassign");
    let summary = h
        .todos
        .get(h.account, todo)
        .await
        .expect("get")
        .expect("todo");
    assert!(summary.tags.is_empty());

    let error = h
        .tags
        .assign(h.account, todo, EntityId::new())
        .await
        .expect_err("unknown tag");
    assert!(matches!(error, CommandError::NotFound));
    let error = h
        .tags
        .assign(AccountId::new(), todo, tag)
        .await
        .expect_err("foreign account");
    assert!(matches!(error, CommandError::NotFound));
}

#[tokio::test]
async fn inherited_context_tags_flow_from_projects_and_areas() {
    let h = harness();
    let area_tag = create_tag(&h, "Area tag", None).await;
    let project_tag = create_tag(&h, "Project tag", None).await;
    let direct_tag = create_tag(&h, "Direct tag", None).await;

    let area = h
        .areas
        .create(h.account, "Home area")
        .await
        .expect("area")
        .entity_id
        .expect("id");
    h.tags
        .assign(h.account, area, area_tag)
        .await
        .expect("area tag");
    let project = h
        .projects
        .create(h.account, "Child project", Some(area))
        .await
        .expect("project")
        .entity_id
        .expect("id");
    h.tags
        .assign(h.account, project, project_tag)
        .await
        .expect("project tag");
    let todo = h
        .todos
        .create_in(
            h.account,
            "Inheriting to-do",
            None,
            ContextRef::Project(project),
        )
        .await
        .expect("todo")
        .entity_id
        .expect("id");
    h.tags
        .assign(h.account, todo, direct_tag)
        .await
        .expect("direct tag");

    let summary = h
        .todos
        .get(h.account, todo)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.tags, vec![direct_tag]);
    assert!(summary.inherited_tags.contains(&project_tag));
    assert!(summary.inherited_tags.contains(&area_tag));
    assert!(!summary.inherited_tags.contains(&direct_tag));
    assert!(
        !summary.tags.contains(&project_tag),
        "inheritance never writes synthetic direct assignments"
    );

    let inbox_todo = h
        .todos
        .create(h.account, "Loose", None)
        .await
        .expect("todo")
        .entity_id
        .expect("id");
    let summary = h
        .todos
        .get(h.account, inbox_todo)
        .await
        .expect("get")
        .expect("todo");
    assert!(summary.inherited_tags.is_empty());
}
