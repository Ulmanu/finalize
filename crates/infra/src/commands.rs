//! PostgreSQL unit of work for command execution.
//!
//! One command maps to one SQL transaction covering the entity row, its context placement,
//! the change-log record, and the per-account sync sequence.

use async_trait::async_trait;
use momentum_app::commands::{
    ChangeRecord, CommandError, DomainEvent, UnitOfWork, UnitOfWorkFactory,
};
use momentum_domain::model::{
    Area, ContextRef, Deadline, EntityMeta, Project, Reminder, ReminderState, Schedule, Status,
    Title, Todo,
};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use serde_json::Value;
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

/// Creates PostgreSQL unit-of-work transactions.
#[derive(Debug, Clone)]
pub struct PgUnitOfWorkFactory {
    pool: PgPool,
}

impl PgUnitOfWorkFactory {
    /// Creates the factory over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn storage(error: sqlx::Error) -> CommandError {
    CommandError::Storage(error.to_string())
}

#[async_trait]
impl UnitOfWorkFactory for PgUnitOfWorkFactory {
    async fn begin(&self, _account_id: AccountId) -> Result<Box<dyn UnitOfWork>, CommandError> {
        let tx = self.pool.begin().await.map_err(storage)?;
        Ok(Box::new(PgUnitOfWork { tx: Some(tx) }))
    }
}

struct PgUnitOfWork {
    tx: Option<Transaction<'static, Postgres>>,
}

impl PgUnitOfWork {
    fn tx(&mut self) -> Result<&mut Transaction<'static, Postgres>, CommandError> {
        self.tx
            .as_mut()
            .ok_or_else(|| CommandError::Storage("transaction already finished".to_string()))
    }
}

fn status_from_str(value: &str) -> Status {
    match value {
        "completed" => Status::Completed,
        "canceled" => Status::Canceled,
        "deleted" => Status::Deleted,
        _ => Status::Open,
    }
}

fn reminder_state_from_str(value: &str) -> ReminderState {
    match value {
        "delivered" => ReminderState::Delivered,
        "snoozed" => ReminderState::Snoozed,
        "canceled" => ReminderState::Canceled,
        _ => ReminderState::Scheduled,
    }
}

fn context_from_parts(parent_kind: &str, parent_id: Option<Uuid>) -> ContextRef {
    match (parent_kind, parent_id) {
        ("area", Some(id)) => ContextRef::Area(EntityId::from_uuid(id)),
        ("project", Some(id)) => ContextRef::Project(EntityId::from_uuid(id)),
        ("heading", Some(id)) => ContextRef::Heading(EntityId::from_uuid(id)),
        _ => ContextRef::Inbox,
    }
}

fn context_parts(context: ContextRef) -> (&'static str, Option<Uuid>) {
    match context {
        ContextRef::Inbox => ("inbox", None),
        ContextRef::Area(id) => ("area", Some(id.as_uuid())),
        ContextRef::Project(id) => ("project", Some(id.as_uuid())),
        ContextRef::Heading(id) => ("heading", Some(id.as_uuid())),
    }
}

async fn next_context_rank(
    tx: &mut Transaction<'static, Postgres>,
    account_id: AccountId,
    context: ContextRef,
) -> Result<i64, CommandError> {
    let (parent_kind, parent_id) = context_parts(context);
    let current: Option<i64> = sqlx::query_scalar(
        "select max(rank::bigint) from placements \
         where account_id = $1 and scope = 'context' and parent_kind = $2 \
         and coalesce(parent_id, '00000000-0000-0000-0000-000000000000'::uuid) = \
             coalesce($3, '00000000-0000-0000-0000-000000000000'::uuid)",
    )
    .bind(account_id.as_uuid())
    .bind(parent_kind)
    .bind(parent_id)
    .fetch_one(&mut **tx)
    .await
    .map_err(storage)?;
    Ok(current.unwrap_or(0) + momentum_app::ordering::STEP)
}

async fn load_context(
    tx: &mut Transaction<'static, Postgres>,
    entity_id: EntityId,
) -> Result<ContextRef, CommandError> {
    let row = sqlx::query(
        "select parent_kind, parent_id from placements \
         where entity_id = $1 and scope = 'context' limit 1",
    )
    .bind(entity_id.as_uuid())
    .fetch_optional(&mut **tx)
    .await
    .map_err(storage)?;
    Ok(row.map_or(ContextRef::Inbox, |row| {
        context_from_parts(
            row.get::<String, _>("parent_kind").as_str(),
            row.get::<Option<Uuid>, _>("parent_id"),
        )
    }))
}

async fn load_checklist(
    tx: &mut Transaction<'static, Postgres>,
    entity_id: EntityId,
) -> Result<Vec<momentum_domain::model::ChecklistItem>, CommandError> {
    let rows = sqlx::query(
        "select id, title, completed from checklist_items \
         where entity_id = $1 order by rank asc, id asc",
    )
    .bind(entity_id.as_uuid())
    .fetch_all(&mut **tx)
    .await
    .map_err(storage)?;
    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let title = Title::parse(row.get::<String, _>("title").as_str())
            .map_err(|error| CommandError::Storage(error.to_string()))?;
        items.push(momentum_domain::model::ChecklistItem {
            id: momentum_domain::ChecklistItemId::from_uuid(row.get("id")),
            title,
            completed: row.get("completed"),
        });
    }
    Ok(items)
}

