//! Offline local replica: snapshot storage, optimistic writes, and a durable outbox.
//!
//! The coordinator is storage-agnostic: the browser adapter persists the snapshot, preferences,
//! sync cursor, and outbox in IndexedDB (implementing [`ReplicaStore`]), while tests use the
//! in-memory store. Writes applied optimistically enqueue a durable outbound operation keyed by
//! its operation id, so replays after reload or reconnect are idempotent.

use std::collections::BTreeMap;
use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_domain::EntityId;
use uuid::Uuid;

use crate::commands::Command;

/// One cached entity in the working set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaEntity {
    /// Canonical identity.
    pub id: EntityId,
    /// Entity kind (`todo`, `project`, ...).
    pub kind: String,
    /// Serialized summary payload.
    pub payload: String,
}

/// The persisted working set, preferences, and sync cursor.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplicaSnapshot {
    /// Cached entities.
    pub entities: Vec<ReplicaEntity>,
    /// Preferences needed offline.
    pub preferences: BTreeMap<String, String>,
    /// Last synchronized change-log sequence.
    pub cursor: i64,
}

impl ReplicaSnapshot {
    /// Inserts or replaces one entity.
    pub fn upsert(&mut self, entity: ReplicaEntity) {
        match self
            .entities
            .iter_mut()
            .find(|existing| existing.id == entity.id)
        {
            Some(existing) => *existing = entity,
            None => self.entities.push(entity),
        }
    }
}

/// One durable outbound operation awaiting acknowledgement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboxEntry {
    /// Operation id reused across retries (server-side deduplication key).
    pub operation_id: Uuid,
    /// Serialized command.
    pub command: String,
    /// Enqueue instant.
    pub enqueued_at: DateTime<Utc>,
    /// Delivery attempts so far.
    pub attempts: u32,
}

/// Persistence for the replica and outbox (IndexedDB in browsers, memory in tests).
#[async_trait]
pub trait ReplicaStore: Send + Sync {
    /// Loads the persisted snapshot, when present.
    async fn load_snapshot(&self) -> Result<Option<ReplicaSnapshot>, String>;

    /// Persists the snapshot.
    async fn save_snapshot(&self, snapshot: &ReplicaSnapshot) -> Result<(), String>;

    /// Loads pending outbound operations in enqueue order.
    async fn load_outbox(&self) -> Result<Vec<OutboxEntry>, String>;

    /// Persists the outbound queue.
    async fn save_outbox(&self, entries: &[OutboxEntry]) -> Result<(), String>;
}

/// What an offline startup restored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaStartup {
    /// Restored snapshot (empty when first run).
    pub snapshot: ReplicaSnapshot,
    /// Pending outbound operations.
    pub pending: Vec<OutboxEntry>,
}

/// Sends an outbox operation to the server.
#[async_trait]
pub trait CommandSender: Send + Sync {
    /// Delivers one command with its stable operation id.
    ///
    /// Returns `Ok(true)` when acknowledged, `Ok(false)` when the server asked for a retry, and
    /// `Err` when the connection failed.
    async fn send(&self, operation_id: Uuid, command: &str) -> Result<bool, String>;
}

/// Performs optimistic local writes and durable outbox replay.
pub struct OfflineCoordinator {
    store: Box<dyn ReplicaStore>,
}

impl std::fmt::Debug for OfflineCoordinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OfflineCoordinator")
    }
}

impl OfflineCoordinator {
    /// Creates a coordinator over a store.
    #[must_use]
    pub fn new(store: Box<dyn ReplicaStore>) -> Self {
        Self { store }
    }

