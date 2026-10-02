//! Versioned REST automation API over the same application commands as the UI.
//!
//! `Authorization: Bearer <token>` authenticates a scoped personal token; every mutation
//! delegates to the application services. Errors are typed JSON payloads
//! (`{"error":{"code","message"}}`) suitable for automation.

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Form, Json, Router};
use momentum_app::api_tokens::{ApiPrincipal, ApiScope};
use momentum_app::commands::ScheduleInput;
use momentum_app::session::SessionState;
use momentum_app::todos::{TodoScope, TodoSummary};
use momentum_domain::time::LocalDate;
use momentum_domain::EntityId;
use serde::{Deserialize, Serialize};

use crate::server::AppState;

/// Automation routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/todos", get(list_todos).post(create_todo))
        .route("/api/v1/todos/{id}", get(show_todo).patch(patch_todo))
        .route("/api/v1/todos/{id}/{action}", post(todo_action))
        .route("/settings/tokens", post(create_token))
        .route("/settings/tokens/{id}/revoke", post(revoke_token))
        .route("/settings/tokens/{id}/rotate", post(rotate_token))
}

#[derive(Debug, Serialize)]
struct ApiErrorBody {
    error: ApiErrorDetail,
}

#[derive(Debug, Serialize)]
struct ApiErrorDetail {
    code: &'static str,
    message: String,
}

struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "unauthorized",
            message: "a valid bearer token is required".to_string(),
        }
    }

    fn forbidden() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "forbidden",
            message: "the token lacks the required scope".to_string(),
        }
    }

    fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "not_found",
            message: "the requested resource does not exist".to_string(),
        }
    }

    fn invalid(message: &str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "invalid_request",
            message: message.to_string(),
        }
    }

    fn unavailable() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "unavailable",
            message: "the automation API is not configured".to_string(),
        }
    }

    fn internal() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "internal",
            message: "the request could not be completed".to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ApiErrorBody {
                error: ApiErrorDetail {
                    code: self.code,
                    message: self.message,
                },
            }),
        )
            .into_response()
    }
}

async fn principal(
    state: &AppState,
    headers: &HeaderMap,
    scope: ApiScope,
) -> Result<ApiPrincipal, ApiError> {
    let Some(service) = state.api_tokens.clone() else {
        return Err(ApiError::unavailable());
    };
    let presented = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(ApiError::unauthorized)?;
    let now = chrono::Utc::now();
    let principal = service
        .authenticate(presented, now)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::unauthorized)?;
    if !principal.has(scope) {
        return Err(ApiError::forbidden());
    }
    Ok(principal)
}

#[derive(Debug, Serialize)]
struct ApiTodo {
    id: String,
    title: String,
    notes: String,
    status: String,
    schedule: Option<String>,
    deadline: Option<String>,
    context: String,
}

