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
use momentum_app::commands::ScheduleInput;
use momentum_app::session::SessionState;
use momentum_app::todos::{QuickSchedule, TodoService};
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
        .route("/todos/{todo_id}/move-up", post(move_up))
        .route("/todos/{todo_id}/move-down", post(move_down))
        .route("/todos/{todo_id}/schedule", post(schedule_todo))
        .route("/todos/{todo_id}/move", post(move_todo))
        .route("/todos/{todo_id}/deadline", post(set_deadline))
        .route("/todos/{todo_id}/checklist", post(add_checklist_item))
        .route("/todos/{todo_id}/checklist/expand", post(expand_checklist))
        .route(
            "/todos/{todo_id}/checklist/{item_id}/toggle",
            post(toggle_checklist_item),
        )
        .route(
            "/todos/{todo_id}/checklist/{item_id}/rename",
            post(rename_checklist_item),
        )
        .route(
            "/todos/{todo_id}/checklist/{item_id}/remove",
            post(remove_checklist_item),
        )
        .route(
            "/todos/{todo_id}/checklist/{item_id}/move_up",
            post(move_checklist_item_up),
        )
        .route(
            "/todos/{todo_id}/checklist/{item_id}/move_down",
            post(move_checklist_item_down),
        )
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
    #[serde(default)]
    context_kind: Option<String>,
    #[serde(default)]
    context_id: Option<String>,
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
    let context = match form.context_kind.as_deref() {
        Some("project") => form
            .context_id
            .as_deref()
            .and_then(|value| value.parse::<EntityId>().ok())
            .map_or(
                momentum_domain::model::ContextRef::Inbox,
                momentum_domain::model::ContextRef::Project,
            ),
        Some("area") => form
            .context_id
            .as_deref()
            .and_then(|value| value.parse::<EntityId>().ok())
            .map_or(
                momentum_domain::model::ContextRef::Inbox,
                momentum_domain::model::ContextRef::Area,
            ),
        Some("heading") => form
            .context_id
            .as_deref()
            .and_then(|value| value.parse::<EntityId>().ok())
            .map_or(
                momentum_domain::model::ContextRef::Inbox,
                momentum_domain::model::ContextRef::Heading,
            ),
        _ => momentum_domain::model::ContextRef::Inbox,
    };
    match service
        .create_in(account_id, &form.title, form.notes, context)
        .await
    {
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

async fn move_up(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
    Form(form): Form<RedirectForm>,
) -> Response {
    reorder(&state, &session, &id, true, &back(&form)).await
}

async fn move_down(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
    Form(form): Form<RedirectForm>,
) -> Response {
    reorder(&state, &session, &id, false, &back(&form)).await
}

async fn reorder(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    up: bool,
    target: &str,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let result = if up {
        service.reorder_up(account_id, id).await
    } else {
        service.reorder_down(account_id, id).await
    };
    match result {
        Ok(_) => redirect(target),
        Err(_) => redirect(&format!("{target}?error=reorder")),
    }
}

#[derive(Debug, Deserialize)]
struct ScheduleForm {
    when: String,
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn schedule_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
    Form(form): Form<ScheduleForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let target = back(&RedirectForm {
        return_to: form.return_to,
    });
    let Ok(id) = id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let today = service.local_today();

    let result = match form.when.as_str() {
        "today" => {
            service
                .schedule_quick(account_id, id, QuickSchedule::Today, today)
                .await
        }
        "evening" => {
            service
                .schedule_quick(account_id, id, QuickSchedule::ThisEvening, today)
                .await
        }
        "tomorrow" => {
            service
                .schedule_quick(account_id, id, QuickSchedule::Tomorrow, today)
                .await
        }
        "someday" => {
            service
                .schedule_quick(account_id, id, QuickSchedule::Someday, today)
                .await
        }
        "anytime" => {
            service
                .schedule_quick(account_id, id, QuickSchedule::Anytime, today)
                .await
        }
        "date" => {
            let parsed = form
                .date
                .as_deref()
                .and_then(|date| date.parse::<momentum_domain::time::LocalDate>().ok());
            match parsed {
                Some(date) => {
                    service
                        .schedule(account_id, id, ScheduleInput::On(date))
                        .await
                }
                None => return redirect(&format!("{target}?error=schedule")),
            }
        }
        _ => return redirect(&format!("{target}?error=schedule")),
    };

    match result {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=schedule")),
    }
}

#[derive(Debug, Deserialize)]
struct MoveForm {
    context_kind: String,
    #[serde(default)]
    context_id: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn move_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
    Form(form): Form<MoveForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let target = back(&RedirectForm {
        return_to: form.return_to,
    });
    let Ok(id) = id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let parsed_id = form
        .context_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<EntityId>().ok());

    let context = match form.context_kind.as_str() {
        "inbox" => momentum_domain::model::ContextRef::Inbox,
        "project" => parsed_id.map_or_else(
            || momentum_domain::model::ContextRef::Inbox,
            momentum_domain::model::ContextRef::Project,
        ),
        "area" => parsed_id.map_or_else(
            || momentum_domain::model::ContextRef::Inbox,
            momentum_domain::model::ContextRef::Area,
        ),
        "heading" => parsed_id.map_or_else(
            || momentum_domain::model::ContextRef::Inbox,
            momentum_domain::model::ContextRef::Heading,
        ),
        _ => return redirect(&format!("{target}?error=move")),
    };

    match service.move_to(account_id, id, context).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=move")),
    }
}

