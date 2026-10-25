//! Authenticated command endpoint for hydrated UI interactions.

use leptos::prelude::*;
use uuid::Uuid;

use crate::protocol::mutation::{UiCommandRequest, UiCommandResponse};

/// Executes a typed command for the account resolved from the current session cookie.
#[server(ExecuteUiCommand, "/api/ui")]
pub async fn execute_ui_command(
    request: UiCommandRequest,
) -> Result<UiCommandResponse, ServerFnError> {
    let Some(account) = crate::session::resolve_session().account().cloned() else {
        return Err(ServerFnError::new("unauthorized"));
    };

    let Some(service) =
        use_context::<std::sync::Arc<momentum_app::ui_commands::UiCommandService>>()
    else {
        return Err(ServerFnError::new("UI command service is unavailable"));
    };

    let outcome = service
        .execute(account.account_id, request.operation_id, request.command)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(UiCommandResponse {
        entity_id: outcome.entity_id,
        invalidated: outcome.invalidated,
        sequence: outcome.sequence,
        replayed: outcome.replayed,
    })
}

/// Constructs a request with a newly allocated operation id.
///
/// Callers that retry a failed transport request should retain and reuse the returned id.
#[must_use]
pub fn new_ui_command_request(command: momentum_app::commands::Command) -> UiCommandRequest {
    UiCommandRequest {
        operation_id: Uuid::now_v7(),
        command,
    }
}
