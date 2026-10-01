//! Multi-tab coordination: leader recovery preserving the outbox and sibling propagation.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::Command;
use momentum_app::realtime::{HintHub, RecordingHintSink};
use momentum_app::replica::{MemoryReplicaStore, OfflineCoordinator};
use momentum_app::tabs::{RecordingTabBroadcaster, TabCoordinator, TabMessage, LEADER_LEASE};
use momentum_domain::{AccountId, EntityId};

#[tokio::test]
async fn leader_recovery_preserves_outbox_work() {
    let start = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let account = AccountId::new();
    let broadcaster = Arc::new(RecordingTabBroadcaster::new());
    let hub = Arc::new(HintHub::new());
    let store = Arc::new(MemoryReplicaStore::new());

    let first = TabCoordinator::new(1, account, broadcaster.clone(), hub.clone());
    let second = TabCoordinator::new(2, account, broadcaster.clone(), hub.clone());
    assert!(first.claim_or_renew(start));
    // Simulate BroadcastChannel delivery of the claim to the sibling tab.
    for message in broadcaster.messages() {
        second.on_message(message);
    }
    assert!(!second.claim_or_renew(start));

    // The leader enqueues outbox work into the shared replica store.
    let coordinator = OfflineCoordinator::new(Box::new(SharedStore(store.clone())));
    coordinator
        .apply_locally(
            &Command::CompleteTodo {
                id: EntityId::new(),
            },
            start,
        )
        .await
        .expect("enqueue");

    // The leader stops; after the lease expires the sibling takes over.
    let later = start + LEADER_LEASE + chrono::Duration::seconds(1);
    assert!(!first.is_leader(later));
    assert!(second.claim_or_renew(later));
    assert!(second.is_leader(later));
    assert_eq!(first.leader(later), None);

    // Outbox work is untouched by the transition.
    let restored = coordinator.startup().await.expect("startup");
    assert_eq!(restored.pending.len(), 1);
    assert!(broadcaster
        .messages()
        .iter()
        .any(|message| matches!(message, TabMessage::LeadershipClaimed { tab_id: 2, .. })));
}

#[tokio::test]
async fn sibling_commits_trigger_a_local_pull() {
    let account = AccountId::new();
    let broadcaster = Arc::new(RecordingTabBroadcaster::new());
    let hub = Arc::new(HintHub::new());
    let sink = Arc::new(RecordingHintSink::new());
    hub.subscribe(account, sink.clone());

    let tab = TabCoordinator::new(1, account, broadcaster.clone(), hub.clone());
    tab.on_message(TabMessage::Committed { sequence: 42 });
    assert_eq!(sink.hints().len(), 1);
    assert_eq!(sink.hints()[0].sequence, 42);

    // This tab propagates its own commit and outbox changes to siblings.
    tab.propagate_committed(43);
    tab.propagate_outbox(2);
    let messages = broadcaster.messages();
    assert!(messages
        .iter()
        .any(|message| matches!(message, TabMessage::Committed { sequence: 43 })));
    assert!(messages
        .iter()
        .any(|message| matches!(message, TabMessage::OutboxChanged { pending: 2 })));
}

/// Shares one memory store between the replica coordinator and assertions.
#[derive(Debug)]
struct SharedStore(Arc<MemoryReplicaStore>);

#[async_trait::async_trait]
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
