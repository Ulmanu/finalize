//! Bulk command semantics over a multi-selection: per-item results and canonical state.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::bulk::{BulkAction, BulkService};
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::movement::Destination;
use momentum_app::projects::ProjectService;
use momentum_app::tags::TagService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::{ContextRef, Deadline, Schedule, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
    tags: Arc<TagService>,
    bulk: BulkService,
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
    let todos = Arc::new(TodoService::new(
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    ));
    let projects = Arc::new(ProjectService::new(
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock.clone(),
    ));
    let tags = Arc::new(TagService::new(
        dispatcher.clone(),
        Arc::new(MemoryTagReadStore::new(database.clone())),
        clock.clone(),
    ));
    let bulk = BulkService::new(Arc::clone(&todos), Some(Arc::clone(&tags)));

    Harness {
        todos,
        projects,
        tags,
        bulk,
        account: AccountId::new(),
    }
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

async fn create(h: &Harness, title: &str) -> EntityId {
    h.todos
        .create(h.account, title, None)
        .await
        .expect("create");
    h.todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == title)
        .expect("created todo")
        .id
}

async fn summary(h: &Harness, id: EntityId) -> momentum_app::todos::TodoSummary {
    h.todos
        .get(h.account, id)
        .await
        .expect("query")
        .expect("todo")
}

#[tokio::test]
async fn bulk_complete_reports_per_item_failures_and_preserves_others() {
    let h = harness();
    let a = create(&h, "A").await;
    let b = create(&h, "B").await;
    let c = create(&h, "C").await;
    let unknown = EntityId::new();

    let outcome = h
        .bulk
        .apply(h.account, &[a, b, unknown], &BulkAction::Complete)
        .await;

    assert_eq!(outcome.succeeded, vec![a, b]);
    assert_eq!(outcome.failed.len(), 1);
    assert_eq!(outcome.failed[0].id, unknown);
    assert!(!outcome.failed[0].message.is_empty());
    assert_eq!(outcome.processed(), 3);
    assert!(!outcome.is_full_success());

    assert_eq!(summary(&h, a).await.status, Status::Completed);
    assert_eq!(summary(&h, b).await.status, Status::Completed);
    assert_eq!(summary(&h, c).await.status, Status::Open);

    let full = h.bulk.apply(h.account, &[c], &BulkAction::Cancel).await;
    assert!(full.is_full_success());
    assert_eq!(summary(&h, c).await.status, Status::Canceled);
}

#[tokio::test]
async fn bulk_schedule_deadline_and_move_reach_every_selected_item() {
    let h = harness();
    let a = create(&h, "A").await;
    let b = create(&h, "B").await;

    h.bulk
        .apply(
            h.account,
            &[a, b],
            &BulkAction::Schedule(ScheduleInput::On(date(12))),
        )
        .await;
    assert_eq!(summary(&h, a).await.schedule, Schedule::on(date(12)));
    assert_eq!(summary(&h, b).await.schedule, Schedule::on(date(12)));

    h.bulk
        .apply(h.account, &[a, b], &BulkAction::SetDeadline(Some(date(20))))
        .await;
    assert_eq!(summary(&h, a).await.deadline, Some(Deadline::new(date(20))));

    h.projects
        .create(h.account, "Project", None)
        .await
        .expect("project");
    let project = h
        .projects
        .list(h.account)
        .await
        .expect("projects")
        .into_iter()
        .find(|project| project.title == "Project")
        .expect("created project")
        .id;
    let outcome = h
        .bulk
        .apply(
            h.account,
            &[a, b],
            &BulkAction::Move(Destination::Project(project)),
        )
        .await;
    assert!(
        outcome.is_full_success(),
        "move failures: {:?}",
        outcome.failed
    );
    assert_eq!(summary(&h, a).await.context, ContextRef::Project(project));
    assert_eq!(summary(&h, b).await.context, ContextRef::Project(project));
}

#[tokio::test]
async fn bulk_tag_assignment_and_delete_update_canonical_state() {
    let h = harness();
    let a = create(&h, "A").await;
    let b = create(&h, "B").await;

    h.tags.create(h.account, "Focus", None).await.expect("tag");
    let tag = h
        .tags
        .list(h.account)
        .await
        .expect("tags")
        .into_iter()
        .find(|tag| tag.title == "Focus")
        .expect("created tag")
        .id;
    let outcome = h
        .bulk
        .apply(h.account, &[a, b], &BulkAction::AssignTag(tag))
        .await;
    assert!(outcome.is_full_success());
    assert_eq!(summary(&h, a).await.tags, vec![tag]);
    assert_eq!(summary(&h, b).await.tags, vec![tag]);

    let outcome = h.bulk.apply(h.account, &[a, b], &BulkAction::Delete).await;
    assert!(outcome.is_full_success());
    assert!(h
        .todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .is_empty());
    let trashed: Vec<EntityId> = h
        .todos
        .list(h.account, TodoScope::Trash)
        .await
        .expect("trash")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    assert_eq!(trashed.len(), 2);
    assert!(trashed.contains(&a) && trashed.contains(&b));
}
