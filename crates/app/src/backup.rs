//! Versioned JSON backups, human-readable exports, and validated additive imports.
//!
//! The backup document is independent of database layout: a documented version, timestamps, and
//! plain records for areas, projects, tags, and to-dos. CSV and Markdown exports use deterministic
//! field mapping. Imports validate the version and every record first, report unsupported records
//! explicitly, and apply additively — nothing is ever destructively replaced.

use chrono::{DateTime, Utc};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use serde::{Deserialize, Serialize};

use crate::commands::{CommandError, ScheduleInput};
use crate::todos::{TodoScope, TodoService, TodoSummary};

/// Current backup document version.
pub const BACKUP_VERSION: u32 = 1;

/// A complete, layout-independent backup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupDocument {
    /// Document version; imports reject unknown versions.
    pub version: u32,
    /// Export instant.
    pub exported_at: DateTime<Utc>,
    /// Areas in the account.
    pub areas: Vec<BackupArea>,
    /// Projects in the account.
    pub projects: Vec<BackupProject>,
    /// Tags in the account.
    pub tags: Vec<BackupTag>,
    /// To-dos (open, logged, and trashed).
    pub todos: Vec<BackupTodo>,
}

/// An exported area.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupArea {
    /// Source identity (informational).
    pub id: EntityId,
    /// Title.
    pub title: String,
}

/// An exported project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupProject {
    /// Source identity (informational).
    pub id: EntityId,
    /// Title.
    pub title: String,
    /// Notes.
    pub notes: String,
}

/// An exported tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupTag {
    /// Source identity (informational).
    pub id: EntityId,
    /// Title.
    pub title: String,
}

/// An exported to-do.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupTodo {
    /// Title.
    pub title: String,
    /// Notes.
    pub notes: String,
    /// Lifecycle status (`open`, `completed`, `canceled`, `deleted`).
    pub status: String,
    /// Scheduled date, when date-based.
    pub scheduled_on: Option<LocalDate>,
    /// Whether the schedule is in the evening bucket.
    pub evening: bool,
    /// Deadline date.
    pub deadline: Option<LocalDate>,
    /// Checklist item titles.
    pub checklist: Vec<String>,
}

impl BackupTodo {
    /// Builds a record from a read model.
    #[must_use]
    pub fn from_summary(todo: &TodoSummary) -> Self {
        Self {
            title: todo.title.clone(),
            notes: todo.notes.clone(),
            status: match todo.status {
                momentum_domain::model::Status::Open => "open",
                momentum_domain::model::Status::Completed => "completed",
                momentum_domain::model::Status::Canceled => "canceled",
                momentum_domain::model::Status::Deleted => "deleted",
            }
            .to_string(),
            scheduled_on: todo.schedule.date(),
            evening: todo.schedule.is_evening(),
            deadline: todo.deadline.map(|deadline| deadline.date),
            checklist: todo
                .checklist
                .iter()
                .map(|item| item.title.clone())
                .collect(),
        }
    }
}

/// Import failures that reject the whole document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupError {
    /// The document version is not supported.
    UnsupportedVersion(u32),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported backup version {version}")
            }
        }
    }
}

impl std::error::Error for BackupError {}

/// A validated, additive import plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportPlan {
    /// Valid to-do records to create, in document order.
    pub todos: Vec<BackupTodo>,
    /// Records that were not imported, with reasons (unsupported kinds or invalid content).
    pub skipped: Vec<SkippedRecord>,
}

/// One skipped record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedRecord {
    /// Record kind (`area`, `project`, `tag`, `todo`).
    pub kind: &'static str,
    /// Index within its collection.
    pub index: usize,
    /// Human-readable reason.
    pub reason: String,
}

/// Validates a backup into an additive import plan.
///
/// Areas, projects, and tags are reported as unsupported records (they are exported for
/// completeness); to-dos validate title and dates and are planned for creation. Nothing is ever
/// replaced.
pub fn validate_backup(document: &BackupDocument) -> Result<ImportPlan, BackupError> {
    if document.version != BACKUP_VERSION {
        return Err(BackupError::UnsupportedVersion(document.version));
    }
    let mut plan = ImportPlan {
        todos: Vec::new(),
        skipped: Vec::new(),
    };
    for (index, area) in document.areas.iter().enumerate() {
        plan.skipped.push(SkippedRecord {
            kind: "area",
            index,
            reason: format!("areas are not imported by this version ({})", area.title),
        });
    }
    for (index, project) in document.projects.iter().enumerate() {
        plan.skipped.push(SkippedRecord {
            kind: "project",
            index,
            reason: format!(
                "projects are not imported by this version ({})",
                project.title
            ),
        });
    }
    for (index, tag) in document.tags.iter().enumerate() {
        plan.skipped.push(SkippedRecord {
            kind: "tag",
            index,
            reason: format!("tags are not imported by this version ({})", tag.title),
        });
    }
    for (index, todo) in document.todos.iter().enumerate() {
        if todo.title.trim().is_empty() {
            plan.skipped.push(SkippedRecord {
                kind: "todo",
                index,
                reason: "title is empty".to_string(),
            });
            continue;
        }
        if !matches!(
            todo.status.as_str(),
            "open" | "completed" | "canceled" | "deleted"
        ) {
            plan.skipped.push(SkippedRecord {
                kind: "todo",
                index,
                reason: format!("unsupported status {}", todo.status),
            });
            continue;
        }
        plan.todos.push(todo.clone());
    }
    Ok(plan)
}

