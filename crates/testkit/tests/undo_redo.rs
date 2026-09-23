//! Undo/redo tests over the in-memory unit of work.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{Command, CommandDispatcher, ScheduleInput};
use momentum_app::undo::UndoEngine;
use momentum_domain::model::{ContextRef, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;

struct Harness {
    engine: UndoEngine,
    database: MemoryDatabase,
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
    let engine = UndoEngine::new(Arc::new(database.clone()), dispatcher, clock);
    Harness {
        engine,
        database,
        account: AccountId::new(),
    }
}

fn create(title: &str) -> Command {
    Command::CreateTodo {
        title: title.to_string(),
        notes: None,
        context: ContextRef::Inbox,
        schedule: ScheduleInput::Anytime,
        deadline: None,
        reminder: None,
    }
}

#[tokio::test]
async fn create_undo_redo_preserves_identity() {
    let h = harness();
    let group = h
        .engine
        .apply_group(h.account, "Create", vec![create("Undo me")])
        .await
        .expect("apply");

    // The inverse of a create is a delete; find the entity by scanning changes.
    let id = h.database.changes(h.account)[0]
        .entity_id
        .expect("created entity recorded");
    assert_eq!(group.entries[0].inverse, vec![Command::DeleteTodo { id }]);

    let undo_group = h.engine.undo(h.account, &group).await.expect("undo");
    assert_eq!(
        h.database.todo(h.account, id).expect("stored").status,
        Status::Deleted
    );
    assert!(!undo_group.is_empty());

    let redo_group = h.engine.redo(h.account, &undo_group).await.expect("redo");
    let restored = h.database.todo(h.account, id).expect("stored");
    assert_eq!(restored.status, Status::Open);
    assert_eq!(restored.id(), id, "identity must survive undo/redo");
    assert!(!redo_group.is_empty());
}

#[tokio::test]
async fn title_change_round_trips() {
    let h = harness();
    let group = h
        .engine
        .apply_group(h.account, "Create", vec![create("Original")])
        .await
        .expect("create");
    let id = h.database.changes(h.account)[0].entity_id.expect("id");
    let _ = group;

    let edit = h
        .engine
        .apply_group(
            h.account,
            "Rename",
            vec![Command::SetTodoTitle {
                id,
                title: "Changed".to_string(),
            }],
        )
        .await
        .expect("rename");
    assert_eq!(
        h.database
            .todo(h.account, id)
            .expect("stored")
            .title
            .as_str(),
        "Changed"
    );

    let undo = h.engine.undo(h.account, &edit).await.expect("undo");
    assert_eq!(
        h.database
            .todo(h.account, id)
            .expect("stored")
            .title
            .as_str(),
        "Original"
    );

    h.engine.redo(h.account, &undo).await.expect("redo");
    assert_eq!(
        h.database
            .todo(h.account, id)
            .expect("stored")
            .title
            .as_str(),
        "Changed"
    );
}

#[tokio::test]
async fn completion_undo_reopens_and_redo_completes() {
    let h = harness();
    h.engine
        .apply_group(h.account, "Create", vec![create("Finish")])
        .await
        .expect("create");
    let id = h.database.changes(h.account)[0].entity_id.expect("id");

    let completion = h
        .engine
        .apply_group(h.account, "Complete", vec![Command::CompleteTodo { id }])
        .await
        .expect("complete");
    assert_eq!(
        h.database.todo(h.account, id).expect("stored").status,
        Status::Completed
    );

    let undo = h.engine.undo(h.account, &completion).await.expect("undo");
    let reopened = h.database.todo(h.account, id).expect("stored");
    assert_eq!(reopened.status, Status::Open);
    assert!(reopened.completed_at.is_none());

    h.engine.redo(h.account, &undo).await.expect("redo");
    let completed = h.database.todo(h.account, id).expect("stored");
    assert_eq!(completed.status, Status::Completed);
    assert!(completed.completed_at.is_some());
}

#[tokio::test]
async fn grouped_commands_undo_in_reverse_order() {
    let h = harness();
    h.engine
        .apply_group(h.account, "Create", vec![create("Grouped")])
        .await
        .expect("create");
    let id = h.database.changes(h.account)[0].entity_id.expect("id");

    let group = h
        .engine
        .apply_group(
            h.account,
            "Plan",
            vec![
                Command::SetTodoTitle {
                    id,
                    title: "Planned".to_string(),
                },
                Command::ScheduleTodo {
                    id,
                    schedule: ScheduleInput::On(LocalDate::from_ymd(2026, 6, 10).expect("date")),
                },
                Command::CompleteTodo { id },
            ],
        )
        .await
        .expect("group");
    assert_eq!(group.len(), 3);

    let undone = h.engine.undo(h.account, &group).await.expect("undo");
    let stored = h.database.todo(h.account, id).expect("stored");
    assert_eq!(stored.title.as_str(), "Grouped");
    assert_eq!(stored.schedule, momentum_domain::model::Schedule::anytime());
    assert_eq!(stored.status, Status::Open);

    h.engine.redo(h.account, &undone).await.expect("redo");
    let redone = h.database.todo(h.account, id).expect("stored");
    assert_eq!(redone.title.as_str(), "Planned");
    assert_eq!(
        redone.schedule.date(),
        Some(LocalDate::from_ymd(2026, 6, 10).expect("date"))
    );
    assert_eq!(redone.status, Status::Completed);
}

#[tokio::test]
async fn multiple_creates_in_one_group_undo_together() {
    let h = harness();
    let group = h
        .engine
        .apply_group(
            h.account,
            "Capture two",
            vec![create("First"), create("Second")],
        )
        .await
        .expect("group");

    let ids: Vec<EntityId> = h
        .database
        .changes(h.account)
        .iter()
        .filter_map(|record| record.entity_id)
        .collect();
    assert_eq!(ids.len(), 2);

    let undo = h.engine.undo(h.account, &group).await.expect("undo");
    for id in &ids {
        assert_eq!(
            h.database.todo(h.account, *id).expect("stored").status,
            Status::Deleted
        );
    }

    h.engine.redo(h.account, &undo).await.expect("redo");
    for id in &ids {
        assert_eq!(
            h.database.todo(h.account, *id).expect("stored").status,
            Status::Open
        );
    }
}
