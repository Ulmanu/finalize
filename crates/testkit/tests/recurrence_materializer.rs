//! Recurrence materializer: bounded horizon, idempotency, restart safety, end conditions.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::CommandDispatcher;
use momentum_app::materializer::{RecurrenceMaterializer, HORIZON_DAYS};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::{ContextRef, Schedule};
use momentum_domain::recurrence::{
    Recurrence, RecurrenceEnd, RecurrencePattern, RecurrenceTemplate,
};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::materializer::MemoryMaterializerStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    database: MemoryDatabase,
    todos: Arc<TodoService>,
    account: AccountId,
    today: LocalDate,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let clock = Arc::new(TestClock::new(now));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock,
    ));
    Harness {
        database,
        todos,
        account: AccountId::new(),
        today: LocalDate::from_ymd(2026, 6, 10).expect("today"),
    }
}

fn template_payload(title: &str) -> RecurrenceTemplate {
    RecurrenceTemplate {
        title: title.to_string(),
        notes: "keep it short".to_string(),
        tag_ids: Vec::new(),
        checklist: vec!["Prepare notes".to_string()],
        headings: Vec::new(),
        deadline_offset_days: Some(1),
        reminder_offset_minutes: None,
        context: ContextRef::Inbox,
    }
}

fn seed(h: &Harness, rule: Recurrence, next: Option<LocalDate>) -> EntityId {
    let id = EntityId::new();
    h.database.seed_recurrence_template(
        h.account,
        momentum_app::materializer::PersistedTemplate {
            id,
            rule,
            template: template_payload("Daily standup"),
            next_occurrence: next,
            horizon: None,
        },
    );
    id
}

fn materializer(h: &Harness) -> RecurrenceMaterializer {
    RecurrenceMaterializer::new(
        Arc::clone(&h.todos),
        None,
        Arc::new(MemoryMaterializerStore::new(h.database.clone())),
    )
}

async fn open_count(h: &Harness) -> usize {
    h.todos
        .list(h.account, TodoScope::AllOpen)
        .await
        .expect("all open")
        .len()
}

#[tokio::test]
async fn generates_a_bounded_horizon_once_and_resumes_after_restart() {
    let h = harness();
    let template_id = seed(
        &h,
        Recurrence::new(RecurrencePattern::Daily, RecurrenceEnd::Never),
        Some(h.today),
    );

    let run = materializer(&h).run(h.account, h.today).await.expect("run");
    let expected = usize::try_from(HORIZON_DAYS).expect("horizon") + 1;
    assert_eq!(run.materialized.len(), expected);
    assert!(run.failed.is_empty());
    assert_eq!(open_count(&h).await, expected);

    // Copies inherit notes, checklist, deadline offsets, and the occurrence schedule.
    let first = h
        .todos
        .get(h.account, run.materialized[0])
        .await
        .unwrap()
        .unwrap();
    assert_eq!(first.title, "Daily standup");
    assert_eq!(first.notes, "keep it short");
    assert_eq!(first.checklist.len(), 1);
    assert_eq!(first.schedule, Schedule::on(h.today));
    assert_eq!(first.deadline.unwrap().date, h.today.next_day());

    // Repeated runs are no-ops and progress is persisted on the template.
    let second = materializer(&h).run(h.account, h.today).await.expect("run");
    assert!(second.materialized.is_empty());
    assert_eq!(open_count(&h).await, expected);
    let stored = h
        .database
        .recurrence_template(h.account, template_id)
        .expect("template");
    assert_eq!(
        stored.next_occurrence,
        Some(advance(h.today, HORIZON_DAYS + 1))
    );
    assert_eq!(stored.horizon, Some(advance(h.today, HORIZON_DAYS)));

    // A restart (fresh materializer over persisted state) resumes from stored progress.
    let later = advance(h.today, HORIZON_DAYS + 1);
    let restarted = materializer(&h).run(h.account, later).await.expect("run");
    assert_eq!(restarted.materialized.len(), expected);
    assert_eq!(open_count(&h).await, expected * 2);
}

#[tokio::test]
async fn end_conditions_stop_and_existing_occurrences_are_skipped() {
    let h = harness();
    let id = seed(
        &h,
        Recurrence::new(RecurrencePattern::Daily, RecurrenceEnd::After(3)),
        Some(h.today),
    );
    let run = materializer(&h).run(h.account, h.today).await.expect("run");
    assert_eq!(run.materialized.len(), 3);
    assert_eq!(open_count(&h).await, 3);
    let stored = h
        .database
        .recurrence_template(h.account, id)
        .expect("template");
    assert_eq!(stored.next_occurrence, None);

    // An ended rule never generates again.
    let later = advance(h.today, 90);
    let ended = materializer(&h).run(h.account, later).await.expect("run");
    assert!(ended.materialized.is_empty());
    assert_eq!(open_count(&h).await, 3);

    // An occurrence already recorded (for example by another worker) is skipped, not duplicated.
    let h2 = harness();
    let id2 = seed(
        &h2,
        Recurrence::new(RecurrencePattern::Daily, RecurrenceEnd::After(3)),
        Some(h2.today),
    );
    h2.database
        .seed_recurrence_instance(h2.account, id2, &h2.today.to_iso());
    let run = materializer(&h2)
        .run(h2.account, h2.today)
        .await
        .expect("run");
    assert_eq!(run.skipped, vec![h2.today]);
    assert_eq!(run.materialized.len(), 2);
    assert_eq!(open_count(&h2).await, 2);
}

fn advance(base: LocalDate, days: i64) -> LocalDate {
    let mut date = base;
    for _ in 0..days {
        date = date.next_day();
    }
    date
}
