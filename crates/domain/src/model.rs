//! Canonical task/context model.
//!
//! This module is the single definition of to-dos, projects, areas, headings, tags,
//! checklists, and their temporal concepts. It has no dependency on UI, database, or provider
//! frameworks. Built-in lists (Inbox, Today, Upcoming, Anytime, Someday, Logbook) are
//! projections over these entities, never storage folders.

use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::ids::{ChecklistItemId, EntityId};
use crate::time::LocalDate;

/// Maximum stored title length.
pub const MAX_TITLE_LEN: usize = 1024;

/// Maximum stored notes length.
pub const MAX_NOTES_LEN: usize = 100_000;

/// Kind of entity stored in the canonical entity table.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    /// A to-do item.
    Todo,
    /// A project grouping to-dos and headings.
    Project,
    /// An ongoing area of responsibility.
    Area,
    /// A project heading grouping to-dos inside one project.
    Heading,
    /// A tag.
    Tag,
}

impl EntityKind {
    /// Stable string representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Todo => "todo",
            Self::Project => "project",
            Self::Area => "area",
            Self::Heading => "heading",
            Self::Tag => "tag",
        }
    }
}

/// Lifecycle status shared by to-dos and projects.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Active and eligible for projections.
    Open,
    /// Completed and eligible for the Logbook.
    Completed,
    /// Canceled and eligible for the Logbook.
    Canceled,
    /// Soft-deleted and eligible for Trash.
    Deleted,
}

impl Status {
    /// Whether the status is terminal (completed or canceled).
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Canceled)
    }

    /// Whether the entity is still actionable.
    #[must_use]
    pub const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }

    /// Stable string representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Completed => "completed",
            Self::Canceled => "canceled",
            Self::Deleted => "deleted",
        }
    }
}

/// Identity and bookkeeping shared by every entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityMeta {
    /// Canonical identity.
    pub id: EntityId,
    /// Creation instant.
    pub created_at: DateTime<Utc>,
    /// Last update instant.
    pub updated_at: DateTime<Utc>,
    /// Optimistic version, incremented on every persisted change.
    pub version: i64,
    /// Soft-deletion instant.
    pub deleted_at: Option<DateTime<Utc>>,
}

impl EntityMeta {
    /// Creates metadata for a new entity at `now`.
    #[must_use]
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            id: EntityId::new(),
            created_at: now,
            updated_at: now,
            version: 1,
            deleted_at: None,
        }
    }

    /// Marks the entity updated at `now`, incrementing the version.
    pub fn touch(&mut self, now: DateTime<Utc>) {
        self.updated_at = now;
        self.version += 1;
    }
}

/// Title validation failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TitleError {
    /// The title exceeds [`MAX_TITLE_LEN`].
    #[error("title is too long")]
    TooLong,
    /// The title contains a control character.
    #[error("title contains control characters")]
    ControlCharacters,
}

/// A validated, trimmed single-line title.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Title(String);

impl Title {
    /// Validates and trims a title. Empty titles are allowed (the UI shows a placeholder).
    ///
    /// # Errors
    ///
    /// Returns [`TitleError`] for over-long titles or embedded control characters.
    pub fn parse(input: &str) -> Result<Self, TitleError> {
        let trimmed = input.trim();
        if trimmed.len() > MAX_TITLE_LEN {
            return Err(TitleError::TooLong);
        }
        if trimmed.chars().any(|c| c.is_control()) {
            return Err(TitleError::ControlCharacters);
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the title text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether the title is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for Title {
    type Err = TitleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// Schedule state of an entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ScheduleState {
    /// Actionable now with no start date.
    Anytime,
    /// Intentionally deferred and excluded from actionable projections.
    Someday,
    /// Scheduled to become actionable on a local date.
    On(LocalDate),
}

/// Schedule errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScheduleError {
    /// This Evening requires a date-based schedule.
    #[error("This Evening requires a scheduled date")]
    EveningRequiresDate,
}

/// Start/schedule state plus the This Evening presentation flag.
///
/// This Evening is a Today presentation state, not a separate schedule: the date remains
/// canonical and the flag only decides which Today section renders the item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    state: ScheduleState,
    evening: bool,
}

impl Schedule {
    /// Actionable with no start date.
    #[must_use]
    pub const fn anytime() -> Self {
        Self {
            state: ScheduleState::Anytime,
            evening: false,
        }
    }

