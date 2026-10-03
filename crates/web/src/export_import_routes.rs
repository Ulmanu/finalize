//! Backup export and validated additive import routes.

use axum::extract::{Extension, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use momentum_app::backup::{
    apply_import, build_backup, todos_to_csv, todos_to_markdown, validate_backup, BackupArea,
    BackupDocument, BackupError, BackupProject, BackupTag, ImportSummary,
};
use momentum_app::session::SessionState;
use serde::Serialize;

use crate::server::AppState;

/// Export/import routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/export/backup.json", get(export_backup))
        .route("/export/todos.csv", get(export_csv))
        .route("/export/todos.md", get(export_markdown))
        .route("/import/backup.json", post(import_backup))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

fn download(content_type: &'static str, body: String) -> Response {
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static(content_type))],
        body,
    )
        .into_response()
}

async fn export_backup(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(todos_service) = state.todos.clone() else {
        return redirect("/login");
    };
    let areas = match &state.areas {
        Some(service) => match service.list(account.account_id).await {
            Ok(areas) => areas
                .into_iter()
                .map(|area| BackupArea {
                    id: area.id,
                    title: area.title,
                })
                .collect(),
            Err(_) => Vec::new(),
        },
        None => Vec::new(),
    };
    let projects = match &state.projects {
        Some(service) => match service.list(account.account_id).await {
            Ok(projects) => projects
                .into_iter()
                .map(|project| BackupProject {
                    id: project.id,
                    title: project.title,
                    notes: project.notes,
                })
                .collect(),
            Err(_) => Vec::new(),
        },
        None => Vec::new(),
    };
    let tags = match &state.tags {
        Some(service) => match service.list(account.account_id).await {
            Ok(tags) => tags
                .into_iter()
                .map(|tag| BackupTag {
                    id: tag.id,
                    title: tag.title,
                })
                .collect(),
            Err(_) => Vec::new(),
        },
        None => Vec::new(),
    };
    match build_backup(
        &todos_service,
        account.account_id,
        areas,
        projects,
        tags,
        chrono::Utc::now(),
    )
    .await
    {
        Ok(document) => Json(document).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn export_csv(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.todos.clone() else {
        return redirect("/login");
    };
    match service
        .list(account.account_id, momentum_app::todos::TodoScope::AllOpen)
        .await
    {
        Ok(todos) => download("text/csv; charset=utf-8", todos_to_csv(&todos)),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn export_markdown(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.todos.clone() else {
        return redirect("/login");
    };
    match service
        .list(account.account_id, momentum_app::todos::TodoScope::AllOpen)
        .await
    {
        Ok(todos) => download("text/markdown; charset=utf-8", todos_to_markdown(&todos)),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Serialize)]
struct ImportErrorBody {
    error: ImportErrorDetail,
}

#[derive(Serialize)]
struct ImportErrorDetail {
    code: &'static str,
    message: String,
}

#[derive(Serialize)]
struct ImportSummaryBody {
    imported: usize,
    skipped: Vec<SkippedBody>,
}

#[derive(Serialize)]
struct SkippedBody {
    kind: &'static str,
    index: usize,
    reason: String,
}

impl From<ImportSummary> for ImportSummaryBody {
    fn from(summary: ImportSummary) -> Self {
        Self {
            imported: summary.imported,
            skipped: summary
                .skipped
                .into_iter()
                .map(|record| SkippedBody {
                    kind: record.kind,
                    index: record.index,
                    reason: record.reason,
                })
                .collect(),
        }
    }
}

async fn import_backup(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Json(document): Json<BackupDocument>,
) -> Response {
    let Some(account) = session.account() else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ImportErrorBody {
                error: ImportErrorDetail {
                    code: "unauthorized",
                    message: "sign in to import".to_string(),
                },
            }),
        )
            .into_response();
    };
    let Some(service) = state.todos.clone() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let plan = match validate_backup(&document) {
        Ok(plan) => plan,
        Err(BackupError::UnsupportedVersion(version)) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ImportErrorBody {
                    error: ImportErrorDetail {
                        code: "unsupported_version",
                        message: format!("unsupported backup version {version}"),
                    },
                }),
            )
                .into_response();
        }
    };
    match apply_import(&service, account.account_id, plan).await {
        Ok(summary) => Json(ImportSummaryBody::from(summary)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
