//! Multi-tab coordination: deterministic local leadership and cross-tab propagation.
//!
//! Browsers coordinate through native channels (BroadcastChannel for messages, Web Locks or a
//! leader lease for ownership); this module holds the deterministic logic those adapters drive.
//! A single leader runs sync/outbox work, committed changes propagate to every tab, and an
//! expired leader is replaced without touching outbox state, which lives in the shared replica
//! store (49).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};

use momentum_domain::AccountId;

use crate::realtime::{HintHub, InvalidationHint};

/// Stable identity for one browser tab.
pub type TabId = u64;

/// Default leadership lease.
pub const LEADER_LEASE: Duration = Duration::seconds(15);

/// Messages exchanged between tabs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabMessage {
    /// A tab committed new server data (its sequence), so others should pull.
    Committed {
        /// Change-log sequence now available.
        sequence: i64,
    },
    /// A tab is claiming leadership until the given instant.
    LeadershipClaimed {
        /// Claiming tab.
        tab_id: TabId,
        /// Lease expiry.
        expires_at: DateTime<Utc>,
    },
    /// A tab's outbox changed, so replicas/pending indicators should refresh.
    OutboxChanged {
        /// Pending operation count.
        pending: usize,
    },
}

/// Outbound channel to sibling tabs (BroadcastChannel in browsers, a recorder in tests).
pub trait TabBroadcaster: Send + Sync {
    /// Posts a message to every sibling tab.
    fn post(&self, message: TabMessage);
}

/// Records broadcast messages for tests.
#[derive(Debug, Default)]
pub struct RecordingTabBroadcaster {
    messages: Mutex<Vec<TabMessage>>,
}

impl RecordingTabBroadcaster {
    /// Creates an empty recorder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the messages posted so far.
    #[must_use]
    pub fn messages(&self) -> Vec<TabMessage> {
        self.messages.lock().expect("messages lock").clone()
    }
}

impl TabBroadcaster for RecordingTabBroadcaster {
    fn post(&self, message: TabMessage) {
        self.messages.lock().expect("messages lock").push(message);
    }
}

/// Leadership view of the tab set.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LeadershipState {
    /// Known lease expiries per tab.
    pub leases: HashMap<TabId, DateTime<Utc>>,
}

impl LeadershipState {
    /// The current leader: the lowest-id tab with a live lease.
    #[must_use]
    pub fn leader(&self, now: DateTime<Utc>) -> Option<TabId> {
        self.leases
            .iter()
            .filter(|(_, expires_at)| **expires_at > now)
            .map(|(tab_id, _)| *tab_id)
            .min()
    }

    /// Records (or extends) a tab's lease, keeping the latest expiry.
    pub fn observe(&mut self, tab_id: TabId, expires_at: DateTime<Utc>) {
        let entry = self.leases.entry(tab_id).or_insert(expires_at);
        if expires_at > *entry {
            *entry = expires_at;
        }
    }

    /// Drops expired leases.
    pub fn prune(&mut self, now: DateTime<Utc>) {
        self.leases.retain(|_, expires_at| *expires_at > now);
    }
}

/// Per-tab coordinator: drives leadership claims and propagates committed changes.
pub struct TabCoordinator {
    tab_id: TabId,
    account_id: AccountId,
    state: Mutex<LeadershipState>,
    broadcaster: Arc<dyn TabBroadcaster>,
    hints: Arc<HintHub>,
}

impl std::fmt::Debug for TabCoordinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TabCoordinator")
    }
}

impl TabCoordinator {
    /// Creates a coordinator for one tab.
    #[must_use]
    pub fn new(
        tab_id: TabId,
        account_id: AccountId,
        broadcaster: Arc<dyn TabBroadcaster>,
        hints: Arc<HintHub>,
    ) -> Self {
        Self {
            tab_id,
            account_id,
            state: Mutex::new(LeadershipState::default()),
            broadcaster,
            hints,
        }
    }

    /// Attempts to become (or remain) the leader.
    ///
    /// Succeeds when no live lease exists, when this tab already holds the lowest live lease, or
    /// when the previous leader's lease expired. The claim is broadcast so siblings converge.
    pub fn claim_or_renew(&self, now: DateTime<Utc>) -> bool {
        let mut state = self.state.lock().expect("leadership lock");
        state.prune(now);
        let leader = state.leader(now);
        let can_lead = leader.is_none_or(|current| current == self.tab_id);
        if !can_lead {
            return false;
        }
        let expires_at = now + LEADER_LEASE;
        state.observe(self.tab_id, expires_at);
        self.broadcaster.post(TabMessage::LeadershipClaimed {
            tab_id: self.tab_id,
            expires_at,
        });
        true
    }