    /// Deferred to Someday.
    #[must_use]
    pub const fn someday() -> Self {
        Self {
            state: ScheduleState::Someday,
            evening: false,
        }
    }

    /// Scheduled for a local date.
    #[must_use]
    pub const fn on(date: LocalDate) -> Self {
        Self {
            state: ScheduleState::On(date),
            evening: false,
        }
    }

    /// Scheduled for a local date in the This Evening section.
    #[must_use]
    pub const fn this_evening(date: LocalDate) -> Self {
        Self {
            state: ScheduleState::On(date),
            evening: true,
        }
    }

    /// Returns the schedule state.
    #[must_use]
    pub const fn state(&self) -> ScheduleState {
        self.state
    }

    /// Whether the This Evening flag is set.
    #[must_use]
    pub const fn is_evening(&self) -> bool {
        self.evening
    }

    /// Returns the scheduled date, when the state is date-based.
    #[must_use]
    pub const fn date(&self) -> Option<LocalDate> {
        match self.state {
            ScheduleState::On(date) => Some(date),
            _ => None,
        }
    }

    /// Replaces the schedule state.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError::EveningRequiresDate`] when `evening` is requested without a
    /// date-based state.
    pub fn with_state(state: ScheduleState, evening: bool) -> Result<Self, ScheduleError> {
        if evening && !matches!(state, ScheduleState::On(_)) {
            return Err(ScheduleError::EveningRequiresDate);
        }
        Ok(Self { state, evening })
    }

    /// Returns the schedule with the evening flag set or cleared.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError::EveningRequiresDate`] when setting the flag on a
    /// non-date schedule.
    pub fn with_evening(self, evening: bool) -> Result<Self, ScheduleError> {
        Self::with_state(self.state, evening)
    }

    /// Whether the entity is actionable on `today`.
    #[must_use]
    pub fn is_actionable(&self, today: LocalDate) -> bool {
        match self.state {
            ScheduleState::Anytime => true,
            ScheduleState::Someday => false,
            ScheduleState::On(date) => date <= today,
        }
    }

    /// Whether the entity is scheduled strictly after `today`.
    #[must_use]
    pub fn is_future(&self, today: LocalDate) -> bool {
        match self.state {
            ScheduleState::On(date) => date > today,
            _ => false,
        }
    }

    /// Whether the entity is deferred to Someday.
    #[must_use]
    pub const fn is_someday(&self) -> bool {
        matches!(self.state, ScheduleState::Someday)
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::anytime()
    }
}

/// Deadline state relative to a local date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeadlineState {
    /// Due after today.
    Future,
    /// Due today.
    DueToday,
    /// Due before today.
    Overdue,
}

/// A deadline: an explicit local date, independent from schedule and reminder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Deadline {
    /// Due date.
    pub date: LocalDate,
}

impl Deadline {
    /// Creates a deadline on a local date.
    #[must_use]
    pub const fn new(date: LocalDate) -> Self {
        Self { date }
    }

    /// Computes the deadline state relative to `today`.
    #[must_use]
    pub fn state(self, today: LocalDate) -> DeadlineState {
        if self.date > today {
            DeadlineState::Future
        } else if self.date == today {
            DeadlineState::DueToday
        } else {
            DeadlineState::Overdue
        }
    }
}

/// How terminal items enter the Logbook.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoggingPolicy {
    /// Completed/canceled items are logged automatically.
    #[default]
    Automatic,
    /// Terminal items stay pending until the user logs them explicitly.
    Manual,
}

impl LoggingPolicy {
    /// Stable storage label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }

    /// Parses the storage label, defaulting to automatic for unknown values.
    #[must_use]
    pub fn parse(value: &str) -> Self {
        match value {
            "manual" => Self::Manual,
            _ => Self::Automatic,
        }
    }
}

/// Delivery state of a reminder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReminderState {
    /// Waiting to fire.
    Scheduled,
    /// Delivered to the user.
    Delivered,
    /// Snoozed until a later instant.
    Snoozed,
    /// Canceled by the user.
    Canceled,
}

/// A reminder instant with its timezone context, independent from schedule and deadline.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Reminder {
    /// Instant the reminder fires.
    pub at: DateTime<Utc>,
    /// IANA timezone used to present and snooze the reminder.
    pub timezone: String,
    /// Delivery state.
    pub state: ReminderState,
}