async fn load_tags(
    tx: &mut Transaction<'static, Postgres>,
    entity_id: EntityId,
) -> Result<std::collections::BTreeSet<EntityId>, CommandError> {
    let rows: Vec<Uuid> =
        sqlx::query_scalar("select tag_id from tag_assignments where entity_id = $1")
            .bind(entity_id.as_uuid())
            .fetch_all(&mut **tx)
            .await
            .map_err(storage)?;
    Ok(rows.into_iter().map(EntityId::from_uuid).collect())
}

async fn upsert_placement(
    tx: &mut Transaction<'static, Postgres>,
    account_id: AccountId,
    entity_id: EntityId,
    context: ContextRef,
) -> Result<(), CommandError> {
    let existing: Option<Uuid> =
        sqlx::query_scalar("select id from placements where entity_id = $1 and scope = 'context'")
            .bind(entity_id.as_uuid())
            .fetch_optional(&mut **tx)
            .await
            .map_err(storage)?;

    let (parent_kind, parent_id) = context_parts(context);
    match existing {
        Some(placement_id) => {
            sqlx::query(
                "update placements set parent_kind = $2, parent_id = $3, updated_at = now(), \
                 version = version + 1 where id = $1",
            )
            .bind(placement_id)
            .bind(parent_kind)
            .bind(parent_id)
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        }
        None => {
            let rank = next_context_rank(tx, account_id, context).await?;
            sqlx::query(
                "insert into placements \
                 (id, account_id, entity_id, scope, parent_kind, parent_id, rank) \
                 values ($1, $2, $3, 'context', $4, $5, $6)",
            )
            .bind(Uuid::now_v7())
            .bind(account_id.as_uuid())
            .bind(entity_id.as_uuid())
            .bind(parent_kind)
            .bind(parent_id)
            .bind(format!("{rank:019}"))
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        }
    }
    Ok(())
}

#[async_trait]
impl UnitOfWork for PgUnitOfWork {
    async fn load_todo(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Todo>, CommandError> {
        let tx = self.tx()?;
        let row = sqlx::query(
            "select id, title, notes, status, schedule_kind, schedule_date, schedule_evening, \
             deadline, completed_at, canceled_at, logged_at, deleted_at, created_at, updated_at, \
             version, reminder_rule \
             from entities where account_id = $1 and id = $2 and kind = 'todo'",
        )
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&mut **tx)
        .await
        .map_err(storage)?;

        let Some(row) = row else {
            return Ok(None);
        };

        let context = load_context(tx, id).await?;
        let checklist = load_checklist(tx, id).await?;
        let tags = load_tags(tx, id).await?;
        let title = Title::parse(row.get::<String, _>("title").as_str())
            .map_err(|error| CommandError::Storage(error.to_string()))?;

        let schedule_kind: String = row.get("schedule_kind");
        let schedule_date: Option<chrono::NaiveDate> = row.get("schedule_date");
        let evening: bool = row.get("schedule_evening");
        let schedule = match schedule_kind.as_str() {
            "someday" => Schedule::someday(),
            "date" => {
                let date = schedule_date
                    .map(LocalDate::from_naive)
                    .unwrap_or_else(|| LocalDate::from_ymd(1970, 1, 1).expect("epoch is valid"));
                Schedule::with_state(momentum_domain::model::ScheduleState::On(date), evening)
                    .unwrap_or_else(|_| Schedule::on(date))
            }
            _ => Schedule::anytime(),
        };

        let reminder = row
            .get::<Option<Value>, _>("reminder_rule")
            .and_then(|value| serde_json::from_value::<Reminder>(value).ok())
            .map(|mut reminder| {
                reminder.state = reminder_state_from_str(reminder.state.as_str());
                reminder
            });

        Ok(Some(Todo {
            meta: momentum_domain::model::EntityMeta {
                id: EntityId::from_uuid(row.get("id")),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                version: row.get("version"),
                deleted_at: row.get("deleted_at"),
            },
            title,
            notes: row.get("notes"),
            status: status_from_str(row.get::<String, _>("status").as_str()),
            schedule,
            deadline: row
                .get::<Option<chrono::NaiveDate>, _>("deadline")
                .map(|date| Deadline::new(LocalDate::from_naive(date))),
            reminder,
            context,
            tags,
            checklist,
            completed_at: row.get("completed_at"),
            canceled_at: row.get("canceled_at"),
            logged_at: row.get("logged_at"),
        }))
    }

