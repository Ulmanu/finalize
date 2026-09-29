//! Central keyboard command registry.
//!
//! Every shortcut is declared once here with the surface it applies to, so components render
//! hints from the registry instead of hard-coding key strings, and browser/OS-reserved bindings
//! carry a documented safe web equivalent.

/// Surface a shortcut applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandSurface {
    /// Available everywhere in the application shell.
    Global,
    /// List pages and task rows.
    List,
    /// Detail/editor pages.
    Detail,
}

impl CommandSurface {
    /// Stable identifier used by markup and tests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::List => "list",
            Self::Detail => "detail",
        }
    }
}

/// Command grouping for the shortcuts reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandCategory {
    /// Creating new items.
    Creation,
    /// Completing, cancelling, deleting.
    Status,
    /// Scheduling and deadlines.
    Scheduling,
    /// Moving around the application.
    Navigation,
    /// Moving, tagging, editing.
    Organization,
    /// Selection management.
    Selection,
    /// Reordering within a list.
    Ordering,
}

impl CommandCategory {
    /// Stable identifier used by markup and tests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Creation => "creation",
            Self::Status => "status",
            Self::Scheduling => "scheduling",
            Self::Navigation => "navigation",
            Self::Organization => "organization",
            Self::Selection => "selection",
            Self::Ordering => "ordering",
        }
    }
}

/// One keyboard command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandDefinition {
    /// Stable action identifier dispatched by the client.
    pub id: &'static str,
    /// Human-readable name.
    pub title: &'static str,
    /// Grouping.
    pub category: CommandCategory,
    /// Primary binding.
    pub keys: &'static str,
    /// Owner of the shortcut when the primary binding is reserved.
    pub reserved_by: Option<&'static str>,
    /// Safe web equivalent used when `reserved_by` is set.
    pub alternative: Option<&'static str>,
    /// Surfaces the command applies to.
    pub surfaces: &'static [CommandSurface],
}

const LIST: &[CommandSurface] = &[CommandSurface::List];
const DETAIL: &[CommandSurface] = &[CommandSurface::Detail];
const GLOBAL: &[CommandSurface] = &[CommandSurface::Global];

const REGISTRY: &[CommandDefinition] = &[
    CommandDefinition {
        id: "new_todo",
        title: "New to-do",
        category: CommandCategory::Creation,
        keys: "n",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "new_todo_above",
        title: "New to-do above",
        category: CommandCategory::Creation,
        keys: "Shift+N",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "complete_todo",
        title: "Complete to-do",
        category: CommandCategory::Status,
        keys: "Enter",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "cancel_todo",
        title: "Cancel to-do",
        category: CommandCategory::Status,
        keys: "x",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "delete_todo",
        title: "Delete to-do",
        category: CommandCategory::Status,
        keys: "Cmd+Backspace",
        reserved_by: Some("macOS text editing"),
        alternative: Some("Shift+Delete"),
        surfaces: LIST,
    },
    CommandDefinition {
        id: "schedule_today",
        title: "Schedule for today",
        category: CommandCategory::Scheduling,
        keys: "t",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "schedule_evening",
        title: "Schedule for this evening",
        category: CommandCategory::Scheduling,
        keys: "e",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "schedule_tomorrow",
        title: "Schedule for tomorrow",
        category: CommandCategory::Scheduling,
        keys: "Shift+T",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "clear_schedule",
        title: "Clear schedule (Anytime)",
        category: CommandCategory::Scheduling,
        keys: "a",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "set_deadline",
        title: "Set deadline",
        category: CommandCategory::Scheduling,
        keys: "d",
        reserved_by: None,
        alternative: None,
        surfaces: DETAIL,
    },
    CommandDefinition {
        id: "clear_deadline",
        title: "Clear deadline",
        category: CommandCategory::Scheduling,
        keys: "Shift+D",
        reserved_by: None,
        alternative: None,
        surfaces: DETAIL,
    },
    CommandDefinition {
        id: "quick_find",
        title: "Quick Find",
        category: CommandCategory::Navigation,
        keys: "Cmd+K",
        reserved_by: Some("Firefox address bar"),
        alternative: Some("/"),
        surfaces: GLOBAL,
    },
    CommandDefinition {
        id: "go_inbox",
        title: "Go to Inbox",
        category: CommandCategory::Navigation,
        keys: "g i",
        reserved_by: None,
        alternative: None,
        surfaces: GLOBAL,
    },
    CommandDefinition {
        id: "go_today",
        title: "Go to Today",
        category: CommandCategory::Navigation,
        keys: "g t",
        reserved_by: None,
        alternative: None,
        surfaces: GLOBAL,
    },
    CommandDefinition {
        id: "go_upcoming",
        title: "Go to Upcoming",
        category: CommandCategory::Navigation,
        keys: "g u",
        reserved_by: None,
        alternative: None,
        surfaces: GLOBAL,
    },
    CommandDefinition {
        id: "go_logbook",
        title: "Go to Logbook",
        category: CommandCategory::Navigation,
        keys: "g l",
        reserved_by: None,
        alternative: None,
        surfaces: GLOBAL,
    },
    CommandDefinition {
        id: "edit_todo",
        title: "Edit to-do",
        category: CommandCategory::Organization,
        keys: "Cmd+E",
        reserved_by: Some("Firefox/Chrome search bar"),
        alternative: Some("F2"),
        surfaces: LIST,
    },
    CommandDefinition {
        id: "move_todo",
        title: "Move to project or area",
        category: CommandCategory::Organization,
        keys: "m",
        reserved_by: None,
        alternative: None,
        surfaces: DETAIL,
    },
    CommandDefinition {
        id: "add_tag",
        title: "Add tag",
        category: CommandCategory::Organization,
        keys: "u",
        reserved_by: None,
        alternative: None,
        surfaces: DETAIL,
    },
    CommandDefinition {
        id: "toggle_selection",
        title: "Toggle selection",
        category: CommandCategory::Selection,
        keys: "s",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "select_all",
        title: "Select all",
        category: CommandCategory::Selection,
        keys: "Cmd+A",
        reserved_by: Some("Browser select all"),
        alternative: Some("Shift+A"),
        surfaces: LIST,
    },
    CommandDefinition {
        id: "clear_selection",
        title: "Clear selection",
        category: CommandCategory::Selection,
        keys: "Escape",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "reorder_up",
        title: "Move item up",
        category: CommandCategory::Ordering,
        keys: "Shift+Up",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
    CommandDefinition {
        id: "reorder_down",
        title: "Move item down",
        category: CommandCategory::Ordering,
        keys: "Shift+Down",
        reserved_by: None,
        alternative: None,
        surfaces: LIST,
    },
];

/// Every declared command.
#[must_use]
pub const fn registry() -> &'static [CommandDefinition] {
    REGISTRY
}

