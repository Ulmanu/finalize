//! The authoritative movement engine: valid destinations for to-dos, projects, and headings.

use momentum_domain::model::ContextRef;
use momentum_domain::{AccountId, EntityId};

use crate::commands::{CommandError, UnitOfWork};

/// A destination an entity can be moved to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Destination {
    /// The account Inbox.
    Inbox,
    /// An area.
    Area(EntityId),
    /// A project.
    Project(EntityId),
    /// A heading inside a project.
    Heading(EntityId),
}

impl Destination {
    /// Builds a destination from an entity context.
    #[must_use]
    pub const fn from_context(context: ContextRef) -> Self {
        match context {
            ContextRef::Inbox => Self::Inbox,
            ContextRef::Area(id) => Self::Area(id),
            ContextRef::Project(id) => Self::Project(id),
            ContextRef::Heading(id) => Self::Heading(id),
        }
    }

    /// Converts the destination into the canonical context reference.
    #[must_use]
    pub const fn context(self) -> ContextRef {
        match self {
            Self::Inbox => ContextRef::Inbox,
            Self::Area(id) => ContextRef::Area(id),
            Self::Project(id) => ContextRef::Project(id),
            Self::Heading(id) => ContextRef::Heading(id),
        }
    }

    /// Stable label used in validation errors.
    #[must_use]
    pub const fn kind(self) -> &'static str {
        match self {
            Self::Inbox => "inbox",
            Self::Area(_) => "area",
            Self::Project(_) => "project",
            Self::Heading(_) => "heading",
        }
    }
}

/// Kinds of entities the move engine can relocate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movable {
    /// A to-do moves between contexts.
    Todo,
    /// A project moves between areas (or loses its area).
    Project,
    /// A heading moves between projects.
    Heading,
}

/// Whether the destination kind is meaningful for the movable kind.
#[must_use]
pub const fn is_valid(movable: Movable, destination: Destination) -> bool {
    match movable {
        Movable::Todo => true,
        Movable::Project => matches!(destination, Destination::Inbox | Destination::Area(_)),
        Movable::Heading => matches!(destination, Destination::Project(_)),
    }
}

/// Validates a move request against the destination matrix.
///
/// # Errors
///
/// Returns a validation error when the destination kind is not allowed for the movable kind.
pub fn validate(movable: Movable, destination: Destination) -> Result<(), CommandError> {
    if is_valid(movable, destination) {
        Ok(())
    } else {
        Err(CommandError::Validation(format!(
            "a {} cannot move to a {} destination",
            match movable {
                Movable::Todo => "to-do",
                Movable::Project => "project",
                Movable::Heading => "heading",
            },
            destination.kind(),
        )))
    }
}

/// Confirms the destination entity exists, is owned by the account, and is alive.
///
/// # Errors
///
/// Returns `NotFound` for unknown or trashed destinations.
pub async fn ensure_destination(
    unit: &mut dyn UnitOfWork,
    account_id: AccountId,
    destination: Destination,
) -> Result<(), CommandError> {
    match destination {
        Destination::Inbox => Ok(()),
        Destination::Area(id) => match unit.load_area(account_id, id).await? {
            Some(area) if area.meta.deleted_at.is_none() => Ok(()),
            _ => Err(CommandError::NotFound),
        },
        Destination::Project(id) => match unit.load_project(account_id, id).await? {
            Some(project) if project.status == momentum_domain::model::Status::Open => Ok(()),
            _ => Err(CommandError::NotFound),
        },
        Destination::Heading(id) => match unit.load_heading(account_id, id).await? {
            Some(_) => Ok(()),
            None => Err(CommandError::NotFound),
        },
    }
}
