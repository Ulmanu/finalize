//! Notes replacement routes backed by the authoritative note commands.

use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Form, Router};
use momentum_app::session::SessionState;
use momentum_domain::{notes, EntityId};
use serde::Deserialize;

use crate::server::AppState;

/// Notes command routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/todos/{todo_id}/notes/replace", post(replace_todo_notes))
        .route(
            "/projects/{project_id}/notes/replace",
            post(replace_project_notes),
        )
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

#[derive(Debug, Deserialize)]
struct ReplaceForm {
    find: String,
    #[serde(default)]
    replace: String,
    #[serde(default)]
    return_to: Option<String>,
}

impl ReplaceForm {
    fn target(&self, fallback: &str) -> String {
        self.return_to
            .clone()
            .filter(|value| value.starts_with('/'))
            .unwrap_or_else(|| fallback.to_string())
    }
}

async fn replace_todo_notes(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<ReplaceForm>,
) -> Response {
    let Some(service) = state.todos.clone() else {
        return redirect("/login");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Ok(id) = todo_id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let target = form.target(&format!("/todos/{id}"));
    if form.find.is_empty() {
        return redirect(&format!("{target}?error=notes"));
    }
    let Ok(Some(todo)) = service.get(account.account_id, id).await else {
        return redirect(&format!("{target}?error=notes"));
    };
    let (replaced, count) = notes::replace_all(&todo.notes, &form.find, &form.replace);
    if count == 0 {
        return redirect(&format!("{target}?error=notes"));
    }
    match service.set_notes(account.account_id, id, &replaced).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=notes")),
    }
}

async fn replace_project_notes(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<ReplaceForm>,
) -> Response {
    let Some(service) = state.projects.clone() else {
        return redirect("/login");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Ok(id) = project_id.parse::<EntityId>() else {
        return redirect("/projects?error=notfound");
    };
    let target = form.target(&format!("/projects/{id}"));
    if form.find.is_empty() {
        return redirect(&format!("{target}?error=notes"));
    }
    let Ok(Some(project)) = service.get(account.account_id, id).await else {
        return redirect(&format!("{target}?error=notes"));
    };
    let (replaced, count) = notes::replace_all(&project.notes, &form.find, &form.replace);
    if count == 0 {
        return redirect(&format!("{target}?error=notes"));
    }
    match service.set_notes(account.account_id, id, &replaced).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=notes")),
    }
}
