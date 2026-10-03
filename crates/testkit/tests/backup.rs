//! Backup import/export: additive imports with skip reporting and export round-trips.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::backup::{
    apply_import, build_backup, validate_backup, BackupArea, BackupDocument, BackupProject,
    BackupTag, BackupTodo, BACKUP_VERSION,
};
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    todos: TodoService,
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
    let todos = TodoService::new(
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    );
    Harness {
        todos,
        account: AccountId::new(),
    }
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

fn todo(title: &str, status: &str) -> BackupTodo {
    BackupTodo {
        title: title.to_string(),
        notes: String::new(),
        status: status.to_string(),
        scheduled_on: None,
        evening: false,
        deadline: None,
        checklist: Vec::new(),
    }
}

#[tokio::test]
async fn imports_are_additive_and_report_unsupported_records() {
    let source = harness();
    let target = harness();
    target
        .todos
        .create(target.account, "Existing", None)
        .await
        .expect("existing");

    let document = BackupDocument {
        version: BACKUP_VERSION,
        exported_at: Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
        areas: vec![BackupArea {
            id: momentum_domain::EntityId::new(),
            title: "Area".to_string(),
        }],
        projects: vec![BackupProject {
            id: momentum_domain::EntityId::new(),
            title: "Project".to_string(),
            notes: String::new(),
        }],
        tags: vec![BackupTag {
            id: momentum_domain::EntityId::new(),
            title: "Tag".to_string(),
        }],
        todos: vec![
            todo("Imported", "open"),
            todo("Finished", "completed"),
            todo("Removed", "deleted"),
            todo("", "open"),
        ],
    };
    let plan = validate_backup(&document).expect("plan");
    let summary = apply_import(&target.todos, target.account, plan)
        .await
        .expect("import");
    assert_eq!(summary.imported, 3);
    assert_eq!(summary.skipped.len(), 4);

    let open = target
        .todos
        .list(target.account, TodoScope::AllOpen)
        .await
        .expect("all open");
    let titles: Vec<&str> = open.iter().map(|todo| todo.title.as_str()).collect();
    assert!(titles.contains(&"Existing"));
    assert!(titles.contains(&"Imported"));
    assert!(!titles.contains(&"Finished"));
    let logged = target
        .todos
        .list(target.account, TodoScope::Logbook { logged_only: false })
        .await
        .expect("logbook");
    assert!(logged.iter().any(|todo| todo.title == "Finished"));
    let trashed = target
        .todos
        .list(target.account, TodoScope::Trash)
        .await
        .expect("trash");
    assert!(trashed.iter().any(|todo| todo.title == "Removed"));
    assert!(source
        .todos
        .list(source.account, TodoScope::AllOpen)
        .await
        .expect("source")
        .is_empty());
}

#[tokio::test]
async fn export_round_trip_reimports_metadata() {
    let source = harness();
    let target = harness();
    source
        .todos
        .create(source.account, "Detailed", Some("Notes".to_string()))
        .await
        .expect("create");
    let id = source
        .todos
        .list(source.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;
    source
        .todos
        .schedule(source.account, id, ScheduleInput::On(date(12)))
        .await
        .expect("schedule");
    source
        .todos
        .set_deadline(source.account, id, Some(date(20)))
        .await
        .expect("deadline");
    source
        .todos
        .add_checklist_item(source.account, id, "Step")
        .await
        .expect("checklist");

    let document = build_backup(
        &source.todos,
        source.account,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
    )
    .await
    .expect("backup");
    assert_eq!(document.version, BACKUP_VERSION);
    assert_eq!(document.todos.len(), 1);
    assert_eq!(document.todos[0].scheduled_on, Some(date(12)));
    assert_eq!(document.todos[0].checklist, vec!["Step".to_string()]);

    let plan = validate_backup(&document).expect("plan");
    let summary = apply_import(&target.todos, target.account, plan)
        .await
        .expect("import");
    assert_eq!(summary.imported, 1);
    let imported = target
        .todos
        .list(target.account, TodoScope::AllOpen)
        .await
        .expect("all open")
        .into_iter()
        .next()
        .expect("imported");
    assert_eq!(imported.notes, "Notes");
    assert_eq!(
        imported.deadline.map(|deadline| deadline.date),
        Some(date(20))
    );
    assert_eq!(imported.checklist.len(), 1);
}
