//! Offline replica: optimistic writes, durable outbox across reloads, idempotent replay.

use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use momentum_app::commands::Command;
use momentum_app::replica::{CommandSender, MemoryReplicaStore, OfflineCoordinator, ReplicaEntity};
use momentum_domain::EntityId;
use uuid::Uuid;

#[derive(Debug, Default)]
struct RecordingSender {
    sent: Mutex<Vec<(Uuid, String)>>,
    mode: Mutex<Mode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Mode {
    #[default]
    Ack,
    Retry,
    Offline,
}

impl RecordingSender {
    fn new() -> Self {
        Self::default()
    }

    fn set_mode(&self, mode: Mode) {
        *self.mode.lock().expect("mode lock") = mode;
    }

    fn sent(&self) -> Vec<(Uuid, String)> {
        self.sent.lock().expect("sent lock").clone()
    }
}

#[async_trait]
impl CommandSender for RecordingSender {
    async fn send(&self, operation_id: Uuid, command: &str) -> Result<bool, String> {
        self.sent
            .lock()
            .expect("sent lock")
            .push((operation_id, command.to_string()));
        match *self.mode.lock().expect("mode lock") {
            Mode::Ack => Ok(true),
            Mode::Retry => Ok(false),
            Mode::Offline => Err("offline".to_string()),
        }
    }
}

fn command(id: EntityId) -> Command {
    Command::CompleteTodo { id }
}

fn now() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap()
}

#[tokio::test]
async fn optimistic_writes_enqueue_and_survive_reload() {
    // Two coordinators over one store simulate a browser reload.
    let store = std::sync::Arc::new(MemoryReplicaStore::new());
    let first = OfflineCoordinator::new(Box::new(SharedStore(store.clone())));
    let todo = EntityId::new();
    let operation = first
        .apply_locally(&command(todo), now())
        .await
        .expect("enqueue");
    first
        .record_entity(ReplicaEntity {
            id: todo,
            kind: "todo".to_string(),
            payload: "{\"title\":\"Local\"}".to_string(),
        })
        .await
        .expect("record");

    // Reload: a fresh coordinator over the same store restores snapshot and outbox.
    let second = OfflineCoordinator::new(Box::new(SharedStore(store.clone())));
    let startup = second.startup().await.expect("startup");
    assert_eq!(startup.snapshot.entities.len(), 1);
    assert_eq!(startup.snapshot.entities[0].id, todo);
    assert_eq!(startup.pending.len(), 1);
    assert_eq!(startup.pending[0].operation_id, operation);
}

#[tokio::test]
async fn replay_is_ordered_idempotent_and_clears_acknowledged_operations() {
    let store = std::sync::Arc::new(MemoryReplicaStore::new());
    let coordinator = OfflineCoordinator::new(Box::new(SharedStore(store)));
    let first = coordinator
        .apply_locally(&command(EntityId::new()), now())
        .await
        .expect("first");
    let second = coordinator
        .apply_locally(&command(EntityId::new()), now())
        .await
        .expect("second");

    let sender = RecordingSender::new();
    let outcome = coordinator.replay(&sender).await.expect("replay");
    assert_eq!(outcome.acknowledged, vec![first, second]);
    assert_eq!(sender.sent().len(), 2);
    assert_eq!(sender.sent()[0].0, first);
    assert_eq!(sender.sent()[1].0, second);
    assert!(sender.sent()[0].1.contains("complete_todo"));

    // A second replay has nothing to send: operations were removed, not resent.
    let outcome = coordinator.replay(&sender).await.expect("replay");
    assert!(outcome.acknowledged.is_empty());
    assert_eq!(sender.sent().len(), 2);
}

#[tokio::test]
async fn retry_and_offline_stop_replay_and_keep_attempt_counts() {
    let store = std::sync::Arc::new(MemoryReplicaStore::new());
    let coordinator = OfflineCoordinator::new(Box::new(SharedStore(store.clone())));
    let first = coordinator
        .apply_locally(&command(EntityId::new()), now())
        .await
        .expect("first");
    let second = coordinator
        .apply_locally(&command(EntityId::new()), now())
        .await
        .expect("second");

    let sender = RecordingSender::new();
    sender.set_mode(Mode::Retry);
    let outcome = coordinator.replay(&sender).await.expect("replay");
    assert_eq!(outcome.retry, vec![first]);
    assert!(outcome.acknowledged.is_empty());
    // The failure stopped the pass; the second operation was never attempted.
    assert_eq!(sender.sent().len(), 1);
    let pending = coordinator.startup().await.expect("startup").pending;
    assert_eq!(pending.len(), 2);
    assert_eq!(pending[0].attempts, 1);
    assert_eq!(pending[1].attempts, 0);

    sender.set_mode(Mode::Offline);
    let outcome = coordinator.replay(&sender).await.expect("replay");
    assert_eq!(outcome.failed, vec![first]);
    sender.set_mode(Mode::Ack);
    let outcome = coordinator.replay(&sender).await.expect("replay");
    assert_eq!(outcome.acknowledged, vec![first, second]);
    assert!(coordinator
        .startup()
        .await
        .expect("startup")
        .pending
        .is_empty());
}

/// Thin wrapper so multiple coordinators share one store without cloning internals.
#[derive(Debug)]
struct SharedStore(std::sync::Arc<MemoryReplicaStore>);

#[async_trait]
impl momentum_app::replica::ReplicaStore for SharedStore {
    async fn load_snapshot(
        &self,
    ) -> Result<Option<momentum_app::replica::ReplicaSnapshot>, String> {
        self.0.load_snapshot().await
    }

    async fn save_snapshot(
        &self,
        snapshot: &momentum_app::replica::ReplicaSnapshot,
    ) -> Result<(), String> {
        self.0.save_snapshot(snapshot).await
    }

    async fn load_outbox(&self) -> Result<Vec<momentum_app::replica::OutboxEntry>, String> {
        self.0.load_outbox().await
    }

    async fn save_outbox(
        &self,
        entries: &[momentum_app::replica::OutboxEntry],
    ) -> Result<(), String> {
        self.0.save_outbox(entries).await
    }
}
