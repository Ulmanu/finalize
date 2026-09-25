//! Typed command layer: the single authoritative mutation boundary.
//!
//! Every mutation enters through a [`Command`] carried by a [`CommandEnvelope`]. The
//! [`CommandDispatcher`] executes it inside a [`UnitOfWork`] transaction: domain rules are
//! applied to account-scoped entities, persistence and change-log metadata are written
//! atomically, and the outcome carries domain events plus projection invalidations. Replaying
//! an operation id returns the recorded outcome without re-applying side effects.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_domain::model::{
    ContextRef, Deadline, ProjectCompletionPolicy, Reminder, Schedule, Status, Title, Todo,
};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_domain::EntityId;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::auth::Clock;

/// Schedule input accepted by commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ScheduleInput {
    /// Actionable with no start date.
    Anytime,
    /// Deferred to Someday.
    Someday,
    /// Scheduled for a local date.
    On(LocalDate),
    /// Scheduled for a local date in the This Evening section.
    ThisEvening(LocalDate),
}

impl ScheduleInput {
    /// Converts the input into a domain schedule.
    ///
    /// # Errors
    ///
    /// Propagates domain schedule errors (evening without a date cannot be expressed here).
    pub fn to_schedule(self) -> Result<Schedule, CommandError> {
        Ok(match self {
            Self::Anytime => Schedule::anytime(),
            Self::Someday => Schedule::someday(),
            Self::On(date) => Schedule::on(date),
            Self::ThisEvening(date) => Schedule::this_evening(date),
        })
    }
}

/// Reminder input accepted by commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReminderInput {
    /// Instant the reminder fires.
    pub at: DateTime<Utc>,
    /// IANA timezone used to present and snooze it.
    pub timezone: String,
}

/// Typed application command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Command {
    /// Creates a to-do.
    CreateTodo {
        /// Title text.
        title: String,
        /// Optional notes.
        notes: Option<String>,
        /// Contextual parent.
        context: ContextRef,
        /// Initial schedule.
        schedule: ScheduleInput,
        /// Optional deadline.
        deadline: Option<LocalDate>,
        /// Optional reminder.
        reminder: Option<ReminderInput>,
    },
    /// Creates a project.
    CreateProject {
        /// Project title.
        title: String,
        /// Owning area, when any.
        area: Option<EntityId>,
    },
    /// Renames a project.
    RenameProject {
        /// Project to rename.
        id: EntityId,
        /// New title.
        title: String,
    },
    /// Replaces project notes.
    SetProjectNotes {
        /// Project to update.
        id: EntityId,
        /// New notes.
        notes: String,
    },
    /// Replaces a project's schedule.
    ScheduleProject {
        /// Project to schedule.
        id: EntityId,
        /// New schedule.
        schedule: ScheduleInput,
    },
    /// Replaces a project's deadline.
    SetProjectDeadline {
        /// Project to update.
        id: EntityId,
        /// New deadline, or `None` to clear.
        deadline: Option<LocalDate>,
    },
    /// Completes a project under an explicit open-children policy.
    CompleteProject {
        /// Project to complete.
        id: EntityId,
        /// Policy for open direct children.
        policy: ProjectCompletionPolicy,
    },
    /// Cancels a project.
    CancelProject {
        /// Project to cancel.
        id: EntityId,
    },
    /// Reopens a completed or canceled project.
    ReopenProject {
        /// Project to reopen.
        id: EntityId,
    },
    /// Assigns a project to an area, or clears the assignment.
    MoveProject {
        /// Project to move.
        id: EntityId,
        /// Destination area, or `None` for no area.
        area: Option<EntityId>,
    },
    /// Deletes a project under an explicit direct-children policy.
    DeleteProject {
        /// Project to delete.
        id: EntityId,
        /// Policy for direct children.
        policy: ProjectDeletionPolicy,
    },
    /// Creates a heading inside a project.
    CreateHeading {
        /// Owning project.
        project: EntityId,
        /// Heading title.
        title: String,
    },
    /// Renames a heading.
    RenameHeading {
        /// Heading to rename.
        id: EntityId,
        /// New title.
        title: String,
    },
    /// Reorders a heading with its assigned child to-dos.
    ReorderHeading {
        /// Heading to move.
        id: EntityId,
        /// Direction within the project scope.
        direction: ReorderDirection,
    },
    /// Archives a heading, hiding it without deleting its children.
    ArchiveHeading {
        /// Heading to archive.
        id: EntityId,
    },
    /// Restores an archived heading.
    RestoreHeading {
        /// Heading to restore.
        id: EntityId,
    },
    /// Creates an area.
    CreateArea {
        /// Area title.
        title: String,
    },
    /// Renames an area.
    RenameArea {
        /// Area to rename.
        id: EntityId,
        /// New title.
        title: String,
    },
    /// Deletes an area under an explicit contents policy.
    DeleteArea {
        /// Area to delete.
        id: EntityId,
        /// What happens to contained projects and to-dos.
        policy: AreaDeletionPolicy,
    },
    /// Moves a to-do one position up or down within its contextual scope.
    ReorderTodo {
        /// Entity to move.
        id: EntityId,
        /// Direction within the scope.
        direction: ReorderDirection,
    },
    /// Duplicates a to-do into a new canonical identity.
    DuplicateTodo {
        /// Entity to duplicate.
        id: EntityId,
    },
    /// Replaces the title.
    SetTodoTitle {
        /// Entity to update.
        id: EntityId,
        /// New title text.
        title: String,
    },
    /// Replaces the notes.
    SetTodoNotes {
        /// Entity to update.
        id: EntityId,
        /// New notes text.
        notes: String,
    },
    /// Completes a to-do.
    CompleteTodo {
        /// Entity to complete.
        id: EntityId,
    },
    /// Cancels a to-do.
    CancelTodo {
        /// Entity to cancel.
        id: EntityId,
    },
    /// Reopens a completed or canceled to-do.
    ReopenTodo {
        /// Entity to reopen.
        id: EntityId,
    },
    /// Moves a to-do to Trash.
    DeleteTodo {
        /// Entity to delete.
        id: EntityId,
    },
    /// Restores a to-do from Trash.
    RestoreTodo {
        /// Entity to restore.
        id: EntityId,
    },
    /// Creates a tag.
    CreateTag {
        /// Tag title.
        title: String,
        /// Parent tag, when nested.
        parent: Option<EntityId>,
    },
    /// Renames a tag.
    RenameTag {
        /// Tag to rename.
        id: EntityId,
        /// New title.
        title: String,
    },
    /// Moves a tag to a new parent (or to the top level).
    MoveTag {
        /// Tag to move.
        id: EntityId,
        /// Destination parent, or `None` for top level.
        parent: Option<EntityId>,
    },
    /// Moves a Today item one position up or down within its Today/Evening bucket.
    ReorderToday {
        /// Entity to move.
        id: EntityId,
        /// Direction within the bucket.
        direction: ReorderDirection,
    },
    /// Moves a tag one position up or down among its siblings.
    ReorderTag {
        /// Tag to move.
        id: EntityId,
        /// Direction among siblings.
        direction: ReorderDirection,
    },
    /// Deletes a tag, reparenting its children to its own parent.
    DeleteTag {
        /// Tag to delete.
        id: EntityId,
    },
    /// Assigns a tag to an open entity (to-do, project, or area).
    AssignTag {
        /// Entity to tag.
        entity: EntityId,
        /// Tag to assign.
        tag: EntityId,
    },
    /// Removes a tag from an open entity (to-do, project, or area).
    UnassignTag {
        /// Entity to update.
        entity: EntityId,
        /// Tag to remove.
        tag: EntityId,
    },
    /// Adds a checklist item to a to-do.
    AddChecklistItem {
        /// Parent to-do.
        todo: EntityId,
        /// Item title.
        title: String,
    },
    /// Replaces a checklist item title.
    SetChecklistItemTitle {
        /// Parent to-do.
        todo: EntityId,
        /// Item to update.
        item: momentum_domain::ChecklistItemId,
        /// New title.
        title: String,
    },
    /// Sets a checklist item's completion state.
    SetChecklistItemCompleted {
        /// Parent to-do.
        todo: EntityId,
        /// Item to update.
        item: momentum_domain::ChecklistItemId,
        /// New completion state.
        completed: bool,
    },
    /// Removes a checklist item.
    RemoveChecklistItem {
        /// Parent to-do.
        todo: EntityId,
        /// Item to remove.
        item: momentum_domain::ChecklistItemId,
    },
    /// Moves a checklist item one position up or down.
    ReorderChecklistItem {
        /// Parent to-do.
        todo: EntityId,
        /// Item to move.
        item: momentum_domain::ChecklistItemId,
        /// Direction within the checklist.
        direction: ReorderDirection,
    },
    /// Expands pasted multi-line content into checklist items.
    ExpandChecklist {
        /// Parent to-do.
        todo: EntityId,
        /// Pasted content.
        text: String,
    },
    /// Moves a to-do to a new contextual parent.
    MoveTodo {
        /// Entity to move.
        id: EntityId,
        /// Destination context.
        context: ContextRef,
    },
    /// Replaces the schedule.
    ScheduleTodo {
        /// Entity to schedule.
        id: EntityId,
        /// New schedule.
        schedule: ScheduleInput,
    },
    /// Replaces the deadline.
    SetTodoDeadline {
        /// Entity to update.
        id: EntityId,
        /// New deadline, or `None` to clear.
        deadline: Option<LocalDate>,
    },
    /// Replaces the reminder.
    SetTodoReminder {
        /// Entity to update.
        id: EntityId,
        /// New reminder, or `None` to clear.
        reminder: Option<ReminderInput>,
    },
}