/// Looks up a command by its stable identifier.
#[must_use]
pub fn command(id: &str) -> Option<&'static CommandDefinition> {
    REGISTRY.iter().find(|command| command.id == id)
}

/// The effective binding for an identifier: the safe web equivalent when the primary binding is
/// reserved, otherwise the primary binding.
#[must_use]
pub fn keys_for(id: &str) -> Option<&'static str> {
    command(id).map(|command| command.alternative.unwrap_or(command.keys))
}

/// Commands available on a surface.
#[must_use]
pub fn commands_for(surface: CommandSurface) -> Vec<&'static CommandDefinition> {
    REGISTRY
        .iter()
        .filter(|command| command.surfaces.contains(&surface))
        .collect()
}

/// Commands whose primary binding is reserved by the browser or OS.
#[must_use]
pub fn conflicts() -> Vec<&'static CommandDefinition> {
    REGISTRY
        .iter()
        .filter(|command| command.reserved_by.is_some())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        command, commands_for, conflicts, keys_for, registry, CommandCategory, CommandSurface,
    };

    #[test]
    fn identifiers_and_primary_bindings_are_unique() {
        let mut ids: Vec<&str> = registry().iter().map(|command| command.id).collect();
        ids.sort_unstable();
        let unique = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), unique, "duplicate command ids");
        let mut keys: Vec<String> = registry()
            .iter()
            .map(|command| command.keys.to_lowercase())
            .collect();
        keys.sort();
        let unique = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), unique, "duplicate primary bindings");
    }

    #[test]
    fn reserved_bindings_document_a_safe_alternative() {
        for conflict in conflicts() {
            assert!(
                conflict.alternative.is_some(),
                "{} is reserved but has no web equivalent",
                conflict.id
            );
            assert_eq!(keys_for(conflict.id), conflict.alternative);
        }
        assert_eq!(keys_for("quick_find"), Some("/"));
        assert_eq!(keys_for("new_todo"), Some("n"));
        assert_eq!(keys_for("missing"), None);
    }

    #[test]
    fn every_category_is_covered_and_surfaces_filter() {
        for category in [
            CommandCategory::Creation,
            CommandCategory::Status,
            CommandCategory::Scheduling,
            CommandCategory::Navigation,
            CommandCategory::Organization,
            CommandCategory::Selection,
            CommandCategory::Ordering,
        ] {
            assert!(
                registry()
                    .iter()
                    .any(|command| command.category == category),
                "no command for {}",
                category.as_str()
            );
        }
        let list = commands_for(CommandSurface::List);
        assert!(list.iter().any(|command| command.id == "complete_todo"));
        assert!(!list.iter().any(|command| command.id == "move_todo"));
        let global = commands_for(CommandSurface::Global);
        assert!(global.iter().any(|command| command.id == "quick_find"));
        assert!(command("complete_todo").is_some());
    }
}
