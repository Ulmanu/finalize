//! Application facade for authenticated first-party UI commands.
//!
//! The browser supplies an idempotency key and a typed command. The session layer supplies
//! the account identity, and all writes continue through [`CommandDispatcher`].

use std::sync::Arc;

use momentum_domain::AccountId;
use uuid::Uuid;

use crate::auth::Clock;
use crate::commands::{Command, CommandDispatcher, CommandEnvelope, CommandError, CommandOutcome};

/// Executes first-party UI commands through the authoritative command dispatcher.
pub struct UiCommandService {
    dispatcher: Arc<CommandDispatcher>,
    clock: Arc<dyn Clock>,
}

impl std::fmt::Debug for UiCommandService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UiCommandService")
    }
}

impl UiCommandService {
    /// Creates the UI facade over the shared command dispatcher and clock.
    #[must_use]
    pub fn new(dispatcher: Arc<CommandDispatcher>, clock: Arc<dyn Clock>) -> Self {
        Self { dispatcher, clock }
    }

    /// Executes one account-scoped command with a client-provided idempotency key.
    ///
    /// # Errors
    ///
    /// Returns the typed command validation, conflict, not-found, or storage error.
    pub async fn execute(
        &self,
        account_id: AccountId,
        operation_id: Uuid,
        command: Command,
    ) -> Result<CommandOutcome, CommandError> {
        self.dispatcher
            .execute(CommandEnvelope {
                operation_id,
                account_id,
                issued_at: self.clock.now(),
                command,
            })
            .await
    }
}
