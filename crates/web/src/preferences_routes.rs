//! Settings preference submission: validates against the typed registry and persists
//! account-scoped values (device/browser scopes are stored client-side by the hydrated client).

use axum::extract::{Extension, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Form, Router};
use momentum_app::preferences::{definition, parse_value, PreferenceScope};
use momentum_app::session::SessionState;
use serde::Deserialize;

use crate::server::AppState;

/// Preference routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/settings/preferences", post(save_preference))
}

#[derive(Debug, Deserialize)]
struct PreferenceForm {
    key: String,
    value: String,
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

async fn save_preference(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<PreferenceForm>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(parsed) = parse_value(&form.key, &form.value) else {
        return redirect("/settings?error=preference");
    };
    let Some(definition) = definition(&form.key) else {
        return redirect("/settings?error=preference");
    };
    match definition.scope {
        PreferenceScope::Account => {
            let Some(store) = state.preferences.clone() else {
                return redirect("/settings?error=preference");
            };
            match store
                .save(
                    account.account_id,
                    &form.key,
                    definition.scope,
                    &parsed.as_str(),
                )
                .await
            {
                Ok(()) => redirect("/settings"),
                Err(_) => redirect("/settings?error=preference"),
            }
        }
        PreferenceScope::Device | PreferenceScope::Browser => {
            // Validated and acknowledged; the hydrated client persists these locally.
            redirect("/settings")
        }
    }
}
