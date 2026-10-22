//! Read-model values must round-trip through the browser hydration payload.

use momentum_domain::model::{ContextRef, Schedule};
use momentum_domain::time::LocalDate;
use momentum_domain::EntityId;

#[test]
fn dated_schedules_and_context_ids_round_trip_as_json() {
    let date = LocalDate::from_ymd(2026, 10, 21).expect("date");
    for schedule in [Schedule::anytime(), Schedule::someday(), Schedule::on(date)] {
        let json = serde_json::to_value(schedule).expect("serialize schedule");
        assert_eq!(serde_json::from_value::<Schedule>(json).unwrap(), schedule);
    }
    let id = EntityId::new();
    for context in [
        ContextRef::Inbox,
        ContextRef::Area(id),
        ContextRef::Project(id),
        ContextRef::Heading(id),
    ] {
        let json = serde_json::to_value(context).expect("serialize context");
        assert_eq!(serde_json::from_value::<ContextRef>(json).unwrap(), context);
    }
}
