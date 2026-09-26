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
        TodoScope::Today => {
            "e.status = 'open' and e.schedule_kind = 'date' and e.schedule_date <= $2"
        }
        TodoScope::Upcoming => {
            "e.status = 'open' and e.schedule_kind = 'date' and e.schedule_date > $2"
        }
        TodoScope::Anytime => {
            "e.status = 'open' and (e.schedule_kind = 'anytime' \
             or (e.schedule_kind = 'date' and e.schedule_date <= $2))"
        }
        TodoScope::Someday => "e.status = 'open' and e.schedule_kind = 'someday'",
        TodoScope::Deadlines => "e.status = 'open' and e.deadline is not null",
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
        checklist: Vec::new(),
        tags: Vec::new(),
        inherited_tags: Vec::new(),
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
        let (extra_join, order) = match scope {
            TodoScope::Deadlines => ("", "e.deadline asc, e.created_at asc, e.id asc"),
            TodoScope::Today => (
                " left join placements rp on rp.entity_id = e.id \
                 and rp.scope = case when e.schedule_evening then 'today_evening' else 'today' end",
                "e.schedule_evening asc, rp.rank asc nulls last, e.created_at asc, e.id asc",
            ),
            TodoScope::Upcoming => (
                "",
                "e.schedule_date asc, e.schedule_evening asc, e.created_at asc, e.id asc",
            ),
            _ => ("", "p.rank asc nulls last, e.created_at asc, e.id asc"),
        };
        let sql = format!(
            "select e.id, e.title, e.notes, e.status, e.schedule_kind, e.schedule_date, \
             e.schedule_evening, e.deadline, e.completed_at, e.created_at, e.version, \
             e.reminder_rule, p.parent_kind, p.parent_id \
             from entities e \
             left join placements p on p.entity_id = e.id and p.scope = 'context' \
             {extra_join} \
             where e.account_id = $1 and e.kind = 'todo' and {} \
             order by {order}",
            scope_filter(scope)
        );
        let rows = sqlx::query(&sql)
            .bind(account_id.as_uuid())
            .bind(today.as_naive())
            .fetch_all(&self.pool)
            .await
            .map_err(storage)?;
        let mut summaries: Vec<TodoSummary> = rows.iter().map(summary_from_row).collect();
        attach_checklists(&self.pool, account_id, &mut summaries).await?;
        attach_tags(&self.pool, account_id, &mut summaries).await?;
        Ok(summaries)
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
        let mut summaries: Vec<TodoSummary> = row.iter().map(summary_from_row).collect();
        attach_checklists(&self.pool, account_id, &mut summaries).await?;
        attach_tags(&self.pool, account_id, &mut summaries).await?;
        Ok(summaries.into_iter().next())
    }
}

async fn attach_tags(
    pool: &PgPool,
    account_id: AccountId,
    summaries: &mut [TodoSummary],
) -> Result<(), CommandError> {
    if summaries.is_empty() {
        return Ok(());
    }
    let ids: Vec<uuid::Uuid> = summaries
        .iter()
        .map(|summary| summary.id.as_uuid())
        .collect();
    let rows = sqlx::query(
        "select entity_id, tag_id from tag_assignments \
         where account_id = $1 and entity_id = any($2)",
    )
    .bind(account_id.as_uuid())
    .bind(&ids)
    .fetch_all(pool)
    .await
    .map_err(storage)?;
    for summary in summaries.iter_mut() {
        summary.tags = rows
            .iter()
            .filter(|row| row.get::<uuid::Uuid, _>("entity_id") == summary.id.as_uuid())
            .map(|row| EntityId::from_uuid(row.get("tag_id")))
            .collect();
        let inherited: Vec<uuid::Uuid> = sqlx::query_scalar(
            "select distinct ta.tag_id from tag_assignments ta \
             where ta.account_id = $1 and ta.entity_id in ( \
               select case \
                 when d.parent_kind in ('project', 'area') then d.parent_id \
                 when d.parent_kind = 'heading' then ( \
                   select h.parent_id from placements h \
                   where h.entity_id = d.parent_id and h.scope = 'context') \
                 else null end \
               from placements d where d.entity_id = $2 and d.scope = 'context' \
               union \
               select pa.parent_id from placements pa \
               where pa.scope = 'context' and pa.parent_kind = 'area' and pa.entity_id in ( \
                 select case \
                   when d2.parent_kind = 'project' then d2.parent_id \
                   when d2.parent_kind = 'heading' then ( \
                     select h2.parent_id from placements h2 \
                     where h2.entity_id = d2.parent_id and h2.scope = 'context') \
                   else null end \
                 from placements d2 where d2.entity_id = $2 and d2.scope = 'context'))",
        )
        .bind(account_id.as_uuid())
        .bind(summary.id.as_uuid())
        .fetch_all(pool)
        .await
        .map_err(storage)?;
        summary.inherited_tags = inherited.into_iter().map(EntityId::from_uuid).collect();
    }
    Ok(())
}

async fn attach_checklists(
    pool: &PgPool,
    account_id: AccountId,
    summaries: &mut [TodoSummary],
) -> Result<(), CommandError> {
    let _ = pool;
    {
        if summaries.is_empty() {
            return Ok(());
        }
        let ids: Vec<uuid::Uuid> = summaries
            .iter()
            .map(|summary| summary.id.as_uuid())
            .collect();
        let rows = sqlx::query(
            "select id, entity_id, title, completed from checklist_items \
             where account_id = $1 and entity_id = any($2) order by rank asc, id asc",
        )
        .bind(account_id.as_uuid())
        .bind(&ids)
        .fetch_all(pool)
        .await
        .map_err(storage)?;
        for summary in summaries.iter_mut() {
            summary.checklist = rows
                .iter()
                .filter(|row| row.get::<uuid::Uuid, _>("entity_id") == summary.id.as_uuid())
                .filter_map(|row| {
                    let title = row.get::<String, _>("title");
                    let title = momentum_domain::model::Title::parse(&title).ok()?;
                    Some(momentum_app::todos::ChecklistItemSummary {
                        id: momentum_domain::ChecklistItemId::from_uuid(row.get("id")),
                        title: title.as_str().to_string(),
                        completed: row.get("completed"),
                    })
                })
                .collect();
        }
    }
    Ok(())
}
