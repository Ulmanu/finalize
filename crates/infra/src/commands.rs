//! PostgreSQL unit of work for command execution.
//!
//! One command maps to one SQL transaction covering the entity row, its context placement,
//! the change-log record, and the per-account sync sequence.

use async_trait::async_trait;
use momentum_app::commands::{
    ChangeRecord, CommandError, DomainEvent, UnitOfWork, UnitOfWorkFactory,
};
use momentum_domain::model::{
    ContextRef, Deadline, Reminder, ReminderState, Schedule, Status, Title, Todo,
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
            tags: std::collections::BTreeSet::new(),
            checklist: Vec::new(),
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