#[derive(Debug, Deserialize)]
struct DeadlineForm {
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn set_deadline(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<DeadlineForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = todo_id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let target = form
        .return_to
        .clone()
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| format!("/todos/{id}"));
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
struct ChecklistAddForm {
    title: String,
    #[serde(default)]
    return_to: Option<String>,
}

fn todo_target(form: &ChecklistAddForm, todo: EntityId) -> String {
    form.return_to
        .clone()
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| format!("/todos/{todo}"))
}

async fn add_checklist_item(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<ChecklistAddForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(todo) = todo_id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let target = todo_target(&form, todo);
    match service
        .add_checklist_item(account_id, todo, &form.title)
        .await
    {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=checklist")),
    }
}

#[derive(Debug, Deserialize)]
struct ChecklistExpandForm {
    text: String,
    #[serde(default)]
    return_to: Option<String>,
}

async fn expand_checklist(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<ChecklistExpandForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(todo) = todo_id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let target = form
        .return_to
        .clone()
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| format!("/todos/{todo}"));
    match service.expand_checklist(account_id, todo, &form.text).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=checklist")),
    }
}

#[derive(Debug, Deserialize)]
struct ChecklistItemForm {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    completed: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

fn item_ids(todo: &str, item: &str) -> Option<(EntityId, momentum_domain::ChecklistItemId)> {
    Some((todo.parse().ok()?, item.parse().ok()?))
}

fn item_target(form: &ChecklistItemForm, todo: EntityId) -> String {
    form.return_to
        .clone()
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| format!("/todos/{todo}"))
}

async fn toggle_checklist_item(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path((todo_id, item_id)): Path<(String, String)>,
    Form(form): Form<ChecklistItemForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Some((todo, item)) = item_ids(&todo_id, &item_id) else {
        return redirect("/inbox?error=notfound");
    };
    let target = item_target(&form, todo);
    let completed = form.completed.as_deref() == Some("1");
    match service
        .set_checklist_item_completed(account_id, todo, item, completed)
        .await
    {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=checklist")),
    }
}

async fn rename_checklist_item(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path((todo_id, item_id)): Path<(String, String)>,
    Form(form): Form<ChecklistItemForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Some((todo, item)) = item_ids(&todo_id, &item_id) else {
        return redirect("/inbox?error=notfound");
    };
    let target = item_target(&form, todo);
    let Some(title) = form.title else {
        return redirect(&format!("{target}?error=checklist"));
    };
    match service
        .rename_checklist_item(account_id, todo, item, &title)
        .await
    {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=checklist")),
    }
}

async fn remove_checklist_item(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path((todo_id, item_id)): Path<(String, String)>,
    Form(form): Form<ChecklistItemForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Some((todo, item)) = item_ids(&todo_id, &item_id) else {
        return redirect("/inbox?error=notfound");
    };
    let target = item_target(&form, todo);
    match service.remove_checklist_item(account_id, todo, item).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=checklist")),
    }
}

async fn move_checklist_item_up(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path((todo_id, item_id)): Path<(String, String)>,
    Form(form): Form<ChecklistItemForm>,
) -> Response {
    reorder_checklist_item(&state, &session, &todo_id, &item_id, form, true).await
}

async fn move_checklist_item_down(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path((todo_id, item_id)): Path<(String, String)>,
    Form(form): Form<ChecklistItemForm>,
) -> Response {
    reorder_checklist_item(&state, &session, &todo_id, &item_id, form, false).await
}

async fn reorder_checklist_item(
    state: &AppState,
    session: &SessionState,
    todo_id: &str,
    item_id: &str,
    form: ChecklistItemForm,
    up: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Some((todo, item)) = item_ids(todo_id, item_id) else {
        return redirect("/inbox?error=notfound");
    };
    let target = item_target(&form, todo);
    let direction = if up {
        momentum_app::commands::ReorderDirection::Up
    } else {
        momentum_app::commands::ReorderDirection::Down
    };
    match service
        .reorder_checklist_item(account_id, todo, item, direction)
        .await
    {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=checklist")),
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
