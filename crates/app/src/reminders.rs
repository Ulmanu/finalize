//! Reminder delivery: due-reminder discovery, idempotent scheduling, and push payloads.
//!
//! The scheduler separates discovery (a read model of due reminders) from notification transport
//! (a trait implemented by Web Push or a test recorder). Delivery is marked through the canonical
//! command layer, so a restart or retry cannot re-deliver an already-delivered reminder.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_domain::{AccountId, EntityId};

use crate::commands::CommandError;
use crate::todos::TodoService;

/// A reminder that is due for delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueReminder {
    /// Owning to-do.
    pub todo_id: EntityId,
    /// Account that owns the reminder.
    pub account_id: AccountId,
    /// To-do title used as the notification body.
    pub title: String,
    /// Instant the reminder fires.
    pub at: DateTime<Utc>,
    /// IANA timezone recorded with the reminder.
    pub timezone: String,
    /// Deep link into the application for the owning item.
    pub deep_link: String,
}

/// Read model of due reminders for one account.
#[async_trait]
pub trait ReminderSource: Send + Sync {
    /// Lists scheduled reminders whose instant is at or before `now`.
    async fn due_reminders(
        &self,
        account_id: AccountId,
        now: DateTime<Utc>,
    ) -> Result<Vec<DueReminder>, CommandError>;
}

/// Delivery transport for reminder notifications.
#[async_trait]
pub trait ReminderNotifier: Send + Sync {
    /// Delivers one notification; failures are reported and retried on the next pass.
    async fn notify(&self, reminder: &DueReminder) -> Result<(), String>;
}

/// Per-item results of one scheduler pass.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReminderRun {
    /// Reminders notified and marked delivered.
    pub delivered: Vec<EntityId>,
    /// Reminders that failed delivery with their reasons.
    pub failed: Vec<(EntityId, String)>,
}

/// Delivers due reminders through a notifier and records delivery canonically.
pub struct ReminderScheduler {
    todos: Arc<TodoService>,
    source: Arc<dyn ReminderSource>,
    notifier: Arc<dyn ReminderNotifier>,
}

impl ReminderScheduler {
    /// Creates the scheduler.
    #[must_use]
    pub fn new(
        todos: Arc<TodoService>,
        source: Arc<dyn ReminderSource>,
        notifier: Arc<dyn ReminderNotifier>,
    ) -> Self {
        Self {
            todos,
            source,
            notifier,
        }
    }

    /// Runs one delivery pass.
    ///
    /// Each due reminder is notified once and then marked delivered. A later pass (including
    /// after a restart) sees no due reminders, so delivery cannot storm; failed notifications are
    /// left scheduled for retry.
    ///
    /// # Errors
    ///
    /// Propagates source read failures.
    pub async fn run_once(
        &self,
        account_id: AccountId,
        now: DateTime<Utc>,
    ) -> Result<ReminderRun, CommandError> {
        let mut run = ReminderRun::default();
        for reminder in self.source.due_reminders(account_id, now).await? {
            match self.notifier.notify(&reminder).await {
                Ok(()) => match self
                    .todos
                    .mark_reminder_delivered(account_id, reminder.todo_id)
                    .await
                {
                    Ok(_) => run.delivered.push(reminder.todo_id),
                    Err(error) => run.failed.push((reminder.todo_id, error.to_string())),
                },
                Err(reason) => run.failed.push((reminder.todo_id, reason)),
            }
        }
        Ok(run)
    }
}

/// A stored browser push subscription.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushSubscription {
    /// Push service endpoint URL.
    pub endpoint: String,
    /// Client public key.
    pub p256dh: String,
    /// Client auth secret.
    pub auth: String,
}

/// Persistence for browser push subscriptions.
#[async_trait]
pub trait PushSubscriptionStore: Send + Sync {
    /// Stores (or refreshes) a subscription for the account.
    async fn add(
        &self,
        account_id: AccountId,
        subscription: PushSubscription,
    ) -> Result<(), CommandError>;

    /// Lists the account's subscriptions.
    async fn list(&self, account_id: AccountId) -> Result<Vec<PushSubscription>, CommandError>;
}

/// Transport that posts an encoded notification to a push endpoint.
#[async_trait]
pub trait PushTransport: Send + Sync {
    /// Sends the payload; failures are reported to the scheduler.
    async fn send(&self, endpoint: &str, payload: &str) -> Result<(), String>;
}

/// Notification content for one reminder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushNotification {
    /// Notification title.
    pub title: String,
    /// Notification body.
    pub body: String,
    /// Deep link opened on activation.
    pub url: String,
}

fn json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            control if control.is_control() => {
                use std::fmt::Write;
                let _ = write!(escaped, "\\u{:04x}", control as u32);
            }
            other => escaped.push(other),
        }
    }
    escaped.push('"');
    escaped
}

/// Builds the notification payload for a due reminder.
#[must_use]
pub fn build_push_notification(reminder: &DueReminder) -> PushNotification {
    PushNotification {
        title: "Reminder".to_string(),
        body: reminder.title.clone(),
        url: reminder.deep_link.clone(),
    }
}

/// Notifier that fans a notification out to every stored subscription.
pub struct WebPushNotifier {
    store: Arc<dyn PushSubscriptionStore>,
    transport: Arc<dyn PushTransport>,
}

impl WebPushNotifier {
    /// Creates the notifier.
    #[must_use]
    pub fn new(store: Arc<dyn PushSubscriptionStore>, transport: Arc<dyn PushTransport>) -> Self {
        Self { store, transport }
    }
}

#[async_trait]
impl ReminderNotifier for WebPushNotifier {
    async fn notify(&self, reminder: &DueReminder) -> Result<(), String> {
        let subscriptions = self
            .store
            .list(reminder.account_id)
            .await
            .map_err(|error| error.to_string())?;
        if subscriptions.is_empty() {
            return Err("no push subscriptions".to_string());
        }
        let notification = build_push_notification(reminder);
        let payload = format!(
            "{{\"title\":{},\"body\":{},\"url\":{}}}",
            json_string(&notification.title),
            json_string(&notification.body),
            json_string(&notification.url),
        );
        for subscription in subscriptions {
            self.transport
                .send(&subscription.endpoint, &payload)
                .await?;
        }
        Ok(())
    }
}

/// In-memory subscription store for development, tests, and single-process runs.
#[derive(Debug, Default)]
pub struct MemoryPushSubscriptionStore {
    subscriptions: Mutex<Vec<(AccountId, PushSubscription)>>,
}

impl MemoryPushSubscriptionStore {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl PushSubscriptionStore for MemoryPushSubscriptionStore {
    async fn add(
        &self,
        account_id: AccountId,
        subscription: PushSubscription,
    ) -> Result<(), CommandError> {
        let mut subscriptions = self.subscriptions.lock().expect("push store lock");
        subscriptions.retain(|(account, existing)| {
            *account != account_id || existing.endpoint != subscription.endpoint
        });
        subscriptions.push((account_id, subscription));
        Ok(())
    }

    async fn list(&self, account_id: AccountId) -> Result<Vec<PushSubscription>, CommandError> {
        Ok(self
            .subscriptions
            .lock()
            .expect("push store lock")
            .iter()
            .filter(|(account, _)| *account == account_id)
            .map(|(_, subscription)| subscription.clone())
            .collect())
    }
}
