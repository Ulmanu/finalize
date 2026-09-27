//! Drag-and-drop operation semantics: positional reorder, day reschedule, cross-context drop.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, CommandError, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
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

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

async fn inbox_order(h: &Harness) -> Vec<EntityId> {
    h.todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .map(|todo| todo.id)
        .collect()
}

#[tokio::test]
async fn positional_move_updates_context_rank_without_touching_data() {
    let h = harness();
    let mut ids = Vec::new();
    for title in ["A", "B", "C", "D"] {
        let id = h
            .todos
            .create(h.account, title, Some(format!("notes {title}")))
            .await
            .expect("create")
            .entity_id
            .expect("id");
        ids.push(id);
    }

    h.todos
        .move_to_position(h.account, ids[3], 0)
        .await
        .expect("drop at top");
    assert_eq!(inbox_order(&h).await, vec![ids[3], ids[0], ids[1], ids[2]]);
    let moved = h
        .todos
        .get(h.account, ids[3])
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(moved.title, "D");
    assert_eq!(moved.notes, "notes D", "drop preserves unrelated data");

    h.todos
        .move_to_position(h.account, ids[3], 99)
        .await
        .expect("clamp to end");
    assert_eq!(inbox_order(&h).await, vec![ids[0], ids[1], ids[2], ids[3]]);

    let before = inbox_order(&h).await;
    h.todos
        .move_to_position(h.account, ids[1], 1)
        .await
        .expect("same position no-op");
    assert_eq!(inbox_order(&h).await, before);

    let error = h
        .todos
        .move_to_position(h.account, EntityId::new(), 0)
        .await
        .expect_err("unknown item");
    assert!(matches!(error, CommandError::NotFound));
}

#[tokio::test]
async fn today_positional_drag_is_independent_from_context_order() {
    let h = harness();
    let mut day = Vec::new();
    let mut evening = Vec::new();
    for (title, schedule) in [
        ("Day A", ScheduleInput::On(date(10))),
        ("Day B", ScheduleInput::On(date(10))),
        ("Day C", ScheduleInput::On(date(10))),
        ("Eve A", ScheduleInput::ThisEvening(date(10))),
        ("Eve B", ScheduleInput::ThisEvening(date(10))),
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
        if title.starts_with("Eve") {
            evening.push(id);
        } else {
            day.push(id);
        }
    }
    let context_before = inbox_order(&h).await;

    h.todos
        .move_today_to_position(h.account, day[2], 0)
        .await
        .expect("drop day item at top");
    let today: Vec<EntityId> = h
        .todos
        .list_at(h.account, TodoScope::Today, 0)
        .await
        .expect("today")
        .into_iter()
        .map(|todo| todo.id)
        .collect();
    let day_order: Vec<EntityId> = today
        .iter()
        .filter(|id| day.contains(id))
        .copied()
        .collect();
    let evening_order: Vec<EntityId> = today
        .iter()
        .filter(|id| evening.contains(id))
        .copied()
        .collect();
    assert_eq!(day_order, vec![day[2], day[0], day[1]]);
    assert_eq!(
        evening_order,
        vec![evening[0], evening[1]],
        "evening untouched"
    );
    assert_eq!(
        inbox_order(&h).await,
        context_before,
        "Today drag never rewrites contextual rank"
    );

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
    let error = h
        .todos
        .move_today_to_position(h.account, future, 0)
        .await
        .expect_err("non-today items cannot be dropped in Today");
    assert!(matches!(error, CommandError::Validation(_)));
}

#[tokio::test]
async fn upcoming_day_drop_reschedules_and_cross_context_drop_uses_the_engine() {
    let h = harness();
    let project = h
        .projects
        .create(h.account, "Destination", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let id = h
        .todos
        .create(h.account, "Dragged", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(15)))
        .await
        .expect("schedule");

    // Dropping on another Upcoming day runs the schedule command.
    h.todos
        .schedule(h.account, id, ScheduleInput::On(date(22)))
        .await
        .expect("drop on day 22");
    let summary = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.schedule.date(), Some(date(22)));
    let agenda = h.todos.upcoming_agenda(h.account).await.expect("agenda");
    assert_eq!(agenda.days.len(), 1);
    assert_eq!(agenda.days[0].date, date(22));
    assert_eq!(agenda.days[0].todos[0].id, id);
    assert_eq!(
        summary.context,
        ContextRef::Inbox,
        "context preserved on reschedule"
    );

    // Dropping on a project destination runs the move engine.
    h.todos
        .move_to(h.account, id, ContextRef::Project(project))
        .await
        .expect("cross-context drop");
    let summary = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.context, ContextRef::Project(project));
    assert_eq!(
        summary.schedule.date(),
        Some(date(22)),
        "schedule preserved on move"
    );
}