impl ReminderState {
    /// Stable string representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scheduled => "scheduled",
            Self::Delivered => "delivered",
            Self::Snoozed => "snoozed",
            Self::Canceled => "canceled",
        }
    }
}

impl Reminder {
    /// Creates a scheduled reminder.
    #[must_use]
    pub fn scheduled(at: DateTime<Utc>, timezone: impl Into<String>) -> Self {
        Self {
            at,
            timezone: timezone.into(),
            state: ReminderState::Scheduled,
        }
    }

    /// Whether the reminder is due at `now` and still waiting to fire.
    #[must_use]
    pub fn is_due(&self, now: DateTime<Utc>) -> bool {
        matches!(
            self.state,
            ReminderState::Scheduled | ReminderState::Snoozed
        ) && self.at <= now
    }

    /// Snoozes a delivered reminder to a later instant, returning whether anything changed.
    ///
    /// Repeating the same snooze target is a no-op, so repeated clicks cannot schedule duplicate
    /// deliveries.
    pub fn snooze(&mut self, until: DateTime<Utc>) -> bool {
        if !matches!(
            self.state,
            ReminderState::Delivered | ReminderState::Snoozed
        ) {
            return false;
        }
        if self.state == ReminderState::Snoozed && self.at == until {
            return false;
        }
        self.at = until;
        self.state = ReminderState::Snoozed;
        true
    }

    /// Marks the reminder delivered, returning whether the state changed.
    ///
    /// Repeating this after a restart or retry is a no-op, so delivery cannot storm.
    pub fn mark_delivered(&mut self) -> bool {
        if matches!(
            self.state,
            ReminderState::Scheduled | ReminderState::Snoozed
        ) {
            self.state = ReminderState::Delivered;
            true
        } else {
            false
        }
    }
}

/// The single contextual parent of a to-do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ContextRef {
    /// Not yet filed: the Inbox projection owns it.
    Inbox,
    /// Filed directly under an area.
    Area(EntityId),
    /// Filed directly under a project.
    Project(EntityId),
    /// Filed under a heading inside a project.
    Heading(EntityId),
}

impl ContextRef {
    /// Returns the parent entity id, when the context is not the Inbox.
    #[must_use]
    pub const fn parent_id(self) -> Option<EntityId> {
        match self {
            Self::Inbox => None,
            Self::Area(id) | Self::Project(id) | Self::Heading(id) => Some(id),
        }
    }

    /// Whether the context is the Inbox.
    #[must_use]
    pub const fn is_inbox(self) -> bool {
        matches!(self, Self::Inbox)
    }
}

/// A checklist item embedded in a to-do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecklistItem {
    /// Identity of the item.
    pub id: ChecklistItemId,
    /// Item text.
    pub title: Title,
    /// Completion state.
    pub completed: bool,
}

impl ChecklistItem {
    /// Creates an open checklist item.
    #[must_use]
    pub fn new(title: Title) -> Self {
        Self {
            id: ChecklistItemId::new(),
            title,
            completed: false,
        }
    }
}

/// Normalizes pasted multi-line checklist content into item titles.
///
/// Lines are trimmed, empty lines dropped, and common bullet/checkbox prefixes removed while
/// preserving order.
#[must_use]
pub fn parse_checklist_lines(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::trim)
        .map(strip_checklist_prefix)
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn strip_checklist_prefix(line: &str) -> &str {
    let line = line
        .strip_prefix("- [ ] ")
        .or_else(|| line.strip_prefix("- [x] "))
        .or_else(|| line.strip_prefix("- [X] "))
        .unwrap_or(line);
    line.strip_prefix("- ")
        .or_else(|| line.strip_prefix("* "))
        .or_else(|| line.strip_prefix("\u{2022} "))
        .unwrap_or(line)
}

/// A to-do: the only entity that appears in built-in list projections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    /// Identity and bookkeeping.
    pub meta: EntityMeta,
    /// Title.
    pub title: Title,
    /// Notes (plain text; rendering rules belong to the notes/Markdown capabilities).
    pub notes: String,
    /// Lifecycle status.
    pub status: Status,
    /// Start/schedule state.
    pub schedule: Schedule,
    /// Optional deadline.
    pub deadline: Option<Deadline>,
    /// Optional reminder.
    pub reminder: Option<Reminder>,
    /// Single contextual parent.
    pub context: ContextRef,
    /// Assigned tags.
    pub tags: BTreeSet<EntityId>,
    /// Embedded checklist, ordered by vector position.
    pub checklist: Vec<ChecklistItem>,
    /// Completion instant.
    pub completed_at: Option<DateTime<Utc>>,
    /// Cancellation instant.
    pub canceled_at: Option<DateTime<Utc>>,
    /// Logbook logging instant.
    pub logged_at: Option<DateTime<Utc>>,
}

