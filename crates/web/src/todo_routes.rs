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
use momentum_app::bulk::{BulkAction, BulkService};
use momentum_app::commands::{ReminderInput, ScheduleInput};
use momentum_app::movement::Destination;
use momentum_app::session::SessionState;
use momentum_app::todos::{QuickSchedule, TodoService};
use momentum_domain::time::{local_datetime_to_utc, parse_local_datetime, LocalDate};
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
        .route("/todos/{todo_id}/purge", post(purge_todo))
        .route("/trash/empty", post(empty_trash))
        .route("/todos/{todo_id}/duplicate", post(duplicate_todo))
        .route("/todos/{todo_id}/move-up", post(move_up))
        .route("/todos/{todo_id}/move-down", post(move_down))
        .route("/todos/{todo_id}/position", post(move_to_position))
        .route("/todos/bulk", post(bulk_action))
        .route(
            "/todos/{todo_id}/today-position",
            post(move_today_to_position),
        )
        .route("/todos/{todo_id}/today-up", post(move_today_up))
        .route("/todos/{todo_id}/today-down", post(move_today_down))
        .route("/todos/{todo_id}/schedule", post(schedule_todo))
        .route("/todos/{todo_id}/move", post(move_todo))
        .route("/todos/{todo_id}/deadline", post(set_deadline))
        .route("/todos/{todo_id}/reminder", post(set_reminder))
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
    #[serde(default)]
    position: Option<String>,
    #[serde(default)]
    position_scope: Option<String>,
    #[serde(default)]
    schedule_kind: Option<String>,
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
    let created = match service
        .create_in(account_id, &form.title, form.notes, context)
        .await
    {
        Ok(outcome) => outcome,
        Err(_) => return redirect(&format!("{target}?error=create")),
    };
    let Some(id) = created.entity_id else {
        return redirect(&target);
    };
    if let Some(quick) = form.schedule_kind.as_deref().and_then(|kind| match kind {
        "today" => Some(QuickSchedule::Today),
        "evening" => Some(QuickSchedule::ThisEvening),
        "tomorrow" => Some(QuickSchedule::Tomorrow),
        "someday" => Some(QuickSchedule::Someday),
        "anytime" => Some(QuickSchedule::Anytime),
        _ => None,
    }) {
        let today = service.local_today();
        if service
            .schedule_quick(account_id, id, quick, today)
            .await
            .is_err()
        {
            return redirect(&format!("{target}?error=schedule"));
        }
    }
    if let Some(position) = form
        .position
        .as_deref()
        .and_then(|value| value.trim().parse::<usize>().ok())
    {
        // Drop positions are 1-based in the UI; commands use zero-based indices.
        let position = position.saturating_sub(1);
        let result = if form.position_scope.as_deref() == Some("today") {
            service
                .move_today_to_position(account_id, id, position)
                .await
        } else {
            service.move_to_position(account_id, id, position).await
        };
        if result.is_err() {
            return redirect(&format!("{target}?error=position"));
        }
    }
    redirect(&target)
}

#[derive(Debug, Deserialize)]
struct UpdateTodoForm {
    title: String,
    #[serde(default)]
    notes: String,
    #[serde(default)]
    return_to: Option<String>,
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
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| format!("/todos/{id}"));
    redirect(&format!("{target}?saved=1"))
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

#[derive(Debug, Deserialize)]
struct PositionForm {
    position: String,
    #[serde(default)]
    return_to: Option<String>,
}

fn form_pairs(body: &str) -> Vec<(String, String)> {
    form_urlencoded::parse(body.as_bytes())
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
}

