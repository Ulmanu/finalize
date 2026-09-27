//! Deterministic natural-language date parsing with locale-aware conventions.
//!
//! The parser is offline and LLM-free: every supported expression maps to a calendar date
//! relative to a supplied base date, with an optional reminder time. Numeric dates that could be
//! read two ways are resolved by the configured locale; without a locale they produce
//! [`NaturalDateError::Ambiguous`] with both candidates instead of silently guessing.

use std::fmt;

use chrono::{Datelike, NaiveTime, Weekday};

use crate::time::LocalDate;

/// Regional conventions that resolve numeric day/month order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    /// Month before day (`June 5` written `06/05`).
    EnUs,
    /// Day before month (`5 June` written `05/06`).
    EnGb,
}

impl Locale {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnUs => "en-US",
            Self::EnGb => "en-GB",
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Parse failures that callers can recover from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NaturalDateError {
    /// The expression was empty after trimming.
    Empty,
    /// No supported expression matched.
    Unknown(String),
    /// A numeric date could be day/month or month/day; both candidates are provided.
    Ambiguous {
        /// The original expression.
        text: String,
        /// Both candidate dates, earliest first.
        candidates: Vec<LocalDate>,
    },
    /// The expression matched a shape but its values are invalid (for example month 13).
    Invalid(String),
}

impl fmt::Display for NaturalDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty date expression"),
            Self::Unknown(text) => write!(f, "unsupported date expression: {text}"),
            Self::Ambiguous { text, .. } => write!(f, "ambiguous date expression: {text}"),
            Self::Invalid(text) => write!(f, "invalid date expression: {text}"),
        }
    }
}

impl std::error::Error for NaturalDateError {}

/// A parsed date with an optional wall-clock reminder time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedNaturalDate {
    /// The resolved calendar date.
    pub date: LocalDate,
    /// The reminder time parsed from the expression, when present.
    pub time: Option<NaiveTime>,
}

/// Parses a supported natural date expression relative to `base`.
///
/// `locale` resolves numeric day/month ambiguity; passing `None` makes such input return
/// [`NaturalDateError::Ambiguous`] with both candidates.
///
/// # Errors
///
/// Returns a [`NaturalDateError`] when the expression is empty, unsupported, invalid, or
/// ambiguous without a locale.
pub fn parse_natural_date(
    expression: &str,
    base: LocalDate,
    locale: Option<Locale>,
) -> Result<ParsedNaturalDate, NaturalDateError> {
    let text = expression.trim().to_lowercase();
    if text.is_empty() {
        return Err(NaturalDateError::Empty);
    }
    let (date_part, time) = split_time(&text)?;
    let date = parse_date_part(date_part.trim(), &text, base, locale)?;
    Ok(ParsedNaturalDate { date, time })
}

fn split_time(text: &str) -> Result<(String, Option<NaiveTime>), NaturalDateError> {
    let mut words: Vec<String> = text.split_whitespace().map(str::to_string).collect();
    let mut time = None;
    let mut index = 0;
    while index < words.len() {
        let word = words[index].clone();
        if word == "at" && index + 1 < words.len() && time.is_none() {
            if let Some(parsed) = parse_time(&words[index + 1]) {
                time = Some(parsed);
                words.drain(index..=index + 1);
                continue;
            }
        }
        if time.is_none() {
            if let Some(parsed) = parse_time(&word) {
                time = Some(parsed);
                words.remove(index);
                continue;
            }
        }
        index += 1;
    }
    if text.contains("tonight") && time.is_none() {
        time = NaiveTime::from_hms_opt(19, 0, 0);
    }
    Ok((words.join(" "), time))
}

fn parse_time(word: &str) -> Option<NaiveTime> {
    let trimmed = word.trim_matches(|character: char| character == ',' || character == '.');
    if let Some((hour, minute, meridiem)) = split_meridiem(trimmed) {
        let mut hour = hour?;
        let minute = minute.unwrap_or(0);
        if !(1..=12).contains(&hour) || minute > 59 {
            return None;
        }
        if meridiem == "pm" && hour < 12 {
            hour += 12;
        }
        if meridiem == "am" && hour == 12 {
            hour = 0;
        }
        return NaiveTime::from_hms_opt(hour, minute, 0);
    }
    let (hour, minute) = trimmed.split_once(':')?;
    let hour: u32 = hour.parse().ok()?;
    let minute: u32 = minute.parse().ok()?;
    if hour > 23 || minute > 59 {
        return None;
    }
    NaiveTime::from_hms_opt(hour, minute, 0)
}

