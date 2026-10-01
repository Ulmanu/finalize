//! Realtime invalidation hints feeding the canonical sync protocol.
//!
//! Hints never carry mutations: they only tell connected clients that newer server sequence data
//! exists, and clients remain responsible for pulling through the cursor-based sync protocol.
//! Hints are coalesced per account while one is still pending delivery, so a busy account cannot
//! stampede clients into unbounded pull loops.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use momentum_domain::AccountId;

/// Depth of change-log sequence a hint refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidationHint {
    /// Newest server sequence observed.
    pub sequence: i64,
}

/// Receives invalidation hints for one connection.
pub trait HintSink: Send + Sync {
    /// Called once per coalesced hint.
    fn hint(&self, hint: InvalidationHint);
}

struct HubState {
    subscribers: HashMap<u64, (AccountId, Arc<dyn HintSink>)>,
    pending: HashMap<AccountId, InvalidationHint>,
}

/// Coalescing hint hub shared by sync pushes and realtime transports.
pub struct HintHub {
    next_id: AtomicU64,
    state: Mutex<HubState>,
}

impl std::fmt::Debug for HintHub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HintHub")
    }
}

impl Default for HintHub {
    fn default() -> Self {
        Self::new()
    }
}

impl HintHub {
    /// Creates an empty hub.
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            state: Mutex::new(HubState {
                subscribers: HashMap::new(),
                pending: HashMap::new(),
            }),
        }
    }

    /// Registers a connection, returning its subscription id.
    pub fn subscribe(&self, account_id: AccountId, sink: Arc<dyn HintSink>) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.state
            .lock()
            .expect("hint hub lock")
            .subscribers
            .insert(id, (account_id, sink));
        id
    }

    /// Removes a connection.
    pub fn unsubscribe(&self, subscription: u64) {
        self.state
            .lock()
            .expect("hint hub lock")
            .subscribers
            .remove(&subscription);
    }

    /// Publishes a hint for an account.
    ///
    /// While a previous hint for the account is still pending delivery, the newest sequence is
    /// recorded but no additional hint is sent (coalescing); the pending hint always carries the
    /// latest sequence. Returns whether a hint was actually emitted.
    pub fn publish(&self, account_id: AccountId, sequence: i64) -> bool {
        let mut state = self.state.lock().expect("hint hub lock");
        let has_subscriber = state
            .subscribers
            .values()
            .any(|(account, _)| *account == account_id);
        if !has_subscriber {
            return false;
        }
        let hint = InvalidationHint { sequence };
        if let Some(pending) = state.pending.get_mut(&account_id) {
            pending.sequence = pending.sequence.max(sequence);
            return false;
        }
        state.pending.insert(account_id, hint);
        for (account, sink) in state.subscribers.values() {
            if *account == account_id {
                sink.hint(hint);
            }
        }
        true
    }

    /// Marks the pending hint as delivered, letting the next publish emit again.
    pub fn delivered(&self, account_id: AccountId) {
        self.state
            .lock()
            .expect("hint hub lock")
            .pending
            .remove(&account_id);
    }

    /// The pending sequence for an account, when a hint is awaiting delivery.
    #[must_use]
    pub fn pending(&self, account_id: AccountId) -> Option<InvalidationHint> {
        self.state
            .lock()
            .expect("hint hub lock")
            .pending
            .get(&account_id)
            .copied()
    }

    /// Number of registered connections.
    #[must_use]
    pub fn subscriber_count(&self) -> usize {
        self.state.lock().expect("hint hub lock").subscribers.len()
    }
}

/// Recording sink for tests.
#[derive(Debug, Default)]
pub struct RecordingHintSink {
    hints: Mutex<Vec<InvalidationHint>>,
}

impl RecordingHintSink {
    /// Creates an empty recorder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the hints received so far.
    #[must_use]
    pub fn hints(&self) -> Vec<InvalidationHint> {
        self.hints.lock().expect("hints lock").clone()
    }
}

impl HintSink for RecordingHintSink {
    fn hint(&self, hint: InvalidationHint) {
        self.hints.lock().expect("hints lock").push(hint);
    }
}

#[cfg(test)]
mod tests {
    use super::{HintHub, RecordingHintSink};
    use std::sync::Arc;

    #[test]
    fn hints_coalesce_until_delivered() {
        let hub = HintHub::new();
        let account = momentum_domain::AccountId::new();
        let sink = Arc::new(RecordingHintSink::new());
        let subscription = hub.subscribe(account, sink.clone());

        assert!(hub.publish(account, 3));
        // A burst while pending coalesces into the newest sequence with no extra sends.
        assert!(!hub.publish(account, 4));
        assert!(!hub.publish(account, 2));
        assert_eq!(hub.pending(account).map(|hint| hint.sequence), Some(4));
        assert_eq!(sink.hints().len(), 1);
        assert_eq!(sink.hints()[0].sequence, 3);

        // Delivery unblocks the next hint, which carries the coalesced newest sequence.
        hub.delivered(account);
        assert!(hub.publish(account, 5));
        assert_eq!(sink.hints().len(), 2);
        assert_eq!(sink.hints()[1].sequence, 5);

        hub.unsubscribe(subscription);
        assert_eq!(hub.subscriber_count(), 0);
        hub.delivered(account);
        assert!(!hub.publish(account, 6));
    }

    #[test]
    fn hints_are_account_scoped_and_reconnect_safe() {
        let hub = HintHub::new();
        let first = momentum_domain::AccountId::new();
        let second = momentum_domain::AccountId::new();
        let sink = Arc::new(RecordingHintSink::new());
        hub.subscribe(first, sink.clone());

        assert!(!hub.publish(second, 9));
        assert!(hub.publish(first, 1));
        assert_eq!(sink.hints().len(), 1);

        // A reconnecting client relies on the cursor protocol; a fresh hint is emitted for the
        // new subscription after the old one leaves a pending hint behind.
        hub.delivered(first);
        assert!(hub.publish(first, 2));
        assert_eq!(sink.hints().last().map(|hint| hint.sequence), Some(2));
    }
}
