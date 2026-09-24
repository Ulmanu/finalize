//! Command-backed HTTP routes for the to-do lifecycle.
//!
//! Forms post here, the handler dispatches an application command for the authenticated
//! account, and the response redirects back to the referring destination.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Form, Router};
use momentum_app::session::SessionState;
use momentum_app::todos::TodoService;
use momentum_domain::AccountId;
use momentum_domain::EntityId;
use serde::Deserialize;

use crate::server::AppState;

/// To-do command routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/todos", post(create_todo))
        .route("/todos/{todo_id}", post(update_todo))
        .route("/todos/{todo_id}/complete", post(complete_todo))
        .route("/todos/{todo_id}/cancel", post(cancel_todo))
        .route("/todos/{todo_id}/reopen", post(reopen_todo))
        .route("/todos/{todo_id}/delete", post(delete_todo))
        .route("/todos/{todo_id}/restore", post(restore_todo))
        .route("/todos/{todo_id}/duplicate", post(duplicate_todo))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

fn back(form: &RedirectForm) -> String {
    form.return_to
        .as_deref()
        .filter(|path| path.starts_with('/') && !path.starts_with("//"))
        .unwrap_or("/inbox")
        .to_string()
}

fn service_and_account(
    state: &AppState,
    session: &SessionState,
) -> Option<(Arc<TodoService>, AccountId)> {
    let service = state.todos.clone()?;
    let account = session.account()?;
    Some((service, account.account_id))
}

#[derive(Debug, Deserialize, Default)]
struct RedirectForm {
    #[serde(default)]
    return_to: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateTodoForm {
    title: String,
    #[serde(default)]
    notes: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn create_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<CreateTodoForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let target = back(&RedirectForm {
        return_to: form.return_to,
    });
    match service.create(account_id, &form.title, form.notes).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=create")),
    }
}

#[derive(Debug, Deserialize)]
struct UpdateTodoForm {
    title: String,
    #[serde(default)]
    notes: String,
}

async fn update_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
    Form(form): Form<UpdateTodoForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };

    if service
        .set_title(account_id, id, &form.title)
        .await
        .is_err()
    {
        return redirect(&format!("/todos/{id}?error=update"));
    }
    if service
        .set_notes(account_id, id, &form.notes)
        .await
        .is_err()
    {
        return redirect(&format!("/todos/{id}?error=update"));
    }
    redirect(&format!("/todos/{id}?saved=1"))
}

async fn complete_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    lifecycle(&state, &session, &id, "complete").await
}

async fn cancel_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    lifecycle(&state, &session, &id, "cancel").await
}

async fn reopen_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    lifecycle(&state, &session, &id, "reopen").await
}

async fn delete_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    lifecycle(&state, &session, &id, "delete").await
}

async fn restore_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    lifecycle(&state, &session, &id, "restore").await
}

async fn duplicate_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    lifecycle(&state, &session, &id, "duplicate").await
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
        return redirect("/inbox?error=notfound");
    };

    let result = match action {
        "complete" => service.complete(account_id, id).await,
        "cancel" => service.cancel(account_id, id).await,
        "reopen" => service.reopen(account_id, id).await,
        "delete" => service.delete(account_id, id).await,
        "restore" => service.restore(account_id, id).await,
        "duplicate" => service.duplicate(account_id, id).await,
        _ => return redirect("/inbox?error=unknown"),
    };

    match result {
        Ok(_) => {
            if action == "delete" {
                redirect("/inbox")
            } else {
                redirect(&format!("/todos/{id}"))
            }
        }
        Err(_) => redirect(&format!("/todos/{id}?error={action}")),
    }
}
