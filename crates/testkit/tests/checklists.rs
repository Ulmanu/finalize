//! Checklist editing, bulk expansion, and copy-semantics tests.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{CommandDispatcher, CommandError, ReorderDirection};
use momentum_app::todos::TodoService;
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::todos::MemoryTodoReadStore;

fn service() -> (TodoService, MemoryDatabase, AccountId) {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 1, 9, 0, 0).unwrap(),
    ));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let service = TodoService::new(
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock,
    );
    (service, database, AccountId::new())
}

#[tokio::test]
async fn checklist_items_are_edited_in_order_on_the_same_todo() {
    let (service, _, account) = service();
    let todo = service
        .create(account, "Ship the release", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let first = service
        .add_checklist_item(account, todo, "Write notes")
        .await
        .expect("add");
    assert_eq!(first.entity_id, Some(todo), "parent identity is stable");
    service
        .add_checklist_item(account, todo, "Tag build")
        .await
        .expect("add");
    service
        .add_checklist_item(account, todo, "Publish")
        .await
        .expect("add");

    let summary = service
        .get(account, todo)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.checklist.len(), 3);
    let items = summary.checklist.clone();
    assert_eq!(items[0].title, "Write notes");

    service
        .set_checklist_item_completed(account, todo, items[0].id, true)
        .await
        .expect("complete");
    service
        .rename_checklist_item(account, todo, items[1].id, "Tag release build")
        .await
        .expect("rename");
    service
        .reorder_checklist_item(account, todo, items[0].id, ReorderDirection::Down)
        .await
        .expect("reorder");

    let summary = service
        .get(account, todo)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.checklist[0].title, "Tag release build");
    assert_eq!(summary.checklist[1].title, "Write notes");
    assert!(summary.checklist[1].completed);
    assert_eq!(summary.checklist[2].title, "Publish");

    service
        .set_checklist_item_completed(account, todo, items[0].id, false)
        .await
        .expect("uncomplete");
    service
        .remove_checklist_item(account, todo, items[2].id)
        .await
        .expect("remove");
    let summary = service
        .get(account, todo)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.checklist.len(), 2);
    assert!(!summary.checklist[1].completed);

    let error = service
        .remove_checklist_item(account, todo, items[2].id)
        .await
        .expect_err("removed item is gone");
    assert!(matches!(error, CommandError::NotFound));
}

#[tokio::test]
async fn bulk_expansion_normalizes_lines_deterministically() {
    let (service, _, account) = service();
    let todo = service
        .create(account, "Prepare", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    service
        .expand_checklist(
            account,
            todo,
            "  \n- [ ] Alpha\n* Beta\n\n\u{2022} Gamma\n- [x] Done\n   ",
        )
        .await
        .expect("expand");

    let summary = service
        .get(account, todo)
        .await
        .expect("get")
        .expect("todo");
    let titles: Vec<&str> = summary
        .checklist
        .iter()
        .map(|item| item.title.as_str())
        .collect();
    assert_eq!(titles, vec!["Alpha", "Beta", "Gamma", "Done"]);
    assert!(summary.checklist.iter().all(|item| !item.completed));

    let error = service
        .expand_checklist(account, todo, "   \n\n  ")
        .await
        .expect_err("empty paste is rejected");
    assert!(matches!(error, CommandError::Validation(_)));
}

#[tokio::test]
async fn duplication_copies_structure_and_resets_completion() {
    let (service, _, account) = service();
    let source = service
        .create(account, "Template", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    service
        .add_checklist_item(account, source, "Step one")
        .await
        .expect("add");
    service
        .add_checklist_item(account, source, "Step two")
        .await
        .expect("add");
    let items = service
        .get(account, source)
        .await
        .expect("get")
        .expect("todo")
        .checklist;
    service
        .set_checklist_item_completed(account, source, items[0].id, true)
        .await
        .expect("complete");

    let copy = service
        .duplicate(account, source)
        .await
        .expect("duplicate")
        .entity_id
        .expect("id");
    let copy_summary = service
        .get(account, copy)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(copy_summary.checklist.len(), 2);
    assert_eq!(copy_summary.checklist[0].title, "Step one");
    assert!(copy_summary.checklist.iter().all(|item| !item.completed));
    assert_ne!(
        copy_summary.checklist[0].id, items[0].id,
        "copies own independent item identities"
    );

    let source_summary = service
        .get(account, source)
        .await
        .expect("get")
        .expect("todo");
    assert!(source_summary.checklist[0].completed, "source is unchanged");
}
