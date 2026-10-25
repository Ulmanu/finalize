//! Projection keys and normalized projection payloads for the client workspace.

use chrono::{DateTime, Utc};
use momentum_app::todos::{Progress, TodoSummary};
use momentum_domain::time::LocalDate;
use momentum_domain::EntityId;
use serde::{Deserialize, Serialize};

/// Stable identity for a built-in or entity-backed application projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectionKey {
    /// Unprocessed inbox items.
    Inbox,
    /// Items scheduled for today.
    Today,
    /// Date-grouped scheduled items.
    Upcoming,
    /// Actionable items without a schedule.
    Anytime,
    /// Deferred items.
    Someday,
    /// Completed and canceled history.
    Logbook,
    /// Items due soon or overdue.
    Deadlines,
    /// The next-day projection.
    Tomorrow,
    /// A specific project.
    Project(EntityId),
    /// A specific area.
    Area(EntityId),
}

/// Identity for a section within a projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SectionKey {
    /// The primary task section.
    Main,
    /// The separate Today evening section.
    Evening,
    /// A date section in an agenda projection.
    Date(LocalDate),
    /// Calendar events.
    Calendar,
}

/// Calendar event row included in a projection section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarRow {
    /// Provider event identifier.
    pub id: String,
    /// User-visible event title.
    pub title: String,
    /// Event start instant.
    pub starts_at: DateTime<Utc>,
    /// Event end instant.
    pub ends_at: DateTime<Utc>,
}

/// A normalized row reference; entities are sent once in the snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectionRow {
    /// Reference to a to-do in `ProjectionSnapshot::todos`.
    Todo(EntityId),
    /// A calendar event.
    Calendar(CalendarRow),
    /// Reference to a project held in the client entity store.
    Project(EntityId),
}

/// One ordered section of a projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionSection {
    /// Stable section identity.
    pub key: SectionKey,
    /// Optional user-visible section heading.
    pub title: Option<String>,
    /// Rows in display order.
    pub rows: Vec<ProjectionRow>,
}

/// Server snapshot for one canonical projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionSnapshot {
    /// Projection identity.
    pub key: ProjectionKey,
    /// Unique to-do entities referenced by the sections.
    pub todos: Vec<TodoSummary>,
    /// Ordered display sections.
    pub sections: Vec<ProjectionSection>,
    /// Optional progress summary for project-like views.
    pub progress: Option<Progress>,
}
