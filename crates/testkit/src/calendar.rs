//! Deterministic fake calendar provider for projection tests.

use std::sync::Mutex;

use async_trait::async_trait;
use momentum_app::calendar::{CalendarError, CalendarEvent, CalendarProvider, CalendarRange};
use momentum_domain::AccountId;

/// Provider over a fixed event list, filtered by range overlap.
#[derive(Debug, Default)]
pub struct FakeCalendarProvider {
    events: Mutex<Vec<CalendarEvent>>,
}

impl FakeCalendarProvider {
    /// Creates a provider with the given events.
    #[must_use]
    pub fn new(events: Vec<CalendarEvent>) -> Self {
        Self {
            events: Mutex::new(events),
        }
    }

    /// Replaces the event list.
    pub fn set_events(&self, events: Vec<CalendarEvent>) {
        *self.events.lock().expect("events lock") = events;
    }
}

#[async_trait]
impl CalendarProvider for FakeCalendarProvider {
    async fn events(
        &self,
        _account_id: AccountId,
        range: CalendarRange,
    ) -> Result<Vec<CalendarEvent>, CalendarError> {
        Ok(self
            .events
            .lock()
            .expect("events lock")
            .iter()
            .filter(|event| event.starts_on <= range.end && event.ends_on >= range.start)
            .cloned()
            .collect())
    }
}