fn split_meridiem(token: &str) -> Option<(Option<u32>, Option<u32>, &'static str)> {
    let (meridiem, prefix) = token
        .strip_suffix("am")
        .map(|prefix| ("am", prefix))
        .or_else(|| token.strip_suffix("pm").map(|prefix| ("pm", prefix)))?;
    let (hour, minute) = match prefix.split_once(':') {
        Some((hour, minute)) => (hour.parse().ok(), minute.parse().ok()),
        None => (prefix.parse().ok(), None),
    };
    Some((hour, minute, meridiem))
}

fn parse_date_part(
    part: &str,
    original: &str,
    base: LocalDate,
    locale: Option<Locale>,
) -> Result<LocalDate, NaturalDateError> {
    if part.is_empty() {
        return Ok(base);
    }
    if let Some(date) = parse_keyword(part, base) {
        return Ok(date);
    }
    if let Some(date) = parse_offset(part, base)? {
        return Ok(date);
    }
    if let Some(date) = parse_weekday(part, base) {
        return Ok(date);
    }
    if let Some(date) = parse_named_month(part, base)? {
        return Ok(date);
    }
    parse_numeric(part, original, base, locale)
}

fn parse_keyword(part: &str, base: LocalDate) -> Option<LocalDate> {
    match part {
        "today" | "tonight" => Some(base),
        "tomorrow" | "tomorrow's" => Some(base.next_day()),
        "yesterday" => Some(base.previous_day()),
        "next week" => Some(add_days(base, 7)),
        _ => None,
    }
}

fn parse_offset(part: &str, base: LocalDate) -> Result<Option<LocalDate>, NaturalDateError> {
    let words: Vec<&str> = part.split_whitespace().collect();
    if words.first() == Some(&"in") && words.len() == 3 {
        if let Ok(count) = words[1].parse::<i64>() {
            return Ok(offset_by(count, words[2], base));
        }
    }
    if words.len() == 4 && words[1] == "days" && words[2] == "from" && words[3] == "now" {
        if let Ok(count) = words[0].parse::<i64>() {
            return Ok(Some(add_days(base, count)));
        }
    }
    if part == "next month" {
        return Ok(Some(add_months(base, 1)));
    }
    Ok(None)
}

fn offset_by(count: i64, unit: &str, base: LocalDate) -> Option<LocalDate> {
    match unit.trim_end_matches('s') {
        "day" => Some(add_days(base, count)),
        "week" => Some(add_days(base, count * 7)),
        "month" => Some(add_months(base, count)),
        _ => None,
    }
}

fn parse_weekday(part: &str, base: LocalDate) -> Option<LocalDate> {
    let (next, day) = match part.strip_prefix("next ") {
        Some(day) => (true, day),
        None => (false, part),
    };
    let target = weekday_from_name(day)?;
    let current = weekday_number(base.as_naive().weekday());
    let target_number = weekday_number(target);
    let mut delta = (target_number - current).rem_euclid(7);
    if next {
        delta = if delta == 0 { 7 } else { delta + 7 };
    }
    Some(add_days(base, i64::from(delta)))
}

fn weekday_number(day: Weekday) -> i32 {
    day.num_days_from_monday() as i32
}

fn weekday_from_name(name: &str) -> Option<Weekday> {
    let name = name.trim_end_matches('.');
    match &name[..name.len().min(3)] {
        "mon" => Some(Weekday::Mon),
        "tue" => Some(Weekday::Tue),
        "wed" => Some(Weekday::Wed),
        "thu" => Some(Weekday::Thu),
        "fri" => Some(Weekday::Fri),
        "sat" => Some(Weekday::Sat),
        "sun" => Some(Weekday::Sun),
        _ => None,
    }
}

fn parse_named_month(part: &str, base: LocalDate) -> Result<Option<LocalDate>, NaturalDateError> {
    let cleaned = strip_ordinals(part);
    let words: Vec<&str> = cleaned.split_whitespace().collect();
    let (month, day) = match words.as_slice() {
        [first, second] => match (month_name(first), month_name(second)) {
            (Some(month), None) => (Some(month), second.parse::<u32>().ok()),
            (None, Some(month)) => (Some(month), first.parse::<u32>().ok()),
            _ => return Ok(None),
        },
        _ => return Ok(None),
    };
    let (Some(month), Some(day)) = (month, day) else {
        return Ok(None);
    };
    if day == 0 || day > 31 {
        return Err(NaturalDateError::Invalid(part.to_string()));
    }
    let date = LocalDate::from_ymd(base.year(), month, day)
        .map_err(|_| NaturalDateError::Invalid(part.to_string()))?;
    Ok(Some(if date < base {
        LocalDate::from_ymd(base.year() + 1, month, day)
            .map_err(|_| NaturalDateError::Invalid(part.to_string()))?
    } else {
        date
    }))
}

