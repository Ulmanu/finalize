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
        .route(
            "/projects/{project_id}/deadline",
            post(set_project_deadline),
        )
        .route("/projects/{project_id}/delete", post(delete_project))
        .route("/projects/{project_id}/restore", post(restore_project))
        .route("/projects/{project_id}/purge", post(purge_project))
        .route("/headings", post(create_heading))
        .route("/headings/{heading_id}/rename", post(rename_heading))
        .route("/headings/{heading_id}/move_up", post(move_heading_up))
        .route("/headings/{heading_id}/move_down", post(move_heading_down))
        .route("/headings/{heading_id}/archive", post(archive_heading))
        .route("/headings/{heading_id}/restore", post(restore_heading))
        .route("/headings/{heading_id}/move_project", post(move_heading))
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
    let today = service.local_today();
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
struct ProjectDeadlineForm {
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn set_project_deadline(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<ProjectDeadlineForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = project_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let target = form
        .return_to
        .clone()
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| format!("/projects/{id}"));
    let deadline = form
        .date
        .as_deref()
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<momentum_domain::time::LocalDate>().ok());
    match service.set_deadline(account_id, id, deadline).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=deadline")),
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

#[derive(Debug, Deserialize)]
struct CreateHeadingForm {
    project_id: String,
    title: String,
    #[serde(default)]
    return_to: Option<String>,
}

async fn create_heading(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<CreateHeadingForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(project_id) = form.project_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| format!("/projects/{project_id}"));
    match service
        .create_heading(account_id, project_id, &form.title)
        .await
    {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=heading")),
    }
}

#[derive(Debug, Deserialize)]
struct HeadingForm {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn rename_heading(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(heading_id): Path<String>,
    Form(form): Form<HeadingForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = heading_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let target = form
        .return_to
        .clone()
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/projects".to_string());
    let Some(title) = form.title else {
        return redirect(&format!("{target}?error=heading"));
    };
    match service.rename_heading(account_id, id, &title).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=heading")),
    }
}

async fn move_heading_up(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(heading_id): Path<String>,
    Form(form): Form<HeadingForm>,
) -> Response {
    reorder_heading(&state, &session, &heading_id, form, true).await
}

async fn move_heading_down(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(heading_id): Path<String>,
    Form(form): Form<HeadingForm>,
) -> Response {
    reorder_heading(&state, &session, &heading_id, form, false).await
}

async fn reorder_heading(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    form: HeadingForm,
    up: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/projects".to_string());
    let direction = if up {
        momentum_app::commands::ReorderDirection::Up
    } else {
        momentum_app::commands::ReorderDirection::Down
    };
    match service.reorder_heading(account_id, id, direction).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=heading")),
    }
}

async fn archive_heading(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(heading_id): Path<String>,
    Form(form): Form<HeadingForm>,
) -> Response {
    heading_lifecycle(&state, &session, &heading_id, form, true).await
}

async fn restore_heading(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(heading_id): Path<String>,
    Form(form): Form<HeadingForm>,
) -> Response {
    heading_lifecycle(&state, &session, &heading_id, form, false).await
}

#[derive(Debug, Deserialize)]
struct MoveHeadingForm {
    project: String,
    #[serde(default)]
    return_to: Option<String>,
}

async fn move_heading(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(heading_id): Path<String>,
    Form(form): Form<MoveHeadingForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(heading) = heading_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let Ok(project) = form.project.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| format!("/projects/{project}"));
    match service.move_heading(account_id, heading, project).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=heading")),
    }
}

async fn heading_lifecycle(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    form: HeadingForm,
    archive: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/projects".to_string());
    let result = if archive {
        service.archive_heading(account_id, id).await
    } else {
        service.restore_heading(account_id, id).await
    };
    match result {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=heading")),
    }
}

async fn restore_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<HeadingForm>,
) -> Response {
    project_trash_action(&state, &session, &project_id, form, true).await
}

async fn purge_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<HeadingForm>,
) -> Response {
    project_trash_action(&state, &session, &project_id, form, false).await
}

async fn project_trash_action(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    form: HeadingForm,
    restore: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/trash?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/trash".to_string());
    let result = if restore {
        service.restore(account_id, id).await
    } else {
        service.purge(account_id, id).await
    };
    match result {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=trash")),
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
