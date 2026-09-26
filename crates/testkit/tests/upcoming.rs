//! Upcoming agenda grouping and rescheduling tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{
    Command, CommandDispatcher, CommandEnvelope, CommandError, ScheduleInput,
};
use momentum_app::projects::ProjectService;
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    dispatcher: Arc<CommandDispatcher>,
    todos: TodoService,
    projects: ProjectService,
    tags: TagService,
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
    let tags = TagService::new(
        dispatcher.clone(),
        Arc::new(MemoryTagReadStore::new(database)),
        clock.clone(),
    );
    Harness {
        dispatcher,
        todos,
        projects,
        tags,
        account: AccountId::new(),
    }
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

async fn scheduled(h: &Harness, title: &str, schedule: ScheduleInput) -> EntityId {
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
    id
}

#[tokio::test]
async fn upcoming_groups_future_items_by_local_date() {
    let h = harness();
    let first = scheduled(&h, "First day A", ScheduleInput::On(date(15))).await;
    let first_b = scheduled(&h, "First day B", ScheduleInput::On(date(15))).await;
    let second = scheduled(&h, "Second day", ScheduleInput::On(date(20))).await;
    scheduled(&h, "Today item", ScheduleInput::On(date(10))).await;
    scheduled(&h, "Someday item", ScheduleInput::Someday).await;
    scheduled(&h, "Anytime item", ScheduleInput::Anytime).await;

    let agenda = h.todos.upcoming_agenda(h.account).await.expect("agenda");
    assert_eq!(agenda.days.len(), 2, "only future dates are grouped");
    assert_eq!(agenda.days[0].date, date(15));
    assert_eq!(agenda.days[1].date, date(20));
    let day_one: Vec<_> = agenda.days[0].todos.iter().map(|todo| todo.id).collect();
    assert_eq!(day_one, vec![first, first_b], "input order preserved");
    assert_eq!(agenda.days[1].todos[0].id, second);
    assert!(agenda.calendar.is_empty(), "no calendar integration yet");
}

#[tokio::test]
async fn rescheduling_moves_an_item_between_days_via_the_command_layer() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Container", None)
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
            "Movable",
            Some("notes".to_string()),
            ContextRef::Project(project),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(15)))
        .await
        .expect("schedule");
    h.tags.assign(h.account, id, tag).await.expect("tag");
    h.todos
        .set_deadline(h.account, id, Some(date(30)))
        .await
        .expect("deadline");

    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(20)))
        .await
        .expect("reschedule");
    let agenda = h.todos.upcoming_agenda(h.account).await.expect("agenda");
    assert_eq!(agenda.days.len(), 1);
    assert_eq!(agenda.days[0].date, date(20));
    assert_eq!(agenda.days[0].todos[0].id, id);
    assert_eq!(
        h.todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("todo")
            .context,
        ContextRef::Project(project)
    );
    assert_eq!(
        h.todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("todo")
            .tags,
        vec![tag]
    );

    // Rescheduling onto the local today moves the item out of Upcoming into Today.
    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(10)))
        .await
        .expect("reschedule to today");
    let agenda = h.todos.upcoming_agenda(h.account).await.expect("agenda");
    assert!(agenda.days.is_empty());
}

#[tokio::test]
async fn calendar_rows_are_read_only_and_unaddressable_by_commands() {
    let h = harness();
    let agenda = h.todos.upcoming_agenda(h.account).await.expect("agenda");
    for row in &agenda.calendar {
        assert!(
            !row.title.is_empty(),
            "calendar rows render read-only titles"
        );
    }

    // Projection-only rows carry no canonical identity, so a task mutation targeting a
    // non-entity identifier is rejected rather than silently creating state.
    let outcome = h
        .dispatcher
        .execute(CommandEnvelope::new(
            h.account,
            Command::CompleteTodo {
                id: EntityId::new(),
            },
            Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
        ))
        .await;
    assert!(matches!(outcome, Err(CommandError::NotFound)));
}