impl Todo {
    /// Creates an open to-do in the Inbox scheduled Anytime.
    #[must_use]
    pub fn new(title: Title, now: DateTime<Utc>) -> Self {
        Self {
            meta: EntityMeta::new(now),
            title,
            notes: String::new(),
            status: Status::Open,
            schedule: Schedule::anytime(),
            deadline: None,
            reminder: None,
            context: ContextRef::Inbox,
            tags: BTreeSet::new(),
            checklist: Vec::new(),
            completed_at: None,
            canceled_at: None,
            logged_at: None,
        }
    }

    /// Returns the canonical identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.meta.id
    }

    /// Whether the to-do is open.
    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.status.is_open()
    }

    /// Completes the to-do.
    pub fn complete(&mut self, now: DateTime<Utc>) {
        if self.status != Status::Deleted {
            self.status = Status::Completed;
            self.completed_at = Some(now);
            self.canceled_at = None;
            self.meta.touch(now);
        }
    }

    /// Cancels the to-do.
    pub fn cancel(&mut self, now: DateTime<Utc>) {
        if self.status != Status::Deleted {
            self.status = Status::Canceled;
            self.canceled_at = Some(now);
            self.completed_at = None;
            self.meta.touch(now);
        }
    }

    /// Reopens a completed or canceled to-do without inventing dates.
    pub fn reopen(&mut self, now: DateTime<Utc>) {
        if self.status.is_terminal() {
            self.status = Status::Open;
            self.completed_at = None;
            self.canceled_at = None;
            self.meta.touch(now);
        }
    }

    /// Moves the to-do to a new contextual parent.
    pub fn move_to(&mut self, context: ContextRef, now: DateTime<Utc>) {
        if self.context != context {
            self.context = context;
            self.meta.touch(now);
        }
    }

    /// Replaces the schedule.
    pub fn set_schedule(&mut self, schedule: Schedule, now: DateTime<Utc>) {
        if self.schedule != schedule {
            self.schedule = schedule;
            self.meta.touch(now);
        }
    }

    /// Replaces the deadline.
    pub fn set_deadline(&mut self, deadline: Option<Deadline>, now: DateTime<Utc>) {
        if self.deadline != deadline {
            self.deadline = deadline;
            self.meta.touch(now);
        }
    }

    /// Replaces the reminder.
    pub fn set_reminder(&mut self, reminder: Option<Reminder>, now: DateTime<Utc>) {
        if self.reminder != reminder {
            self.reminder = reminder;
            self.meta.touch(now);
        }
    }

    /// Snoozes the attached delivered reminder to `until`, returning whether anything changed.
    pub fn snooze_reminder(&mut self, until: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        let Some(reminder) = self.reminder.as_mut() else {
            return false;
        };
        if reminder.snooze(until) {
            self.meta.touch(now);
            true
        } else {
            false
        }
    }

    /// Marks the attached reminder delivered, returning whether anything changed.
    ///
    /// Idempotent: a to-do without a scheduled reminder is left untouched.
    pub fn mark_reminder_delivered(&mut self, now: DateTime<Utc>) -> bool {
        let Some(reminder) = self.reminder.as_mut() else {
            return false;
        };
        if reminder.mark_delivered() {
            self.meta.touch(now);
            true
        } else {
            false
        }
    }

    /// Adds a checklist item, returning its identity.
    pub fn add_checklist_item(&mut self, title: Title, now: DateTime<Utc>) -> ChecklistItemId {
        let item = ChecklistItem::new(title);
        let id = item.id;
        self.checklist.push(item);
        self.meta.touch(now);
        id
    }
}

/// Completion policy for a project with open children.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectCompletionPolicy {
    /// Refuse to complete while open children remain.
    RejectWhileOpen,
    /// Complete the project and cascade completion to open children.
    CompleteOpenChildren,
}

/// Decision produced by [`ProjectCompletionPolicy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionDecision {
    /// Completion may proceed.
    Allowed,
    /// Completion is refused because open children remain.
    RejectedOpenChildren,
    /// Completion may proceed and must cascade to open children.
    CascadeToChildren,
}

