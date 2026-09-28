//! Recurrence patterns, end conditions, and copy templates.
//!
//! Regular recurrences advance from the scheduled date; after-completion recurrences advance
//! from the completion date. End conditions are evaluated against the occurrence count so a
//! recurrence stops deterministically. Templates are plain owned values: future copies inherit
//! the structure without sharing any mutable state.

use std::collections::BTreeSet;

use chrono::Datelike;

use crate::model::{ContextRef, Deadline};
use crate::time::LocalDate;
use crate::EntityId;

/// How a recurring item repeats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecurrencePattern {
    /// Every calendar day.
    Daily,
    /// Selected weekdays, at most one occurrence per selected day.
    Weekly {
        /// Chosen weekdays as days from Monday (`0` = Monday through `6` = Sunday).
        weekdays: BTreeSet<u32>,
    },
    /// Selected days of each month.
    Monthly {
        /// Chosen day numbers (1-31).
        days: BTreeSet<u32>,
    },
    /// The same calendar date each year.
    Yearly,
    /// A fixed number of days after the item is completed.
    AfterCompletion {
        /// Delay in days.
        days: u32,
    },
}

impl RecurrencePattern {
    /// Whether this pattern advances from completion rather than the schedule.
    #[must_use]
    pub const fn is_after_completion(&self) -> bool {
        matches!(self, Self::AfterCompletion { .. })
    }

    /// The next date strictly after `base`.
    ///
    /// `base` is the scheduled date for regular patterns and the completion date for
    /// after-completion patterns.
    #[must_use]
    pub fn next_after(&self, base: LocalDate) -> LocalDate {
        match self {
            Self::Daily => base.next_day(),
            Self::Weekly { weekdays } => next_weekday(base, weekdays),
            Self::Monthly { days } => next_month_day(base, days),
            Self::Yearly => next_year_same_date(base),
            Self::AfterCompletion { days } => advance(base, i64::from(*days)),
        }
    }
}

/// When a recurrence stops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecurrenceEnd {
    /// Repeats indefinitely.
    Never,
    /// The last occurrence is on this date (inclusive).
    On(LocalDate),
    /// Stops after this many occurrences (at least one).
    After(u32),
}

/// A configured recurrence rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recurrence {
    /// Repetition pattern.
    pub pattern: RecurrencePattern,
    /// Stop condition.
    pub end: RecurrenceEnd,
}

impl Recurrence {
    /// Creates a rule.
    #[must_use]
    pub const fn new(pattern: RecurrencePattern, end: RecurrenceEnd) -> Self {
        Self { pattern, end }
    }

    /// Advances the rule from `base`, given how many occurrences already exist.
    ///
    /// Returns `None` when an end condition is reached. After-completion patterns stop when the
    /// occurrence count is exhausted; date-bounded patterns stop when the next date would pass
    /// the bound.
    #[must_use]
    pub fn next_occurrence(&self, base: LocalDate, occurrences: u32) -> Option<LocalDate> {
        if let RecurrenceEnd::After(limit) = self.end {
            if occurrences >= limit {
                return None;
            }
        }
        let next = self.pattern.next_after(base);
        if let RecurrenceEnd::On(bound) = self.end {
            if next > bound {
                return None;
            }
        }
        Some(next)
    }
}

fn advance(base: LocalDate, days: i64) -> LocalDate {
    let mut date = base;
    for _ in 0..days.max(0) {
        date = date.next_day();
    }
    date
}

fn next_weekday(base: LocalDate, weekdays: &BTreeSet<u32>) -> LocalDate {
    if weekdays.is_empty() {
        return base.next_day();
    }
    let current = base.as_naive().weekday().num_days_from_monday();
    for step in 1..=7 {
        let candidate = (current + step) % 7;
        if weekdays.contains(&candidate) {
            return advance(base, i64::from(step));
        }
    }
    base.next_day()
}

fn next_month_day(base: LocalDate, days: &BTreeSet<u32>) -> LocalDate {
    if days.is_empty() {
        return base.next_day();
    }
    let mut date = base.next_day();
    for _ in 0..400 {
        if days.contains(&date.day()) {
            return date;
        }
        date = date.next_day();
    }
    date
}

fn next_year_same_date(base: LocalDate) -> LocalDate {
    let year = base.year() + 1;
    let month = base.month();
    let day = base.day();
    for candidate in (1..=day).rev() {
        if let Ok(date) = LocalDate::from_ymd(year, month, candidate) {
            return date;
        }
    }
    base.next_day()
}

/// A heading inherited by future copies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateHeading {
    /// Heading title.
    pub title: String,
    /// Checklist item titles under the heading.
    pub checklist: Vec<String>,
}

