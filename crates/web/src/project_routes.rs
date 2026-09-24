//! Command-backed HTTP routes for the project lifecycle.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Form, Router};
use momentum_app::commands::{ProjectDeletionPolicy, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::session::SessionState;
use momentum_domain::model::ProjectCompletionPolicy;
use momentum_domain::{AccountId, EntityId};
use serde::Deserialize;

use crate::server::AppState;

/// Project command routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project))
        .route("/projects/{project_id}/rename", post(rename_project))
        .route("/projects/{project_id}/notes", post(set_notes))
        .route("/projects/{project_id}/complete", post(complete_project))
        .route("/projects/{project_id}/cancel", post(cancel_project))
        .route("/projects/{project_id}/reopen", post(reopen_project))
        .route("/projects/{project_id}/schedule", post(schedule_project))
        .route("/projects/{project_id}/delete", post(delete_project))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

fn service_and_account(
    state: &AppState,
    session: &SessionState,
) -> Option<(Arc<ProjectService>, AccountId)> {
    let service = state.projects.clone()?;
    let account = session.account()?;
    Some((service, account.account_id))
}

#[derive(Debug, Deserialize)]
struct CreateProjectForm {
    title: String,
    #[serde(default)]
    area: Option<String>,
}

async fn create_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<CreateProjectForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let area = form
        .area
        .as_deref()
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<EntityId>().ok());
    match service.create(account_id, &form.title, area).await {
        Ok(_) => redirect("/projects"),
        Err(_) => redirect("/projects?error=create"),
    }
}

#[derive(Debug, Deserialize)]
struct RenameForm {
    title: String,
}

async fn rename_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<RenameForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = project_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    match service.rename(account_id, id, &form.title).await {
        Ok(_) => redirect(&format!("/projects/{id}")),
        Err(_) => redirect(&format!("/projects/{id}?error=rename")),
    }
}

#[derive(Debug, Deserialize)]
struct NotesForm {
    notes: String,
}

async fn set_notes(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<NotesForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = project_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    match service.set_notes(account_id, id, &form.notes).await {
        Ok(_) => redirect(&format!("/projects/{id}")),
        Err(_) => redirect(&format!("/projects/{id}?error=notes")),
    }
}

#[derive(Debug, Deserialize)]
struct CompleteForm {
    #[serde(default)]
    policy: Option<String>,
}

async fn complete_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<CompleteForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = project_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let policy = match form.policy.as_deref() {
        Some("cascade") => ProjectCompletionPolicy::CompleteOpenChildren,
        _ => ProjectCompletionPolicy::RejectWhileOpen,
    };
    match service.complete(account_id, id, policy).await {
        Ok(_) => redirect(&format!("/projects/{id}")),
        Err(_) => redirect(&format!("/projects/{id}?error=open-children")),
    }
}

async fn cancel_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
) -> Response {
    lifecycle(&state, &session, &project_id, "cancel").await
}

async fn reopen_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
) -> Response {
    lifecycle(&state, &session, &project_id, "reopen").await
}

#[derive(Debug, Deserialize)]
struct ScheduleForm {
    when: String,
    #[serde(default)]
    date: Option<String>,
}

async fn schedule_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<ScheduleForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = project_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let today = momentum_domain::time::LocalDate::today_from_utc(chrono::Utc::now(), 0)
        .unwrap_or_else(|_| momentum_domain::time::LocalDate::from_ymd(1970, 1, 1).expect("epoch"));
    let schedule = match form.when.as_str() {
        "today" => Some(ScheduleInput::On(today)),
        "evening" => Some(ScheduleInput::ThisEvening(today)),
        "tomorrow" => Some(ScheduleInput::On(today.next_day())),
        "someday" => Some(ScheduleInput::Someday),
        "anytime" => Some(ScheduleInput::Anytime),
        "date" => form
            .date
            .as_deref()
            .and_then(|date| date.parse::<momentum_domain::time::LocalDate>().ok())
            .map(ScheduleInput::On),
        _ => None,
    };
    let Some(schedule) = schedule else {
        return redirect(&format!("/projects/{id}?error=schedule"));
    };
    match service.schedule(account_id, id, schedule).await {
        Ok(_) => redirect(&format!("/projects/{id}")),
        Err(_) => redirect(&format!("/projects/{id}?error=schedule")),
    }
}

#[derive(Debug, Deserialize)]
struct DeleteForm {
    #[serde(default)]
    policy: Option<String>,
}

async fn delete_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<DeleteForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = project_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let policy = match form.policy.as_deref() {
        Some("trash") => ProjectDeletionPolicy::TrashChildren,
        _ => ProjectDeletionPolicy::DetachChildren,
    };
    match service.delete(account_id, id, policy).await {
        Ok(_) => redirect("/projects"),
        Err(_) => redirect(&format!("/projects/{id}?error=delete")),
    }
}

async fn lifecycle(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    action: &str,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let result = match action {
        "cancel" => service.cancel(account_id, id).await,
        "reopen" => service.reopen(account_id, id).await,
        _ => return redirect("/projects?error=unknown"),
    };
    match result {
        Ok(_) => redirect(&format!("/projects/{id}")),
        Err(_) => redirect(&format!("/projects/{id}?error={action}")),
    }
}
