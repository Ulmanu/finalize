//! Documented command URLs for automation: add, add-project, update, show, and search.
//!
//! These URLs are the automation/bookmark boundary (the same role `/capture` plays for browser
//! capture). Parameters are parsed strictly: entity ids must parse, dates must parse, list names
//! come from a fixed allow-list, and entity references are scoped to the authenticated account by
//! the application services, so unauthorized references resolve to not-found redirects. Show and
//! search URLs are non-mutating and stay bookmarkable.

use axum::extract::{Query, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Form, Router};
use momentum_app::commands::ScheduleInput;
use momentum_app::session::SessionState;
use momentum_domain::time::LocalDate;
use momentum_domain::EntityId;
use serde::Deserialize;

use crate::server::AppState;
use crate::todo_routes::{apply_quick_entry, QuickEntryForm};

/// Command URL routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/add", get(add_get).post(add_post))
        .route("/add-project", get(add_project))
        .route("/update", get(update))
        .route("/show", get(show))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

async fn add_get(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Query(form): Query<QuickEntryForm>,
) -> Response {
    apply_quick_entry(&state, &session, form).await
}

async fn add_post(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<QuickEntryForm>,
) -> Response {
    apply_quick_entry(&state, &session, form).await
}

#[derive(Debug, Deserialize)]
struct AddProjectQuery {
    title: String,
    #[serde(default)]
    area: Option<String>,
    #[serde(default)]
    notes: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn add_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Query(form): Query<AddProjectQuery>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.projects.clone() else {
        return redirect("/projects?error=add");
    };
    let target = form
        .return_to
        .as_deref()
        .filter(|value| value.starts_with('/'))
        .unwrap_or("/projects")
        .to_string();
    let area = match form.area.as_deref() {
        Some(value) if !value.is_empty() => match value.parse::<EntityId>() {
            Ok(area) => Some(area),
            Err(_) => return redirect(&format!("{target}?error=add")),
        },
        _ => None,
    };
    let outcome = match service.create(account.account_id, &form.title, area).await {
        Ok(outcome) => outcome,
        Err(_) => return redirect(&format!("{target}?error=add")),
    };
    let Some(id) = outcome.entity_id else {
        return redirect(&format!("{target}?error=add"));
    };
    if let Some(notes) = form.notes.filter(|notes| !notes.trim().is_empty()) {
        if service
            .set_notes(account.account_id, id, &notes)
            .await
            .is_err()
        {
            let _ = service
                .delete(
                    account.account_id,
                    id,
                    momentum_app::commands::ProjectDeletionPolicy::DetachChildren,
                )
                .await;
            return redirect(&format!("{target}?error=add"));
        }
    }
    redirect(&format!("/projects/{id}"))
}

#[derive(Debug, Deserialize)]
struct UpdateQuery {
    todo: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    notes: Option<String>,
    #[serde(default)]
    when: Option<String>,
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    deadline: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn update(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Query(form): Query<UpdateQuery>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.todos.clone() else {
        return redirect("/login");
    };
    let Ok(todo) = form.todo.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let target = form
        .return_to
        .as_deref()
        .filter(|value| value.starts_with('/'))
        .unwrap_or("/inbox")
        .to_string();
    let today = service.local_today();
    let schedule = match form.when.as_deref() {
        None => None,
        Some("today") => Some(ScheduleInput::On(today)),
        Some("evening") => Some(ScheduleInput::ThisEvening(today)),
        Some("tomorrow") => Some(ScheduleInput::On(today.next_day())),
        Some("someday") => Some(ScheduleInput::Someday),
        Some("anytime") => Some(ScheduleInput::Anytime),
        Some("date") => match form
            .date
            .as_deref()
            .and_then(|value| value.parse::<LocalDate>().ok())
        {
            Some(date) => Some(ScheduleInput::On(date)),
            None => return redirect(&format!("{target}?error=update")),
        },
        Some(_) => return redirect(&format!("{target}?error=update")),
    };
    let deadline = match form.deadline.as_deref() {
        None => None,
        Some("") => Some(None),
        Some(value) => match value.parse::<LocalDate>() {
            Ok(date) => Some(Some(date)),
            Err(_) => return redirect(&format!("{target}?error=update")),
        },
    };
    if let Some(title) = form.title.filter(|title| !title.trim().is_empty()) {
        if service
            .set_title(account.account_id, todo, &title)
            .await
            .is_err()
        {
            return redirect(&format!("{target}?error=update"));
        }
    }
    if let Some(notes) = form.notes {
        if service
            .set_notes(account.account_id, todo, &notes)
            .await
            .is_err()
        {
            return redirect(&format!("{target}?error=update"));
        }
    }
    if let Some(schedule) = schedule {
        if service
            .schedule(account.account_id, todo, schedule)
            .await
            .is_err()
        {
            return redirect(&format!("{target}?error=update"));
        }
    }
    if let Some(deadline) = deadline {
        if service
            .set_deadline(account.account_id, todo, deadline)
            .await
            .is_err()
        {
            return redirect(&format!("{target}?error=update"));
        }
    }
    redirect(&format!("{target}?saved=1"))
}

#[derive(Debug, Deserialize)]
struct ShowQuery {
    #[serde(default)]
    todo: Option<String>,
    #[serde(default)]
    project: Option<String>,
    #[serde(default)]
    area: Option<String>,
    #[serde(default)]
    list: Option<String>,
}

const LIST_PATHS: &[(&str, &str)] = &[
    ("inbox", "/inbox"),
    ("today", "/today"),
    ("tomorrow", "/tomorrow"),
    ("upcoming", "/upcoming"),
    ("anytime", "/anytime"),
    ("someday", "/someday"),
    ("deadlines", "/deadlines"),
    ("logbook", "/logbook"),
    ("repeating", "/repeating"),
    ("projects", "/projects"),
    ("areas", "/areas"),
    ("trash", "/trash"),
    ("settings", "/settings"),
];

async fn show(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Query(form): Query<ShowQuery>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let provided = [
        form.todo.is_some(),
        form.project.is_some(),
        form.area.is_some(),
        form.list.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    if provided != 1 {
        return redirect("/inbox?error=show");
    }
    if let Some(todo) = form.todo {
        let Ok(id) = todo.parse::<EntityId>() else {
            return redirect("/inbox?error=notfound");
        };
        let Some(service) = state.todos.clone() else {
            return redirect("/login");
        };
        return match service.get(account.account_id, id).await {
            Ok(Some(_)) => redirect(&format!("/todos/{id}")),
            _ => redirect("/inbox?error=notfound"),
        };
    }
    if let Some(project) = form.project {
        let Ok(id) = project.parse::<EntityId>() else {
            return redirect("/projects?error=notfound");
        };
        let Some(service) = state.projects.clone() else {
            return redirect("/login");
        };
        return match service.get(account.account_id, id).await {
            Ok(Some(_)) => redirect(&format!("/projects/{id}")),
            _ => redirect("/projects?error=notfound"),
        };
    }
    if let Some(area) = form.area {
        let Ok(id) = area.parse::<EntityId>() else {
            return redirect("/areas?error=notfound");
        };
        let Some(service) = state.areas.clone() else {
            return redirect("/login");
        };
        return match service.get(account.account_id, id).await {
            Ok(Some(_)) => redirect(&format!("/areas/{id}")),
            _ => redirect("/areas?error=notfound"),
        };
    }
    let list = form.list.unwrap_or_default().to_lowercase();
    match LIST_PATHS.iter().find(|(name, _)| *name == list) {
        Some((_, path)) => redirect(path),
        None => redirect("/inbox?error=show"),
    }
}