/// Computes the completion decision for a project.
#[must_use]
pub fn completion_decision(
    open_children: usize,
    policy: ProjectCompletionPolicy,
) -> CompletionDecision {
    if open_children == 0 {
        return CompletionDecision::Allowed;
    }
    match policy {
        ProjectCompletionPolicy::RejectWhileOpen => CompletionDecision::RejectedOpenChildren,
        ProjectCompletionPolicy::CompleteOpenChildren => CompletionDecision::CascadeToChildren,
    }
}

/// A project grouping to-dos and headings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    /// Identity and bookkeeping.
    pub meta: EntityMeta,
    /// Title.
    pub title: Title,
    /// Notes.
    pub notes: String,
    /// Lifecycle status.
    pub status: Status,
    /// Start/schedule state; projects participate in projections without copying values to children.
    pub schedule: Schedule,
    /// Optional deadline.
    pub deadline: Option<Deadline>,
    /// Optional reminder.
    pub reminder: Option<Reminder>,
    /// Owning area, when any.
    pub area: Option<EntityId>,
    /// Tags.
    pub tags: BTreeSet<EntityId>,
    /// Completion instant.
    pub completed_at: Option<DateTime<Utc>>,
    /// Cancellation instant.
    pub canceled_at: Option<DateTime<Utc>>,
    /// Logbook logging instant.
    pub logged_at: Option<DateTime<Utc>>,
}

impl Project {
    /// Creates an open project scheduled Anytime.
    #[must_use]
    pub fn new(title: Title, now: DateTime<Utc>) -> Self {
        Self {
            meta: EntityMeta::new(now),
            title,
            notes: String::new(),
            status: Status::Open,
            schedule: Schedule::anytime(),
            deadline: None,
            reminder: None,
            area: None,
            tags: BTreeSet::new(),
            completed_at: None,
            canceled_at: None,
            logged_at: None,
        }
    }

    /// Returns the canonical identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.meta.id
    }

    /// Completes the project when the policy allows it.
    ///
    /// # Errors
    ///
    /// Returns [`CompletionDecision::RejectedOpenChildren`] as an error when the policy
    /// refuses completion with open children.
    pub fn complete(
        &mut self,
        open_children: usize,
        policy: ProjectCompletionPolicy,
        now: DateTime<Utc>,
    ) -> Result<CompletionDecision, CompletionDecision> {
        match completion_decision(open_children, policy) {
            CompletionDecision::RejectedOpenChildren => {
                Err(CompletionDecision::RejectedOpenChildren)
            }
            decision => {
                self.status = Status::Completed;
                self.completed_at = Some(now);
                self.canceled_at = None;
                self.meta.touch(now);
                Ok(decision)
            }
        }
    }

    /// Reopens the project.
    pub fn reopen(&mut self, now: DateTime<Utc>) {
        if self.status.is_terminal() {
            self.status = Status::Open;
            self.completed_at = None;
            self.canceled_at = None;
            self.meta.touch(now);
        }
    }
}

/// An ongoing area of responsibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    /// Identity and bookkeeping.
    pub meta: EntityMeta,
    /// Title.
    pub title: Title,
    /// Tags.
    pub tags: BTreeSet<EntityId>,
}

impl Area {
    /// Creates an area.
    #[must_use]
    pub fn new(title: Title, now: DateTime<Utc>) -> Self {
        Self {
            meta: EntityMeta::new(now),
            title,
            tags: BTreeSet::new(),
        }
    }

    /// Returns the canonical identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.meta.id
    }
}

/// A heading inside a project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    /// Identity and bookkeeping.
    pub meta: EntityMeta,
    /// Title.
    pub title: Title,
    /// Owning project.
    pub project: EntityId,
    /// Whether the heading is archived (hidden but preserved).
    pub archived: bool,
}

impl Heading {
    /// Returns the canonical identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.meta.id
    }

    /// Creates a heading inside a project.
    #[must_use]
    pub fn new(title: Title, project: EntityId, now: DateTime<Utc>) -> Self {
        Self {
            meta: EntityMeta::new(now),
            title,
            project,
            archived: false,
        }
    }
}

/// A tag, optionally nested under another tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    /// Identity and bookkeeping.
    pub meta: EntityMeta,
    /// Title.
    pub title: Title,
    /// Parent tag, when nested.
    pub parent: Option<EntityId>,
}