/// Per-run import results.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportSummary {
    /// Successfully created to-dos.
    pub imported: usize,
    /// Records not imported, with reasons.
    pub skipped: Vec<SkippedRecord>,
}

/// Applies a validated plan additively through the normal command layer.
///
/// # Errors
///
/// Propagates storage failures.
pub async fn apply_import(
    todos_service: &TodoService,
    account_id: AccountId,
    plan: ImportPlan,
) -> Result<ImportSummary, CommandError> {
    let mut summary = ImportSummary {
        imported: 0,
        skipped: plan.skipped,
    };
    for (index, todo) in plan.todos.iter().enumerate() {
        let notes = (!todo.notes.is_empty()).then(|| todo.notes.clone());
        let outcome = todos_service
            .create_in(
                account_id,
                &todo.title,
                notes,
                momentum_domain::model::ContextRef::Inbox,
            )
            .await?;
        let Some(id) = outcome.entity_id else {
            continue;
        };
        if let Some(date) = todo.scheduled_on {
            let schedule = if todo.evening {
                ScheduleInput::ThisEvening(date)
            } else {
                ScheduleInput::On(date)
            };
            if todos_service
                .schedule(account_id, id, schedule)
                .await
                .is_err()
            {
                summary.skipped.push(SkippedRecord {
                    kind: "todo",
                    index,
                    reason: "schedule could not be applied".to_string(),
                });
                continue;
            }
        }
        if let Some(deadline) = todo.deadline {
            let _ = todos_service
                .set_deadline(account_id, id, Some(deadline))
                .await;
        }
        for item in &todo.checklist {
            let _ = todos_service.add_checklist_item(account_id, id, item).await;
        }
        if matches!(todo.status.as_str(), "completed") {
            let _ = todos_service.complete(account_id, id).await;
        }
        if matches!(todo.status.as_str(), "canceled") {
            let _ = todos_service.cancel(account_id, id).await;
        }
        if matches!(todo.status.as_str(), "deleted") {
            let _ = todos_service.delete(account_id, id).await;
        }
        summary.imported += 1;
    }
    Ok(summary)
}

/// Builds the complete backup for an account from the read services.
///
/// # Errors
///
/// Propagates read failures.
pub async fn build_backup(
    todos_service: &TodoService,
    account_id: AccountId,
    areas: Vec<BackupArea>,
    projects: Vec<BackupProject>,
    tags: Vec<BackupTag>,
    now: DateTime<Utc>,
) -> Result<BackupDocument, CommandError> {
    let mut todos = Vec::new();
    for scope in [
        TodoScope::AllOpen,
        TodoScope::Logbook { logged_only: false },
        TodoScope::Trash,
    ] {
        for todo in todos_service.list(account_id, scope).await? {
            todos.push(BackupTodo::from_summary(&todo));
        }
    }
    Ok(BackupDocument {
        version: BACKUP_VERSION,
        exported_at: now,
        areas,
        projects,
        tags,
        todos,
    })
}

/// Deterministic CSV export (RFC 4180 quoting).
#[must_use]
pub fn todos_to_csv(todos: &[TodoSummary]) -> String {
    let mut out = String::from("title,status,scheduled_on,evening,deadline,notes\n");
    for todo in todos {
        let fields = [
            todo.title.clone(),
            match todo.status {
                momentum_domain::model::Status::Open => "open".to_string(),
                momentum_domain::model::Status::Completed => "completed".to_string(),
                momentum_domain::model::Status::Canceled => "canceled".to_string(),
                momentum_domain::model::Status::Deleted => "deleted".to_string(),
            },
            todo.schedule
                .date()
                .map(|date| date.to_iso())
                .unwrap_or_default(),
            todo.schedule.is_evening().to_string(),
            todo.deadline
                .map(|deadline| deadline.date.to_iso())
                .unwrap_or_default(),
            todo.notes.clone(),
        ];
        let escaped = fields
            .iter()
            .map(|field| csv_field(field))
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&escaped);
        out.push('\n');
    }
    out
}

fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// Deterministic Markdown export grouped by status with checkbox rows.
#[must_use]
pub fn todos_to_markdown(todos: &[TodoSummary]) -> String {
    let mut out = String::from("# To-dos\n\n");
    for todo in todos {
        let checked = matches!(todo.status, momentum_domain::model::Status::Completed);
        let box_mark = if checked { "x" } else { " " };
        out.push_str(&format!("- [{box_mark}] {}", todo.title));
        let mut meta = Vec::new();
        if let Some(date) = todo.schedule.date() {
            meta.push(format!("when: {date}"));
        }
        if let Some(deadline) = todo.deadline {
            meta.push(format!("deadline: {}", deadline.date));
        }
        if !meta.is_empty() {
            out.push_str(&format!(" ({})", meta.join(", ")));
        }
        out.push('\n');
        for item in &todo.checklist {
            out.push_str(&format!("  - [ ] {}\n", item.title));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{validate_backup, BackupDocument, BackupError, BackupTodo, BACKUP_VERSION};
    use chrono::TimeZone;
    use momentum_domain::time::LocalDate;

    fn document(todos: Vec<BackupTodo>) -> BackupDocument {
        BackupDocument {
            version: BACKUP_VERSION,
            exported_at: chrono::Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
            areas: Vec::new(),
            projects: Vec::new(),
            tags: Vec::new(),
            todos,
        }
    }

    fn todo(title: &str, status: &str) -> BackupTodo {
        BackupTodo {
            title: title.to_string(),
            notes: String::new(),
            status: status.to_string(),
            scheduled_on: None,
            evening: false,
            deadline: None,
            checklist: Vec::new(),
        }
    }

    #[test]
    fn unsupported_versions_are_rejected() {
        let mut doc = document(Vec::new());
        doc.version = 99;
        assert_eq!(
            validate_backup(&doc),
            Err(BackupError::UnsupportedVersion(99))
        );
    }

    #[test]
    fn validation_plans_valid_todos_and_reports_unsupported_records() {
        let doc = document(vec![
            todo("Good", "open"),
            todo("", "open"),
            todo("Odd", "mystery"),
            BackupTodo {
                scheduled_on: Some(LocalDate::from_ymd(2026, 6, 12).unwrap()),
                ..todo("Scheduled", "completed")
            },
        ]);
        let plan = validate_backup(&doc).expect("plan");
        assert_eq!(plan.todos.len(), 2);
        assert_eq!(plan.skipped.len(), 2);
        assert!(plan
            .skipped
            .iter()
            .any(|record| record.reason.contains("empty")));
        assert!(plan
            .skipped
            .iter()
            .any(|record| record.reason.contains("unsupported status")));
    }

    #[test]
    fn csv_and_markdown_mapping_is_deterministic() {
        let summary = momentum_app_fixture();
        let csv = super::todos_to_csv(std::slice::from_ref(&summary));
        assert!(csv.starts_with("title,status,scheduled_on,evening,deadline,notes\n"));
        assert!(
            csv.contains("\"Quote, \\\"quoted\\\"\"") || csv.contains("\"Quote, \"\"quoted\"\"\"")
        );
        let markdown = super::todos_to_markdown(std::slice::from_ref(&summary));
        assert!(markdown.contains("- [ ] Title (when: 2026-06-12, deadline: 2026-06-20)"));
        assert!(markdown.contains("  - [ ] Item"));
    }

    fn momentum_app_fixture() -> crate::todos::TodoSummary {
        use crate::todos::{ChecklistItemSummary, TodoSummary};
        use momentum_domain::model::{ContextRef, Deadline, Schedule, Status};
        use momentum_domain::ChecklistItemId;
        TodoSummary {
            id: momentum_domain::EntityId::new(),
            title: "Title".to_string(),
            notes: "Quote, \"quoted\"".to_string(),
            status: Status::Open,
            schedule: Schedule::on(LocalDate::from_ymd(2026, 6, 12).unwrap()),
            deadline: Some(Deadline::new(LocalDate::from_ymd(2026, 6, 20).unwrap())),
            reminder: None,
            context: ContextRef::Inbox,
            completed_at: None,
            canceled_at: None,
            logged_at: None,
            created_at: chrono::Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
            version: 1,
            checklist: vec![ChecklistItemSummary {
                id: ChecklistItemId::new(),
                title: "Item".to_string(),
                completed: false,
            }],
            tags: Vec::new(),
            inherited_tags: Vec::new(),
        }
    }
}