    async fn save_todo(
        &mut self,
        account_id: AccountId,
        todo: &Todo,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        let schedule_kind = match todo.schedule.state() {
            momentum_domain::model::ScheduleState::Anytime => "anytime",
            momentum_domain::model::ScheduleState::Someday => "someday",
            momentum_domain::model::ScheduleState::On(_) => "date",
        };
        let schedule_date = todo.schedule.date().map(|date| date.as_naive());
        let reminder = todo
            .reminder
            .as_ref()
            .and_then(|reminder| serde_json::to_value(reminder).ok());

        match expected_version {
            None => {
                sqlx::query(
                    "insert into entities \
                     (id, account_id, kind, title, notes, status, schedule_kind, schedule_date, \
                      schedule_evening, deadline, completed_at, canceled_at, logged_at, deleted_at, \
                      reminder_rule, created_at, updated_at, version) \
                     values ($1, $2, 'todo', $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, \
                             $14, $15, $16, $17)",
                )
                .bind(todo.id().as_uuid())
                .bind(account_id.as_uuid())
                .bind(todo.title.as_str())
                .bind(&todo.notes)
                .bind(todo.status.as_str())
                .bind(schedule_kind)
                .bind(schedule_date)
                .bind(todo.schedule.is_evening())
                .bind(todo.deadline.map(|deadline| deadline.date.as_naive()))
                .bind(todo.completed_at)
                .bind(todo.canceled_at)
                .bind(todo.logged_at)
                .bind(todo.meta.deleted_at)
                .bind(reminder)
                .bind(todo.meta.created_at)
                .bind(todo.meta.updated_at)
                .bind(todo.meta.version)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
            }
            Some(expected) => {
                let result = sqlx::query(
                    "update entities set title = $3, notes = $4, status = $5, schedule_kind = $6, \
                     schedule_date = $7, schedule_evening = $8, deadline = $9, completed_at = $10, \
                     canceled_at = $11, logged_at = $12, deleted_at = $13, reminder_rule = $14, \
                     updated_at = $15, version = version + 1 \
                     where account_id = $1 and id = $2 and version = $16",
                )
                .bind(account_id.as_uuid())
                .bind(todo.id().as_uuid())
                .bind(todo.title.as_str())
                .bind(&todo.notes)
                .bind(todo.status.as_str())
                .bind(schedule_kind)
                .bind(schedule_date)
                .bind(todo.schedule.is_evening())
                .bind(todo.deadline.map(|deadline| deadline.date.as_naive()))
                .bind(todo.completed_at)
                .bind(todo.canceled_at)
                .bind(todo.logged_at)
                .bind(todo.meta.deleted_at)
                .bind(reminder)
                .bind(todo.meta.updated_at)
                .bind(expected)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;

                if result.rows_affected() == 0 {
                    let actual: Option<i64> = sqlx::query_scalar(
                        "select version from entities where account_id = $1 and id = $2",
                    )
                    .bind(account_id.as_uuid())
                    .bind(todo.id().as_uuid())
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(storage)?;
                    return match actual {
                        Some(actual) => Err(CommandError::Conflict { expected, actual }),
                        None => Err(CommandError::NotFound),
                    };
                }
            }
        }

        upsert_placement(tx, account_id, todo.id(), todo.context).await?;
        sqlx::query("delete from checklist_items where entity_id = $1")
            .bind(todo.id().as_uuid())
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        for (index, item) in todo.checklist.iter().enumerate() {
            let rank = momentum_app::ordering::Rank::new(
                (i64::try_from(index).unwrap_or(0) + 1) * momentum_app::ordering::STEP,
            )
            .map_err(|error| CommandError::Storage(error.to_string()))?;
            sqlx::query(
                "insert into checklist_items \
                 (id, account_id, entity_id, title, completed, rank, created_at, updated_at, \
                  version) values ($1, $2, $3, $4, $5, $6, now(), now(), 1)",
            )
            .bind(item.id.as_uuid())
            .bind(account_id.as_uuid())
            .bind(todo.id().as_uuid())
            .bind(item.title.as_str())
            .bind(item.completed)
            .bind(rank.encode())
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        }
        sqlx::query("delete from tag_assignments where entity_id = $1")
            .bind(todo.id().as_uuid())
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        for tag in &todo.tags {
            sqlx::query(
                "insert into tag_assignments (account_id, entity_id, tag_id) values ($1, $2, $3) \
                 on conflict (entity_id, tag_id) do nothing",
            )
            .bind(account_id.as_uuid())
            .bind(todo.id().as_uuid())
            .bind(tag.as_uuid())
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        }
        Ok(())
    }

    async fn load_project(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Project>, CommandError> {
        let tx = self.tx()?;
        let row = sqlx::query(
            "select id, title, notes, status, schedule_kind, schedule_date, schedule_evening, \
             deadline, completed_at, canceled_at, logged_at, deleted_at, created_at, updated_at, \
             version, reminder_rule \
             from entities where account_id = $1 and id = $2 and kind = 'project'",
        )
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&mut **tx)
        .await
        .map_err(storage)?;

        let Some(row) = row else {
            return Ok(None);
        };
        let title = Title::parse(row.get::<String, _>("title").as_str())
            .map_err(|error| CommandError::Storage(error.to_string()))?;
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
        let reminder = row
            .get::<Option<Value>, _>("reminder_rule")
            .and_then(|value| serde_json::from_value::<Reminder>(value).ok());

        Ok(Some(Project {
            meta: EntityMeta {
                id: EntityId::from_uuid(row.get("id")),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                version: row.get("version"),
                deleted_at: row.get("deleted_at"),
            },
            title,
            notes: row.get("notes"),
            status: status_from_str(row.get::<String, _>("status").as_str()),
            schedule,
            deadline: row
                .get::<Option<chrono::NaiveDate>, _>("deadline")
                .map(|date| Deadline::new(LocalDate::from_naive(date))),
            reminder,
            area: None,
            tags: std::collections::BTreeSet::new(),
            completed_at: row.get("completed_at"),
            canceled_at: row.get("canceled_at"),
            logged_at: row.get("logged_at"),
        }))
    }

