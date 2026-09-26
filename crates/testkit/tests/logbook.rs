//! Logbook membership, history timestamps, and logging policy tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, CommandError};
use momentum_app::logbook::LogbookService;
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
use momentum_domain::model::LoggingPolicy;
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::logbook::MemoryLoggingPolicyStore;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    logbook: LogbookService,
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
    account: AccountId,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
    ));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    ));
    let projects = Arc::new(ProjectService::new(
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock.clone(),
    ));
    let logbook = LogbookService::new(
        dispatcher,
        todos.clone(),
        projects.clone(),
        Arc::new(MemoryLoggingPolicyStore::new()),
        clock,
    );
    Harness {
        logbook,
        todos,
        projects,
        account: AccountId::new(),
    }
}

#[tokio::test]
async fn automatic_policy_logs_terminal_items_with_history_timestamps() {
    let h = harness();
    let done = h
        .todos
        .create(h.account, "Finished", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.complete(h.account, done).await.expect("complete");
    let canceled = h
        .todos
        .create(h.account, "Dropped", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.cancel(h.account, canceled).await.expect("cancel");
    let project = h
        .projects
        .create(h.account, "Shipped", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    h.projects
        .complete(
            h.account,
            project,
            momentum_domain::model::ProjectCompletionPolicy::CompleteOpenChildren,
        )
        .await
        .expect("complete project");

    assert_eq!(
        h.logbook.policy(h.account).await.expect("policy"),
        LoggingPolicy::Automatic
    );
    let agenda = h.logbook.agenda(h.account).await.expect("agenda");
    assert!(agenda.pending.is_empty());
    assert_eq!(agenda.days.len(), 1, "same-day entries group together");
    assert_eq!(agenda.days[0].entries.len(), 3);

    let todo_entry = agenda.days[0]
        .entries
        .iter()
        .find(|entry| entry.id() == done)
        .expect("completed entry");
    let LogbookEntrySummary {
        completed, logged, ..
    } = summarize(todo_entry);
    assert!(completed.is_some(), "completion timestamp queryable");
    assert!(
        logged.is_none(),
        "automatic logging does not fabricate logged_at"
    );

    let project_entry = agenda.days[0]
        .entries
        .iter()
        .find(|entry| entry.id() == project)
        .expect("project entry");
    assert!(matches!(
        project_entry,
        momentum_app::logbook::LogbookEntry::Project(_)
    ));
}

struct LogbookEntrySummary {
    completed: Option<chrono::DateTime<Utc>>,
    logged: Option<chrono::DateTime<Utc>>,
}

fn summarize(entry: &momentum_app::logbook::LogbookEntry) -> LogbookEntrySummary {
    match entry {
        momentum_app::logbook::LogbookEntry::Todo(todo) => LogbookEntrySummary {
            completed: todo.completed_at,
            logged: todo.logged_at,
        },
        momentum_app::logbook::LogbookEntry::Project(project) => LogbookEntrySummary {
            completed: project.completed_at,
            logged: project.logged_at,
        },
    }
}

#[tokio::test]
async fn manual_policy_keeps_terminal_items_pending_until_logged() {
    let h = harness();
    let id = h
        .todos
        .create(h.account, "Pending", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.complete(h.account, id).await.expect("complete");
    let completed_at = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo")
        .completed_at
        .expect("completed_at");

    h.logbook
        .set_policy(h.account, LoggingPolicy::Manual)
        .await
        .expect("policy");
    let agenda = h.logbook.agenda(h.account).await.expect("agenda");
    assert!(agenda.days.is_empty(), "nothing logged yet");
    assert_eq!(agenda.pending.len(), 1);
    assert_eq!(agenda.pending[0].id(), id);
    let summary = summarize(&agenda.pending[0]);
    assert_eq!(
        summary.completed,
        Some(completed_at),
        "policy changes never rewrite completion timestamps"
    );

    h.logbook.log_todo(h.account, id).await.expect("log");
    let agenda = h.logbook.agenda(h.account).await.expect("agenda");
    assert!(agenda.pending.is_empty());
    assert_eq!(agenda.days.len(), 1);
    let summary = summarize(&agenda.days[0].entries[0]);
    assert!(summary.logged.is_some());
    assert_eq!(summary.completed, Some(completed_at));

    let open = h
        .todos
        .create(h.account, "Open item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let error = h
        .logbook
        .log_todo(h.account, open)
        .await
        .expect_err("open items cannot be logged");
    assert!(matches!(error, CommandError::Validation(_)));
    let error = h
        .logbook
        .log_todo(h.account, momentum_domain::EntityId::new())
        .await
        .expect_err("unknown items cannot be logged");
    assert!(matches!(error, CommandError::NotFound));
}

#[tokio::test]
async fn policy_switching_preserves_completed_state_and_timestamps() {
    let h = harness();
    let id = h
        .todos
        .create(h.account, "Switch", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.complete(h.account, id).await.expect("complete");
    let completed_at = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo")
        .completed_at;

    h.logbook
        .set_policy(h.account, LoggingPolicy::Manual)
        .await
        .expect("manual");
    h.logbook
        .set_policy(h.account, LoggingPolicy::Automatic)
        .await
        .expect("automatic");
    let agenda = h.logbook.agenda(h.account).await.expect("agenda");
    assert!(agenda.pending.is_empty());
    let summary = summarize(&agenda.days[0].entries[0]);
    assert_eq!(summary.completed, completed_at);
    let todo = h
        .todos
        .get(h.account, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        todo.status,
        momentum_domain::model::Status::Completed,
        "completed state is preserved independently of logging policy"
    );
}
