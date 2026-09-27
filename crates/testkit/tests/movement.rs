//! Authoritative move engine tests: destination matrix, preservation, and adapter reuse.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::areas::AreaService;
use momentum_app::commands::{
    Command, CommandDispatcher, CommandEnvelope, CommandError, ScheduleInput,
};
use momentum_app::movement::{self, Destination, Movable};
use momentum_app::projects::{ProjectOrderItem, ProjectService};
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::areas::MemoryAreaReadStore;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    dispatcher: Arc<CommandDispatcher>,
    todos: TodoService,
    projects: ProjectService,
    areas: AreaService,
    tags: TagService,
    account: AccountId,
    now: chrono::DateTime<Utc>,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let clock = Arc::new(TestClock::new(now));
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
        dispatcher.clone(),
        Arc::new(MemoryTagReadStore::new(database)),
        clock,
    );
    Harness {
        dispatcher,
        todos,
        projects,
        areas,
        tags,
        account: AccountId::new(),
        now,
    }
}

#[test]
fn destination_matrix_is_single_sourced() {
    let id = EntityId::new();
    assert!(movement::is_valid(Movable::Todo, Destination::Inbox));
    assert!(movement::is_valid(Movable::Todo, Destination::Area(id)));
    assert!(movement::is_valid(Movable::Todo, Destination::Project(id)));
    assert!(movement::is_valid(Movable::Todo, Destination::Heading(id)));

    assert!(movement::is_valid(Movable::Project, Destination::Inbox));
    assert!(movement::is_valid(Movable::Project, Destination::Area(id)));
    assert!(!movement::is_valid(
        Movable::Project,
        Destination::Project(id)
    ));
    assert!(!movement::is_valid(
        Movable::Project,
        Destination::Heading(id)
    ));

    assert!(movement::is_valid(
        Movable::Heading,
        Destination::Project(id)
    ));
    assert!(!movement::is_valid(Movable::Heading, Destination::Inbox));
    assert!(!movement::is_valid(Movable::Heading, Destination::Area(id)));
    assert!(!movement::is_valid(
        Movable::Heading,
        Destination::Heading(id)
    ));

    let error = movement::validate(Movable::Heading, Destination::Inbox)
        .expect_err("headings cannot move to the Inbox");
    assert!(matches!(error, CommandError::Validation(_)));
}

#[tokio::test]
async fn move_preserves_unrelated_state_and_validates_destinations() {
    let h = harness();
    let source = h
        .projects
        .create(h.account, "Source", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let target = h
        .projects
        .create(h.account, "Target", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let tag = h
        .tags
        .create(h.account, "Keep", None)
        .await
        .expect("tag")
        .entity_id
        .expect("id");
    let id = h
        .todos
        .create_in(
            h.account,
            "Traveler",
            Some("notes stay".to_string()),
            ContextRef::Project(source),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .set_deadline(
            h.account,
            id,
            Some(LocalDate::from_ymd(2026, 6, 30).expect("date")),
        )
        .await
        .expect("deadline");
    h.todos
        .schedule(
            h.account,
            id,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 20).expect("date")),
        )
        .await
        .expect("schedule");
    h.tags.assign(h.account, id, tag).await.expect("tag");
    h.todos
        .add_checklist_item(h.account, id, "Step")
        .await
        .expect("checklist");

    let before = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    h.todos
        .move_to(h.account, id, ContextRef::Project(target))
        .await
        .expect("move");
    let after = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(after.id, before.id, "identity preserved");
    assert_eq!(after.title, before.title);
    assert_eq!(after.notes, "notes stay");
    assert_eq!(after.tags, before.tags);
    assert_eq!(after.deadline, before.deadline);
    assert_eq!(after.schedule.date(), before.schedule.date());
    assert_eq!(after.checklist.len(), 1);
    assert_eq!(after.context, ContextRef::Project(target));

    let error = h
        .todos
        .move_to(h.account, id, ContextRef::Project(EntityId::new()))
        .await
        .expect_err("unknown destination");
    assert!(matches!(error, CommandError::NotFound));
    assert_eq!(
        h.todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("todo")
            .context,
        ContextRef::Project(target),
        "failed moves leave state untouched"
    );
}

#[tokio::test]
async fn headings_move_between_projects_with_children_following() {
    let h = harness();
    let source = h
        .projects
        .create(h.account, "Source", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let target = h
        .projects
        .create(h.account, "Target", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let heading = h
        .projects
        .create_heading(h.account, source, "Phase")
        .await
        .expect("heading")
        .entity_id
        .expect("id");
    let child = h
        .todos
        .create_in(h.account, "Child", None, ContextRef::Heading(heading))
        .await
        .expect("create")
        .entity_id
        .expect("id");

    h.projects
        .move_heading(h.account, heading, target)
        .await
        .expect("move heading");
    let source_summary = h
        .projects
        .get(h.account, source)
        .await
        .expect("get")
        .expect("project");
    let target_summary = h
        .projects
        .get(h.account, target)
        .await
        .expect("get")
        .expect("project");
    assert!(source_summary.headings.is_empty());
    assert_eq!(target_summary.headings.len(), 1);
    assert_eq!(target_summary.headings[0].id, heading);
    assert_eq!(target_summary.headings[0].todo_ids, vec![child]);
    assert_eq!(
        target_summary.order,
        vec![ProjectOrderItem::Heading(heading)]
    );
    let child_summary = h
        .todos
        .get(h.account, child)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(child_summary.context, ContextRef::Heading(heading));

    let error = h
        .projects
        .move_heading(h.account, heading, EntityId::new())
        .await
        .expect_err("unknown project destination");
    assert!(matches!(error, CommandError::NotFound));
}

#[tokio::test]
async fn area_destinations_are_validated_for_todos_and_projects() {
    let h = harness();
    let area = h
        .areas
        .create(h.account, "Home")
        .await
        .expect("area")
        .entity_id
        .expect("id");
    let todo = h
        .todos
        .create(h.account, "In area", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .move_to(h.account, todo, ContextRef::Area(area))
        .await
        .expect("todo to area");
    let summary = h
        .todos
        .get(h.account, todo)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.context, ContextRef::Area(area));

    let project = h
        .projects
        .create(h.account, "Area project", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    h.projects
        .move_to_area(h.account, project, Some(area))
        .await
        .expect("project to area");
    let summary = h
        .projects
        .get(h.account, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.area, Some(area));

    let error = h
        .projects
        .move_to_area(h.account, project, Some(EntityId::new()))
        .await
        .expect_err("unknown area destination");
    assert!(matches!(error, CommandError::NotFound));
}

#[tokio::test]
async fn adapters_share_the_same_move_operation() {
    let h = harness();
    let target = h
        .projects
        .create(h.account, "Target", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");

    let via_service = h
        .todos
        .create(h.account, "Service path", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .move_to(h.account, via_service, ContextRef::Project(target))
        .await
        .expect("service move");

    let via_command = h
        .todos
        .create(h.account, "Command path", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.dispatcher
        .execute(CommandEnvelope::new(
            h.account,
            Command::MoveTodo {
                id: via_command,
                context: ContextRef::Project(target),
            },
            h.now,
        ))
        .await
        .expect("command move");

    let service_summary = h
        .todos
        .get(h.account, via_service)
        .await
        .expect("get")
        .expect("todo");
    let command_summary = h
        .todos
        .get(h.account, via_command)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(service_summary.context, command_summary.context);
    assert_eq!(service_summary.status, command_summary.status);
    assert_eq!(
        service_summary.context,
        ContextRef::Project(target),
        "both adapter paths reach the same destination"
    );
}