impl Tag {
    /// Returns the canonical identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.meta.id
    }

    /// Creates a tag.
    #[must_use]
    pub fn new(title: Title, parent: Option<EntityId>, now: DateTime<Utc>) -> Self {
        Self {
            meta: EntityMeta::new(now),
            title,
            parent,
        }
    }
}

#[cfg(test)]
mod reminder_delivery_tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn due_and_delivery_are_idempotent() {
        let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
        let mut reminder = Reminder::scheduled(now - chrono::Duration::minutes(5), "UTC");
        assert!(reminder.is_due(now));
        assert!(reminder.mark_delivered());
        assert!(!reminder.is_due(now));
        assert!(!reminder.mark_delivered());
        assert_eq!(reminder.state, ReminderState::Delivered);

        let future = Reminder::scheduled(now + chrono::Duration::minutes(5), "UTC");
        assert!(!future.is_due(now));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 1, 12, 0, 0).unwrap()
    }

    fn date(year: i32, month: u32, day: u32) -> LocalDate {
        LocalDate::from_ymd(year, month, day).expect("date")
    }

    fn todo(title: &str) -> Todo {
        Todo::new(Title::parse(title).expect("title"), now())
    }

    #[test]
    fn new_todo_is_inbox_anytime_open_with_canonical_identity() {
        let todo = todo("Write the report");
        assert!(todo.context.is_inbox());
        assert_eq!(todo.schedule, Schedule::anytime());
        assert_eq!(todo.status, Status::Open);
        assert!(todo.deadline.is_none());
        assert!(todo.reminder.is_none());
        assert_eq!(todo.meta.version, 1);
        assert_eq!(todo.id(), todo.meta.id);
    }

    #[test]
    fn temporal_concepts_are_independent() {
        let mut item = todo("Independent fields");
        let schedule = Schedule::on(date(2026, 6, 10));
        let deadline = Deadline::new(date(2026, 6, 12));
        let reminder = Reminder::scheduled(now(), "Europe/Bucharest");

        item.set_schedule(schedule, now());
        item.set_deadline(Some(deadline), now());
        item.set_reminder(Some(reminder.clone()), now());

        item.set_deadline(None, now());
        assert_eq!(item.schedule, schedule, "schedule must be untouched");
        assert_eq!(item.reminder, Some(reminder), "reminder must be untouched");

        item.set_reminder(None, now());
        item.set_schedule(Schedule::someday(), now());
        assert!(item.deadline.is_none());
    }

    #[test]
    fn evening_requires_a_date_and_clears_with_state() {
        let evening = Schedule::this_evening(date(2026, 6, 1));
        assert!(evening.is_evening());
        assert_eq!(evening.date(), Some(date(2026, 6, 1)));

        assert_eq!(
            Schedule::with_state(ScheduleState::Anytime, true),
            Err(ScheduleError::EveningRequiresDate)
        );

        let cleared = Schedule::with_state(ScheduleState::Someday, false).expect("valid");
        assert!(!cleared.is_evening());
        assert!(cleared.is_someday());
    }

    #[test]
    fn schedule_actionability_uses_local_today() {
        let today = date(2026, 6, 1);
        assert!(Schedule::anytime().is_actionable(today));
        assert!(!Schedule::someday().is_actionable(today));
        assert!(Schedule::on(date(2026, 5, 31)).is_actionable(today));
        assert!(Schedule::on(today).is_actionable(today));
        assert!(!Schedule::on(date(2026, 6, 2)).is_actionable(today));
        assert!(Schedule::on(date(2026, 6, 2)).is_future(today));
    }

    #[test]
    fn deadline_states_are_derived_from_local_date() {
        let today = date(2026, 6, 1);
        assert_eq!(
            Deadline::new(date(2026, 6, 2)).state(today),
            DeadlineState::Future
        );
        assert_eq!(Deadline::new(today).state(today), DeadlineState::DueToday);
        assert_eq!(
            Deadline::new(date(2026, 5, 31)).state(today),
            DeadlineState::Overdue
        );
    }

    #[test]
    fn moving_preserves_identity_and_unrelated_fields() {
        let mut item = todo("Move me");
        item.set_schedule(Schedule::on(date(2026, 6, 5)), now());
        item.set_deadline(Some(Deadline::new(date(2026, 6, 6))), now());
        let id = item.id();
        let schedule = item.schedule;
        let deadline = item.deadline;

        let project = EntityId::new();
        item.move_to(ContextRef::Project(project), now());
        assert_eq!(item.context, ContextRef::Project(project));
        assert_eq!(item.id(), id);
        assert_eq!(item.schedule, schedule);
        assert_eq!(item.deadline, deadline);

        item.move_to(ContextRef::Heading(EntityId::new()), now());
        assert!(matches!(item.context, ContextRef::Heading(_)));
    }

    #[test]
    fn completion_and_reopen_timestamps_are_consistent() {
        let mut item = todo("Finish me");
        item.complete(now());
        assert_eq!(item.status, Status::Completed);
        assert_eq!(item.completed_at, Some(now()));
        assert!(item.canceled_at.is_none());

        item.reopen(now());
        assert_eq!(item.status, Status::Open);
        assert!(item.completed_at.is_none());

        item.cancel(now());
        assert_eq!(item.status, Status::Canceled);
        assert!(item.completed_at.is_none());
        assert_eq!(item.canceled_at, Some(now()));
    }

    #[test]
    fn project_completion_policy_is_explicit() {
        assert_eq!(
            completion_decision(0, ProjectCompletionPolicy::RejectWhileOpen),
            CompletionDecision::Allowed
        );
        assert_eq!(
            completion_decision(3, ProjectCompletionPolicy::RejectWhileOpen),
            CompletionDecision::RejectedOpenChildren
        );
        assert_eq!(
            completion_decision(3, ProjectCompletionPolicy::CompleteOpenChildren),
            CompletionDecision::CascadeToChildren
        );

        let mut project = Project::new(Title::parse("Launch").expect("title"), now());
        let rejected = project.complete(2, ProjectCompletionPolicy::RejectWhileOpen, now());
        assert_eq!(rejected, Err(CompletionDecision::RejectedOpenChildren));
        assert_eq!(project.status, Status::Open);

        let decision = project
            .complete(2, ProjectCompletionPolicy::CompleteOpenChildren, now())
            .expect("allowed");
        assert_eq!(decision, CompletionDecision::CascadeToChildren);
        assert_eq!(project.status, Status::Completed);
    }

    #[test]
    fn checklist_completion_does_not_touch_temporal_fields() {
        let mut item = todo("Trip packing");
        item.set_schedule(Schedule::on(date(2026, 6, 20)), now());
        item.set_deadline(Some(Deadline::new(date(2026, 6, 19))), now());
        item.add_checklist_item(Title::parse("Passport").expect("title"), now());
        item.add_checklist_item(Title::parse("Charger").expect("title"), now());

        let schedule = item.schedule;
        let deadline = item.deadline;
        let first_id = item.checklist[0].id;
        item.checklist[0].completed = true;

        assert_eq!(item.checklist.len(), 2);
        assert!(item.checklist[0].completed);
        assert!(!item.checklist[1].completed);
        assert_eq!(item.checklist[0].id, first_id);
        assert_eq!(item.schedule, schedule);
        assert_eq!(item.deadline, deadline);
    }

    #[test]
    fn title_validation_trims_and_rejects_control_characters() {
        assert_eq!(Title::parse("  hello  ").expect("title").as_str(), "hello");
        assert!(Title::parse("").expect("empty allowed").is_empty());
        assert_eq!(
            Title::parse("bad\u{0007}title"),
            Err(TitleError::ControlCharacters)
        );
        let long = "x".repeat(MAX_TITLE_LEN + 1);
        assert_eq!(Title::parse(&long), Err(TitleError::TooLong));
    }

    #[test]
    fn tags_are_a_set_of_canonical_entities() {
        let mut item = todo("Tagged");
        let work = EntityId::new();
        let home = EntityId::new();
        item.tags.insert(work);
        item.tags.insert(home);
        item.tags.insert(work);
        assert_eq!(item.tags.len(), 2);
    }

    #[test]
    fn area_and_heading_contexts_are_distinct() {
        let area = EntityId::new();
        let heading = EntityId::new();
        assert_eq!(ContextRef::Area(area).parent_id(), Some(area));
        assert_eq!(ContextRef::Heading(heading).parent_id(), Some(heading));
        assert_eq!(ContextRef::Inbox.parent_id(), None);
        assert!(!ContextRef::Area(area).is_inbox());
    }
}
