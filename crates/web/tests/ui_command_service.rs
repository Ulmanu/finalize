#![cfg(feature = "ssr")]

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{Command, CommandDispatcher, Projection, ScheduleInput};
use momentum_app::ui_commands::UiCommandService;
use momentum_domain::model::ContextRef;
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_web::protocol::mutation::UiCommandRequest;
use uuid::Uuid;

fn create_inbox_todo(title: &str) -> Command {
    Command::CreateTodo {
        title: title.to_string(),
        notes: None,
        context: ContextRef::Inbox,
        schedule: ScheduleInput::Anytime,
        deadline: None,
        reminder: None,
    }
}

#[tokio::test]
async fn ui_command_facade_preserves_dispatcher_idempotency_and_invalidations() {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 10, 22, 9, 0, 0).unwrap(),
    ));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let service = UiCommandService::new(dispatcher, clock);
    let account = AccountId::new();
    let operation_id = Uuid::now_v7();
    let command = create_inbox_todo("Write the release notes");

    let first = service
        .execute(account, operation_id, command.clone())
        .await
        .expect("first command succeeds");
    let replay = service
        .execute(account, operation_id, command)
        .await
        .expect("retry replays the first result");

    assert_eq!(first.sequence, 1);
    assert_eq!(first.invalidated, vec![Projection::Inbox]);
    assert!(!first.replayed);
    assert!(replay.replayed);
    assert_eq!(replay.entity_id, first.entity_id);
    assert_eq!(replay.sequence, first.sequence);
    assert_eq!(database.changes(account).len(), 1);
}

#[test]
fn ui_command_request_does_not_include_the_account_identity() {
    let request = UiCommandRequest {
        operation_id: Uuid::now_v7(),
        command: create_inbox_todo("Account comes from the session"),
    };

    let payload = serde_json::to_value(request).expect("request serializes");
    assert!(payload.get("account_id").is_none());
    assert!(payload.get("operation_id").is_some());
    assert!(payload.get("command").is_some());
}