fn parse_numeric(
    part: &str,
    original: &str,
    base: LocalDate,
    locale: Option<Locale>,
) -> Result<LocalDate, NaturalDateError> {
    let cleaned = part.trim_end_matches('.').replace(['.', '-'], "/");
    let Some((first, second)) = cleaned.split_once('/') else {
        return Err(NaturalDateError::Unknown(original.to_string()));
    };
    let (Ok(first), Ok(second)) = (first.trim().parse::<u32>(), second.trim().parse::<u32>())
    else {
        return Err(NaturalDateError::Invalid(original.to_string()));
    };
    if !(1..=31).contains(&first) || !(1..=31).contains(&second) {
        return Err(NaturalDateError::Invalid(original.to_string()));
    }
    let month_first = |month: u32, day: u32| roll_forward(base, month, day);
    let day_first = |day: u32, month: u32| roll_forward(base, month, day);
    if first > 12 && second > 12 {
        return Err(NaturalDateError::Invalid(original.to_string()));
    }
    if first > 12 {
        return month_first(second, first)
            .map_err(|_| NaturalDateError::Invalid(original.to_string()));
    }
    if second > 12 {
        return month_first(first, second)
            .map_err(|_| NaturalDateError::Invalid(original.to_string()));
    }
    match locale {
        Some(Locale::EnUs) => {
            month_first(first, second).map_err(|_| NaturalDateError::Invalid(original.to_string()))
        }
        Some(Locale::EnGb) => {
            day_first(first, second).map_err(|_| NaturalDateError::Invalid(original.to_string()))
        }
        None => {
            let month_first = month_first(first, second)
                .map_err(|_| NaturalDateError::Invalid(original.to_string()))?;
            let day_first = day_first(first, second)
                .map_err(|_| NaturalDateError::Invalid(original.to_string()))?;
            let mut candidates = vec![month_first, day_first];
            candidates.sort_unstable();
            candidates.dedup();
            Err(NaturalDateError::Ambiguous {
                text: original.to_string(),
                candidates,
            })
        }
    }
}

fn roll_forward(base: LocalDate, month: u32, day: u32) -> Result<LocalDate, NaturalDateError> {
    let date = LocalDate::from_ymd(base.year(), month, day)
        .map_err(|_| NaturalDateError::Invalid(format!("{month}/{day}")))?;
    Ok(if date < base {
        LocalDate::from_ymd(base.year() + 1, month, day)
            .map_err(|_| NaturalDateError::Invalid(format!("{month}/{day}")))?
    } else {
        date
    })
}

fn month_name(name: &str) -> Option<u32> {
    let name = name.trim_end_matches('.');
    match &name[..name.len().min(3)] {
        "jan" => Some(1),
        "feb" => Some(2),
        "mar" => Some(3),
        "apr" => Some(4),
        "may" => Some(5),
        "jun" => Some(6),
        "jul" => Some(7),
        "aug" => Some(8),
        "sep" => Some(9),
        "oct" => Some(10),
        "nov" => Some(11),
        "dec" => Some(12),
        _ => None,
    }
}

