//! Deterministic share and print render models.
//!
//! Share models are plain text/markdown derived only from read models — never from interactive UI
//! markup — so the same data always renders the same bytes. Print documents escape all
//! untrusted text for HTML, and markdown models escape markdown control characters; visibility
//! rules come from the scopes the caller reads (open lists only, no trashed content).

use crate::projects::ProjectSummary;
use crate::todos::TodoSummary;

/// Escapes Markdown control characters in untrusted text.
#[must_use]
pub fn escape_markdown(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '.'
            | '!' | '|' | '>' => {
                escaped.push('\\');
                escaped.push(character);
            }
            other => escaped.push(other),
        }
    }
    escaped
}

/// Escapes untrusted text for HTML documents.
#[must_use]
pub fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            other => escaped.push(other),
        }
    }
    escaped
}

/// Deterministic markdown share model for one to-do.
#[must_use]
pub fn share_todo_markdown(todo: &TodoSummary) -> String {
    let mut out = format!("# {}\n\n", escape_markdown(&todo.title));
    if !todo.notes.is_empty() {
        out.push_str(&format!("{}\n\n", escape_markdown(&todo.notes)));
    }
    let mut meta = Vec::new();
    if let Some(date) = todo.schedule.date() {
        meta.push(format!("when: {date}"));
    }
    if let Some(deadline) = todo.deadline {
        meta.push(format!("deadline: {}", deadline.date));
    }
    meta.push(format!(
        "status: {}",
        match todo.status {
            momentum_domain::model::Status::Open => "open",
            momentum_domain::model::Status::Completed => "completed",
            momentum_domain::model::Status::Canceled => "canceled",
            momentum_domain::model::Status::Deleted => "deleted",
        }
    ));
    out.push_str(&format!("{}\n", meta.join(" · ")));
    if !todo.checklist.is_empty() {
        out.push('\n');
        for item in &todo.checklist {
            let mark = if item.completed { "x" } else { " " };
            out.push_str(&format!("- [{mark}] {}\n", escape_markdown(&item.title)));
        }
    }
    out
}

/// Deterministic markdown share model for a project.
#[must_use]
pub fn share_project_markdown(project: &ProjectSummary) -> String {
    let mut out = format!("# {}\n\n", escape_markdown(&project.title));
    if !project.notes.is_empty() {
        out.push_str(&format!("{}\n\n", escape_markdown(&project.notes)));
    }
    out.push_str(&format!(
        "{} open · {} completed\n",
        project.open_count, project.completed_count
    ));
    out
}

/// Deterministic markdown share model for a list.
#[must_use]
pub fn share_list_markdown(title: &str, todos: &[TodoSummary]) -> String {
    let mut out = format!("# {}\n\n", escape_markdown(title));
    for todo in todos {
        let mark = if matches!(todo.status, momentum_domain::model::Status::Completed) {
            "x"
        } else {
            " "
        };
        out.push_str(&format!("- [{mark}] {}", escape_markdown(&todo.title)));
        if let Some(date) = todo.schedule.date() {
            out.push_str(&format!(" (when: {date})"));
        }
        out.push('\n');
    }
    out
}

/// A deterministic, print-only HTML document for a list.
#[must_use]
pub fn print_list_html(title: &str, todos: &[TodoSummary]) -> String {
    let mut rows = String::new();
    for todo in todos {
        let mark = if matches!(todo.status, momentum_domain::model::Status::Completed) {
            "&#10003;"
        } else {
            "&#9744;"
        };
        let mut meta = Vec::new();
        if let Some(date) = todo.schedule.date() {
            meta.push(format!("when {date}"));
        }
        if let Some(deadline) = todo.deadline {
            meta.push(format!("deadline {}", deadline.date));
        }
        rows.push_str(&format!(
            "<li>{mark} <span class=\"title\">{}</span> <span class=\"meta\">{}</span></li>\n",
            escape_html(&todo.title),
            escape_html(&meta.join(" · "))
        ));
    }
    print_document(title, &format!("<ul>\n{rows}</ul>"))
}

/// A deterministic, print-only HTML document for a project.
#[must_use]
pub fn print_project_html(project: &ProjectSummary) -> String {
    let body = format!(
        "<p class=\"meta\">{} open · {} completed</p>\n<p>{}</p>",
        project.open_count,
        project.completed_count,
        escape_html(&project.notes)
    );
    print_document(&project.title, &body)
}

fn print_document(title: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\"><head><meta charset=\"utf-8\"/>\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"/>\
<title>{title}</title>\
<style>body{{font-family:system-ui,sans-serif;margin:2rem auto;max-width:40rem;color:#0f172a}}\
h1{{font-size:1.4rem}}ul{{list-style:none;padding:0}}li{{padding:.35rem 0;border-bottom:1px solid #e2e8f0}}\
.meta{{color:#64748b;font-size:.85rem}}\
@media print{{body{{margin:0}}}}\
</style></head><body><h1>{title}</h1>{body}</body></html>",
        title = escape_html(title),
        body = body
    )
}

#[cfg(test)]
mod tests {
    use super::{
        escape_html, escape_markdown, print_list_html, share_list_markdown, share_todo_markdown,
    };
    use crate::todos::{ChecklistItemSummary, TodoSummary};
    use momentum_domain::model::{ContextRef, Deadline, Schedule, Status};
    use momentum_domain::time::LocalDate;
    use momentum_domain::{ChecklistItemId, EntityId};

    fn todo(title: &str) -> TodoSummary {
        TodoSummary {
            id: EntityId::new(),
            title: title.to_string(),
            notes: "Notes *with* markup".to_string(),
            status: Status::Open,
            schedule: Schedule::on(LocalDate::from_ymd(2026, 6, 12).unwrap()),
            deadline: Some(Deadline::new(LocalDate::from_ymd(2026, 6, 20).unwrap())),
            reminder: None,
            context: ContextRef::Inbox,
            completed_at: None,
            canceled_at: None,
            logged_at: None,
            created_at: chrono::Utc::now(),
            version: 1,
            checklist: vec![ChecklistItemSummary {
                id: ChecklistItemId::new(),
                title: "Step".to_string(),
                completed: true,
            }],
            tags: Vec::new(),
            inherited_tags: Vec::new(),
        }
    }

    #[test]
    fn markdown_models_are_deterministic_and_escape_controls() {
        let item = todo("*Bold* [link]");
        let first = share_todo_markdown(&item);
        let second = share_todo_markdown(&item);
        assert_eq!(first, second);
        assert!(first.contains("# \\*Bold\\* \\[link\\]"), "{first}");
        assert!(first.contains("when: 2026-06-12"), "{first}");
        assert!(first.contains("deadline: 2026-06-20"), "{first}");
        assert!(first.contains("- [x] Step"), "{first}");
        assert_eq!(escape_markdown("a_b"), "a\\_b");

        let list = share_list_markdown("Today", std::slice::from_ref(&item));
        assert!(list.starts_with("# Today\n\n- [ ] \\*Bold\\* \\[link\\] (when: 2026-06-12)"));
    }

    #[test]
    fn print_documents_escape_untrusted_text() {
        let item = todo("<script>alert(1)</script>");
        let html = print_list_html("Today & Tomorrow", std::slice::from_ref(&item));
        assert!(
            html.contains("<title>Today &amp; Tomorrow</title>"),
            "{html}"
        );
        assert!(
            html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"),
            "{html}"
        );
        assert!(!html.contains("<script>alert(1)</script>"), "{html}");
        assert_eq!(
            escape_html("\"quoted\" & <tag>"),
            "&quot;quoted&quot; &amp; &lt;tag&gt;"
        );
    }
}
