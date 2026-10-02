//! Browser/PWA capture: sanitizing untrusted shared page data into a capture draft.
//!
//! Shared titles, URLs, and selected text are third-party content: URLs must be http/https and
//! bounded, text is stripped of control characters and capped, and the composed notes keep the
//! URL first so the source is always identifiable.

/// Maximum captured title length.
pub const MAX_TITLE: usize = 200;
/// Maximum captured text length.
pub const MAX_TEXT: usize = 4000;
/// Maximum captured URL length.
pub const MAX_URL: usize = 2048;

/// Sanitizes an external URL; only `http`/`https` survive, controls are stripped, length capped.
#[must_use]
pub fn sanitize_url(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.len() > MAX_URL {
        return None;
    }
    let lower = trimmed.to_ascii_lowercase();
    if !(lower.starts_with("http://") || lower.starts_with("https://")) {
        return None;
    }
    let cleaned: String = trimmed
        .chars()
        .filter(|character| !character.is_control())
        .collect();
    (!cleaned.is_empty()).then_some(cleaned)
}

/// Sanitizes external text: control characters removed, whitespace collapsed, length capped.
#[must_use]
pub fn sanitize_text(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for character in raw.chars() {
        match character {
            '\n' | '\r' | '\t' => out.push(' '),
            control if control.is_control() => {}
            other => out.push(other),
        }
    }
    out.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(MAX_TEXT)
        .collect()
}

/// Composes a sanitized capture draft: title (with fallback) and notes (URL first, then text).
#[must_use]
pub fn build_capture(title: &str, url: Option<&str>, text: Option<&str>) -> (String, String) {
    let title = sanitize_text(title);
    let title = if title.is_empty() {
        "Captured page".to_string()
    } else {
        title.chars().take(MAX_TITLE).collect()
    };
    let mut notes = Vec::new();
    if let Some(url) = url.and_then(sanitize_url) {
        notes.push(url);
    }
    if let Some(text) = text.map(sanitize_text).filter(|text| !text.is_empty()) {
        notes.push(text);
    }
    (title, notes.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::{build_capture, sanitize_text, sanitize_url, MAX_TEXT};

    #[test]
    fn urls_allow_only_http_and_https() {
        assert_eq!(
            sanitize_url("  https://example.com/page?q=1  ").as_deref(),
            Some("https://example.com/page?q=1")
        );
        assert_eq!(
            sanitize_url("http://example.com").as_deref(),
            Some("http://example.com")
        );
        assert_eq!(sanitize_url("javascript:alert(1)"), None);
        assert_eq!(sanitize_url("data:text/html,<b>"), None);
        assert_eq!(sanitize_url("file:///etc/passwd"), None);
        let long = format!("https://example.com/{}", "a".repeat(4096));
        assert_eq!(sanitize_url(&long), None);
    }

    #[test]
    fn text_strips_controls_collapses_whitespace_and_caps() {
        assert_eq!(
            sanitize_text("Hello\u{7}\u{1}   world\n \t again"),
            "Hello world again"
        );
        assert_eq!(sanitize_text("  \n\t "), "");
        let long = "x".repeat(MAX_TEXT + 100);
        assert_eq!(sanitize_text(&long).len(), MAX_TEXT);
    }

    #[test]
    fn drafts_put_the_url_first_and_fall_back_on_empty_titles() {
        let (title, notes) = build_capture(
            "  Example\u{7} Page ",
            Some("https://example.com"),
            Some("Selected text"),
        );
        assert_eq!(title, "Example Page");
        assert_eq!(notes, "https://example.com\nSelected text");

        let (title, notes) = build_capture("", Some("javascript:alert(1)"), Some("Body\u{8}"));
        assert_eq!(title, "Captured page");
        assert_eq!(notes, "Body");
    }
}
