//! Local calendar dates.
//!
//! Date-only concepts (schedule dates, deadlines) are real calendar dates. They are never
//! represented as UTC-midnight instants, and "today" is derived from an instant using the
//! user's UTC offset.

use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// Date construction and parsing failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TimeError {
    /// The year/month/day combination is not a valid calendar date.
    #[error("invalid calendar date")]
    InvalidDate,
    /// The text is not an ISO `YYYY-MM-DD` date.
    #[error("date must use the YYYY-MM-DD format")]
    InvalidFormat,
    /// The UTC offset is outside the supported range.
    #[error("UTC offset in minutes is out of range")]
    InvalidOffset,
}

/// A local calendar date without a time component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalDate(NaiveDate);

impl LocalDate {
    /// Wraps an existing `chrono` naive date.
    #[must_use]
    pub const fn from_naive(date: NaiveDate) -> Self {
        Self(date)
    }

    /// Constructs a date from year, month, and day.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::InvalidDate`] for impossible dates such as February 30.
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, TimeError> {
        NaiveDate::from_ymd_opt(year, month, day)
            .map(Self)
            .ok_or(TimeError::InvalidDate)
    }

    /// Derives the user's local date from an instant and a UTC offset in minutes.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::InvalidOffset`] when the offset is outside ±24 hours.
    pub fn today_from_utc(now: DateTime<Utc>, offset_minutes: i32) -> Result<Self, TimeError> {
        let seconds = offset_minutes
            .checked_mul(60)
            .ok_or(TimeError::InvalidOffset)?;
        let offset = FixedOffset::east_opt(seconds).ok_or(TimeError::InvalidOffset)?;
        Ok(Self(now.with_timezone(&offset).date_naive()))
    }

    /// Derives the local date from an instant in a named IANA timezone.
    ///
    /// Unknown timezone names fall back to UTC so date derivation never panics.
    #[must_use]
    pub fn today_in_timezone(now: DateTime<Utc>, timezone: &str) -> Self {
        if let Ok(zone) = timezone.parse::<chrono_tz::Tz>() {
            Self(now.with_timezone(&zone).date_naive())
        } else {
            Self(now.date_naive())
        }
    }

    /// Returns the underlying `chrono` naive date.
    #[must_use]
    pub const fn as_naive(&self) -> NaiveDate {
        self.0
    }

    /// Returns the following day.
    #[must_use]
    pub fn next_day(self) -> Self {
        Self(self.0.succ_opt().unwrap_or(self.0))
    }

    /// Returns the previous day.
    #[must_use]
    pub fn previous_day(self) -> Self {
        Self(self.0.pred_opt().unwrap_or(self.0))
    }

    /// Returns the signed number of days from `self` to `other`.
    #[must_use]
    pub fn days_until(self, other: Self) -> i64 {
        other.0.signed_duration_since(self.0).num_days()
    }

    /// Returns the ISO `YYYY-MM-DD` representation.
    #[must_use]
    pub fn to_iso(self) -> String {
        self.0.format("%Y-%m-%d").to_string()
    }

    /// Returns the year.
    #[must_use]
    pub fn year(self) -> i32 {
        self.0.year()
    }

    /// Returns the month (1–12).
    #[must_use]
    pub fn month(self) -> u32 {
        self.0.month()
    }

    /// Returns the day of month.
    #[must_use]
    pub fn day(self) -> u32 {
        self.0.day()
    }
}

impl fmt::Display for LocalDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_iso())
    }
}

impl FromStr for LocalDate {
    type Err = TimeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map(Self)
            .map_err(|_| TimeError::InvalidFormat)
    }
}

impl Serialize for LocalDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_iso())
    }
}

impl<'de> Deserialize<'de> for LocalDate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

/// Converts an instant to the local date in the given IANA timezone.
///
/// This helper exists so callers do not reach for UTC dates by accident.
#[must_use]
pub fn local_today(now: DateTime<Utc>, timezone: &str) -> LocalDate {
    LocalDate::today_in_timezone(now, timezone)
}

/// Converts an instant to a local date at a fixed UTC offset.
///
/// # Errors
///
/// Returns [`TimeError::InvalidOffset`] for out-of-range offsets.
pub fn local_today_at_offset(
    now: DateTime<Utc>,
    offset_minutes: i32,
) -> Result<LocalDate, TimeError> {
    LocalDate::today_from_utc(now, offset_minutes)
}

/// Utility used by tests and adapters to build an instant from a date at UTC midnight.
///
/// Only call this for diagnostics or fixtures; date-only domain values must not use it.
#[must_use]
pub fn utc_midnight(date: LocalDate) -> DateTime<Utc> {
    Utc.from_utc_datetime(&date.as_naive().and_hms_opt(0, 0, 0).unwrap_or_else(|| {
        date.as_naive()
            .and_hms_opt(0, 0, 0)
            .expect("midnight exists")
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn derives_local_date_from_offset() {
        let now = Utc.with_ymd_and_hms(2026, 3, 10, 1, 30, 0).unwrap();
        let utc = LocalDate::today_from_utc(now, 0).expect("utc");
        assert_eq!(utc.to_iso(), "2026-03-10");
        let behind = LocalDate::today_from_utc(now, -180).expect("utc-3");
        assert_eq!(behind.to_iso(), "2026-03-09");
        let ahead = LocalDate::today_from_utc(now, 600).expect("utc+10");
        assert_eq!(ahead.to_iso(), "2026-03-10");
    }

    #[test]
    fn named_timezone_derivation_handles_dst() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 23, 30, 0).unwrap();
        assert_eq!(
            LocalDate::today_in_timezone(now, "Europe/Bucharest").to_iso(),
            "2026-01-02"
        );
        assert_eq!(
            LocalDate::today_in_timezone(now, "America/New_York").to_iso(),
            "2026-01-01"
        );
        assert_eq!(
            LocalDate::today_in_timezone(now, "not/a-zone").to_iso(),
            "2026-01-01"
        );
    }

    #[test]
    fn arithmetic_and_difference() {
        let date = LocalDate::from_ymd(2026, 2, 28).expect("date");
        assert_eq!(date.next_day().to_iso(), "2026-03-01");
        assert_eq!(date.previous_day().to_iso(), "2026-02-27");
        assert_eq!(date.days_until(date.next_day()), 1);
        assert_eq!(date.days_until(date.previous_day()), -1);
    }

    #[test]
    fn parsing_and_serialization_round_trip() {
        let date: LocalDate = "2026-12-31".parse().expect("parse");
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 31);
        assert_eq!(date.to_string(), "2026-12-31");
        let json = serde_json::to_string(&date).expect("serialize");
        assert_eq!(json, "\"2026-12-31\"");
        let decoded: LocalDate = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, date);
    }

    #[test]
    fn rejects_invalid_dates_and_formats() {
        assert_eq!(
            LocalDate::from_ymd(2026, 2, 30),
            Err(TimeError::InvalidDate)
        );
        assert_eq!(
            "2026/12/31".parse::<LocalDate>(),
            Err(TimeError::InvalidFormat)
        );
        assert_eq!(
            LocalDate::today_from_utc(Utc::now(), 10_000),
            Err(TimeError::InvalidOffset)
        );
    }

    #[test]
    fn utc_midnight_helper_is_diagnostic_only() {
        let date = LocalDate::from_ymd(2026, 6, 1).expect("date");
        let instant = utc_midnight(date);
        assert_eq!(instant.date_naive(), date.as_naive());
    }
}