impl Command {
    /// Entity the command targets, when it is not a creation.
    #[must_use]
    pub const fn target(&self) -> Option<EntityId> {
        match self {
            Self::CreateTodo { .. }
            | Self::CreateArea { .. }
            | Self::CreateProject { .. }
            | Self::CreateHeading { .. }
            | Self::CreateTag { .. } => None,
            Self::RenameArea { id, .. }
            | Self::DeleteArea { id, .. }
            | Self::RenameProject { id, .. }
            | Self::SetProjectNotes { id, .. }
            | Self::ScheduleProject { id, .. }
            | Self::SetProjectDeadline { id, .. }
            | Self::CompleteProject { id, .. }
            | Self::CancelProject { id }
            | Self::ReopenProject { id }
            | Self::MoveProject { id, .. }
            | Self::DeleteProject { id, .. }
            | Self::ReorderToday { id, .. }
            | Self::RenameHeading { id, .. }
            | Self::ReorderHeading { id, .. }
            | Self::ArchiveHeading { id }
            | Self::RestoreHeading { id }
            | Self::RenameTag { id, .. }
            | Self::MoveTag { id, .. }
            | Self::ReorderTag { id, .. }
            | Self::DeleteTag { id } => Some(*id),
            Self::AssignTag { entity, .. } | Self::UnassignTag { entity, .. } => Some(*entity),
            Self::DuplicateTodo { id }
            | Self::ReorderTodo { id, .. }
            | Self::SetTodoTitle { id, .. }
            | Self::SetTodoNotes { id, .. }
            | Self::CompleteTodo { id }
            | Self::CancelTodo { id }
            | Self::ReopenTodo { id }
            | Self::DeleteTodo { id }
            | Self::RestoreTodo { id }
            | Self::MoveTodo { id, .. }
            | Self::ScheduleTodo { id, .. }
            | Self::SetTodoDeadline { id, .. }
            | Self::SetTodoReminder { id, .. } => Some(*id),
            Self::AddChecklistItem { todo, .. }
            | Self::SetChecklistItemTitle { todo, .. }
            | Self::SetChecklistItemCompleted { todo, .. }
            | Self::RemoveChecklistItem { todo, .. }
            | Self::ReorderChecklistItem { todo, .. }
            | Self::ExpandChecklist { todo, .. } => Some(*todo),
        }
    }

    /// Stable command name used in diagnostics.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::CreateTodo { .. } => "create_todo",
            Self::CreateArea { .. } => "create_area",
            Self::CreateProject { .. } => "create_project",
            Self::CreateHeading { .. } => "create_heading",
            Self::ReorderToday { .. } => "reorder_today",
            Self::CreateTag { .. } => "create_tag",
            Self::RenameTag { .. } => "rename_tag",
            Self::MoveTag { .. } => "move_tag",
            Self::ReorderTag { .. } => "reorder_tag",
            Self::DeleteTag { .. } => "delete_tag",
            Self::AssignTag { .. } => "assign_tag",
            Self::UnassignTag { .. } => "unassign_tag",
            Self::RenameHeading { .. } => "rename_heading",
            Self::ReorderHeading { .. } => "reorder_heading",
            Self::ArchiveHeading { .. } => "archive_heading",
            Self::RestoreHeading { .. } => "restore_heading",
            Self::RenameProject { .. } => "rename_project",
            Self::SetProjectNotes { .. } => "set_project_notes",
            Self::ScheduleProject { .. } => "schedule_project",
            Self::SetProjectDeadline { .. } => "set_project_deadline",
            Self::CompleteProject { .. } => "complete_project",
            Self::CancelProject { .. } => "cancel_project",
            Self::ReopenProject { .. } => "reopen_project",
            Self::MoveProject { .. } => "move_project",
            Self::DeleteProject { .. } => "delete_project",
            Self::RenameArea { .. } => "rename_area",
            Self::DeleteArea { .. } => "delete_area",
            Self::DuplicateTodo { .. } => "duplicate_todo",
            Self::ReorderTodo { .. } => "reorder_todo",
            Self::SetTodoTitle { .. } => "set_todo_title",
            Self::SetTodoNotes { .. } => "set_todo_notes",
            Self::CompleteTodo { .. } => "complete_todo",
            Self::CancelTodo { .. } => "cancel_todo",
            Self::ReopenTodo { .. } => "reopen_todo",
            Self::DeleteTodo { .. } => "delete_todo",
            Self::RestoreTodo { .. } => "restore_todo",
            Self::MoveTodo { .. } => "move_todo",
            Self::AddChecklistItem { .. } => "add_checklist_item",
            Self::SetChecklistItemTitle { .. } => "set_checklist_item_title",
            Self::SetChecklistItemCompleted { .. } => "set_checklist_item_completed",
            Self::RemoveChecklistItem { .. } => "remove_checklist_item",
            Self::ReorderChecklistItem { .. } => "reorder_checklist_item",
            Self::ExpandChecklist { .. } => "expand_checklist",
            Self::ScheduleTodo { .. } => "schedule_todo",
            Self::SetTodoDeadline { .. } => "set_todo_deadline",
            Self::SetTodoReminder { .. } => "set_todo_reminder",
        }
    }
}

/// Policy applied to a project's direct children when the project is deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectDeletionPolicy {
    /// Direct to-dos return to the Inbox.
    DetachChildren,
    /// Direct to-dos go to Trash.
    TrashChildren,
}

/// Policy applied to an area's contents when the area is deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AreaDeletionPolicy {
    /// Projects lose the area assignment and loose to-dos return to the Inbox.
    Detach,
    /// Contained to-dos go to Trash and contained projects are canceled.
    TrashContents,
}

/// Direction of a manual reorder within a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReorderDirection {
    /// Move one position toward the top of the list.
    Up,
    /// Move one position toward the bottom of the list.
    Down,
}

/// A command with its idempotency and authorization envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandEnvelope {
    /// Stable client operation id; retries reuse it.
    pub operation_id: Uuid,
    /// Account issuing the command.
    pub account_id: AccountId,
    /// Issue instant.
    pub issued_at: DateTime<Utc>,
    /// The command itself.
    pub command: Command,
}

impl CommandEnvelope {
    /// Creates an envelope with a fresh operation id.
    #[must_use]
    pub fn new(account_id: AccountId, command: Command, issued_at: DateTime<Utc>) -> Self {
        Self {
            operation_id: Uuid::now_v7(),
            account_id,
            issued_at,
            command,
        }
    }
}

