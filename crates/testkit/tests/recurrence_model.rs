//! Recurrence model integration: patterns, end conditions, and template copying.

use momentum_domain::model::ContextRef;
use momentum_domain::recurrence::{
    Recurrence, RecurrenceEnd, RecurrencePattern, RecurrenceTemplate, TemplateHeading,
};
use momentum_domain::time::LocalDate;
use momentum_domain::EntityId;

fn date(year: i32, month: u32, day: u32) -> LocalDate {
    LocalDate::from_ymd(year, month, day).expect("date")
}

#[test]
fn recurrence_rules_advance_and_respect_end_conditions() {
    let weekly = Recurrence::new(
        RecurrencePattern::Weekly {
            weekdays: [0_u32, 2].into_iter().collect(),
        },
        RecurrenceEnd::After(3),
    );
    let first = weekly.next_occurrence(date(2026, 6, 10), 0).expect("first");
    assert_eq!(first, date(2026, 6, 15));
    let second = weekly.next_occurrence(first, 1).expect("second");
    assert_eq!(second, date(2026, 6, 17));
    assert!(weekly.next_occurrence(second, 3).is_none());

    let monthly = Recurrence::new(
        RecurrencePattern::Monthly {
            days: [31_u32].into_iter().collect(),
        },
        RecurrenceEnd::On(date(2026, 8, 31)),
    );
    let july = monthly.next_occurrence(date(2026, 6, 30), 0).expect("july");
    assert_eq!(july, date(2026, 7, 31));
    assert!(monthly.next_occurrence(date(2026, 8, 31), 1).is_none());

    let after = Recurrence::new(
        RecurrencePattern::AfterCompletion { days: 7 },
        RecurrenceEnd::Never,
    );
    assert_eq!(
        after.next_occurrence(date(2026, 6, 10), 0),
        Some(date(2026, 6, 17))
    );
}

#[test]
fn templates_materialize_independent_copies() {
    let template = RecurrenceTemplate {
        title: "Deep clean".to_string(),
        notes: "Whole house".to_string(),
        tag_ids: vec![EntityId::new()],
        checklist: vec!["Kitchen".to_string()],
        headings: vec![TemplateHeading {
            title: "Floors".to_string(),
            checklist: vec!["Sweep".to_string()],
        }],
        deadline_offset_days: Some(1),
        reminder_offset_minutes: None,
        context: ContextRef::Inbox,
    };
    let occurrence = date(2026, 7, 1);
    let mut copy = template.copy_for(occurrence);
    copy.title.push_str(" (copy)");
    copy.headings[0].checklist.clear();
    assert_eq!(template.title, "Deep clean");
    assert_eq!(template.headings[0].checklist, vec!["Sweep".to_string()]);
    assert_eq!(
        template.deadline_for(occurrence),
        Some(momentum_domain::model::Deadline::new(date(2026, 7, 2)))
    );
}
