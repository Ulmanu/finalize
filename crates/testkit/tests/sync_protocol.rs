//! Sync protocol: cursors, idempotent operation ids, and explicit conflict rules.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{Command, CommandDispatcher};
use momentum_app::realtime::{HintHub, RecordingHintSink};
use momentum_app::sync::{
    ConflictResolution, ConflictRule, SyncOperation, SyncRequest, SyncService,
};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::sync::MemoryChangeLogReader;
use momentum_testkit::todos::MemoryTodoReadStore;
use uuid::Uuid;

struct Harness {
    service: SyncService,
    todos: Arc<TodoService>,
    sink: Arc<RecordingHintSink>,
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
    let hub = Arc::new(HintHub::new());
    let sink = Arc::new(RecordingHintSink::new());
    let service = SyncService::new(
        dispatcher,
        todos.clone(),
        Arc::new(MemoryChangeLogReader::new(database)),
        clock,
    )
    .with_hints(hub.clone());
    let account = AccountId::new();
    hub.subscribe(account, sink.clone());
    Harness {
        service,
        todos,
        sink,
        account,
    }
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
        .expect("created")
        .id
}

/// Pulls without pushing to learn the current cursor.
async fn cursor(h: &Harness) -> i64 {
    h.service
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations: Vec::new(),
            },
        )
        .await
        .expect("pull")
        .cursor
}

fn title_op(id: EntityId, title: &str) -> SyncOperation {
    SyncOperation {
        operation_id: Uuid::now_v7(),
        command: Command::SetTodoTitle {
            id,
            title: title.to_string(),
        },
    }
}

#[tokio::test]
async fn push_acknowledges_monotonically_and_dedupes_operation_ids() {
    let h = harness();
    let a = create(&h, "A").await;
    let b = create(&h, "B").await;
    let baseline = cursor(&h).await;
    let operations = vec![title_op(a, "A2"), title_op(b, "B2")];

    let response = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: baseline,
                operations: operations.clone(),
            },
        )
        .await
        .expect("sync");
    assert_eq!(response.acknowledged.len(), 2);
    assert!(response.conflicts.is_empty());
    assert!(response.acknowledged[0].sequence < response.acknowledged[1].sequence);
    assert!(!response.acknowledged[0].replayed);
    assert!(response.acknowledged[0].sequence > baseline);
    assert_eq!(response.cursor, baseline + 2);
    assert_eq!(response.changes.len(), 2);
    // A realtime hint tells other sessions to pull; the burst coalesces into one pending hint.
    assert_eq!(h.sink.hints().len(), 1);
    assert_eq!(
        h.sink.hints()[0].sequence,
        response.acknowledged[0].sequence
    );

    // Replaying the same operation ids is acknowledged as replays with the same sequences and
    // does not create new changes.
    let replayed = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: baseline,
                operations,
            },
        )
        .await
        .expect("sync");
    assert!(replayed.acknowledged.iter().all(|ack| ack.replayed));
    assert_eq!(
        replayed.acknowledged[0].sequence,
        response.acknowledged[0].sequence
    );
    assert_eq!(replayed.changes.len(), 2);

    // The cursor is monotonic: pulling past the end returns nothing.
    let empty = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: response.cursor,
                operations: Vec::new(),
            },
        )
        .await
        .expect("sync");
    assert!(empty.changes.is_empty());
    assert_eq!(empty.cursor, response.cursor);
}

#[tokio::test]
async fn pull_returns_only_changes_after_the_cursor() {
    let h = harness();
    let a = create(&h, "A").await;
    let baseline = cursor(&h).await;
    let first = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: baseline,
                operations: vec![title_op(a, "A2")],
            },
        )
        .await
        .expect("sync");
    assert_eq!(first.changes.len(), 1);
    assert_eq!(first.cursor, baseline + 1);

    let b = create(&h, "B").await;
    let after_create = cursor(&h).await;
    let second = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: after_create,
                operations: vec![title_op(b, "B2")],
            },
        )
        .await
        .expect("sync");
    assert_eq!(second.changes.len(), 1);
    assert_eq!(second.changes[0].sequence, after_create + 1);
    assert_eq!(second.cursor, after_create + 1);
}

#[tokio::test]
async fn conflicts_are_classified_without_destructive_writes() {
    let h = harness();
    let todo = create(&h, "Monotonic").await;

    // Completion is monotonic: completing again is an idempotent acknowledgement.
    let first = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations: vec![SyncOperation {
                    operation_id: Uuid::now_v7(),
                    command: Command::CompleteTodo { id: todo },
                }],
            },
        )
        .await
        .expect("sync");
    assert!(first.conflicts.is_empty());
    let completed_at = h
        .todos
        .get(h.account, todo)
        .await
        .unwrap()
        .unwrap()
        .completed_at;
    let again = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations: vec![SyncOperation {
                    operation_id: Uuid::now_v7(),
                    command: Command::CompleteTodo { id: todo },
                }],
            },
        )
        .await
        .expect("sync");
    assert!(again.conflicts.is_empty());
    assert_eq!(
        h.todos
            .get(h.account, todo)
            .await
            .unwrap()
            .unwrap()
            .completed_at,
        completed_at
    );

    // Delete beats edit: a deleted entity never resurrects through a pushed edit.
    let doomed = create(&h, "Doomed").await;
    h.todos.delete(h.account, doomed).await.expect("delete");
    let deleted_conflict = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations: vec![title_op(doomed, "Resurrected")],
            },
        )
        .await
        .expect("sync");
    assert_eq!(deleted_conflict.conflicts.len(), 1);
    assert_eq!(
        deleted_conflict.conflicts[0].rule,
        ConflictRule::DeleteBeatsEdit
    );
    assert_eq!(
        deleted_conflict.conflicts[0].resolution,
        ConflictResolution::ClientRefreshes
    );
    let trashed = h
        .todos
        .list(h.account, TodoScope::Trash)
        .await
        .expect("trash");
    assert!(trashed.iter().any(|item| item.id == doomed));

    // Ordering is relative: ordering a missing entity is ignored, server rank retained.
    let ordering = h
        .service
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations: vec![SyncOperation {
                    operation_id: Uuid::now_v7(),
                    command: Command::MoveTodoToPosition {
                        id: EntityId::new(),
                        position: 0,
                    },
                }],
            },
        )
        .await
        .expect("sync");
    assert_eq!(ordering.conflicts.len(), 1);
    assert_eq!(ordering.conflicts[0].rule, ConflictRule::OrderingRelative);
    assert_eq!(
        ordering.conflicts[0].resolution,
        ConflictResolution::Ignored
    );
}
