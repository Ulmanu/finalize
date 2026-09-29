//! Calendar projection: provider-neutral reads, graceful degradation, no corruption.

use std::sync::Arc;

use chrono::NaiveTime;
use momentum_app::calendar::{
    CalendarError, CalendarEvent, CalendarProvider, CalendarRange, CalendarService,
    FlakyCalendarProvider,
};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::calendar::FakeCalendarProvider;

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

fn event(id: &str, title: &str, day: u32) -> CalendarEvent {
    CalendarEvent {
        id: id.to_string(),
        title: title.to_string(),
        starts_on: date(day),
        starts_at: NaiveTime::from_hms_opt(9, 0, 0),
        ends_on: date(day),
        calendar_name: "Work".to_string(),
    }
}

#[tokio::test]
async fn projection_returns_read_only_events_for_the_range() {
    let provider = Arc::new(FakeCalendarProvider::new(vec![
        event("a", "Standup", 10),
        event("b", "Far away", 25),
    ]));
    let service = CalendarService::new(Some(provider));
    let account = AccountId::new();
    let projection = service
        .events(account, CalendarRange::new(date(9), date(12)))
        .await;
    assert_eq!(projection.events.len(), 1);
    assert_eq!(projection.events[0].title, "Standup");
    assert!(projection.events[0].read_only());
    assert!(projection.degraded.is_none());
    assert!(!projection.stale);
}

#[tokio::test]
async fn provider_failure_degrades_to_cached_events_without_corruption() {
    let fake = Arc::new(FakeCalendarProvider::new(vec![event("a", "Standup", 10)]));
    let flaky = Arc::new(FlakyCalendarProvider::new(fake.clone()));
    let service = CalendarService::new(Some(flaky.clone()));
    let account = AccountId::new();
    let range = CalendarRange::new(date(9), date(12));

    let fresh = service.events(account, range).await;
    assert_eq!(fresh.events.len(), 1);
    assert!(fresh.degraded.is_none());

    flaky.fail_with(CalendarError::AuthExpired);
    let degraded = service.events(account, range).await;
    assert_eq!(degraded.events.len(), 1);
    assert_eq!(degraded.events[0].title, "Standup");
    assert!(degraded.stale);
    assert!(degraded
        .degraded
        .as_deref()
        .unwrap_or_default()
        .contains("authorization expired"));

    // The cache survives repeated failures; recovery returns fresh data again.
    let again = service.events(account, range).await;
    assert_eq!(again.events.len(), 1);
    assert!(again.stale);
    flaky.recover();
    let recovered = service.events(account, range).await;
    assert!(recovered.degraded.is_none());
    assert!(!recovered.stale);
}

#[tokio::test]
async fn failure_without_cache_is_empty_and_non_blocking() {
    let fake = Arc::new(FakeCalendarProvider::new(vec![event("a", "Standup", 10)]));
    let flaky = Arc::new(FlakyCalendarProvider::new(fake));
    flaky.fail_with(CalendarError::Network);
    let service = CalendarService::new(Some(flaky));
    let projection = service
        .events(AccountId::new(), CalendarRange::new(date(9), date(12)))
        .await;
    assert!(projection.events.is_empty());
    assert!(projection.stale);
    assert!(projection
        .degraded
        .as_deref()
        .unwrap_or_default()
        .contains("unreachable"));

    // A service without a provider is simply empty and not degraded.
    let none = CalendarService::new(None);
    let projection = none
        .events(AccountId::new(), CalendarRange::new(date(9), date(12)))
        .await;
    assert!(projection.events.is_empty());
    assert!(projection.degraded.is_none());
}

// Keeps the trait import meaningful for implementors reading this file.
#[allow(dead_code)]
fn _assert_object_safe(provider: Arc<dyn CalendarProvider>) -> Arc<dyn CalendarProvider> {
    provider
}