    /// Whether this tab currently leads.
    #[must_use]
    pub fn is_leader(&self, now: DateTime<Utc>) -> bool {
        self.state.lock().expect("leadership lock").leader(now) == Some(self.tab_id)
    }

    /// Handles a message from a sibling tab.
    ///
    /// Committed changes surface as a local invalidation hint (so this tab pulls), leadership
    /// claims are adopted, and outbox changes are acknowledged without touching local work.
    pub fn on_message(&self, message: TabMessage) {
        match message {
            TabMessage::Committed { sequence } => {
                self.hints
                    .notify(self.account_id, InvalidationHint { sequence });
            }
            TabMessage::LeadershipClaimed { tab_id, expires_at } => {
                self.state
                    .lock()
                    .expect("leadership lock")
                    .observe(tab_id, expires_at);
            }
            TabMessage::OutboxChanged { .. } => {}
        }
    }

    /// Propagates a committed change to sibling tabs after this tab pushes successfully.
    pub fn propagate_committed(&self, sequence: i64) {
        self.broadcaster.post(TabMessage::Committed { sequence });
    }

    /// Broadcasts an outbox size change.
    pub fn propagate_outbox(&self, pending: usize) {
        self.broadcaster.post(TabMessage::OutboxChanged { pending });
    }

    /// The tab's current leader view.
    #[must_use]
    pub fn leader(&self, now: DateTime<Utc>) -> Option<TabId> {
        self.state.lock().expect("leadership lock").leader(now)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LeadershipState, RecordingTabBroadcaster, TabCoordinator, TabMessage, LEADER_LEASE,
    };
    use crate::realtime::{HintHub, RecordingHintSink};
    use chrono::{TimeZone, Utc};
    use std::sync::Arc;

    fn now() -> chrono::DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap()
    }

    #[test]
    fn election_prefers_the_lowest_live_tab() {
        let start = now();
        let mut state = LeadershipState::default();
        state.observe(7, start + LEADER_LEASE);
        state.observe(3, start + LEADER_LEASE);
        assert_eq!(state.leader(start), Some(3));
        // An expired leader loses without affecting the next candidate.
        assert_eq!(
            state.leader(start + LEADER_LEASE + chrono::Duration::seconds(1)),
            None
        );
        state.observe(9, start + LEADER_LEASE * 2);
        assert_eq!(
            state.leader(start + LEADER_LEASE + chrono::Duration::seconds(1)),
            Some(9)
        );
    }

    #[test]
    fn leadership_takeover_and_message_propagation() {
        let start = now();
        let broadcaster = Arc::new(RecordingTabBroadcaster::new());
        let hub = Arc::new(HintHub::new());
        let sink = Arc::new(RecordingHintSink::new());
        let account = momentum_domain::AccountId::new();
        hub.subscribe(account, sink.clone());

        let first = TabCoordinator::new(1, account, broadcaster.clone(), hub.clone());
        assert!(first.claim_or_renew(start));
        assert!(first.is_leader(start));

        // A sibling observes the claim and cannot lead while the lease is live.
        let second = TabCoordinator::new(2, account, broadcaster.clone(), hub.clone());
        second.on_message(TabMessage::LeadershipClaimed {
            tab_id: 1,
            expires_at: start + LEADER_LEASE,
        });
        assert_eq!(second.leader(start), Some(1));
        assert!(!second.claim_or_renew(start));

        // After the lease expires the sibling takes over and broadcasts its claim.
        let later = start + LEADER_LEASE + chrono::Duration::seconds(1);
        assert!(!first.is_leader(later));
        assert!(second.claim_or_renew(later));
        assert_eq!(second.leader(later), Some(2));

        // Committed changes from a sibling pull this tab through the local hint sink.
        second.propagate_committed(12);
        assert_eq!(
            broadcaster
                .messages()
                .iter()
                .filter(|message| matches!(message, TabMessage::Committed { .. }))
                .count(),
            1
        );
        first.on_message(TabMessage::Committed { sequence: 12 });
        assert_eq!(sink.hints().len(), 1);
        assert_eq!(sink.hints()[0].sequence, 12);

        // Outbox changes are broadcast, not mutated locally.
        second.propagate_outbox(3);
        assert!(broadcaster
            .messages()
            .iter()
            .any(|message| matches!(message, TabMessage::OutboxChanged { pending: 3 })));
    }
}
