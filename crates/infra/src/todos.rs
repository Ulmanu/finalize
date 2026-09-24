//! PostgreSQL read store for to-dos.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::todos::{TodoReadStore, TodoScope, TodoSummary};
use momentum_domain::model::{ContextRef, Deadline, Reminder, Schedule, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use serde_json::Value;
use sqlx::{PgPool, Row};

/// PostgreSQL implementation of [`TodoReadStore`].
#[derive(Debug, Clone)]
pub struct PgTodoReadStore {
    pool: PgPool,
}

impl PgTodoReadStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn storage(error: sqlx::Error) -> CommandError {
    CommandError::Storage(error.to_string())
}

fn scope_filter(scope: TodoScope) -> &'static str {
    match scope {
        TodoScope::Inbox => {
            "e.status = 'open' and coalesce(p.parent_kind, 'inbox') = 'inbox' \
             and (e.schedule_kind = 'anytime' or (e.schedule_kind = 'date' and e.schedule_date <= $2))"
        }
        TodoScope::AllOpen => "e.status = 'open'",
        TodoScope::Logbook => "e.status in ('completed', 'canceled')",
        TodoScope::Trash => "e.status = 'deleted'",
    }
}

fn summary_from_row(row: &sqlx::postgres::PgRow) -> TodoSummary {
    let schedule_kind: String = row.get("schedule_kind");
    let schedule_date: Option<chrono::NaiveDate> = row.get("schedule_date");
    let evening: bool = row.get("schedule_evening");
    let schedule = match schedule_kind.as_str() {
        "someday" => Schedule::someday(),
        "date" => {
            let date = schedule_date
                .map(LocalDate::from_naive)
                .unwrap_or_else(|| LocalDate::from_ymd(1970, 1, 1).expect("epoch"));
            Schedule::with_state(momentum_domain::model::ScheduleState::On(date), evening)
                .unwrap_or_else(|_| Schedule::on(date))
        }
        _ => Schedule::anytime(),
    };

    let context = match (
        row.get::<Option<String>, _>("parent_kind").as_deref(),
        row.get::<Option<uuid::Uuid>, _>("parent_id"),
    ) {
        (Some("area"), Some(id)) => ContextRef::Area(EntityId::from_uuid(id)),
        (Some("project"), Some(id)) => ContextRef::Project(EntityId::from_uuid(id)),
        (Some("heading"), Some(id)) => ContextRef::Heading(EntityId::from_uuid(id)),
        _ => ContextRef::Inbox,
    };

    TodoSummary {
        id: EntityId::from_uuid(row.get("id")),
        title: row.get("title"),
        notes: row.get("notes"),
        status: match row.get::<String, _>("status").as_str() {
            "completed" => Status::Completed,
            "canceled" => Status::Canceled,
            "deleted" => Status::Deleted,
            _ => Status::Open,
        },
        schedule,
        deadline: row
            .get::<Option<chrono::NaiveDate>, _>("deadline")
            .map(|date| Deadline::new(LocalDate::from_naive(date))),
        reminder: row
            .get::<Option<Value>, _>("reminder_rule")
            .and_then(|value| serde_json::from_value::<Reminder>(value).ok()),
        context,
        completed_at: row.get("completed_at"),
        created_at: row.get("created_at"),
        version: row.get("version"),
    }
}

#[async_trait]
impl TodoReadStore for PgTodoReadStore {
    async fn list_todos(
        &self,
        account_id: AccountId,
        scope: TodoScope,
        today: LocalDate,
    ) -> Result<Vec<TodoSummary>, CommandError> {
        let sql = format!(
            "select e.id, e.title, e.notes, e.status, e.schedule_kind, e.schedule_date, \
             e.schedule_evening, e.deadline, e.completed_at, e.created_at, e.version, \
             e.reminder_rule, p.parent_kind, p.parent_id \
             from entities e \
             left join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'todo' and {} \
             order by p.rank asc nulls last, e.created_at asc, e.id asc",
            scope_filter(scope)
        );
        let rows = sqlx::query(&sql)
            .bind(account_id.as_uuid())
            .bind(today.as_naive())
            .fetch_all(&self.pool)
            .await
            .map_err(storage)?;
        Ok(rows.iter().map(summary_from_row).collect())
    }

    async fn get_todo(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<TodoSummary>, CommandError> {
        let row = sqlx::query(
            "select e.id, e.title, e.notes, e.status, e.schedule_kind, e.schedule_date, \
             e.schedule_evening, e.deadline, e.completed_at, e.created_at, e.version, \
             e.reminder_rule, p.parent_kind, p.parent_id \
             from entities e \
             left join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.id = $2 and e.kind = 'todo'",
        )
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(storage)?;
        Ok(row.as_ref().map(summary_from_row))
    }
}