/// Domain event emitted by a command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum DomainEvent {
    /// A to-do was created.
    TodoCreated {
        /// Entity identity.
        id: EntityId,
    },
    /// A project was created.
    ProjectCreated {
        /// Entity identity.
        id: EntityId,
    },
    /// A project changed (title, notes, schedule, deadline, area, or status).
    ProjectChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// A project was deleted.
    ProjectDeleted {
        /// Entity identity.
        id: EntityId,
    },
    /// A Today item was reordered within its bucket.
    TodayReordered {
        /// Entity identity.
        id: EntityId,
    },
    /// A tag was created.
    TagCreated {
        /// Entity identity.
        id: EntityId,
    },
    /// A tag changed (title or placement).
    TagChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// A tag was deleted.
    TagDeleted {
        /// Entity identity.
        id: EntityId,
    },
    /// A tag was assigned to a to-do.
    TagAssigned {
        /// To-do identity.
        todo: EntityId,
        /// Tag identity.
        tag: EntityId,
    },
    /// A tag was removed from a to-do.
    TagUnassigned {
        /// To-do identity.
        todo: EntityId,
        /// Tag identity.
        tag: EntityId,
    },
    /// A heading was created.
    HeadingCreated {
        /// Entity identity.
        id: EntityId,
        /// Owning project.
        project: EntityId,
    },
    /// A heading changed (title or placement).
    HeadingChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// A heading was reordered within its project.
    HeadingReordered {
        /// Entity identity.
        id: EntityId,
    },
    /// A heading was archived.
    HeadingArchived {
        /// Entity identity.
        id: EntityId,
    },
    /// A heading was restored.
    HeadingRestored {
        /// Entity identity.
        id: EntityId,
    },
    /// An area was created.
    AreaCreated {
        /// Entity identity.
        id: EntityId,
    },
    /// An area was renamed.
    AreaRenamed {
        /// Entity identity.
        id: EntityId,
    },
    /// An area was deleted.
    AreaDeleted {
        /// Entity identity.
        id: EntityId,
    },
    /// A to-do was reordered within its scope.
    TodoReordered {
        /// Entity identity.
        id: EntityId,
    },
    /// A to-do was duplicated.
    TodoDuplicated {
        /// Identity of the new to-do.
        id: EntityId,
        /// Identity of the source to-do.
        source_id: EntityId,
    },
    /// The title changed.
    TodoTitleChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// The notes changed.
    TodoNotesChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was completed.
    TodoCompleted {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was canceled.
    TodoCanceled {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was reopened.
    TodoReopened {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was deleted.
    TodoDeleted {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do was restored.
    TodoRestored {
        /// Entity identity.
        id: EntityId,
    },
    /// The to-do moved to another context.
    TodoMoved {
        /// Entity identity.
        id: EntityId,
    },
    /// The schedule changed.
    TodoScheduled {
        /// Entity identity.
        id: EntityId,
    },
    /// The deadline changed.
    TodoDeadlineChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// The reminder changed.
    TodoReminderChanged {
        /// Entity identity.
        id: EntityId,
    },
    /// The embedded checklist changed.
    ChecklistChanged {
        /// Parent to-do identity.
        id: EntityId,
    },
}

impl DomainEvent {
    /// Entity the event refers to.
    #[must_use]
    pub const fn entity_id(&self) -> EntityId {
        match self {
            Self::ProjectCreated { id }
            | Self::ProjectChanged { id }
            | Self::ProjectDeleted { id }
            | Self::AreaCreated { id }
            | Self::AreaRenamed { id }
            | Self::AreaDeleted { id }
            | Self::TodayReordered { id }
            | Self::TagCreated { id }
            | Self::TagChanged { id }
            | Self::TagDeleted { id }
            | Self::HeadingCreated { id, .. }
            | Self::HeadingChanged { id }
            | Self::HeadingReordered { id }
            | Self::HeadingArchived { id }
            | Self::HeadingRestored { id }
            | Self::TodoCreated { id }
            | Self::TodoReordered { id }
            | Self::TodoDuplicated { id, .. }
            | Self::TodoTitleChanged { id }
            | Self::TodoNotesChanged { id }
            | Self::TodoCompleted { id }
            | Self::TodoCanceled { id }
            | Self::TodoReopened { id }
            | Self::TodoDeleted { id }
            | Self::TodoRestored { id }
            | Self::TodoMoved { id }
            | Self::TodoScheduled { id }
            | Self::TodoDeadlineChanged { id }
            | Self::TodoReminderChanged { id }
            | Self::ChecklistChanged { id } => *id,
            Self::TagAssigned { todo, .. } | Self::TagUnassigned { todo, .. } => *todo,
        }
    }
}

/// Projection that must be re-derived after a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "projection", rename_all = "snake_case")]
pub enum Projection {
    /// Inbox list.
    Inbox,
    /// Today list (including This Evening).
    Today,
    /// Upcoming list.
    Upcoming,
    /// Anytime list.
    Anytime,
    /// Someday list.
    Someday,
    /// Logbook list.
    Logbook,
    /// Trash list.
    Trash,
    /// Deadlines special list.
    Deadlines,
    /// Repeating special list.
    Repeating,
    /// All Projects special list.
    AllProjects,
    /// Logged Projects special list.
    LoggedProjects,
    /// A specific project.
    Project(EntityId),
    /// A specific area.
    Area(EntityId),
}

/// Typed command failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommandError {
    /// The command input failed validation.
    #[error("validation failed: {0}")]
    Validation(String),
    /// The entity does not exist for this account.
    #[error("entity not found")]
    NotFound,
    /// The entity changed since the caller last read it.
    #[error("version conflict: expected {expected}, actual {actual}")]
    Conflict {
        /// Version the caller expected.
        expected: i64,
        /// Version currently stored.
        actual: i64,
    },
    /// The storage adapter failed.
    #[error("storage error")]
    Storage(String),
}

/// Direct children of a project.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectChildren {
    /// Direct to-do identities.
    pub todo_ids: Vec<EntityId>,
    /// Heading identities.
    pub heading_ids: Vec<EntityId>,
}

/// Direct children of an area.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AreaChildren {
    /// To-do identities filed directly under the area.
    pub todo_ids: Vec<EntityId>,
    /// Project identities assigned to the area.
    pub project_ids: Vec<EntityId>,
}

/// Persisted change record used for sync and idempotent replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeRecord {
    /// Operation id of the command that produced the record.
    pub operation_id: Uuid,
    /// Per-account monotonic sequence.
    pub sequence: i64,
    /// Entity the change concerns.
    pub entity_id: Option<EntityId>,
    /// Emitted events.
    pub events: Vec<DomainEvent>,
    /// Record creation instant.
    pub created_at: DateTime<Utc>,
}

/// Result of executing a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    /// Operation id of the executed command.
    pub operation_id: Uuid,
    /// Entity the command created or changed.
    pub entity_id: Option<EntityId>,
    /// Emitted domain events.
    pub events: Vec<DomainEvent>,
    /// Projections to invalidate.
    pub invalidated: Vec<Projection>,
    /// Change-log sequence assigned to this command.
    pub sequence: i64,
    /// Whether this outcome came from a replayed operation id.
    pub replayed: bool,
}

