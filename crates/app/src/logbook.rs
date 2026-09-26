//! Logbook history, logging policy, and logging commands.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use momentum_domain::model::LoggingPolicy;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};

use crate::auth::Clock;
use crate::commands::{Command, CommandDispatcher, CommandEnvelope, CommandError, CommandOutcome};
use crate::projects::{ProjectService, ProjectSummary};
use crate::todos::{TodoScope, TodoService, TodoSummary};

/// Persistence port for the account's logging policy.
#[async_trait]
pub trait LoggingPolicyStore: Send + Sync {
    /// Loads the account policy, defaulting to automatic.
    async fn load_policy(&self, account_id: AccountId) -> Result<LoggingPolicy, CommandError>;

    /// Persists the account policy.
    async fn save_policy(
        &self,
        account_id: AccountId,
        policy: LoggingPolicy,
    ) -> Result<(), CommandError>;
}

/// One Logbook entry: a terminal to-do or project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogbookEntry {
    /// A logged/canceled/completed to-do.
    Todo(TodoSummary),
    /// A logged/canceled/completed project.
    Project(ProjectSummary),
}

impl LogbookEntry {
    /// Canonical identity of the entry.
    #[must_use]
    pub fn id(&self) -> EntityId {
        match self {
            Self::Todo(todo) => todo.id,
            Self::Project(project) => project.id,
        }
    }

    /// Display title.
    #[must_use]
    pub fn title(&self) -> &str {
        match self {
            Self::Todo(todo) => &todo.title,
            Self::Project(project) => &project.title,
        }
    }

    /// Whether the entry was explicitly logged.
    #[must_use]
    pub fn is_logged(&self) -> bool {
        self.logged_at().is_some()
    }

    /// Timestamp used for history grouping and ordering.
    #[must_use]
    pub fn history_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Todo(todo) => todo
                .logged_at
                .or(todo.completed_at)
                .or(todo.canceled_at)
                .or(Some(todo.created_at)),
            Self::Project(project) => project
                .logged_at
                .or(project.completed_at)
                .or(project.canceled_at),
        }
    }

    fn logged_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Todo(todo) => todo.logged_at,
            Self::Project(project) => project.logged_at,
        }
    }
}

/// One day of Logbook history, newest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogbookDay {
    /// Local calendar date of the entries.
    pub date: LocalDate,
    /// Entries on that date, newest first.
    pub entries: Vec<LogbookEntry>,
}

/// The Logbook projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogbookAgenda {
    /// Active logging policy.
    pub policy: LoggingPolicy,
    /// Terminal entries awaiting explicit logging under the manual policy.
    pub pending: Vec<LogbookEntry>,
    /// Logged (or automatically logged) history grouped by date, descending.
    pub days: Vec<LogbookDay>,
}

impl LogbookAgenda {
    /// Builds the agenda from terminal entries.
    #[must_use]
    pub fn from_entries(
        policy: LoggingPolicy,
        todos: Vec<TodoSummary>,
        projects: Vec<ProjectSummary>,
    ) -> Self {
        let mut entries: Vec<LogbookEntry> = todos
            .into_iter()
            .map(LogbookEntry::Todo)
            .chain(projects.into_iter().map(LogbookEntry::Project))
            .collect();
        let mut pending = Vec::new();
        if policy == LoggingPolicy::Manual {
            entries.retain(|entry| {
                if entry.is_logged() {
                    true
                } else {
                    pending.push(entry.clone());
                    false
                }
            });
        }
        entries.sort_by_key(|entry| std::cmp::Reverse(entry.history_at()));
        pending.sort_by_key(|entry| std::cmp::Reverse(entry.history_at()));

        let mut days: Vec<LogbookDay> = Vec::new();
        for entry in entries {
            let Some(timestamp) = entry.history_at() else {
                continue;
            };
            let date = LocalDate::today_in_timezone(timestamp, "UTC");
            match days.last_mut() {
                Some(day) if day.date == date => day.entries.push(entry),
                _ => days.push(LogbookDay {
                    date,
                    entries: vec![entry],
                }),
            }
        }
        days.sort_by_key(|day| std::cmp::Reverse(day.date));
        Self {
            policy,
            pending,
            days,
        }
    }
}

/// Logbook service: policy management, explicit logging, and history reads.
pub struct LogbookService {
    dispatcher: Arc<CommandDispatcher>,
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
    store: Arc<dyn LoggingPolicyStore>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for LogbookService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LogbookService")
    }
}

impl LogbookService {
    /// Creates the service.
    #[must_use]
    pub fn new(
        dispatcher: Arc<CommandDispatcher>,
        todos: Arc<TodoService>,
        projects: Arc<ProjectService>,
        store: Arc<dyn LoggingPolicyStore>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            dispatcher,
            todos,
            projects,
            store,
            clock,
        }
    }

    /// Loads the account's logging policy.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn policy(&self, account_id: AccountId) -> Result<LoggingPolicy, CommandError> {
        self.store.load_policy(account_id).await
    }

    /// Persists the account's logging policy.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn set_policy(
        &self,
        account_id: AccountId,
        policy: LoggingPolicy,
    ) -> Result<(), CommandError> {
        self.store.save_policy(account_id, policy).await
    }

    /// Logs a terminal to-do.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn log_todo(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::LogTodo { id }).await
    }

    /// Logs a terminal project.
    ///
    /// # Errors
    ///
    /// Propagates command failures.
    pub async fn log_project(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatch(account_id, Command::LogProject { id }).await
    }

    /// Loads the Logbook history under the account policy.
    ///
    /// # Errors
    ///
    /// Propagates storage failures.
    pub async fn agenda(&self, account_id: AccountId) -> Result<LogbookAgenda, CommandError> {
        let policy = self.policy(account_id).await?;
        let todos = self
            .todos
            .list(account_id, TodoScope::Logbook { logged_only: false })
            .await?;
        let projects: Vec<ProjectSummary> = self
            .projects
            .list(account_id)
            .await?
            .into_iter()
            .filter(|project| project.status.is_terminal())
            .collect();
        Ok(LogbookAgenda::from_entries(policy, todos, projects))
    }

    async fn dispatch(
        &self,
        account_id: AccountId,
        command: Command,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatcher
            .execute(CommandEnvelope::new(account_id, command, self.clock.now()))
            .await
    }
}