    async fn save_project(
        &mut self,
        account_id: AccountId,
        project: &Project,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        let schedule_kind = match project.schedule.state() {
            momentum_domain::model::ScheduleState::Anytime => "anytime",
            momentum_domain::model::ScheduleState::Someday => "someday",
            momentum_domain::model::ScheduleState::On(_) => "date",
        };
        let schedule_date = project.schedule.date().map(|date| date.as_naive());
        let reminder = project
            .reminder
            .as_ref()
            .and_then(|reminder| serde_json::to_value(reminder).ok());

        match expected_version {
            None => {
                sqlx::query(
                    "insert into entities \
                     (id, account_id, kind, title, notes, status, schedule_kind, schedule_date, \
                      schedule_evening, deadline, completed_at, canceled_at, logged_at, deleted_at, \
                      reminder_rule, created_at, updated_at, version) \
                     values ($1, $2, 'project', $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, \
                             $14, $15, $16, $17)",
                )
                .bind(project.id().as_uuid())
                .bind(account_id.as_uuid())
                .bind(project.title.as_str())
                .bind(&project.notes)
                .bind(project.status.as_str())
                .bind(schedule_kind)
                .bind(schedule_date)
                .bind(project.schedule.is_evening())
                .bind(project.deadline.map(|deadline| deadline.date.as_naive()))
                .bind(project.completed_at)
                .bind(project.canceled_at)
                .bind(project.logged_at)
                .bind(project.meta.deleted_at)
                .bind(reminder)
                .bind(project.meta.created_at)
                .bind(project.meta.updated_at)
                .bind(project.meta.version)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
            }
            Some(expected) => {
                let result = sqlx::query(
                    "update entities set title = $3, notes = $4, status = $5, schedule_kind = $6, \
                     schedule_date = $7, schedule_evening = $8, deadline = $9, completed_at = $10, \
                     canceled_at = $11, logged_at = $12, deleted_at = $13, reminder_rule = $14, \
                     updated_at = $15, version = version + 1 \
                     where account_id = $1 and id = $2 and version = $16 and kind = 'project'",
                )
                .bind(account_id.as_uuid())
                .bind(project.id().as_uuid())
                .bind(project.title.as_str())
                .bind(&project.notes)
                .bind(project.status.as_str())
                .bind(schedule_kind)
                .bind(schedule_date)
                .bind(project.schedule.is_evening())
                .bind(project.deadline.map(|deadline| deadline.date.as_naive()))
                .bind(project.completed_at)
                .bind(project.canceled_at)
                .bind(project.logged_at)
                .bind(project.meta.deleted_at)
                .bind(reminder)
                .bind(project.meta.updated_at)
                .bind(expected)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
                if result.rows_affected() == 0 {
                    let actual: Option<i64> = sqlx::query_scalar(
                        "select version from entities where account_id = $1 and id = $2",
                    )
                    .bind(account_id.as_uuid())
                    .bind(project.id().as_uuid())
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(storage)?;
                    return match actual {
                        Some(actual) => Err(CommandError::Conflict { expected, actual }),
                        None => Err(CommandError::NotFound),
                    };
                }
            }
        }
        Ok(())
    }

    async fn load_heading(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<momentum_domain::model::Heading>, CommandError> {
        let tx = self.tx()?;
        let row = sqlx::query(
            "select e.id, e.title, e.archived, e.created_at, e.updated_at, e.version, \
             e.deleted_at, p.parent_id \
             from entities e \
             left join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.id = $2 and e.kind = 'heading'",
        )
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&mut **tx)
        .await
        .map_err(storage)?;
        let Some(row) = row else {
            return Ok(None);
        };
        let title = Title::parse(row.get::<String, _>("title").as_str())
            .map_err(|error| CommandError::Storage(error.to_string()))?;
        let project = row
            .get::<Option<Uuid>, _>("parent_id")
            .map(EntityId::from_uuid)
            .unwrap_or_else(EntityId::new);
        Ok(Some(momentum_domain::model::Heading {
            meta: EntityMeta {
                id: EntityId::from_uuid(row.get("id")),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                version: row.get("version"),
                deleted_at: row.get("deleted_at"),
            },
            title,
            project,
            archived: row.get("archived"),
        }))
    }

    async fn save_heading(
        &mut self,
        account_id: AccountId,
        heading: &momentum_domain::model::Heading,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        match expected_version {
            None => {
                sqlx::query(
                    "insert into entities \
                     (id, account_id, kind, title, notes, status, archived, created_at, updated_at, \
                      version) \
                     values ($1, $2, 'heading', $3, '', 'open', $4, $5, $6, $7)",
                )
                .bind(heading.id().as_uuid())
                .bind(account_id.as_uuid())
                .bind(heading.title.as_str())
                .bind(heading.archived)
                .bind(heading.meta.created_at)
                .bind(heading.meta.updated_at)
                .bind(heading.meta.version)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
            }
            Some(expected) => {
                let updated = sqlx::query(
                    "update entities set title = $3, archived = $4, updated_at = $5, \
                     version = $6 where account_id = $1 and id = $2 and version = $7",
                )
                .bind(account_id.as_uuid())
                .bind(heading.id().as_uuid())
                .bind(heading.title.as_str())
                .bind(heading.archived)
                .bind(heading.meta.updated_at)
                .bind(heading.meta.version)
                .bind(expected)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
                if updated.rows_affected() == 0 {
                    let actual: Option<i64> = sqlx::query_scalar(
                        "select version from entities where account_id = $1 and id = $2",
                    )
                    .bind(account_id.as_uuid())
                    .bind(heading.id().as_uuid())
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(storage)?;
                    return match actual {
                        Some(actual) => Err(CommandError::Conflict { expected, actual }),
                        None => Err(CommandError::NotFound),
                    };
                }
            }
        }
        upsert_placement(
            tx,
            account_id,
            heading.id(),
            ContextRef::Project(heading.project),
        )
        .await?;
        Ok(())
    }

