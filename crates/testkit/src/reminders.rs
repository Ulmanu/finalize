//! Reminder scheduler test doubles: a memory due-reminder source and recording transports.

use std::collections::HashSet;
use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_app::commands::CommandError;
use momentum_app::reminders::{DueReminder, PushTransport, ReminderNotifier, ReminderSource};
use momentum_domain::{AccountId, EntityId};

use crate::commands::MemoryDatabase;

/// Reads due reminders from the in-memory database.
#[derive(Debug, Clone)]
pub struct MemoryReminderSource {
    database: MemoryDatabase,
}

impl MemoryReminderSource {
    /// Creates the source over a database.
    #[must_use]
    pub fn new(database: MemoryDatabase) -> Self {
        Self { database }
    }
}

#[async_trait]
impl ReminderSource for MemoryReminderSource {
    async fn due_reminders(
        &self,
        account_id: AccountId,
        now: DateTime<Utc>,
    ) -> Result<Vec<DueReminder>, CommandError> {
        Ok(self
            .database
            .todos_for(account_id)
            .into_iter()
            .filter_map(|todo| {
                let reminder = todo.reminder?;
                reminder.is_due(now).then(|| DueReminder {
                    todo_id: todo.id,
                    account_id,
                    title: todo.title.clone(),
                    at: reminder.at,
                    timezone: reminder.timezone,
                    deep_link: format!("/todos/{}", todo.id),
                })
            })
            .collect())
    }
}

/// Records notifications and can be told to fail specific reminders.
#[derive(Debug, Default)]
pub struct RecordingNotifier {
    sent: Mutex<Vec<DueReminder>>,
    failing: Mutex<HashSet<EntityId>>,
}

impl RecordingNotifier {
    /// Creates an empty recorder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Makes every delivery for this reminder fail until cleared.
    pub fn fail_for(&self, todo_id: EntityId) {
        self.failing.lock().expect("failing lock").insert(todo_id);
    }

    /// Removes the failure marker.
    pub fn clear_failures(&self) {
        self.failing.lock().expect("failing lock").clear();
    }

    /// Returns the notifications received so far.
    #[must_use]
    pub fn sent(&self) -> Vec<DueReminder> {
        self.sent.lock().expect("sent lock").clone()
    }
}

#[async_trait]
impl ReminderNotifier for RecordingNotifier {
    async fn notify(&self, reminder: &DueReminder) -> Result<(), String> {
        if self
            .failing
            .lock()
            .expect("failing lock")
            .contains(&reminder.todo_id)
        {
            return Err("transport offline".to_string());
        }
        self.sent.lock().expect("sent lock").push(reminder.clone());
        Ok(())
    }
}

/// Records push payloads per endpoint.
#[derive(Debug, Default)]
pub struct RecordingPushTransport {
    sent: Mutex<Vec<(String, String)>>,
}

impl RecordingPushTransport {
    /// Creates an empty recorder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the `(endpoint, payload)` pairs sent so far.
    #[must_use]
    pub fn sent(&self) -> Vec<(String, String)> {
        self.sent.lock().expect("sent lock").clone()
    }
}

#[async_trait]
impl PushTransport for RecordingPushTransport {
    async fn send(&self, endpoint: &str, payload: &str) -> Result<(), String> {
        self.sent
            .lock()
            .expect("sent lock")
            .push((endpoint.to_string(), payload.to_string()));
        Ok(())
    }
}
