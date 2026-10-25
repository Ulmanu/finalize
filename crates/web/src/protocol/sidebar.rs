//! Serializable sidebar snapshot contracts.

use momentum_domain::EntityId;
use serde::{Deserialize, Serialize};

use super::projection::ProjectionKey;

/// One built-in destination and its optional unread/open count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuiltinSidebarItem {
    /// Destination key.
    pub key: ProjectionKey,
    /// Visible label.
    pub title: String,
    /// Optional count displayed beside the destination.
    pub count: Option<usize>,
}

/// A project displayed in the sidebar hierarchy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SidebarProject {
    /// Project identity.
    pub id: EntityId,
    /// Visible project title.
    pub title: String,
    /// Completion ratio, when the project has eligible tasks.
    pub progress: Option<f32>,
}

/// An area and its child projects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SidebarArea {
    /// Area identity.
    pub id: EntityId,
    /// Visible area title.
    pub title: String,
    /// Projects assigned to the area.
    pub projects: Vec<SidebarProject>,
}

/// Account-scoped sidebar data, ready to render without direct database access.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SidebarSnapshot {
    /// Number of open inbox items.
    pub inbox_count: usize,
    /// Built-in destinations in display order.
    pub builtins: Vec<BuiltinSidebarItem>,
    /// Projects that do not belong to an area.
    pub standalone_projects: Vec<SidebarProject>,
    /// Areas and their projects.
    pub areas: Vec<SidebarArea>,
}