    /// Restores the replica and pending operations after a reload.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn startup(&self) -> Result<ReplicaStartup, String> {
        Ok(ReplicaStartup {
            snapshot: self.store.load_snapshot().await?.unwrap_or_default(),
            pending: self.store.load_outbox().await?,
        })
    }

    /// Applies a command locally first and enqueues it durably.
    ///
    /// Returns the operation id that identifies this write for every future retry.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn apply_locally(
        &self,
        command: &Command,
        now: DateTime<Utc>,
    ) -> Result<Uuid, String> {
        let operation_id = Uuid::now_v7();
        let command = serde_json::to_string(command).map_err(|error| error.to_string())?;
        let mut outbox = self.store.load_outbox().await?;
        outbox.push(OutboxEntry {
            operation_id,
            command,
            enqueued_at: now,
            attempts: 0,
        });
        self.store.save_outbox(&outbox).await?;
        Ok(operation_id)
    }

    /// Updates one cached entity in the working set (optimistic UI state).
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn record_entity(&self, entity: ReplicaEntity) -> Result<(), String> {
        let mut snapshot = self.store.load_snapshot().await?.unwrap_or_default();
        snapshot.upsert(entity);
        self.store.save_snapshot(&snapshot).await
    }

    /// Removes an acknowledged operation from the outbox.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn acknowledge(&self, operation_id: Uuid) -> Result<(), String> {
        let mut outbox = self.store.load_outbox().await?;
        outbox.retain(|entry| entry.operation_id != operation_id);
        self.store.save_outbox(&outbox).await
    }

    /// Replays the outbox in order until the first failure.
    ///
    /// Acknowledged operations are removed (idempotent because operation ids never change);
    /// retryable or failed operations stay queued, so the next replay resumes where this one
    /// stopped.
    ///
    /// # Errors
    ///
    /// Propagates store failures.
    pub async fn replay(&self, sender: &dyn CommandSender) -> Result<ReplayOutcome, String> {
        let mut outbox = self.store.load_outbox().await?;
        let mut outcome = ReplayOutcome::default();
        let mut remaining: Vec<OutboxEntry> = Vec::new();
        let mut stopped = false;
        for mut entry in outbox.drain(..) {
            if stopped {
                remaining.push(entry);
                continue;
            }
            match sender.send(entry.operation_id, &entry.command).await {
                Ok(true) => outcome.acknowledged.push(entry.operation_id),
                Ok(false) => {
                    entry.attempts += 1;
                    outcome.retry.push(entry.operation_id);
                    remaining.push(entry);
                    stopped = true;
                }
                Err(_) => {
                    entry.attempts += 1;
                    outcome.failed.push(entry.operation_id);
                    remaining.push(entry);
                    stopped = true;
                }
            }
        }
        self.store.save_outbox(&remaining).await?;
        Ok(outcome)
    }
}

/// Result of one replay pass.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayOutcome {
    /// Operations acknowledged and removed.
    pub acknowledged: Vec<Uuid>,
    /// Operations the server asked to retry.
    pub retry: Vec<Uuid>,
    /// Operations that hit a connection failure.
    pub failed: Vec<Uuid>,
}

/// In-memory store used for tests and development.
#[derive(Debug, Default)]
pub struct MemoryReplicaStore {
    snapshot: Mutex<Option<ReplicaSnapshot>>,
    outbox: Mutex<Vec<OutboxEntry>>,
}

impl MemoryReplicaStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ReplicaStore for MemoryReplicaStore {
    async fn load_snapshot(&self) -> Result<Option<ReplicaSnapshot>, String> {
        Ok(self.snapshot.lock().expect("snapshot lock").clone())
    }

    async fn save_snapshot(&self, snapshot: &ReplicaSnapshot) -> Result<(), String> {
        *self.snapshot.lock().expect("snapshot lock") = Some(snapshot.clone());
        Ok(())
    }

    async fn load_outbox(&self) -> Result<Vec<OutboxEntry>, String> {
        Ok(self.outbox.lock().expect("outbox lock").clone())
    }

    async fn save_outbox(&self, entries: &[OutboxEntry]) -> Result<(), String> {
        *self.outbox.lock().expect("outbox lock") = entries.to_vec();
        Ok(())
    }
}