async fn bulk_action(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    body: String,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let pairs = form_pairs(&body);
    let field = |name: &str| {
        pairs
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.clone())
    };
    let target = field("return_to")
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/inbox".to_string());
    let ids: Vec<EntityId> = pairs
        .iter()
        .filter(|(key, _)| key == "ids")
        .filter_map(|(_, value)| value.parse::<EntityId>().ok())
        .collect();
    if ids.is_empty() {
        return redirect(&format!("{target}?error=selection"));
    }
    let today = service.local_today();
    let date = || field("date").and_then(|value| value.parse::<LocalDate>().ok());
    let action = match field("action").as_deref() {
        Some("complete") => Some(BulkAction::Complete),
        Some("cancel") => Some(BulkAction::Cancel),
        Some("delete") => Some(BulkAction::Delete),
        Some("schedule_today") => Some(BulkAction::Schedule(ScheduleInput::On(today))),
        Some("schedule_evening") => Some(BulkAction::Schedule(ScheduleInput::ThisEvening(today))),
        Some("schedule_tomorrow") => {
            Some(BulkAction::Schedule(ScheduleInput::On(today.next_day())))
        }
        Some("schedule_someday") => Some(BulkAction::Schedule(ScheduleInput::Someday)),
        Some("schedule_anytime") => Some(BulkAction::Schedule(ScheduleInput::Anytime)),
        Some("deadline_on") => date().map(|date| BulkAction::SetDeadline(Some(date))),
        Some("deadline_none") => Some(BulkAction::SetDeadline(None)),
        Some("move") => match field("destination").as_deref() {
            Some("inbox") => Some(BulkAction::Move(Destination::Inbox)),
            Some(destination) => destination
                .strip_prefix("project:")
                .and_then(|raw| raw.parse::<EntityId>().ok())
                .map(|project| BulkAction::Move(Destination::Project(project))),
            None => None,
        },
        Some("tag") => field("tag")
            .and_then(|value| value.parse::<EntityId>().ok())
            .map(BulkAction::AssignTag),
        _ => None,
    };
    let Some(action) = action else {
        return redirect(&format!("{target}?error=bulk"));
    };
    let bulk = BulkService::new(service, state.tags.clone());
    let outcome = bulk.apply(account_id, &ids, &action).await;
    if outcome.is_full_success() {
        redirect(&target)
    } else {
        redirect(&format!("{target}?error=bulk"))
    }
}

async fn move_to_position(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<PositionForm>,
) -> Response {
    position_request(&state, &session, &todo_id, form, false).await
}

async fn move_today_to_position(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<PositionForm>,
) -> Response {
    position_request(&state, &session, &todo_id, form, true).await
}

async fn position_request(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    form: PositionForm,
    today_scope: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/inbox".to_string());
    // Drop positions are 1-based in the UI; commands use zero-based indices.
    let Ok(display_position) = form.position.trim().parse::<usize>() else {
        return redirect(&format!("{target}?error=position"));
    };
    let position = display_position.saturating_sub(1);
    let result = if today_scope {
        service
            .move_today_to_position(account_id, id, position)
            .await
    } else {
        service.move_to_position(account_id, id, position).await
    };
    match result {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=position")),
    }
}

async fn move_today_up(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<RedirectForm>,
) -> Response {
    reorder_today(&state, &session, &todo_id, form, true).await
}

async fn move_today_down(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<RedirectForm>,
) -> Response {
    reorder_today(&state, &session, &todo_id, form, false).await
}

async fn reorder_today(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    form: RedirectForm,
    up: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/today?error=notfound");
    };
    let target = back(&form);
    let direction = if up {
        momentum_app::commands::ReorderDirection::Up
    } else {
        momentum_app::commands::ReorderDirection::Down
    };
    match service.reorder_today(account_id, id, direction).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=reorder")),
    }
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
struct ReminderForm {
    #[serde(default)]
    at: Option<String>,
    #[serde(default)]
    timezone: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

async fn set_reminder(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<ReminderForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = todo_id.parse::<EntityId>() else {
        return redirect("/inbox?error=notfound");
    };
    let target = form
        .return_to
        .as_deref()
        .filter(|value| value.starts_with('/'))
        .unwrap_or("/inbox")
        .to_string();
    let reminder = match form
        .at
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(text) => {
            let timezone = form
                .timezone
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "UTC".to_string());
            match parse_local_datetime(text)
                .and_then(|local| local_datetime_to_utc(local, &timezone))
            {
                Ok(at) => Some(ReminderInput { at, timezone }),
                Err(_) => return redirect(&format!("{target}?error=reminder")),
            }
        }
        None => None,
    };
    match service.set_reminder(account_id, id, reminder).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=reminder")),
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

async fn purge_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<RedirectForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = todo_id.parse::<EntityId>() else {
        return redirect("/trash?error=notfound");
    };
    let target = back(&form);
    match service.purge(account_id, id).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=purge")),
    }
}

async fn empty_trash(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<RedirectForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let target = back(&form);
    match service.empty_trash(account_id).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=purge")),
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