impl ApiTodo {
    fn from_summary(todo: &TodoSummary) -> Self {
        Self {
            id: todo.id.to_string(),
            title: todo.title.clone(),
            notes: todo.notes.clone(),
            status: match todo.status {
                momentum_domain::model::Status::Open => "open",
                momentum_domain::model::Status::Completed => "completed",
                momentum_domain::model::Status::Canceled => "canceled",
                momentum_domain::model::Status::Deleted => "deleted",
            }
            .to_string(),
            schedule: todo.schedule.date().map(|date| date.to_iso()),
            deadline: todo.deadline.map(|deadline| deadline.date.to_iso()),
            context: match todo.context {
                momentum_domain::model::ContextRef::Inbox => "inbox".to_string(),
                momentum_domain::model::ContextRef::Area(_) => "area".to_string(),
                momentum_domain::model::ContextRef::Project(_) => "project".to_string(),
                momentum_domain::model::ContextRef::Heading(_) => "heading".to_string(),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    #[serde(default)]
    scope: Option<String>,
}

fn parse_scope(value: Option<&str>) -> Result<TodoScope, ApiError> {
    match value.unwrap_or("inbox") {
        "inbox" => Ok(TodoScope::Inbox),
        "all" => Ok(TodoScope::AllOpen),
        "today" => Ok(TodoScope::Today),
        "upcoming" => Ok(TodoScope::Upcoming),
        "anytime" => Ok(TodoScope::Anytime),
        "someday" => Ok(TodoScope::Someday),
        "deadlines" => Ok(TodoScope::Deadlines),
        "logbook" => Ok(TodoScope::Logbook { logged_only: false }),
        "trash" => Ok(TodoScope::Trash),
        _ => Err(ApiError::invalid("unsupported scope")),
    }
}

async fn list_todos(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ListQuery>,
) -> Response {
    let principal = match principal(&state, &headers, ApiScope::Read).await {
        Ok(principal) => principal,
        Err(error) => return error.into_response(),
    };
    let Some(service) = state.todos.clone() else {
        return ApiError::unavailable().into_response();
    };
    let scope = match parse_scope(query.scope.as_deref()) {
        Ok(scope) => scope,
        Err(error) => return error.into_response(),
    };
    match service.list(principal.account_id, scope).await {
        Ok(todos) => {
            Json(todos.iter().map(ApiTodo::from_summary).collect::<Vec<_>>()).into_response()
        }
        Err(_) => ApiError::internal().into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct TodoBody {
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
}

fn parse_schedule(body: &TodoBody, today: LocalDate) -> Result<Option<ScheduleInput>, ApiError> {
    let Some(when) = body.when.as_deref() else {
        return Ok(None);
    };
    match when {
        "today" => Ok(Some(ScheduleInput::On(today))),
        "evening" => Ok(Some(ScheduleInput::ThisEvening(today))),
        "tomorrow" => Ok(Some(ScheduleInput::On(today.next_day()))),
        "someday" => Ok(Some(ScheduleInput::Someday)),
        "anytime" => Ok(Some(ScheduleInput::Anytime)),
        "date" => match body
            .date
            .as_deref()
            .and_then(|value| value.parse::<LocalDate>().ok())
        {
            Some(date) => Ok(Some(ScheduleInput::On(date))),
            None => Err(ApiError::invalid("when=date requires a valid date")),
        },
        _ => Err(ApiError::invalid("unsupported when value")),
    }
}

fn parse_deadline(body: &TodoBody) -> Result<Option<Option<LocalDate>>, ApiError> {
    match body.deadline.as_deref() {
        None => Ok(None),
        Some("") => Ok(Some(None)),
        Some(value) => match value.parse::<LocalDate>() {
            Ok(date) => Ok(Some(Some(date))),
            Err(_) => Err(ApiError::invalid("deadline must be an ISO date or empty")),
        },
    }
}

async fn create_todo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<TodoBody>,
) -> Response {
    let principal = match principal(&state, &headers, ApiScope::Write).await {
        Ok(principal) => principal,
        Err(error) => return error.into_response(),
    };
    let Some(service) = state.todos.clone() else {
        return ApiError::unavailable().into_response();
    };
    let Some(title) = body
        .title
        .as_deref()
        .filter(|title| !title.trim().is_empty())
    else {
        return ApiError::invalid("title is required").into_response();
    };
    let today = service.local_today();
    let schedule = match parse_schedule(&body, today) {
        Ok(schedule) => schedule,
        Err(error) => return error.into_response(),
    };
    let deadline = match parse_deadline(&body) {
        Ok(deadline) => deadline,
        Err(error) => return error.into_response(),
    };
    let outcome = match service
        .create_in(
            principal.account_id,
            title,
            body.notes.clone(),
            momentum_domain::model::ContextRef::Inbox,
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(_) => return ApiError::internal().into_response(),
    };
    let Some(id) = outcome.entity_id else {
        return ApiError::internal().into_response();
    };
    let applied = async {
        if let Some(schedule) = schedule {
            service.schedule(principal.account_id, id, schedule).await?;
        }
        if let Some(deadline) = deadline {
            service
                .set_deadline(principal.account_id, id, deadline)
                .await?;
        }
        Ok::<(), momentum_app::commands::CommandError>(())
    }
    .await;
    if applied.is_err() {
        let _ = service.delete(principal.account_id, id).await;
        return ApiError::internal().into_response();
    }
    match service.get(principal.account_id, id).await {
        Ok(Some(todo)) => (StatusCode::CREATED, Json(ApiTodo::from_summary(&todo))).into_response(),
        _ => ApiError::internal().into_response(),
    }
}

async fn show_todo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let principal = match principal(&state, &headers, ApiScope::Read).await {
        Ok(principal) => principal,
        Err(error) => return error.into_response(),
    };
    let Some(service) = state.todos.clone() else {
        return ApiError::unavailable().into_response();
    };
    let Ok(id) = id.parse::<EntityId>() else {
        return ApiError::not_found().into_response();
    };
    match service.get(principal.account_id, id).await {
        Ok(Some(todo)) => Json(ApiTodo::from_summary(&todo)).into_response(),
        _ => ApiError::not_found().into_response(),
    }
}

async fn patch_todo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(body): Json<TodoBody>,
) -> Response {
    let principal = match principal(&state, &headers, ApiScope::Write).await {
        Ok(principal) => principal,
        Err(error) => return error.into_response(),
    };
    let Some(service) = state.todos.clone() else {
        return ApiError::unavailable().into_response();
    };
    let Ok(id) = id.parse::<EntityId>() else {
        return ApiError::not_found().into_response();
    };
    if service
        .get(principal.account_id, id)
        .await
        .ok()
        .flatten()
        .is_none()
    {
        return ApiError::not_found().into_response();
    }
    let today = service.local_today();
    let schedule = match parse_schedule(&body, today) {
        Ok(schedule) => schedule,
        Err(error) => return error.into_response(),
    };
    let deadline = match parse_deadline(&body) {
        Ok(deadline) => deadline,
        Err(error) => return error.into_response(),
    };
    let applied = async {
        if let Some(title) = body
            .title
            .as_deref()
            .filter(|title| !title.trim().is_empty())
        {
            service.set_title(principal.account_id, id, title).await?;
        }
        if let Some(notes) = body.notes.as_deref() {
            service.set_notes(principal.account_id, id, notes).await?;
        }
        if let Some(schedule) = schedule {
            service.schedule(principal.account_id, id, schedule).await?;
        }
        if let Some(deadline) = deadline {
            service
                .set_deadline(principal.account_id, id, deadline)
                .await?;
        }
        Ok::<(), momentum_app::commands::CommandError>(())
    }
    .await;
    if applied.is_err() {
        return ApiError::internal().into_response();
    }
    match service.get(principal.account_id, id).await {
        Ok(Some(todo)) => Json(ApiTodo::from_summary(&todo)).into_response(),
        _ => ApiError::internal().into_response(),
    }
}

#[derive(Debug, Serialize)]
struct ActionResponse {
    status: &'static str,
}

async fn todo_action(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((id, action)): Path<(String, String)>,
) -> Response {
    let principal = match principal(&state, &headers, ApiScope::Write).await {
        Ok(principal) => principal,
        Err(error) => return error.into_response(),
    };
    let Some(service) = state.todos.clone() else {
        return ApiError::unavailable().into_response();
    };
    let Ok(id) = id.parse::<EntityId>() else {
        return ApiError::not_found().into_response();
    };
    let result = match action.as_str() {
        "complete" => service.complete(principal.account_id, id).await,
        "cancel" => service.cancel(principal.account_id, id).await,
        "reopen" => service.reopen(principal.account_id, id).await,
        "delete" => service.delete(principal.account_id, id).await,
        _ => return ApiError::invalid("unsupported action").into_response(),
    };
    match result {
        Ok(_) => Json(ActionResponse { status: "ok" }).into_response(),
        Err(momentum_app::commands::CommandError::NotFound) => {
            ApiError::not_found().into_response()
        }
        Err(_) => ApiError::internal().into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct CreateTokenForm {
    name: String,
    #[serde(default)]
    scopes: Option<String>,
}

fn token_settings_redirect(token: Option<&str>) -> Response {
    let target = match token {
        Some(token) => {
            let encoded: String = form_urlencoded::byte_serialize(token.as_bytes()).collect();
            format!("/settings?token={encoded}")
        }
        None => "/settings".to_string(),
    };
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(&target) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

async fn create_token(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<CreateTokenForm>,
) -> Response {
    let Some(service) = state.api_tokens.clone() else {
        return token_settings_redirect(None);
    };
    let Some(account) = session.account() else {
        return token_settings_redirect(None);
    };
    let scopes: Vec<ApiScope> = form
        .scopes
        .as_deref()
        .unwrap_or("read")
        .split(',')
        .filter_map(|scope| ApiScope::parse(scope.trim()))
        .collect();
    let scopes = if scopes.is_empty() {
        vec![ApiScope::Read]
    } else {
        scopes
    };
    match service
        .create(account.account_id, &form.name, &scopes, chrono::Utc::now())
        .await
    {
        Ok(created) => token_settings_redirect(Some(&created.token)),
        Err(_) => token_settings_redirect(None),
    }
}

async fn revoke_token(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    let (Some(service), Some(account), Ok(id)) = (
        state.api_tokens.clone(),
        session.account(),
        id.parse::<EntityId>(),
    ) else {
        return token_settings_redirect(None);
    };
    let _ = service
        .revoke(account.account_id, id, chrono::Utc::now())
        .await;
    token_settings_redirect(None)
}

async fn rotate_token(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    let (Some(service), Some(account), Ok(id)) = (
        state.api_tokens.clone(),
        session.account(),
        id.parse::<EntityId>(),
    ) else {
        return token_settings_redirect(None);
    };
    match service.rotate(account.account_id, id).await {
        Ok(Some(created)) => token_settings_redirect(Some(&created.token)),
        _ => token_settings_redirect(None),
    }
}
