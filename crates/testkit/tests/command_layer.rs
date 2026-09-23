//! Command-layer tests over the in-memory unit of work.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{
    Command, CommandDispatcher, CommandEnvelope, CommandError, DomainEvent, Projection,
    ScheduleInput, UnitOfWorkFactory,
};
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;

struct Harness {
    dispatcher: CommandDispatcher,
    database: MemoryDatabase,
    account: AccountId,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 1, 9, 0, 0).unwrap(),
    ));
    let dispatcher = CommandDispatcher::new(Arc::new(database.clone()), clock);
    Harness {
        dispatcher,
        database,
        account: AccountId::new(),
    }
}

fn envelope(account: AccountId, command: Command) -> CommandEnvelope {
    CommandEnvelope::new(account, command, Utc::now())
}

fn create_command(title: &str) -> Command {
    Command::CreateTodo {
        title: title.to_string(),
        notes: None,
        context: ContextRef::Inbox,
        schedule: ScheduleInput::Anytime,
        deadline: None,
        reminder: None,
    }
}

async fn create(h: &Harness, title: &str) -> EntityId {
    h.dispatcher
        .execute(envelope(h.account, create_command(title)))
        .await
        .expect("create")
        .entity_id
        .expect("created id")
}

#[tokio::test]
async fn create_todo_persists_emits_event_and_change_record() {
    let h = harness();
    let outcome = h
        .dispatcher
        .execute(envelope(h.account, create_command("Write the report")))
        .await
        .expect("create");

    assert!(!outcome.replayed);
    assert_eq!(outcome.sequence, 1);
    assert!(matches!(
        outcome.events.as_slice(),
        [DomainEvent::TodoCreated { .. }]
    ));
    assert_eq!(outcome.invalidated, vec![Projection::Inbox]);

    let id = outcome.entity_id.expect("id");
    let stored = h.database.todo(h.account, id).expect("stored");
    assert_eq!(stored.title.as_str(), "Write the report");
    assert_eq!(h.database.changes(h.account).len(), 1);
    assert_eq!(
        h.database.changes(h.account)[0].operation_id,
        outcome.operation_id
    );
}

#[tokio::test]
async fn lifecycle_commands_transition_status_and_emit_events() {
    let h = harness();
    let id = create(&h, "Lifecycle").await;

    let completed = h
        .dispatcher
        .execute(envelope(h.account, Command::CompleteTodo { id }))
        .await
        .expect("complete");
    assert!(matches!(
        completed.events.as_slice(),
        [DomainEvent::TodoCompleted { .. }]
    ));
    let stored = h.database.todo(h.account, id).expect("stored");
    assert_eq!(stored.status, momentum_domain::model::Status::Completed);
    assert!(stored.completed_at.is_some());

    let reopened = h
        .dispatcher
        .execute(envelope(h.account, Command::ReopenTodo { id }))
        .await
        .expect("reopen");
    assert!(matches!(
        reopened.events.as_slice(),
        [DomainEvent::TodoReopened { .. }]
    ));

    let canceled = h
        .dispatcher
        .execute(envelope(h.account, Command::CancelTodo { id }))
        .await
        .expect("cancel");
    assert!(matches!(
        canceled.events.as_slice(),
        [DomainEvent::TodoCanceled { .. }]
    ));

    let deleted = h
        .dispatcher
        .execute(envelope(h.account, Command::DeleteTodo { id }))
        .await
        .expect("delete");
    assert!(matches!(
        deleted.events.as_slice(),
        [DomainEvent::TodoDeleted { .. }]
    ));
    assert_eq!(
        h.database.todo(h.account, id).expect("stored").status,
        momentum_domain::model::Status::Deleted
    );

    let restored = h
        .dispatcher
        .execute(envelope(h.account, Command::RestoreTodo { id }))
        .await
        .expect("restore");
    assert!(matches!(
        restored.events.as_slice(),
        [DomainEvent::TodoRestored { .. }]
    ));
    assert_eq!(
        h.database.todo(h.account, id).expect("stored").status,
        momentum_domain::model::Status::Open
    );
}

#[tokio::test]
async fn temporal_commands_are_independent() {
    let h = harness();
    let id = create(&h, "Temporal").await;
    let date = LocalDate::from_ymd(2026, 6, 10).expect("date");

    h.dispatcher
        .execute(envelope(
            h.account,
            Command::ScheduleTodo {
                id,
                schedule: ScheduleInput::On(date),
            },
        ))
        .await
        .expect("schedule");
    h.dispatcher
        .execute(envelope(
            h.account,
            Command::SetTodoDeadline {
                id,
                deadline: Some(LocalDate::from_ymd(2026, 6, 12).expect("date")),
            },
        ))
        .await
        .expect("deadline");
    h.dispatcher
        .execute(envelope(
            h.account,
            Command::SetTodoReminder {
                id,
                reminder: Some(momentum_app::commands::ReminderInput {
                    at: Utc.with_ymd_and_hms(2026, 6, 10, 8, 0, 0).unwrap(),
                    timezone: "Europe/Bucharest".to_string(),
                }),
            },
        ))
        .await
        .expect("reminder");

    let stored = h.database.todo(h.account, id).expect("stored");
    assert_eq!(stored.schedule.date(), Some(date));
    assert_eq!(
        stored.deadline.map(|deadline| deadline.date),
        Some(LocalDate::from_ymd(2026, 6, 12).expect("date"))
    );
    assert!(stored.reminder.is_some());
}