/// Transactional unit of work for command execution.
#[async_trait]
pub trait UnitOfWork: Send {
    /// Loads an account-scoped to-do.
    async fn load_todo(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Todo>, CommandError>;

    /// Persists a to-do, enforcing the expected version.
    async fn save_todo(
        &mut self,
        account_id: AccountId,
        todo: &Todo,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError>;

    /// Loads the ordered items of a contextual scope.
    async fn load_context_order(
        &mut self,
        account_id: AccountId,
        context: ContextRef,
    ) -> Result<Vec<(EntityId, crate::ordering::Rank)>, CommandError>;

    /// Loads an account-scoped project.
    async fn load_project(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<momentum_domain::model::Project>, CommandError>;

    /// Persists a project, enforcing the expected version when updating.
    async fn save_project(
        &mut self,
        account_id: AccountId,
        project: &momentum_domain::model::Project,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError>;

    /// Loads an account-scoped heading.
    async fn load_heading(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<momentum_domain::model::Heading>, CommandError>;

    /// Persists a heading, enforcing the expected version when updating.
    async fn save_heading(
        &mut self,
        account_id: AccountId,
        heading: &momentum_domain::model::Heading,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError>;

    /// Lists a project's direct to-dos and headings.
    async fn list_project_children(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<ProjectChildren, CommandError>;

    /// Loads an account-scoped tag.
    async fn load_tag(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<momentum_domain::model::Tag>, CommandError>;

    /// Persists a tag with its sibling position, enforcing the expected version when updating.
    async fn save_tag(
        &mut self,
        account_id: AccountId,
        tag: &momentum_domain::model::Tag,
        position: i64,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError>;

    /// Lists an account's tags with sibling positions.
    async fn list_tags(
        &mut self,
        account_id: AccountId,
    ) -> Result<Vec<(momentum_domain::model::Tag, i64)>, CommandError>;

    /// Removes a tag and its assignments.
    async fn delete_tag(&mut self, account_id: AccountId, id: EntityId)
        -> Result<(), CommandError>;

    /// Loads an account-scoped area.
    async fn load_area(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<momentum_domain::model::Area>, CommandError>;

    /// Persists an area, enforcing the expected version when updating.
    async fn save_area(
        &mut self,
        account_id: AccountId,
        area: &momentum_domain::model::Area,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError>;

    /// Lists the direct children of an area.
    async fn list_area_children(
        &mut self,
        account_id: AccountId,
        area_id: EntityId,
    ) -> Result<AreaChildren, CommandError>;

    /// Returns the area currently assigned to a project.
    async fn project_area(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<Option<Option<EntityId>>, CommandError>;

    /// Sets or clears a project's area assignment.
    async fn set_project_area(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
        area_id: Option<EntityId>,
    ) -> Result<(), CommandError>;

    /// Persists one entity's rank within its contextual scope.
    async fn set_context_rank(
        &mut self,
        account_id: AccountId,
        entity_id: EntityId,
        rank: crate::ordering::Rank,
    ) -> Result<(), CommandError>;

    /// Loads a projection ordering bucket.
    async fn load_projection_order(
        &mut self,
        account_id: AccountId,
        scope: crate::ordering::ProjectionScope,
    ) -> Result<Vec<(EntityId, crate::ordering::Rank)>, CommandError>;

    /// Persists one entity's rank within a projection ordering bucket.
    async fn set_projection_rank(
        &mut self,
        account_id: AccountId,
        scope: crate::ordering::ProjectionScope,
        entity_id: EntityId,
        rank: crate::ordering::Rank,
    ) -> Result<(), CommandError>;

    /// Removes an entity from every Today ordering bucket.
    async fn clear_today_ranks(
        &mut self,
        account_id: AccountId,
        entity_id: EntityId,
    ) -> Result<(), CommandError>;

    /// Looks up a change record by operation id (idempotency check).
    async fn change_by_operation(
        &mut self,
        account_id: AccountId,
        operation_id: Uuid,
    ) -> Result<Option<ChangeRecord>, CommandError>;

    /// Appends a change record, assigning the next per-account sequence.
    async fn append_change(
        &mut self,
        account_id: AccountId,
        record: &ChangeRecord,
    ) -> Result<ChangeRecord, CommandError>;

    /// Commits the transaction.
    async fn commit(self: Box<Self>) -> Result<(), CommandError>;

    /// Rolls the transaction back.
    async fn rollback(self: Box<Self>) -> Result<(), CommandError>;
}

/// Creates unit-of-work transactions.
#[async_trait]
pub trait UnitOfWorkFactory: Send + Sync {
    /// Begins a transaction for an account.
    async fn begin(&self, account_id: AccountId) -> Result<Box<dyn UnitOfWork>, CommandError>;
}

/// Executes commands atomically.
pub struct CommandDispatcher {
    factory: Arc<dyn UnitOfWorkFactory>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for CommandDispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CommandDispatcher")
    }
}

impl CommandDispatcher {
    /// Creates a dispatcher from its ports.
    #[must_use]
    pub fn new(factory: Arc<dyn UnitOfWorkFactory>, clock: Arc<dyn Clock>) -> Self {
        Self { factory, clock }
    }

    /// Executes a command inside one transaction.
    ///
    /// # Errors
    ///
    /// Returns a typed [`CommandError`] for validation, not-found, conflict, and storage
    /// failures.
    pub async fn execute(&self, envelope: CommandEnvelope) -> Result<CommandOutcome, CommandError> {
        let mut unit = self.factory.begin(envelope.account_id).await?;

        if let Some(record) = unit
            .change_by_operation(envelope.account_id, envelope.operation_id)
            .await?
        {
            unit.rollback().await?;
            return Ok(CommandOutcome {
                operation_id: envelope.operation_id,
                entity_id: record.entity_id,
                invalidated: invalidations_for_events(&record.events),
                events: record.events,
                sequence: record.sequence,
                replayed: true,
            });
        }

        let now = self.clock.now();
        let outcome = self.apply(&mut *unit, &envelope, now).await;

        match outcome {
            Ok((entity_id, events)) => {
                let record = ChangeRecord {
                    operation_id: envelope.operation_id,
                    sequence: 0,
                    entity_id,
                    events: events.clone(),
                    created_at: now,
                };
                let stored = unit.append_change(envelope.account_id, &record).await?;
                unit.commit().await?;
                Ok(CommandOutcome {
                    operation_id: envelope.operation_id,
                    entity_id,
                    invalidated: invalidations_for_events(&events),
                    events,
                    sequence: stored.sequence,
                    replayed: false,
                })
            }
            Err(error) => {
                unit.rollback().await?;
                Err(error)
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn apply(
        &self,
        unit: &mut dyn UnitOfWork,
        envelope: &CommandEnvelope,
        now: DateTime<Utc>,
    ) -> Result<(Option<EntityId>, Vec<DomainEvent>), CommandError> {
        let account_id = envelope.account_id;
        match &envelope.command {
            Command::CreateTodo {
                title,
                notes,
                context,
                schedule,
                deadline,
                reminder,
            } => {
                let title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                if let ContextRef::Heading(heading_id) = context {
                    if unit.load_heading(account_id, *heading_id).await?.is_none() {
                        return Err(CommandError::Validation("unknown heading".to_string()));
                    }
                }
                let mut todo = Todo::new(title, now);
                if let Some(notes) = notes {
                    todo.notes = notes.clone();
                }
                todo.context = *context;
                todo.schedule = schedule.to_schedule()?;
                todo.deadline = deadline.map(Deadline::new);
                todo.reminder = reminder
                    .clone()
                    .map(|reminder| Reminder::scheduled(reminder.at, reminder.timezone));
                unit.save_todo(account_id, &todo, None).await?;
                let items = unit.load_context_order(account_id, *context).await?;
                let mut order = crate::ordering::ScopeOrder::from_items(
                    crate::ordering::OrderingScope::Context(*context),
                    items
                        .into_iter()
                        .map(|(entity_id, rank)| crate::ordering::OrderedItem { entity_id, rank })
                        .collect(),
                );
                let change = order
                    .append(todo.id())
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                unit.set_context_rank(account_id, todo.id(), change.rank)
                    .await?;
                for item in change.rebalanced {
                    unit.set_context_rank(account_id, item.entity_id, item.rank)
                        .await?;
                }
                sync_today_rank(unit, account_id, &todo, now).await?;
                Ok((
                    Some(todo.id()),
                    vec![DomainEvent::TodoCreated { id: todo.id() }],
                ))
            }
            Command::ReorderTodo { id, direction } => {
                let todo = unit
                    .load_todo(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let context = todo.context;
                let items = unit.load_context_order(account_id, context).await?;
                let mut order = crate::ordering::ScopeOrder::from_items(
                    crate::ordering::OrderingScope::Context(context),
                    items
                        .into_iter()
                        .map(|(entity_id, rank)| crate::ordering::OrderedItem { entity_id, rank })
                        .collect(),
                );
                let position = order.position_of(*id).ok_or(CommandError::NotFound)?;
                let target = match direction {
                    ReorderDirection::Up => position.checked_sub(1),
                    ReorderDirection::Down => {
                        (position + 1 < order.items().len()).then_some(position + 1)
                    }
                };
                let event = DomainEvent::TodoReordered { id: *id };
                if let Some(target) = target {
                    let change = order
                        .move_item(*id, target)
                        .map_err(|error| CommandError::Validation(error.to_string()))?;
                    unit.set_context_rank(account_id, *id, change.rank).await?;
                    for item in change.rebalanced {
                        unit.set_context_rank(account_id, item.entity_id, item.rank)
                            .await?;
                    }
                }
                Ok((Some(*id), vec![event]))
            }
            Command::CreateProject { title, area } => {
                let title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                let mut project = momentum_domain::model::Project::new(title, now);
                project.area = *area;
                unit.save_project(account_id, &project, None).await?;
                if let Some(area_id) = area {
                    unit.set_project_area(account_id, project.id(), Some(*area_id))
                        .await?;
                }
                Ok((
                    Some(project.id()),
                    vec![DomainEvent::ProjectCreated { id: project.id() }],
                ))
            }
            Command::RenameProject { id, title } => {
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                project.title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                let expected = project.meta.version;
                project.meta.touch(now);
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::ProjectChanged { id: *id }]))
            }
            Command::SetProjectNotes { id, notes } => {
                if notes.len() > momentum_domain::model::MAX_NOTES_LEN {
                    return Err(CommandError::Validation("notes are too long".to_string()));
                }
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                project.notes = notes.clone();
                let expected = project.meta.version;
                project.meta.touch(now);
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::ProjectChanged { id: *id }]))
            }
            Command::ScheduleProject { id, schedule } => {
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                project.schedule = schedule.to_schedule()?;
                let expected = project.meta.version;
                project.meta.touch(now);
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::ProjectChanged { id: *id }]))
            }
            Command::SetProjectDeadline { id, deadline } => {
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                project.deadline = deadline.map(Deadline::new);
                let expected = project.meta.version;
                project.meta.touch(now);
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::ProjectChanged { id: *id }]))
            }
            Command::CompleteProject { id, policy } => {
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let children = unit.list_project_children(account_id, *id).await?;
                let mut open_todos = Vec::new();
                for todo_id in &children.todo_ids {
                    if let Some(todo) = unit.load_todo(account_id, *todo_id).await? {
                        if todo.is_open() {
                            open_todos.push(todo);
                        }
                    }
                }
                let expected = project.meta.version;
                let decision = project
                    .complete(open_todos.len(), *policy, now)
                    .map_err(|_| CommandError::Conflict {
                        expected,
                        actual: expected,
                    })?;
                if matches!(
                    decision,
                    momentum_domain::model::CompletionDecision::CascadeToChildren
                ) {
                    for mut todo in open_todos {
                        let todo_expected = todo.meta.version;
                        todo.complete(now);
                        unit.save_todo(account_id, &todo, Some(todo_expected))
                            .await?;
                    }
                }
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::ProjectChanged { id: *id }]))
            }
            Command::CancelProject { id } => {
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                if project.status != Status::Deleted {
                    project.status = Status::Canceled;
                    project.canceled_at = Some(now);
                    project.completed_at = None;
                }
                let expected = project.meta.version;
                project.meta.touch(now);
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::ProjectChanged { id: *id }]))
            }
            Command::ReopenProject { id } => {
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let expected = project.meta.version;
                project.reopen(now);
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::ProjectChanged { id: *id }]))
            }
            Command::MoveProject { id, area } => {
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let expected = project.meta.version;
                project.area = *area;
                project.meta.touch(now);
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                unit.set_project_area(account_id, *id, *area).await?;
                Ok((Some(*id), vec![DomainEvent::ProjectChanged { id: *id }]))
            }
            Command::DeleteProject { id, policy } => {
                let mut project = unit
                    .load_project(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let children = unit.list_project_children(account_id, *id).await?;
                for todo_id in children.todo_ids {
                    let Some(mut todo) = unit.load_todo(account_id, todo_id).await? else {
                        continue;
                    };
                    if !todo.is_open() {
                        continue;
                    }
                    let todo_expected = todo.meta.version;
                    match policy {
                        ProjectDeletionPolicy::DetachChildren => {
                            todo.move_to(ContextRef::Inbox, now)
                        }
                        ProjectDeletionPolicy::TrashChildren => {
                            todo.status = Status::Deleted;
                            todo.meta.deleted_at = Some(now);
                            todo.meta.touch(now);
                        }
                    }
                    unit.save_todo(account_id, &todo, Some(todo_expected))
                        .await?;
                }
                let expected = project.meta.version;
                project.status = Status::Deleted;
                project.meta.deleted_at = Some(now);
                project.meta.touch(now);
                unit.save_project(account_id, &project, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::ProjectDeleted { id: *id }]))
            }
            Command::ReorderToday { id, direction } => {
                let todo = unit
                    .load_todo(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let today = LocalDate::today_in_timezone(now, "UTC");
                if !todo.is_open() || !todo.schedule.is_actionable(today) {
                    return Err(CommandError::Validation(
                        "only open Today items can be reordered in Today".to_string(),
                    ));
                }
                let scope = if todo.schedule.is_evening() {
                    crate::ordering::ProjectionScope::ThisEvening
                } else {
                    crate::ordering::ProjectionScope::Today
                };
                let items = unit.load_projection_order(account_id, scope).await?;
                let mut order = crate::ordering::ScopeOrder::from_items(
                    crate::ordering::OrderingScope::Today,
                    items
                        .into_iter()
                        .map(|(entity_id, rank)| crate::ordering::OrderedItem { entity_id, rank })
                        .collect(),
                );
                if order.position_of(*id).is_none() {
                    let change = order
                        .append(*id)
                        .map_err(|error| CommandError::Validation(error.to_string()))?;
                    unit.set_projection_rank(account_id, scope, *id, change.rank)
                        .await?;
                    for item in change.rebalanced {
                        unit.set_projection_rank(account_id, scope, item.entity_id, item.rank)
                            .await?;
                    }
                }
                let position = order.position_of(*id).ok_or(CommandError::NotFound)?;
                let target = match direction {
                    ReorderDirection::Up => position.checked_sub(1),
                    ReorderDirection::Down => {
                        (position + 1 < order.items().len()).then_some(position + 1)
                    }
                };
                if let Some(target) = target {
                    let change = order
                        .move_item(*id, target)
                        .map_err(|error| CommandError::Validation(error.to_string()))?;
                    unit.set_projection_rank(account_id, scope, *id, change.rank)
                        .await?;
                    for item in change.rebalanced {
                        unit.set_projection_rank(account_id, scope, item.entity_id, item.rank)
                            .await?;
                    }
                }
                Ok((Some(*id), vec![DomainEvent::TodayReordered { id: *id }]))
            }
            Command::CreateHeading { project, title } => {
                let project_entity = unit
                    .load_project(account_id, *project)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                if project_entity.status == Status::Deleted {
                    return Err(CommandError::NotFound);
                }
                let title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                let heading = momentum_domain::model::Heading::new(title, *project, now);
                unit.save_heading(account_id, &heading, None).await?;
                let items = unit
                    .load_context_order(account_id, ContextRef::Project(*project))
                    .await?;
                let mut order = crate::ordering::ScopeOrder::from_items(
                    crate::ordering::OrderingScope::Context(ContextRef::Project(*project)),
                    items
                        .into_iter()
                        .map(|(entity_id, rank)| crate::ordering::OrderedItem { entity_id, rank })
                        .collect(),
                );
                let change = order
                    .append(heading.id())
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                unit.set_context_rank(account_id, heading.id(), change.rank)
                    .await?;
                for item in change.rebalanced {
                    unit.set_context_rank(account_id, item.entity_id, item.rank)
                        .await?;
                }
                Ok((
                    Some(heading.id()),
                    vec![DomainEvent::HeadingCreated {
                        id: heading.id(),
                        project: *project,
                    }],
                ))
            }
            Command::RenameHeading { id, title } => {
                let mut heading = unit
                    .load_heading(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                heading.title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                let expected = heading.meta.version;
                heading.meta.touch(now);
                unit.save_heading(account_id, &heading, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::HeadingChanged { id: *id }]))
            }
            Command::ReorderHeading { id, direction } => {
                let heading = unit
                    .load_heading(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let context = ContextRef::Project(heading.project);
                let items = unit.load_context_order(account_id, context).await?;
                let mut order = crate::ordering::ScopeOrder::from_items(
                    crate::ordering::OrderingScope::Context(context),
                    items
                        .into_iter()
                        .map(|(entity_id, rank)| crate::ordering::OrderedItem { entity_id, rank })
                        .collect(),
                );
                let position = order.position_of(*id).ok_or(CommandError::NotFound)?;
                let target = match direction {
                    ReorderDirection::Up => position.checked_sub(1),
                    ReorderDirection::Down => {
                        (position + 1 < order.items().len()).then_some(position + 1)
                    }
                };
                let event = DomainEvent::HeadingReordered { id: *id };
                if let Some(target) = target {
                    let change = order
                        .move_item(*id, target)
                        .map_err(|error| CommandError::Validation(error.to_string()))?;
                    unit.set_context_rank(account_id, *id, change.rank).await?;
                    for item in change.rebalanced {
                        unit.set_context_rank(account_id, item.entity_id, item.rank)
                            .await?;
                    }
                }
                Ok((Some(*id), vec![event]))
            }
            Command::ArchiveHeading { id } => {
                let mut heading = unit
                    .load_heading(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                if !heading.archived {
                    heading.archived = true;
                    let expected = heading.meta.version;
                    heading.meta.touch(now);
                    unit.save_heading(account_id, &heading, Some(expected))
                        .await?;
                }
                Ok((Some(*id), vec![DomainEvent::HeadingArchived { id: *id }]))
            }
            Command::RestoreHeading { id } => {
                let mut heading = unit
                    .load_heading(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                if heading.archived {
                    heading.archived = false;
                    let expected = heading.meta.version;
                    heading.meta.touch(now);
                    unit.save_heading(account_id, &heading, Some(expected))
                        .await?;
                }
                Ok((Some(*id), vec![DomainEvent::HeadingRestored { id: *id }]))
            }
            Command::CreateTag { title, parent } => {
                if let Some(parent_id) = parent {
                    if unit.load_tag(account_id, *parent_id).await?.is_none() {
                        return Err(CommandError::NotFound);
                    }
                }
                let title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                let tag = momentum_domain::model::Tag::new(title, *parent, now);
                let position = next_tag_position(unit, account_id, *parent).await?;
                unit.save_tag(account_id, &tag, position, None).await?;
                Ok((
                    Some(tag.id()),
                    vec![DomainEvent::TagCreated { id: tag.id() }],
                ))
            }
            Command::RenameTag { id, title } => {
                let mut tag = unit
                    .load_tag(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                tag.title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                let position = tag_position_of(unit, account_id, *id).await?;
                let expected = tag.meta.version;
                tag.meta.touch(now);
                unit.save_tag(account_id, &tag, position, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::TagChanged { id: *id }]))
            }
            Command::MoveTag { id, parent } => {
                let mut tag = unit
                    .load_tag(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                if let Some(parent_id) = parent {
                    if *parent_id == *id {
                        return Err(CommandError::Validation(
                            "a tag cannot be its own parent".to_string(),
                        ));
                    }
                    let mut cursor = Some(*parent_id);
                    let mut depth = 0;
                    while let Some(current) = cursor {
                        if current == *id {
                            return Err(CommandError::Validation("tag cycle".to_string()));
                        }
                        depth += 1;
                        if depth > 64 {
                            return Err(CommandError::Validation(
                                "tag hierarchy is too deep".to_string(),
                            ));
                        }
                        cursor = unit
                            .load_tag(account_id, current)
                            .await?
                            .ok_or(CommandError::NotFound)?
                            .parent;
                    }
                }
                tag.parent = *parent;
                let position = next_tag_position(unit, account_id, *parent).await?;
                let expected = tag.meta.version;
                tag.meta.touch(now);
                unit.save_tag(account_id, &tag, position, Some(expected))
                    .await?;
                Ok((Some(*id), vec![DomainEvent::TagChanged { id: *id }]))
            }
            Command::ReorderTag { id, direction } => {
                let tag = unit
                    .load_tag(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let mut siblings: Vec<(momentum_domain::model::Tag, i64)> = unit
                    .list_tags(account_id)
                    .await?
                    .into_iter()
                    .filter(|(entry, _)| entry.parent == tag.parent)
                    .collect();
                siblings.sort_by_key(|(entry, position)| {
                    (*position, entry.meta.created_at, entry.id())
                });
                let position = siblings
                    .iter()
                    .position(|(entry, _)| entry.id() == *id)
                    .ok_or(CommandError::NotFound)?;
                let target = match direction {
                    ReorderDirection::Up => position.checked_sub(1),
                    ReorderDirection::Down => {
                        (position + 1 < siblings.len()).then_some(position + 1)
                    }
                };
                if let Some(target) = target {
                    let (left, left_position) = &siblings[position];
                    let (right, right_position) = &siblings[target];
                    let left = left.clone();
                    let right = right.clone();
                    let left_position = *left_position;
                    let right_position = *right_position;
                    let left_expected = left.meta.version;
                    let right_expected = right.meta.version;
                    unit.save_tag(account_id, &left, right_position, Some(left_expected))
                        .await?;
                    unit.save_tag(account_id, &right, left_position, Some(right_expected))
                        .await?;
                }
                Ok((Some(*id), vec![DomainEvent::TagChanged { id: *id }]))
            }
            Command::DeleteTag { id } => {
                let tag = unit
                    .load_tag(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let tags = unit.list_tags(account_id).await?;
                for (mut child, position) in tags {
                    if child.parent == Some(*id) {
                        child.parent = tag.parent;
                        let expected = child.meta.version;
                        child.meta.touch(now);
                        unit.save_tag(account_id, &child, position, Some(expected))
                            .await?;
                    }
                }
                unit.delete_tag(account_id, *id).await?;
                Ok((Some(*id), vec![DomainEvent::TagDeleted { id: *id }]))
            }
            Command::AssignTag { entity, tag } => {
                let entity_id = *entity;
                if unit.load_tag(account_id, *tag).await?.is_none() {
                    return Err(CommandError::NotFound);
                }
                if let Some(mut todo) = unit.load_todo(account_id, entity_id).await? {
                    let expected = todo.meta.version;
                    todo.tags.insert(*tag);
                    todo.meta.touch(now);
                    unit.save_todo(account_id, &todo, Some(expected)).await?;
                } else if let Some(mut project) = unit.load_project(account_id, entity_id).await? {
                    let expected = project.meta.version;
                    project.tags.insert(*tag);
                    project.meta.touch(now);
                    unit.save_project(account_id, &project, Some(expected))
                        .await?;
                } else if let Some(mut area) = unit.load_area(account_id, entity_id).await? {
                    let expected = area.meta.version;
                    area.tags.insert(*tag);
                    area.meta.touch(now);
                    unit.save_area(account_id, &area, Some(expected)).await?;
                } else {
                    return Err(CommandError::NotFound);
                }
                Ok((
                    Some(entity_id),
                    vec![DomainEvent::TagAssigned {
                        todo: entity_id,
                        tag: *tag,
                    }],
                ))
            }
            Command::UnassignTag { entity, tag } => {
                let entity_id = *entity;
                if let Some(mut todo) = unit.load_todo(account_id, entity_id).await? {
                    let expected = todo.meta.version;
                    todo.tags.remove(tag);
                    todo.meta.touch(now);
                    unit.save_todo(account_id, &todo, Some(expected)).await?;
                } else if let Some(mut project) = unit.load_project(account_id, entity_id).await? {
                    let expected = project.meta.version;
                    project.tags.remove(tag);
                    project.meta.touch(now);
                    unit.save_project(account_id, &project, Some(expected))
                        .await?;
                } else if let Some(mut area) = unit.load_area(account_id, entity_id).await? {
                    let expected = area.meta.version;
                    area.tags.remove(tag);
                    area.meta.touch(now);
                    unit.save_area(account_id, &area, Some(expected)).await?;
                } else {
                    return Err(CommandError::NotFound);
                }
                Ok((
                    Some(entity_id),
                    vec![DomainEvent::TagUnassigned {
                        todo: entity_id,
                        tag: *tag,
                    }],
                ))
            }
            Command::CreateArea { title } => {
                let title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                let area = momentum_domain::model::Area::new(title, now);
                unit.save_area(account_id, &area, None).await?;
                Ok((
                    Some(area.id()),
                    vec![DomainEvent::AreaCreated { id: area.id() }],
                ))
            }
            Command::RenameArea { id, title } => {
                let mut area = unit
                    .load_area(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                area.title = Title::parse(title)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                area.meta.touch(now);
                let expected = area.meta.version - 1;
                unit.save_area(account_id, &area, Some(expected)).await?;
                Ok((Some(*id), vec![DomainEvent::AreaRenamed { id: *id }]))
            }
            Command::DeleteArea { id, policy } => {
                let mut area = unit
                    .load_area(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let children = unit.list_area_children(account_id, *id).await?;

                for todo_id in children.todo_ids {
                    let Some(mut todo) = unit.load_todo(account_id, todo_id).await? else {
                        continue;
                    };
                    let expected = todo.meta.version;
                    match policy {
                        AreaDeletionPolicy::Detach => todo.move_to(ContextRef::Inbox, now),
                        AreaDeletionPolicy::TrashContents => {
                            todo.status = Status::Deleted;
                            todo.meta.deleted_at = Some(now);
                            todo.meta.touch(now);
                        }
                    }
                    unit.save_todo(account_id, &todo, Some(expected)).await?;
                }
                for project_id in children.project_ids {
                    if matches!(policy, AreaDeletionPolicy::Detach) {
                        unit.set_project_area(account_id, project_id, None).await?;
                    }
                }

                area.meta.deleted_at = Some(now);
                area.meta.touch(now);
                let expected = area.meta.version - 1;
                unit.save_area(account_id, &area, Some(expected)).await?;
                Ok((Some(*id), vec![DomainEvent::AreaDeleted { id: *id }]))
            }
            Command::DuplicateTodo { id } => {
                let source = unit
                    .load_todo(account_id, *id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let mut copy = Todo::new(source.title.clone(), now);
                copy.notes = source.notes.clone();
                copy.schedule = source.schedule;
                copy.deadline = source.deadline;
                copy.reminder = source.reminder.clone();
                copy.context = source.context;
                copy.tags = source.tags.clone();
                copy.checklist = source
                    .checklist
                    .iter()
                    .map(|item| momentum_domain::model::ChecklistItem::new(item.title.clone()))
                    .collect();
                unit.save_todo(account_id, &copy, None).await?;
                Ok((
                    Some(copy.id()),
                    vec![DomainEvent::TodoDuplicated {
                        id: copy.id(),
                        source_id: source.id(),
                    }],
                ))
            }
            command => {
                if let Command::MoveTodo {
                    context: ContextRef::Heading(heading_id),
                    ..
                } = command
                {
                    if unit.load_heading(account_id, *heading_id).await?.is_none() {
                        return Err(CommandError::Validation("unknown heading".to_string()));
                    }
                }
                let id = command.target().ok_or(CommandError::NotFound)?;
                let mut todo = unit
                    .load_todo(account_id, id)
                    .await?
                    .ok_or(CommandError::NotFound)?;
                let expected_version = todo.meta.version;
                let event = apply_to_todo(&mut todo, command, now)?;
                unit.save_todo(account_id, &todo, Some(expected_version))
                    .await?;
                if matches!(
                    command,
                    Command::ScheduleTodo { .. } | Command::ReopenTodo { .. }
                ) {
                    sync_today_rank(unit, account_id, &todo, now).await?;
                }
                Ok((Some(todo.id()), vec![event]))
            }
        }
    }
}

async fn sync_today_rank(
    unit: &mut dyn UnitOfWork,
    account_id: AccountId,
    todo: &Todo,
    now: DateTime<Utc>,
) -> Result<(), CommandError> {
    let today = LocalDate::today_in_timezone(now, "UTC");
    let eligible = todo.is_open()
        && matches!(
            todo.schedule.state(),
            momentum_domain::model::ScheduleState::On(date) if date <= today
        );
    if !eligible {
        return unit.clear_today_ranks(account_id, todo.id()).await;
    }
    let scope = if todo.schedule.is_evening() {
        crate::ordering::ProjectionScope::ThisEvening
    } else {
        crate::ordering::ProjectionScope::Today
    };
    let items = unit.load_projection_order(account_id, scope).await?;
    if items.iter().any(|(entity, _)| *entity == todo.id()) {
        return Ok(());
    }
    let mut order = crate::ordering::ScopeOrder::from_items(
        crate::ordering::OrderingScope::Today,
        items
            .into_iter()
            .map(|(entity_id, rank)| crate::ordering::OrderedItem { entity_id, rank })
            .collect(),
    );
    let change = order
        .append(todo.id())
        .map_err(|error| CommandError::Validation(error.to_string()))?;
    unit.set_projection_rank(account_id, scope, todo.id(), change.rank)
        .await?;
    for item in change.rebalanced {
        unit.set_projection_rank(account_id, scope, item.entity_id, item.rank)
            .await?;
    }
    Ok(())
}

async fn next_tag_position(
    unit: &mut dyn UnitOfWork,
    account_id: AccountId,
    parent: Option<EntityId>,
) -> Result<i64, CommandError> {
    let tags = unit.list_tags(account_id).await?;
    Ok(tags
        .into_iter()
        .filter(|(tag, _)| tag.parent == parent)
        .map(|(_, position)| position)
        .max()
        .unwrap_or(0)
        + 1)
}

async fn tag_position_of(
    unit: &mut dyn UnitOfWork,
    account_id: AccountId,
    id: EntityId,
) -> Result<i64, CommandError> {
    let tags = unit.list_tags(account_id).await?;
    tags.into_iter()
        .find(|(tag, _)| tag.id() == id)
        .map(|(_, position)| position)
        .ok_or(CommandError::NotFound)
}

fn apply_to_todo(
    todo: &mut Todo,
    command: &Command,
    now: DateTime<Utc>,
) -> Result<DomainEvent, CommandError> {
    let id = todo.id();
    match command {
        Command::CreateTodo { .. }
        | Command::DuplicateTodo { .. }
        | Command::ReorderTodo { .. }
        | Command::CreateArea { .. }
        | Command::RenameArea { .. }
        | Command::DeleteArea { .. }
        | Command::CreateProject { .. }
        | Command::RenameProject { .. }
        | Command::SetProjectNotes { .. }
        | Command::ScheduleProject { .. }
        | Command::SetProjectDeadline { .. }
        | Command::CompleteProject { .. }
        | Command::CancelProject { .. }
        | Command::ReopenProject { .. }
        | Command::MoveProject { .. }
        | Command::DeleteProject { .. }
        | Command::CreateHeading { .. }
        | Command::RenameHeading { .. }
        | Command::ReorderHeading { .. }
        | Command::ArchiveHeading { .. }
        | Command::RestoreHeading { .. }
        | Command::ReorderToday { .. }
        | Command::CreateTag { .. }
        | Command::RenameTag { .. }
        | Command::MoveTag { .. }
        | Command::ReorderTag { .. }
        | Command::DeleteTag { .. }
        | Command::AssignTag { .. }
        | Command::UnassignTag { .. } => Err(CommandError::Validation(
            "commands handled separately must not reach the generic path".to_string(),
        )),
        Command::SetTodoTitle { title, .. } => {
            todo.title =
                Title::parse(title).map_err(|error| CommandError::Validation(error.to_string()))?;
            todo.meta.touch(now);
            Ok(DomainEvent::TodoTitleChanged { id })
        }
        Command::SetTodoNotes { notes, .. } => {
            if notes.len() > momentum_domain::model::MAX_NOTES_LEN {
                return Err(CommandError::Validation("notes are too long".to_string()));
            }
            todo.notes = notes.clone();
            todo.meta.touch(now);
            Ok(DomainEvent::TodoNotesChanged { id })
        }
        Command::CompleteTodo { .. } => {
            todo.complete(now);
            Ok(DomainEvent::TodoCompleted { id })
        }
        Command::CancelTodo { .. } => {
            todo.cancel(now);
            Ok(DomainEvent::TodoCanceled { id })
        }
        Command::ReopenTodo { .. } => {
            todo.reopen(now);
            Ok(DomainEvent::TodoReopened { id })
        }
        Command::DeleteTodo { .. } => {
            if todo.status != Status::Deleted {
                todo.status = Status::Deleted;
                todo.meta.deleted_at = Some(now);
                todo.meta.touch(now);
            }
            Ok(DomainEvent::TodoDeleted { id })
        }
        Command::RestoreTodo { .. } => {
            if todo.status == Status::Deleted {
                todo.status = Status::Open;
                todo.meta.deleted_at = None;
                todo.meta.touch(now);
            }
            Ok(DomainEvent::TodoRestored { id })
        }
        Command::MoveTodo { context, .. } => {
            todo.move_to(*context, now);
            Ok(DomainEvent::TodoMoved { id })
        }
        Command::ScheduleTodo { schedule, .. } => {
            todo.set_schedule(schedule.to_schedule()?, now);
            Ok(DomainEvent::TodoScheduled { id })
        }
        Command::SetTodoDeadline { deadline, .. } => {
            todo.set_deadline(deadline.map(Deadline::new), now);
            Ok(DomainEvent::TodoDeadlineChanged { id })
        }
        Command::SetTodoReminder { reminder, .. } => {
            let reminder = reminder
                .clone()
                .map(|reminder| Reminder::scheduled(reminder.at, reminder.timezone));
            todo.set_reminder(reminder, now);
            Ok(DomainEvent::TodoReminderChanged { id })
        }
        Command::AddChecklistItem { title, .. } => {
            let title =
                Title::parse(title).map_err(|error| CommandError::Validation(error.to_string()))?;
            todo.checklist
                .push(momentum_domain::model::ChecklistItem::new(title));
            todo.meta.touch(now);
            Ok(DomainEvent::ChecklistChanged { id })
        }
        Command::SetChecklistItemTitle { item, title, .. } => {
            let item = todo
                .checklist
                .iter_mut()
                .find(|entry| entry.id == *item)
                .ok_or(CommandError::NotFound)?;
            item.title =
                Title::parse(title).map_err(|error| CommandError::Validation(error.to_string()))?;
            todo.meta.touch(now);
            Ok(DomainEvent::ChecklistChanged { id })
        }
        Command::SetChecklistItemCompleted {
            item, completed, ..
        } => {
            let item = todo
                .checklist
                .iter_mut()
                .find(|entry| entry.id == *item)
                .ok_or(CommandError::NotFound)?;
            item.completed = *completed;
            todo.meta.touch(now);
            Ok(DomainEvent::ChecklistChanged { id })
        }
        Command::RemoveChecklistItem { item, .. } => {
            let before = todo.checklist.len();
            todo.checklist.retain(|entry| entry.id != *item);
            if todo.checklist.len() == before {
                return Err(CommandError::NotFound);
            }
            todo.meta.touch(now);
            Ok(DomainEvent::ChecklistChanged { id })
        }
        Command::ReorderChecklistItem {
            item, direction, ..
        } => {
            let position = todo
                .checklist
                .iter()
                .position(|entry| entry.id == *item)
                .ok_or(CommandError::NotFound)?;
            let target = match direction {
                ReorderDirection::Up => position.checked_sub(1),
                ReorderDirection::Down => {
                    (position + 1 < todo.checklist.len()).then_some(position + 1)
                }
            };
            if let Some(target) = target {
                todo.checklist.swap(position, target);
                todo.meta.touch(now);
            }
            Ok(DomainEvent::ChecklistChanged { id })
        }
        Command::ExpandChecklist { text, .. } => {
            let lines = momentum_domain::model::parse_checklist_lines(text);
            if lines.is_empty() {
                return Err(CommandError::Validation(
                    "no checklist lines to add".to_string(),
                ));
            }
            for line in lines {
                let title = Title::parse(&line)
                    .map_err(|error| CommandError::Validation(error.to_string()))?;
                todo.checklist
                    .push(momentum_domain::model::ChecklistItem::new(title));
            }
            todo.meta.touch(now);
            Ok(DomainEvent::ChecklistChanged { id })
        }
    }
}

/// Maps events to the projections that must be re-derived.
#[must_use]
pub fn invalidations_for_events(events: &[DomainEvent]) -> Vec<Projection> {
    let mut projections = Vec::new();
    for event in events {
        for projection in invalidations_for(event) {
            if !projections.contains(&projection) {
                projections.push(projection);
            }
        }
    }
    projections
}

/// Maps one event to projections.
#[must_use]
pub fn invalidations_for(event: &DomainEvent) -> Vec<Projection> {
    match event {
        DomainEvent::TodoCreated { .. } | DomainEvent::TodoDuplicated { .. } => {
            vec![Projection::Inbox]
        }
        DomainEvent::TodoReordered { .. } => vec![Projection::Inbox, Projection::Anytime],
        DomainEvent::ProjectCreated { .. }
        | DomainEvent::ProjectChanged { .. }
        | DomainEvent::ProjectDeleted { .. } => vec![
            Projection::AllProjects,
            Projection::LoggedProjects,
            Projection::Today,
            Projection::Upcoming,
            Projection::Anytime,
            Projection::Deadlines,
            Projection::Logbook,
        ],
        DomainEvent::TodayReordered { .. } => vec![Projection::Today],
        DomainEvent::TagCreated { .. }
        | DomainEvent::TagChanged { .. }
        | DomainEvent::TagDeleted { .. }
        | DomainEvent::TagAssigned { .. }
        | DomainEvent::TagUnassigned { .. } => {
            vec![Projection::Inbox, Projection::Today, Projection::Anytime]
        }
        DomainEvent::HeadingCreated { .. }
        | DomainEvent::HeadingChanged { .. }
        | DomainEvent::HeadingReordered { .. }
        | DomainEvent::HeadingArchived { .. }
        | DomainEvent::HeadingRestored { .. } => vec![
            Projection::AllProjects,
            Projection::Today,
            Projection::Upcoming,
            Projection::Anytime,
            Projection::Deadlines,
        ],
        DomainEvent::AreaCreated { id } | DomainEvent::AreaRenamed { id } => {
            vec![Projection::Area(*id), Projection::AllProjects]
        }
        DomainEvent::AreaDeleted { id } => vec![
            Projection::Area(*id),
            Projection::AllProjects,
            Projection::Inbox,
            Projection::Trash,
        ],
        DomainEvent::TodoTitleChanged { .. }
        | DomainEvent::TodoNotesChanged { .. }
        | DomainEvent::ChecklistChanged { .. } => {
            vec![Projection::Inbox, Projection::Today, Projection::Anytime]
        }
        DomainEvent::TodoCompleted { .. }
        | DomainEvent::TodoCanceled { .. }
        | DomainEvent::TodoReopened { .. } => vec![
            Projection::Inbox,
            Projection::Today,
            Projection::Upcoming,
            Projection::Anytime,
            Projection::Someday,
            Projection::Logbook,
        ],
        DomainEvent::TodoDeleted { .. } | DomainEvent::TodoRestored { .. } => vec![
            Projection::Inbox,
            Projection::Today,
            Projection::Anytime,
            Projection::Trash,
        ],
        DomainEvent::TodoMoved { .. } => vec![Projection::Inbox, Projection::Anytime],
        DomainEvent::TodoScheduled { .. } => vec![
            Projection::Inbox,
            Projection::Today,
            Projection::Upcoming,
            Projection::Anytime,
            Projection::Someday,
        ],
        DomainEvent::TodoDeadlineChanged { .. } => vec![Projection::Deadlines, Projection::Today],
        DomainEvent::TodoReminderChanged { .. } => vec![Projection::Today],
    }
}
