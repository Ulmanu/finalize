//! Cross-account isolation: every user-owned family scopes reads and writes by account.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::api_tokens::{ApiScope, ApiTokenService};
use momentum_app::areas::AreaService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::preferences::{effective, PreferenceScope, PreferenceStore, PreferenceValue};
use momentum_app::projects::ProjectService;
use momentum_app::tags::TagService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::api_tokens::MemoryApiTokenStore;
use momentum_testkit::areas::MemoryAreaReadStore;
use momentum_testkit::auth::{DeterministicTokenGenerator, TestClock};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::preferences::MemoryPreferenceStore;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    database: MemoryDatabase,
    todos: TodoService,
    projects: ProjectService,
    areas: AreaService,
    tags: TagService,
    preferences: Arc<MemoryPreferenceStore>,
    tokens: ApiTokenService,
    alice: AccountId,
    bob: AccountId,
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
        dispatcher.clone(),
        Arc::new(MemoryAreaReadStore::new(database.clone())),
        clock.clone(),
    );
    let tags = TagService::new(
        dispatcher,
        Arc::new(MemoryTagReadStore::new(database.clone())),
        clock,
    );
    Harness {
        database,
        todos,
        projects,
        areas,
        tags,
        preferences: Arc::new(MemoryPreferenceStore::new()),
        tokens: ApiTokenService::new(
            Arc::new(MemoryApiTokenStore::new()),
            Arc::new(DeterministicTokenGenerator::new()),
        ),
        alice: AccountId::new(),
        bob: AccountId::new(),
    }
}

#[tokio::test]
async fn todo_project_area_and_tag_families_are_account_scoped() {
    let h = harness();

    h.todos
        .create(h.alice, "Alice task", None)
        .await
        .expect("todo");
    let todo = h
        .todos
        .list(h.alice, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;
    h.projects
        .create(h.alice, "Alice project", None)
        .await
        .expect("project");
    let project = h
        .projects
        .list(h.alice)
        .await
        .expect("projects")
        .into_iter()
        .next()
        .expect("project")
        .id;
    h.areas.create(h.alice, "Alice area").await.expect("area");
    let area = h
        .areas
        .list(h.alice)
        .await
        .expect("areas")
        .into_iter()
        .next()
        .expect("area")
        .id;
    h.tags
        .create(h.alice, "Alice tag", None)
        .await
        .expect("tag");
    let tag = h
        .tags
        .list(h.alice)
        .await
        .expect("tags")
        .into_iter()
        .next()
        .expect("tag")
        .id;

    // Bob sees nothing of Alice's.
    assert!(h
        .todos
        .list(h.bob, TodoScope::Inbox)
        .await
        .expect("inbox")
        .is_empty());
    assert!(h.projects.list(h.bob).await.expect("projects").is_empty());
    assert!(h.areas.list(h.bob).await.expect("areas").is_empty());
    assert!(h.tags.list(h.bob).await.expect("tags").is_empty());
    assert!(h.todos.get(h.bob, todo).await.expect("get").is_none());
    assert!(h.projects.get(h.bob, project).await.expect("get").is_none());
    assert!(h.areas.get(h.bob, area).await.expect("get").is_none());

    // Bob cannot mutate Alice's entities or reference her ids.
    assert!(h.todos.complete(h.bob, todo).await.is_err());
    assert!(h.todos.delete(h.bob, todo).await.is_err());
    assert!(h
        .projects
        .delete(
            h.bob,
            project,
            momentum_app::commands::ProjectDeletionPolicy::DetachChildren
        )
        .await
        .is_err());
    assert!(h
        .areas
        .delete(
            h.bob,
            area,
            momentum_app::commands::AreaDeletionPolicy::Detach,
        )
        .await
        .is_err());
    assert!(h.tags.assign(h.bob, EntityId::new(), tag).await.is_err());
    assert!(h.tags.rename(h.bob, tag, "Stolen").await.is_err());

    // Alice's entities are untouched.
    assert_eq!(
        h.todos
            .get(h.alice, todo)
            .await
            .expect("get")
            .expect("todo")
            .title,
        "Alice task"
    );
    assert_eq!(
        h.tags.list(h.alice).await.expect("tags")[0].title,
        "Alice tag"
    );
    let _ = h.database;
}

#[tokio::test]
async fn preferences_and_api_tokens_are_account_scoped() {
    let h = harness();
    h.preferences
        .save(
            h.alice,
            "general.timezone",
            PreferenceScope::Account,
            "Europe/Berlin",
        )
        .await
        .expect("save");

    assert_eq!(
        effective(&*h.preferences, h.alice, "general.timezone")
            .await
            .expect("effective"),
        Some(PreferenceValue::Text("Europe/Berlin".to_string()))
    );
    assert_eq!(
        effective(&*h.preferences, h.bob, "general.timezone")
            .await
            .expect("effective"),
        Some(PreferenceValue::Text("UTC".to_string()))
    );

    let created = h
        .tokens
        .create(h.alice, "Alice automation", &[ApiScope::Read], Utc::now())
        .await
        .expect("token");
    let principal = h
        .tokens
        .authenticate(&created.token, Utc::now())
        .await
        .expect("auth")
        .expect("principal");
    assert_eq!(principal.account_id, h.alice);
    assert!(h.tokens.list(h.bob).await.expect("list").is_empty());
    assert!(h
        .tokens
        .revoke(h.bob, created.summary.id, Utc::now())
        .await
        .expect("revoke")
        .eq(&false));
}