    async fn list_project_children(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<momentum_app::commands::ProjectChildren, CommandError> {
        let tx = self.tx()?;
        let todo_ids: Vec<Uuid> = sqlx::query_scalar(
            "select e.id from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'todo' and e.status <> 'deleted' \
             and p.parent_kind = 'project' and p.parent_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .fetch_all(&mut **tx)
        .await
        .map_err(storage)?;
        let heading_ids: Vec<Uuid> = sqlx::query_scalar(
            "select e.id from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'heading' \
             and p.parent_kind = 'project' and p.parent_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .fetch_all(&mut **tx)
        .await
        .map_err(storage)?;
        Ok(momentum_app::commands::ProjectChildren {
            todo_ids: todo_ids.into_iter().map(EntityId::from_uuid).collect(),
            heading_ids: heading_ids.into_iter().map(EntityId::from_uuid).collect(),
        })
    }

    async fn load_tag(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<momentum_domain::model::Tag>, CommandError> {
        let tx = self.tx()?;
        let row = sqlx::query(
            "select id, title, tag_parent, created_at, updated_at, version, deleted_at \
             from entities where account_id = $1 and id = $2 and kind = 'tag'",
        )
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&mut **tx)
        .await
        .map_err(storage)?;
        let Some(row) = row else {
            return Ok(None);
        };
        let title = Title::parse(row.get::<String, _>("title").as_str())
            .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok(Some(momentum_domain::model::Tag {
            meta: EntityMeta {
                id: EntityId::from_uuid(row.get("id")),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                version: row.get("version"),
                deleted_at: row.get("deleted_at"),
            },
            title,
            parent: row
                .get::<Option<Uuid>, _>("tag_parent")
                .map(EntityId::from_uuid),
        }))
    }

    async fn save_tag(
        &mut self,
        account_id: AccountId,
        tag: &momentum_domain::model::Tag,
        position: i64,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        match expected_version {
            None => {
                sqlx::query(
                    "insert into entities \
                     (id, account_id, kind, title, notes, status, tag_parent, tag_position, \
                      created_at, updated_at, version) \
                     values ($1, $2, 'tag', $3, '', 'open', $4, $5, $6, $7, $8)",
                )
                .bind(tag.id().as_uuid())
                .bind(account_id.as_uuid())
                .bind(tag.title.as_str())
                .bind(tag.parent.map(|parent| parent.as_uuid()))
                .bind(position)
                .bind(tag.meta.created_at)
                .bind(tag.meta.updated_at)
                .bind(tag.meta.version)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
            }
            Some(expected) => {
                let updated = sqlx::query(
                    "update entities set title = $3, tag_parent = $4, tag_position = $5, \
                     updated_at = $6, version = $7 \
                     where account_id = $1 and id = $2 and version = $8",
                )
                .bind(account_id.as_uuid())
                .bind(tag.id().as_uuid())
                .bind(tag.title.as_str())
                .bind(tag.parent.map(|parent| parent.as_uuid()))
                .bind(position)
                .bind(tag.meta.updated_at)
                .bind(tag.meta.version)
                .bind(expected)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
                if updated.rows_affected() == 0 {
                    let actual: Option<i64> = sqlx::query_scalar(
                        "select version from entities where account_id = $1 and id = $2",
                    )
                    .bind(account_id.as_uuid())
                    .bind(tag.id().as_uuid())
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(storage)?;
                    return match actual {
                        Some(actual) => Err(CommandError::Conflict { expected, actual }),
                        None => Err(CommandError::NotFound),
                    };
                }
            }
        }
        Ok(())
    }

    async fn list_tags(
        &mut self,
        account_id: AccountId,
    ) -> Result<Vec<(momentum_domain::model::Tag, i64)>, CommandError> {
        let tx = self.tx()?;
        let rows = sqlx::query(
            "select id, title, tag_parent, tag_position, created_at, updated_at, version, \
             deleted_at from entities where account_id = $1 and kind = 'tag' \
             order by tag_position asc, created_at asc, id asc",
        )
        .bind(account_id.as_uuid())
        .fetch_all(&mut **tx)
        .await
        .map_err(storage)?;
        let mut tags = Vec::with_capacity(rows.len());
        for row in rows {
            let title = Title::parse(row.get::<String, _>("title").as_str())
                .map_err(|error| CommandError::Storage(error.to_string()))?;
            tags.push((
                momentum_domain::model::Tag {
                    meta: EntityMeta {
                        id: EntityId::from_uuid(row.get("id")),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                        version: row.get("version"),
                        deleted_at: row.get("deleted_at"),
                    },
                    title,
                    parent: row
                        .get::<Option<Uuid>, _>("tag_parent")
                        .map(EntityId::from_uuid),
                },
                row.get("tag_position"),
            ));
        }
        Ok(tags)
    }

    async fn delete_tag(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        sqlx::query("delete from tag_assignments where account_id = $1 and tag_id = $2")
            .bind(account_id.as_uuid())
            .bind(id.as_uuid())
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        sqlx::query("delete from entities where account_id = $1 and id = $2 and kind = 'tag'")
            .bind(account_id.as_uuid())
            .bind(id.as_uuid())
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        Ok(())
    }

    async fn load_area(
        &mut self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<Area>, CommandError> {
        let tx = self.tx()?;
        let row = sqlx::query(
            "select id, title, deleted_at, created_at, updated_at, version \
             from entities where account_id = $1 and id = $2 and kind = 'area'",
        )
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&mut **tx)
        .await
        .map_err(storage)?;

        let Some(row) = row else {
            return Ok(None);
        };
        let title = Title::parse(row.get::<String, _>("title").as_str())
            .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok(Some(Area {
            meta: EntityMeta {
                id: EntityId::from_uuid(row.get("id")),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                version: row.get("version"),
                deleted_at: row.get("deleted_at"),
            },
            title,
            tags: std::collections::BTreeSet::new(),
        }))
    }

    async fn save_area(
        &mut self,
        account_id: AccountId,
        area: &Area,
        expected_version: Option<i64>,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        let status = if area.meta.deleted_at.is_some() {
            "deleted"
        } else {
            "open"
        };
        match expected_version {
            None => {
                sqlx::query(
                    "insert into entities \
                     (id, account_id, kind, title, notes, status, schedule_kind, deleted_at, \
                      created_at, updated_at, version) \
                     values ($1, $2, 'area', $3, '', $4, 'anytime', $5, $6, $7, $8)",
                )
                .bind(area.id().as_uuid())
                .bind(account_id.as_uuid())
                .bind(area.title.as_str())
                .bind(status)
                .bind(area.meta.deleted_at)
                .bind(area.meta.created_at)
                .bind(area.meta.updated_at)
                .bind(area.meta.version)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
            }
            Some(expected) => {
                let result = sqlx::query(
                    "update entities set title = $3, status = $4, deleted_at = $5, \
                     updated_at = $6, version = version + 1 \
                     where account_id = $1 and id = $2 and version = $7 and kind = 'area'",
                )
                .bind(account_id.as_uuid())
                .bind(area.id().as_uuid())
                .bind(area.title.as_str())
                .bind(status)
                .bind(area.meta.deleted_at)
                .bind(area.meta.updated_at)
                .bind(expected)
                .execute(&mut **tx)
                .await
                .map_err(storage)?;
                if result.rows_affected() == 0 {
                    let actual: Option<i64> = sqlx::query_scalar(
                        "select version from entities where account_id = $1 and id = $2",
                    )
                    .bind(account_id.as_uuid())
                    .bind(area.id().as_uuid())
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(storage)?;
                    return match actual {
                        Some(actual) => Err(CommandError::Conflict { expected, actual }),
                        None => Err(CommandError::NotFound),
                    };
                }
            }
        }
        Ok(())
    }

    async fn list_area_children(
        &mut self,
        account_id: AccountId,
        area_id: EntityId,
    ) -> Result<momentum_app::commands::AreaChildren, CommandError> {
        let tx = self.tx()?;
        let todo_ids: Vec<Uuid> = sqlx::query_scalar(
            "select e.id from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'todo' and e.status <> 'deleted' \
             and p.parent_kind = 'area' and p.parent_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(area_id.as_uuid())
        .fetch_all(&mut **tx)
        .await
        .map_err(storage)?;

        let project_ids: Vec<Uuid> = sqlx::query_scalar(
            "select e.id from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'project' and e.status = 'open' \
             and p.parent_kind = 'area' and p.parent_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(area_id.as_uuid())
        .fetch_all(&mut **tx)
        .await
        .map_err(storage)?;

        Ok(momentum_app::commands::AreaChildren {
            todo_ids: todo_ids.into_iter().map(EntityId::from_uuid).collect(),
            project_ids: project_ids.into_iter().map(EntityId::from_uuid).collect(),
        })
    }

    async fn project_area(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<Option<Option<EntityId>>, CommandError> {
        let tx = self.tx()?;
        let exists: Option<Uuid> = sqlx::query_scalar(
            "select id from entities where account_id = $1 and id = $2 and kind = 'project'",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .fetch_optional(&mut **tx)
        .await
        .map_err(storage)?;
        if exists.is_none() {
            return Ok(None);
        }

        let area_id: Option<Uuid> = sqlx::query_scalar(
            "select parent_id from placements \
             where account_id = $1 and entity_id = $2 and scope = 'context' \
             and parent_kind = 'area' limit 1",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .fetch_optional(&mut **tx)
        .await
        .map_err(storage)?;
        Ok(Some(area_id.map(EntityId::from_uuid)))
    }

    async fn set_project_area(
        &mut self,
        account_id: AccountId,
        project_id: EntityId,
        area_id: Option<EntityId>,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        sqlx::query(
            "delete from placements \
             where account_id = $1 and entity_id = $2 and scope = 'context' and parent_kind = 'area'",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .execute(&mut **tx)
        .await
        .map_err(storage)?;

        if let Some(area_id) = area_id {
            let rank = next_context_rank(tx, account_id, ContextRef::Area(area_id)).await?;
            sqlx::query(
                "insert into placements \
                 (id, account_id, entity_id, scope, parent_kind, parent_id, rank) \
                 values ($1, $2, $3, 'context', 'area', $4, $5)",
            )
            .bind(Uuid::now_v7())
            .bind(account_id.as_uuid())
            .bind(project_id.as_uuid())
            .bind(area_id.as_uuid())
            .bind(format!("{rank:019}"))
            .execute(&mut **tx)
            .await
            .map_err(storage)?;
        }
        Ok(())
    }

    async fn load_context_order(
        &mut self,
        account_id: AccountId,
        context: ContextRef,
    ) -> Result<Vec<(EntityId, momentum_app::ordering::Rank)>, CommandError> {
        let tx = self.tx()?;
        let (parent_kind, parent_id) = context_parts(context);
        let rows = sqlx::query(
            "select entity_id, rank from placements \
             where account_id = $1 and scope = 'context' and parent_kind = $2 \
             and coalesce(parent_id, '00000000-0000-0000-0000-000000000000'::uuid) = \
                 coalesce($3, '00000000-0000-0000-0000-000000000000'::uuid) \
             order by rank asc",
        )
        .bind(account_id.as_uuid())
        .bind(parent_kind)
        .bind(parent_id)
        .fetch_all(&mut **tx)
        .await
        .map_err(storage)?;

        let mut order = Vec::with_capacity(rows.len());
        for row in rows {
            let rank_text: String = row.get("rank");
            if let Ok(rank) = momentum_app::ordering::Rank::parse(&rank_text) {
                order.push((EntityId::from_uuid(row.get("entity_id")), rank));
            }
        }
        Ok(order)
    }

    async fn set_context_rank(
        &mut self,
        account_id: AccountId,
        entity_id: EntityId,
        rank: momentum_app::ordering::Rank,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        sqlx::query(
            "update placements set rank = $3, updated_at = now(), version = version + 1 \
             where account_id = $1 and entity_id = $2 and scope = 'context'",
        )
        .bind(account_id.as_uuid())
        .bind(entity_id.as_uuid())
        .bind(rank.encode())
        .execute(&mut **tx)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn load_projection_order(
        &mut self,
        account_id: AccountId,
        scope: momentum_app::ordering::ProjectionScope,
    ) -> Result<Vec<(EntityId, momentum_app::ordering::Rank)>, CommandError> {
        let tx = self.tx()?;
        let rows = sqlx::query(
            "select entity_id, rank from placements \
             where account_id = $1 and scope = $2 order by rank asc",
        )
        .bind(account_id.as_uuid())
        .bind(scope.as_str())
        .fetch_all(&mut **tx)
        .await
        .map_err(storage)?;
        let mut order = Vec::with_capacity(rows.len());
        for row in rows {
            let rank_text: String = row.get("rank");
            if let Ok(rank) = momentum_app::ordering::Rank::parse(&rank_text) {
                order.push((EntityId::from_uuid(row.get("entity_id")), rank));
            }
        }
        Ok(order)
    }

    async fn set_projection_rank(
        &mut self,
        account_id: AccountId,
        scope: momentum_app::ordering::ProjectionScope,
        entity_id: EntityId,
        rank: momentum_app::ordering::Rank,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        sqlx::query(
            "insert into placements \
             (id, account_id, entity_id, scope, parent_kind, parent_id, rank, created_at, \
              updated_at, version) \
             values ($1, $2, $3, $4, 'inbox', null, $5, now(), now(), 1) \
             on conflict (entity_id, scope) do update \
             set rank = excluded.rank, updated_at = now(), version = placements.version + 1",
        )
        .bind(Uuid::now_v7())
        .bind(account_id.as_uuid())
        .bind(entity_id.as_uuid())
        .bind(scope.as_str())
        .bind(rank.encode())
        .execute(&mut **tx)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn clear_today_ranks(
        &mut self,
        account_id: AccountId,
        entity_id: EntityId,
    ) -> Result<(), CommandError> {
        let tx = self.tx()?;
        sqlx::query(
            "delete from placements \
             where account_id = $1 and entity_id = $2 and scope in ('today', 'today_evening')",
        )
        .bind(account_id.as_uuid())
        .bind(entity_id.as_uuid())
        .execute(&mut **tx)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn change_by_operation(
        &mut self,
        account_id: AccountId,
        operation_id: Uuid,
    ) -> Result<Option<ChangeRecord>, CommandError> {
        let tx = self.tx()?;
        let row = sqlx::query(
            "select sequence, operation_id, entity_id, payload, created_at \
             from change_log where account_id = $1 and operation_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(operation_id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(storage)?;

        Ok(row.and_then(|row| {
            let events: Vec<DomainEvent> =
                serde_json::from_value(row.get::<Value, _>("payload")).ok()?;
            Some(ChangeRecord {
                operation_id: row.get("operation_id"),
                sequence: row.get("sequence"),
                entity_id: row
                    .get::<Option<Uuid>, _>("entity_id")
                    .map(EntityId::from_uuid),
                events,
                created_at: row.get("created_at"),
            })
        }))
    }

    async fn append_change(
        &mut self,
        account_id: AccountId,
        record: &ChangeRecord,
    ) -> Result<ChangeRecord, CommandError> {
        let tx = self.tx()?;
        let sequence: i64 = sqlx::query_scalar(
            "insert into sync_metadata (account_id, sequence) values ($1, 1) \
             on conflict (account_id) do update \
             set sequence = sync_metadata.sequence + 1, updated_at = now() \
             returning sequence",
        )
        .bind(account_id.as_uuid())
        .fetch_one(&mut **tx)
        .await
        .map_err(storage)?;

        let kind = record
            .events
            .first()
            .map_or("unknown", |event| match event {
                DomainEvent::TodoCreated { .. } => "todo_created",
                DomainEvent::TodoDuplicated { .. } => "todo_duplicated",
                DomainEvent::TodoReordered { .. } => "todo_reordered",
                DomainEvent::ProjectCreated { .. } => "project_created",
                DomainEvent::ProjectChanged { .. } => "project_changed",
                DomainEvent::ProjectDeleted { .. } => "project_deleted",
                DomainEvent::HeadingCreated { .. } => "heading_created",
                DomainEvent::HeadingChanged { .. } => "heading_changed",
                DomainEvent::HeadingReordered { .. } => "heading_reordered",
                DomainEvent::HeadingArchived { .. } => "heading_archived",
                DomainEvent::HeadingRestored { .. } => "heading_restored",
                DomainEvent::AreaCreated { .. } => "area_created",
                DomainEvent::AreaRenamed { .. } => "area_renamed",
                DomainEvent::AreaDeleted { .. } => "area_deleted",
                DomainEvent::TodoTitleChanged { .. } => "todo_title_changed",
                DomainEvent::TodoNotesChanged { .. } => "todo_notes_changed",
                DomainEvent::TodoCompleted { .. } => "todo_completed",
                DomainEvent::TodoCanceled { .. } => "todo_canceled",
                DomainEvent::TodoReopened { .. } => "todo_reopened",
                DomainEvent::TodoDeleted { .. } => "todo_deleted",
                DomainEvent::TodoRestored { .. } => "todo_restored",
                DomainEvent::TodoMoved { .. } => "todo_moved",
                DomainEvent::TodoScheduled { .. } => "todo_scheduled",
                DomainEvent::TodoDeadlineChanged { .. } => "todo_deadline_changed",
                DomainEvent::TodoReminderChanged { .. } => "todo_reminder_changed",
                DomainEvent::ChecklistChanged { .. } => "checklist_changed",
                DomainEvent::TodayReordered { .. } => "today_reordered",
                DomainEvent::TagCreated { .. } => "tag_created",
                DomainEvent::TagChanged { .. } => "tag_changed",
                DomainEvent::TagDeleted { .. } => "tag_deleted",
                DomainEvent::TagAssigned { .. } => "tag_assigned",
                DomainEvent::TagUnassigned { .. } => "tag_unassigned",
            });

        let payload = serde_json::to_value(&record.events)
            .map_err(|error| CommandError::Storage(error.to_string()))?;

        sqlx::query(
            "insert into change_log \
             (account_id, sequence, operation_id, entity_id, entity_kind, kind, payload, created_at) \
             values ($1, $2, $3, $4, 'todo', $5, $6, $7)",
        )
        .bind(account_id.as_uuid())
        .bind(sequence)
        .bind(record.operation_id)
        .bind(record.entity_id.map(|id| id.as_uuid()))
        .bind(kind)
        .bind(payload)
        .bind(record.created_at)
        .execute(&mut **tx)
        .await
        .map_err(storage)?;

        Ok(ChangeRecord {
            sequence,
            ..record.clone()
        })
    }

    async fn commit(mut self: Box<Self>) -> Result<(), CommandError> {
        let tx = self
            .tx
            .take()
            .ok_or_else(|| CommandError::Storage("transaction already finished".to_string()))?;
        tx.commit().await.map_err(storage)
    }

    async fn rollback(mut self: Box<Self>) -> Result<(), CommandError> {
        let Some(tx) = self.tx.take() else {
            return Ok(());
        };
        tx.rollback().await.map_err(storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_round_trips_through_parts() {
        for context in [
            ContextRef::Inbox,
            ContextRef::Area(EntityId::new()),
            ContextRef::Project(EntityId::new()),
            ContextRef::Heading(EntityId::new()),
        ] {
            let (kind, id) = context_parts(context);
            assert_eq!(context_from_parts(kind, id), context);
        }
    }

    #[test]
    fn status_and_reminder_state_mapping() {
        assert_eq!(status_from_str("completed"), Status::Completed);
        assert_eq!(status_from_str("canceled"), Status::Canceled);
        assert_eq!(status_from_str("deleted"), Status::Deleted);
        assert_eq!(status_from_str("open"), Status::Open);
        assert_eq!(
            reminder_state_from_str("delivered"),
            ReminderState::Delivered
        );
        assert_eq!(reminder_state_from_str("snoozed"), ReminderState::Snoozed);
        assert_eq!(reminder_state_from_str("canceled"), ReminderState::Canceled);
        assert_eq!(
            reminder_state_from_str("scheduled"),
            ReminderState::Scheduled
        );
    }
}

#[cfg(test)]
mod live_tests {
    use super::*;
    use momentum_app::auth::Clock;
    use momentum_app::commands::{Command, CommandDispatcher, CommandEnvelope, ScheduleInput};
    use momentum_domain::model::{Title, Todo};
    use std::sync::Arc;

    #[derive(Debug, Clone, Copy)]
    struct FixedClock;

    impl Clock for FixedClock {
        fn now(&self) -> chrono::DateTime<chrono::Utc> {
            chrono::Utc::now()
        }
    }

    /// Live PostgreSQL checks are conditional on `DATABASE_URL`; without a database the test
    /// records the skip and returns successfully.
    #[tokio::test]
    async fn command_transaction_round_trip() {
        let Ok(url) = std::env::var("DATABASE_URL") else {
            eprintln!(
                "SKIP: DATABASE_URL is not set; live PostgreSQL command-layer test was not run"
            );
            return;
        };
        let Ok(database) = crate::db::Database::connect(&url).await else {
            eprintln!("SKIP: could not connect to DATABASE_URL");
            return;
        };
        database.migrate().await.expect("migrations");

        let factory = Arc::new(PgUnitOfWorkFactory::new(database.pool().clone()));
        let dispatcher = CommandDispatcher::new(factory, Arc::new(FixedClock));

        let account_id = AccountId::new();
        sqlx::query("insert into accounts (id, email, email_normalized) values ($1, $2, $3)")
            .bind(account_id.as_uuid())
            .bind(format!("{account_id}@example.com"))
            .bind(format!("{account_id}@example.com"))
            .execute(database.pool())
            .await
            .expect("account row");

        let envelope = CommandEnvelope::new(
            account_id,
            Command::CreateTodo {
                title: "Live command".to_string(),
                notes: None,
                context: ContextRef::Inbox,
                schedule: ScheduleInput::Anytime,
                deadline: None,
                reminder: None,
            },
            chrono::Utc::now(),
        );
        let outcome = dispatcher.execute(envelope.clone()).await.expect("create");
        assert_eq!(outcome.sequence, 1);

        let replay = dispatcher.execute(envelope).await.expect("replay");
        assert!(replay.replayed);

        let todo = Todo::new(Title::parse("x").expect("title"), chrono::Utc::now());
        let _ = todo;
    }
}
