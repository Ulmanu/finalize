//! Provider-neutral calendar integration with a read-only, failure-tolerant projection.
//!
//! Core application code depends only on [`CalendarProvider`]; provider adapters (Google,
//! Microsoft, CalDAV, or test fakes) live behind it. The projection caches successful reads and
//! degrades to the last known events when a provider fails or its authorization expires, so
//! calendar trouble never blocks or corrupts task data.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::NaiveTime;
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;

/// A calendar event as exposed to the application (always read-only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarEvent {
    /// Provider-stable event identity.
    pub id: String,
    /// Event title.
    pub title: String,
    /// First calendar date the event covers.
    pub starts_on: LocalDate,
    /// Start time, when the event is not all-day.
    pub starts_at: Option<NaiveTime>,
    /// Last calendar date the event covers.
    pub ends_on: LocalDate,
    /// Name of the source calendar.
    pub calendar_name: String,
}

impl CalendarEvent {
    /// External events are never editable from task lists.
    #[must_use]
    pub const fn read_only(&self) -> bool {
        true
    }
}

/// Inclusive date range queried from a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarRange {
    /// First date of the range.
    pub start: LocalDate,
    /// Last date of the range.
    pub end: LocalDate,
}

impl CalendarRange {
    /// Creates a range, ordering the bounds.
    #[must_use]
    pub fn new(start: LocalDate, end: LocalDate) -> Self {
        if start <= end {
            Self { start, end }
        } else {
            Self {
                start: end,
                end: start,
            }
        }
    }
}

/// Provider failures that the projection degrades on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarError {
    /// The provider authorization expired or was revoked.
    AuthExpired,
    /// The provider was unreachable.
    Network,
    /// Any provider-specific failure, described.
    Provider(String),
}

impl CalendarError {
    /// Message shown to the user in the degraded banner.
    #[must_use]
    pub fn degraded_reason(&self) -> String {
        match self {
            Self::AuthExpired => {
                "Calendar authorization expired; reconnect to see current events.".to_string()
            }
            Self::Network => "Calendar service unreachable; showing cached events.".to_string(),
            Self::Provider(message) => format!("Calendar provider error: {message}"),
        }
    }
}

impl std::fmt::Display for CalendarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.degraded_reason())
    }
}

impl std::error::Error for CalendarError {}

/// Provider contract implemented by calendar adapters.
#[async_trait]
pub trait CalendarProvider: Send + Sync {
    /// Lists events overlapping the inclusive range.
    async fn events(
        &self,
        account_id: AccountId,
        range: CalendarRange,
    ) -> Result<Vec<CalendarEvent>, CalendarError>;
}

/// Read-only projection of provider events with graceful degradation.
pub struct CalendarService {
    provider: Option<Arc<dyn CalendarProvider>>,
    cache: Mutex<HashMap<(AccountId, LocalDate, LocalDate), Vec<CalendarEvent>>>,
}

impl std::fmt::Debug for CalendarService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CalendarService")
    }
}

/// Result of a projection read.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CalendarProjection {
    /// Events overlapping the range (possibly cached after a failure).
    pub events: Vec<CalendarEvent>,
    /// Degradation reason, when the provider failed.
    pub degraded: Option<String>,
    /// Whether the returned events came from the cache after a failure.
    pub stale: bool,
}

impl CalendarService {
    /// Creates the service; without a provider the projection is empty and not degraded.
    #[must_use]
    pub fn new(provider: Option<Arc<dyn CalendarProvider>>) -> Self {
        Self {
            provider,
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Projects events for a range.
    ///
    /// On provider success the cache is refreshed; on failure the last cached value is returned
    /// with a degraded reason, and a missing cache yields an empty, non-blocking projection.
    pub async fn events(&self, account_id: AccountId, range: CalendarRange) -> CalendarProjection {
        let Some(provider) = &self.provider else {
            return CalendarProjection::default();
        };
        match provider.events(account_id, range).await {
            Ok(events) => {
                self.cache
                    .lock()
                    .expect("calendar cache lock")
                    .insert((account_id, range.start, range.end), events.clone());
                CalendarProjection {
                    events,
                    degraded: None,
                    stale: false,
                }
            }
            Err(error) => {
                let cached = self
                    .cache
                    .lock()
                    .expect("calendar cache lock")
                    .get(&(account_id, range.start, range.end))
                    .cloned()
                    .unwrap_or_default();
                CalendarProjection {
                    events: cached,
                    degraded: Some(error.degraded_reason()),
                    stale: true,
                }
            }
        }
    }
}

/// In-process provider used for development and tests; wraps any provider with failure injection.
pub struct FlakyCalendarProvider {
    inner: Arc<dyn CalendarProvider>,
    failure: Mutex<Option<CalendarError>>,
}

impl std::fmt::Debug for FlakyCalendarProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FlakyCalendarProvider")
    }
}

impl FlakyCalendarProvider {
    /// Wraps a provider.
    #[must_use]
    pub fn new(inner: Arc<dyn CalendarProvider>) -> Self {
        Self {
            inner,
            failure: Mutex::new(None),
        }
    }

    /// Makes every subsequent read fail with this error until cleared.
    pub fn fail_with(&self, error: CalendarError) {
        *self.failure.lock().expect("failure lock") = Some(error);
    }

    /// Clears failure injection.
    pub fn recover(&self) {
        *self.failure.lock().expect("failure lock") = None;
    }
}

#[async_trait]
impl CalendarProvider for FlakyCalendarProvider {
    async fn events(
        &self,
        account_id: AccountId,
        range: CalendarRange,
    ) -> Result<Vec<CalendarEvent>, CalendarError> {
        if let Some(error) = self.failure.lock().expect("failure lock").clone() {
            return Err(error);
        }
        self.inner.events(account_id, range).await
    }
}