/// Immutable snapshot of everything a future copy inherits.
///
/// All fields are owned values, so materializing a copy never shares mutable state with the
/// completed original.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecurrenceTemplate {
    /// Future-copy title.
    pub title: String,
    /// Future-copy notes.
    pub notes: String,
    /// Tag identities applied to the copy.
    pub tag_ids: Vec<EntityId>,
    /// Checklist item titles for a to-do.
    pub checklist: Vec<String>,
    /// Heading structure for a project.
    pub headings: Vec<TemplateHeading>,
    /// Days from the occurrence date to the deadline.
    pub deadline_offset_days: Option<i64>,
    /// Minutes from the occurrence to the reminder.
    pub reminder_offset_minutes: Option<i64>,
    /// Canonical context of the copy.
    pub context: ContextRef,
}

impl RecurrenceTemplate {
    /// The deadline for one occurrence, when the template defines an offset.
    #[must_use]
    pub fn deadline_for(&self, occurrence: LocalDate) -> Option<Deadline> {
        self.deadline_offset_days
            .map(|offset| Deadline::new(advance(occurrence, offset)))
    }

    /// Materializes a fresh, fully owned copy template for one occurrence.
    #[must_use]
    pub fn copy_for(&self, _occurrence: LocalDate) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> LocalDate {
        LocalDate::from_ymd(year, month, day).expect("date")
    }

    fn weekdays(days: &[u32]) -> BTreeSet<u32> {
        days.iter().copied().collect()
    }

    #[test]
    fn regular_patterns_advance_from_the_base_date() {
        let base = date(2026, 6, 10); // Wednesday
        assert_eq!(RecurrencePattern::Daily.next_after(base), date(2026, 6, 11));
        assert_eq!(
            RecurrencePattern::Weekly {
                weekdays: weekdays(&[0, 4])
            }
            .next_after(base),
            date(2026, 6, 12)
        );
        assert_eq!(
            RecurrencePattern::Weekly {
                weekdays: weekdays(&[2])
            }
            .next_after(base),
            date(2026, 6, 17)
        );
        assert_eq!(
            RecurrencePattern::Monthly {
                days: [15_u32].into_iter().collect()
            }
            .next_after(base),
            date(2026, 6, 15)
        );
        assert_eq!(
            RecurrencePattern::Monthly {
                days: [1_u32].into_iter().collect()
            }
            .next_after(base),
            date(2026, 7, 1)
        );
        assert_eq!(
            RecurrencePattern::Yearly.next_after(base),
            date(2027, 6, 10)
        );
        assert_eq!(
            RecurrencePattern::Yearly.next_after(date(2028, 2, 29)),
            date(2029, 2, 28)
        );
    }

    #[test]
    fn after_completion_advances_from_completion() {
        let completed = date(2026, 6, 10);
        let pattern = RecurrencePattern::AfterCompletion { days: 3 };
        assert!(pattern.is_after_completion());
        assert_eq!(pattern.next_after(completed), date(2026, 6, 13));
        let rule = Recurrence::new(pattern, RecurrenceEnd::Never);
        assert_eq!(rule.next_occurrence(completed, 0), Some(date(2026, 6, 13)));
    }

    #[test]
    fn end_conditions_stop_recurrence() {
        let base = date(2026, 6, 10);
        let rule = Recurrence::new(RecurrencePattern::Daily, RecurrenceEnd::After(2));
        assert_eq!(rule.next_occurrence(base, 0), Some(date(2026, 6, 11)));
        assert_eq!(rule.next_occurrence(base, 1), Some(date(2026, 6, 11)));
        assert_eq!(rule.next_occurrence(base, 2), None);

        let bounded = Recurrence::new(
            RecurrencePattern::Daily,
            RecurrenceEnd::On(date(2026, 6, 11)),
        );
        assert_eq!(bounded.next_occurrence(base, 0), Some(date(2026, 6, 11)));
        assert_eq!(bounded.next_occurrence(date(2026, 6, 11), 0), None);
    }

    #[test]
    fn templates_inherit_structure_without_sharing_state() {
        let tag = EntityId::new();
        let template = RecurrenceTemplate {
            title: "Weekly review".to_string(),
            notes: "Check inbox".to_string(),
            tag_ids: vec![tag],
            checklist: vec!["Empty inbox".to_string()],
            headings: vec![TemplateHeading {
                title: "Prep".to_string(),
                checklist: vec!["Print report".to_string()],
            }],
            deadline_offset_days: Some(2),
            reminder_offset_minutes: Some(-30),
            context: ContextRef::Inbox,
        };
        let occurrence = date(2026, 6, 10);
        let mut copy = template.copy_for(occurrence);
        assert_eq!(copy, template);
        copy.checklist.push("Extra".to_string());
        copy.headings[0].checklist.push("Extra".to_string());
        copy.tag_ids.clear();
        assert_eq!(template.checklist, vec!["Empty inbox".to_string()]);
        assert_eq!(
            template.headings[0].checklist,
            vec!["Print report".to_string()]
        );
        assert_eq!(template.tag_ids, vec![tag]);
        assert_eq!(
            template.deadline_for(occurrence),
            Some(Deadline::new(date(2026, 6, 12)))
        );
    }
}