#[tokio::test]
async fn move_preserves_identity_and_schedule() {
    let h = harness();
    let id = create(&h, "Move me").await;
    let project = EntityId::new();

    let outcome = h
        .dispatcher
        .execute(envelope(
            h.account,
            Command::MoveTodo {
                id,
                context: ContextRef::Project(project),
            },
        ))
        .await
        .expect("move");

    assert_eq!(outcome.entity_id, Some(id));
    let stored = h.database.todo(h.account, id).expect("stored");
    assert_eq!(stored.context, ContextRef::Project(project));
    assert_eq!(stored.id(), id);
}

#[tokio::test]
async fn replaying_an_operation_id_is_idempotent() {
    let h = harness();
    let command = create_command("Replay me");
    let envelope = envelope(h.account, command);
    let first = h.dispatcher.execute(envelope.clone()).await.expect("first");
    let second = h.dispatcher.execute(envelope).await.expect("replay");

    assert!(!first.replayed);
    assert!(second.replayed);
    assert_eq!(first.sequence, second.sequence);
    assert_eq!(first.entity_id, second.entity_id);
    assert_eq!(
        h.database.changes(h.account).len(),
        1,
        "no extra change record"
    );
}

#[tokio::test]
async fn sequences_are_strictly_increasing() {
    let h = harness();
    let first = h
        .dispatcher
        .execute(envelope(h.account, create_command("One")))
        .await
        .expect("first");
    let second = h
        .dispatcher
        .execute(envelope(h.account, create_command("Two")))
        .await
        .expect("second");
    assert_eq!(first.sequence, 1);
    assert_eq!(second.sequence, 2);
    let changes = h.database.changes(h.account);
    assert_eq!(changes.len(), 2);
    assert!(changes[0].sequence < changes[1].sequence);
}

#[tokio::test]
async fn foreign_account_entities_are_not_found() {
    let h = harness();
    let id = create(&h, "Mine").await;
    let other = AccountId::new();

    let error = h
        .dispatcher
        .execute(envelope(other, Command::CompleteTodo { id }))
        .await
        .expect_err("foreign account");
    assert_eq!(error, CommandError::NotFound);
    assert_eq!(
        h.database.todo(h.account, id).expect("stored").status,
        momentum_domain::model::Status::Open,
        "foreign command must not mutate state"
    );
}

#[tokio::test]
async fn stale_version_is_a_conflict() {
    let h = harness();
    let id = create(&h, "Conflict").await;

    // Simulate a concurrent update that advanced the stored version.
    let mut stored = h.database.todo(h.account, id).expect("stored");
    stored.meta.version += 5;
    stored.title = momentum_domain::model::Title::parse("Concurrent").expect("title");
    let mut unit = h.database.begin(h.account).await.expect("begin");
    unit.save_todo(h.account, &stored, None)
        .await
        .expect("save concurrent");
    unit.commit().await.expect("commit concurrent");

    // A writer that still expects the old version must be rejected.
    let mut stale = h.database.todo(h.account, id).expect("stored");
    stale.meta.version -= 5;
    let mut unit = h.database.begin(h.account).await.expect("begin");
    let error = unit
        .save_todo(h.account, &stale, Some(stale.meta.version))
        .await
        .expect_err("conflict");
    assert!(matches!(error, CommandError::Conflict { .. }));
    unit.rollback().await.expect("rollback");
}

#[tokio::test]
async fn injected_storage_failure_leaves_no_partial_state() {
    let h = harness();
    let id = create(&h, "Fragile").await;
    let changes_before = h.database.changes(h.account).len();

    h.database.fail_next_save();
    let error = h
        .dispatcher
        .execute(envelope(h.account, Command::CompleteTodo { id }))
        .await
        .expect_err("storage failure");
    assert!(matches!(error, CommandError::Storage(_)));

    assert_eq!(
        h.database.todo(h.account, id).expect("stored").status,
        momentum_domain::model::Status::Open,
        "entity must be unchanged"
    );
    assert_eq!(
        h.database.changes(h.account).len(),
        changes_before,
        "no change record may be appended"
    );
}

#[tokio::test]
async fn invalid_input_is_a_validation_error() {
    let h = harness();
    let error = h
        .dispatcher
        .execute(envelope(
            h.account,
            Command::CreateTodo {
                title: "bad\u{0007}title".to_string(),
                notes: None,
                context: ContextRef::Inbox,
                schedule: ScheduleInput::Anytime,
                deadline: None,
                reminder: None,
            },
        ))
        .await
        .expect_err("validation");
    assert!(matches!(error, CommandError::Validation(_)));

    let id = create(&h, "Long notes").await;
    let error = h
        .dispatcher
        .execute(envelope(
            h.account,
            Command::SetTodoNotes {
                id,
                notes: "x".repeat(momentum_domain::model::MAX_NOTES_LEN + 1),
            },
        ))
        .await
        .expect_err("notes validation");
    assert!(matches!(error, CommandError::Validation(_)));
}

#[tokio::test]
async fn title_and_notes_updates_persist() {
    let h = harness();
    let id = create(&h, "Original").await;

    h.dispatcher
        .execute(envelope(
            h.account,
            Command::SetTodoTitle {
                id,
                title: "  Updated  ".to_string(),
            },
        ))
        .await
        .expect("title");
    h.dispatcher
        .execute(envelope(
            h.account,
            Command::SetTodoNotes {
                id,
                notes: "Some notes".to_string(),
            },
        ))
        .await
        .expect("notes");

    let stored = h.database.todo(h.account, id).expect("stored");
    assert_eq!(stored.title.as_str(), "Updated");
    assert_eq!(stored.notes, "Some notes");
}
