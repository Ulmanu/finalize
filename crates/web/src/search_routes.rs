//! Quick Find navigation route that records recents before redirecting.

use axum::extract::{Query, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Extension;
use axum::Router;
use momentum_app::search::SearchService;
use momentum_app::session::SessionState;
use serde::Deserialize;

use crate::server::AppState;

/// Quick Find navigation routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/search/go", get(go))
}

#[derive(Debug, Deserialize)]
struct GoQuery {
    url: String,
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

async fn go(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Query(query): Query<GoQuery>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    if !query.url.starts_with('/') || query.url.starts_with("//") {
        return redirect("/search");
    }
    let (Some(todos), Some(projects)) = (state.todos.clone(), state.projects.clone()) else {
        return redirect("/search");
    };
    let service = SearchService::new(
        todos,
        projects,
        state.areas.clone(),
        state.tags.clone(),
        state.recents.clone(),
    );
    let _ = service.record_visit(account.account_id, &query.url).await;
    redirect(&query.url)
}
