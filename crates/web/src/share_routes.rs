//! Share and print routes built from deterministic render models.

use axum::extract::{Extension, Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use momentum_app::session::SessionState;
use momentum_app::share::{
    print_list_html, print_project_html, share_list_markdown, share_project_markdown,
    share_todo_markdown,
};
use momentum_app::todos::TodoScope;
use momentum_domain::EntityId;

use crate::server::AppState;

/// Share and print routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/share/todos/{id}", get(share_todo))
        .route("/share/projects/{id}", get(share_project))
        .route("/share/lists/{list}", get(share_list))
        .route("/print/lists/{list}", get(print_list))
        .route("/print/projects/{id}", get(print_project))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

fn text(content_type: &'static str, body: String) -> Response {
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static(content_type))],
        body,
    )
        .into_response()
}

fn scope_for(list: &str) -> Option<(TodoScope, &'static str)> {
    match list {
        "inbox" => Some((TodoScope::Inbox, "Inbox")),
        "today" => Some((TodoScope::Today, "Today")),
        "upcoming" => Some((TodoScope::Upcoming, "Upcoming")),
        "anytime" => Some((TodoScope::Anytime, "Anytime")),
        "someday" => Some((TodoScope::Someday, "Someday")),
        "deadlines" => Some((TodoScope::Deadlines, "Deadlines")),
        "logbook" => Some((TodoScope::Logbook { logged_only: false }, "Logbook")),
        _ => None,
    }
}

async fn share_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.todos.clone() else {
        return redirect("/login");
    };
    let Ok(id) = id.parse::<EntityId>() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    match service.get(account.account_id, id).await {
        Ok(Some(todo)) => text("text/markdown; charset=utf-8", share_todo_markdown(&todo)),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn share_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.projects.clone() else {
        return redirect("/login");
    };
    let Ok(id) = id.parse::<EntityId>() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    match service.get(account.account_id, id).await {
        Ok(Some(project)) => text(
            "text/markdown; charset=utf-8",
            share_project_markdown(&project),
        ),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn share_list(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(list): Path<String>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.todos.clone() else {
        return redirect("/login");
    };
    let Some((scope, title)) = scope_for(&list) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    match service.list(account.account_id, scope).await {
        Ok(todos) => text(
            "text/markdown; charset=utf-8",
            share_list_markdown(title, &todos),
        ),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn print_list(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(list): Path<String>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.todos.clone() else {
        return redirect("/login");
    };
    let Some((scope, title)) = scope_for(&list) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    match service.list(account.account_id, scope).await {
        Ok(todos) => text("text/html; charset=utf-8", print_list_html(title, &todos)),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn print_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(id): Path<String>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(service) = state.projects.clone() else {
        return redirect("/login");
    };
    let Ok(id) = id.parse::<EntityId>() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    match service.get(account.account_id, id).await {
        Ok(Some(project)) => text("text/html; charset=utf-8", print_project_html(&project)),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}