fn strip_ordinals(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let digits = word.trim_end_matches(|character: char| character.is_ascii_alphabetic());
            if digits.is_empty() || digits.len() == word.len() {
                word.to_string()
            } else {
                digits.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn add_days(base: LocalDate, days: i64) -> LocalDate {
    let mut date = base;
    for _ in 0..days.max(0) {
        date = date.next_day();
    }
    date
}

fn add_months(base: LocalDate, months: i64) -> LocalDate {
    let total = i64::from(base.year()) * 12 + i64::from(base.month() - 1) + months.max(0);
    let year = i32::try_from(total / 12).unwrap_or(base.year());
    let month = u32::try_from(total % 12 + 1).unwrap_or(1);
    let day = base.day();
    for candidate in (1..=day).rev() {
        if let Ok(date) = LocalDate::from_ymd(year, month, candidate) {
            return date;
        }
    }
    base
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> LocalDate {
        LocalDate::from_ymd(2026, 6, 10).expect("base")
    }

    fn date(year: i32, month: u32, day: u32) -> LocalDate {
        LocalDate::from_ymd(year, month, day).expect("date")
    }

    #[test]
    fn relative_keywords_and_offsets() {
        let locale = Some(Locale::EnUs);
        assert_eq!(
            parse_natural_date("today", base(), locale).unwrap().date,
            date(2026, 6, 10)
        );
        assert_eq!(
            parse_natural_date("Tomorrow", base(), locale).unwrap().date,
            date(2026, 6, 11)
        );
        assert_eq!(
            parse_natural_date("in 3 days", base(), locale)
                .unwrap()
                .date,
            date(2026, 6, 13)
        );
        assert_eq!(
            parse_natural_date("in 2 weeks", base(), locale)
                .unwrap()
                .date,
            date(2026, 6, 24)
        );
        assert_eq!(
            parse_natural_date("next week", base(), locale)
                .unwrap()
                .date,
            date(2026, 6, 17)
        );
        assert_eq!(
            parse_natural_date("in 1 month", base(), locale)
                .unwrap()
                .date,
            date(2026, 7, 10)
        );
    }

    #[test]
    fn weekdays_use_upcoming_and_following_week_semantics() {
        let locale = Some(Locale::EnUs);
        assert_eq!(
            parse_natural_date("friday", base(), locale).unwrap().date,
            date(2026, 6, 12)
        );
        assert_eq!(
            parse_natural_date("wednesday", base(), locale)
                .unwrap()
                .date,
            date(2026, 6, 10)
        );
        assert_eq!(
            parse_natural_date("next wednesday", base(), locale)
                .unwrap()
                .date,
            date(2026, 6, 17)
        );
        assert_eq!(
            parse_natural_date("next friday", base(), locale)
                .unwrap()
                .date,
            date(2026, 6, 19)
        );
    }

    #[test]
    fn named_month_expressions_roll_forward() {
        let locale = Some(Locale::EnUs);
        assert_eq!(
            parse_natural_date("june 20", base(), locale).unwrap().date,
            date(2026, 6, 20)
        );
        assert_eq!(
            parse_natural_date("20 jun", base(), locale).unwrap().date,
            date(2026, 6, 20)
        );
        assert_eq!(
            parse_natural_date("jun 20th", base(), locale).unwrap().date,
            date(2026, 6, 20)
        );
        assert_eq!(
            parse_natural_date("january 5", base(), locale)
                .unwrap()
                .date,
            date(2027, 1, 5)
        );
    }

    #[test]
    fn numeric_dates_respect_locale_and_report_ambiguity() {
        assert_eq!(
            parse_natural_date("05/06", base(), Some(Locale::EnUs))
                .unwrap()
                .date,
            date(2027, 5, 6)
        );
        assert_eq!(
            parse_natural_date("05/06", base(), Some(Locale::EnGb))
                .unwrap()
                .date,
            date(2027, 6, 5)
        );
        match parse_natural_date("05/06", base(), None) {
            Err(NaturalDateError::Ambiguous { candidates, .. }) => {
                assert_eq!(candidates, vec![date(2027, 5, 6), date(2027, 6, 5)]);
            }
            other => panic!("expected ambiguity, got {other:?}"),
        }
        assert_eq!(
            parse_natural_date("20/06", base(), None).unwrap().date,
            date(2026, 6, 20)
        );
        assert_eq!(
            parse_natural_date("06/25", base(), None).unwrap().date,
            date(2026, 6, 25)
        );
        assert!(matches!(
            parse_natural_date("13/14", base(), None),
            Err(NaturalDateError::Invalid(_))
        ));
    }

    #[test]
    fn reminder_times_parse_with_the_date() {
        let locale = Some(Locale::EnUs);
        let parsed = parse_natural_date("tomorrow at 9:30am", base(), locale).unwrap();
        assert_eq!(parsed.date, date(2026, 6, 11));
        assert_eq!(parsed.time, NaiveTime::from_hms_opt(9, 30, 0));
        let parsed = parse_natural_date("21:00", base(), locale).unwrap();
        assert_eq!(parsed.date, date(2026, 6, 10));
        assert_eq!(parsed.time, NaiveTime::from_hms_opt(21, 0, 0));
        let parsed = parse_natural_date("tonight", base(), locale).unwrap();
        assert_eq!(parsed.date, date(2026, 6, 10));
        assert_eq!(parsed.time, NaiveTime::from_hms_opt(19, 0, 0));
        let parsed = parse_natural_date("friday 7pm", base(), locale).unwrap();
        assert_eq!(parsed.date, date(2026, 6, 12));
        assert_eq!(parsed.time, NaiveTime::from_hms_opt(19, 0, 0));
    }

    #[test]
    fn empty_unknown_and_invalid_expressions_are_recoverable() {
        let locale = Some(Locale::EnUs);
        assert_eq!(
            parse_natural_date("   ", base(), locale),
            Err(NaturalDateError::Empty)
        );
        assert!(matches!(
            parse_natural_date("whenever", base(), locale),
            Err(NaturalDateError::Unknown(_))
        ));
        assert!(matches!(
            parse_natural_date("february 30", base(), locale),
            Err(NaturalDateError::Invalid(_))
        ));
    }
}
