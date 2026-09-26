//! PostgreSQL read store for projects.

use async_trait::async_trait;
use momentum_app::commands::CommandError;
use momentum_app::projects::{HeadingSummary, ProjectOrderItem, ProjectReadStore, ProjectSummary};
use momentum_domain::model::{Deadline, Schedule, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use sqlx::{PgPool, Row};

/// PostgreSQL implementation of [`ProjectReadStore`].
#[derive(Debug, Clone)]
pub struct PgProjectReadStore {
    pool: PgPool,
}

impl PgProjectReadStore {
    /// Creates the store over a pool.
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn counts(
        &self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<(usize, usize), CommandError> {
        let open: i64 = sqlx::query_scalar(
            "select count(*) from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'todo' and e.status = 'open' \
             and p.parent_kind = 'project' and p.parent_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        let completed: i64 = sqlx::query_scalar(
            "select count(*) from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'todo' and e.status in ('completed', 'canceled') \
             and p.parent_kind = 'project' and p.parent_id = $2",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok((
            usize::try_from(open).unwrap_or(0),
            usize::try_from(completed).unwrap_or(0),
        ))
    }

    async fn summarize(
        &self,
        account_id: AccountId,
        row: &sqlx::postgres::PgRow,
    ) -> Result<ProjectSummary, CommandError> {
        let id = EntityId::from_uuid(row.get("id"));
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
        let (open_count, completed_count) = self.counts(account_id, id).await?;
        let headings = self.headings(account_id, id).await?;
        let order = self.order(account_id, id).await?;
        Ok(ProjectSummary {
            id,
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
            area: row
                .get::<Option<uuid::Uuid>, _>("parent_id")
                .map(EntityId::from_uuid),
            completed_at: row.get("completed_at"),
            canceled_at: row.get("canceled_at"),
            open_count,
            completed_count,
            headings,
            order,
        })
    }

    async fn headings(
        &self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<Vec<HeadingSummary>, CommandError> {
        let rows = sqlx::query(
            "select e.id, e.title, e.archived from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and e.kind = 'heading' and p.parent_kind = 'project' \
             and p.parent_id = $2 order by p.rank asc, e.created_at asc, e.id asc",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;

        let mut headings = Vec::with_capacity(rows.len());
        for row in rows {
            let id = EntityId::from_uuid(row.get("id"));
            let todo_ids: Vec<uuid::Uuid> = sqlx::query_scalar(
                "select e.id from entities e \
                 join placements p on p.entity_id = e.id and p.scope = 'context' \
                 where e.account_id = $1 and e.kind = 'todo' and e.status <> 'deleted' \
                 and p.parent_kind = 'heading' and p.parent_id = $2 \
                 order by p.rank asc, e.created_at asc, e.id asc",
            )
            .bind(account_id.as_uuid())
            .bind(id.as_uuid())
            .fetch_all(&self.pool)
            .await
            .map_err(|error| CommandError::Storage(error.to_string()))?;
            headings.push(HeadingSummary {
                id,
                title: row.get("title"),
                archived: row.get("archived"),
                todo_ids: todo_ids.into_iter().map(EntityId::from_uuid).collect(),
            });
        }
        Ok(headings)
    }

    async fn order(
        &self,
        account_id: AccountId,
        project_id: EntityId,
    ) -> Result<Vec<ProjectOrderItem>, CommandError> {
        let rows = sqlx::query(
            "select e.id, e.kind from entities e \
             join placements p on p.entity_id = e.id and p.scope = 'context' \
             where e.account_id = $1 and p.parent_kind = 'project' and p.parent_id = $2 \
             and e.status <> 'deleted' and e.kind in ('todo', 'heading') \
             and (e.kind <> 'heading' or e.archived = false) \
             order by p.rank asc, e.created_at asc, e.id asc",
        )
        .bind(account_id.as_uuid())
        .bind(project_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;
        Ok(rows
            .into_iter()
            .map(|row| {
                let id = EntityId::from_uuid(row.get("id"));
                if row.get::<String, _>("kind") == "heading" {
                    ProjectOrderItem::Heading(id)
                } else {
                    ProjectOrderItem::Todo(id)
                }
            })
            .collect())
    }
}

const PROJECT_SELECT: &str = "select e.id, e.title, e.notes, e.status, e.schedule_kind, \
     e.schedule_date, e.schedule_evening, e.deadline, e.completed_at, e.canceled_at, p.parent_id \
     from entities e \
     left join placements p on p.entity_id = e.id and p.scope = 'context' and p.parent_kind = 'area'";

#[async_trait]
impl ProjectReadStore for PgProjectReadStore {
    async fn list_projects(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<ProjectSummary>, CommandError> {
        let rows = sqlx::query(&format!(
            "{PROJECT_SELECT} \
             where e.account_id = $1 and e.kind = 'project' and e.status <> 'deleted' \
             order by e.created_at asc, e.id asc"
        ))
        .bind(account_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;

        let mut projects = Vec::with_capacity(rows.len());
        for row in rows {
            projects.push(self.summarize(account_id, &row).await?);
        }
        Ok(projects)
    }

    async fn get_project(
        &self,
        account_id: AccountId,
        id: EntityId,
    ) -> Result<Option<ProjectSummary>, CommandError> {
        let row = sqlx::query(&format!(
            "{PROJECT_SELECT} where e.account_id = $1 and e.id = $2 and e.kind = 'project'"
        ))
        .bind(account_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| CommandError::Storage(error.to_string()))?;

        match row {
            Some(row) => Ok(Some(self.summarize(account_id, &row).await?)),
            None => Ok(None),
        }
    }
}
