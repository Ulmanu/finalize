//! Notes text utilities: link detection, URL policy, and find/replace.
//!
//! Everything here is pure and deterministic so the editor's observable behavior can be
//! unit-tested without a browser.

/// A detected link candidate inside note text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteLink {
    /// Byte offset of the candidate start.
    pub start: usize,
    /// Byte offset one past the candidate end.
    pub end: usize,
    /// Candidate URL (with `https://` prepended for `www.` targets).
    pub url: String,
    /// Whether the candidate passes the safe-navigation policy.
    pub safe: bool,
}

/// Returns whether a URL may be offered as a navigable link.
///
/// Only `http`, `https`, and `mailto` targets are allowed; script-capable or opaque schemes and
/// control characters are rejected.
#[must_use]
pub fn is_safe_url(url: &str) -> bool {
    let trimmed = url.trim();
    if trimmed.is_empty() || trimmed.chars().any(char::is_control) {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    (lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("mailto:"))
        && !trimmed.chars().any(char::is_whitespace)
}

/// Finds whitespace-delimited link candidates in note text in source order.
#[must_use]
pub fn find_links(text: &str) -> Vec<NoteLink> {
    let mut links = Vec::new();
    for (start, token) in token_spans(text) {
        let lower = token.to_ascii_lowercase();
        let (candidate, url, navigable) = if lower.starts_with("http://")
            || lower.starts_with("https://")
            || lower.starts_with("mailto:")
        {
            let trimmed = trim_trailing_punctuation(token);
            (trimmed, trimmed.to_string(), true)
        } else if lower.starts_with("www.") {
            let trimmed = trim_trailing_punctuation(token);
            (trimmed, format!("https://{trimmed}"), true)
        } else if let Some((scheme, _)) = token.split_once(':') {
            let scheme = scheme.to_ascii_lowercase();
            if !scheme.is_empty() && scheme.chars().all(|ch| ch.is_ascii_alphabetic()) {
                (token, token.to_string(), false)
            } else {
                continue;
            }
        } else {
            continue;
        };
        if candidate.is_empty() {
            continue;
        }
        let safe = navigable && is_safe_url(&url);
        links.push(NoteLink {
            start,
            end: start + candidate.len(),
            url,
            safe,
        });
    }
    links
}

fn trim_trailing_punctuation(token: &str) -> &str {
    token.trim_end_matches(|ch: char| {
        matches!(
            ch,
            '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}' | '"' | '\''
        )
    })
}

fn token_spans(text: &str) -> Vec<(usize, &str)> {
    let mut spans = Vec::new();
    let mut start = None;
    for (index, ch) in text.char_indices() {
        if ch.is_whitespace() {
            if let Some(begin) = start.take() {
                spans.push((begin, &text[begin..index]));
            }
        } else if start.is_none() {
            start = Some(index);
        }
    }
    if let Some(begin) = start {
        spans.push((begin, &text[begin..]));
    }
    spans
}

/// Returns every literal match of `needle` as `(start, end)` byte ranges.
#[must_use]
pub fn find_matches(text: &str, needle: &str) -> Vec<(usize, usize)> {
    if needle.is_empty() {
        return Vec::new();
    }
    text.match_indices(needle)
        .map(|(start, matched)| (start, start + matched.len()))
        .collect()
}

/// Returns the first match starting at or after `from`.
#[must_use]
pub fn next_match(text: &str, needle: &str, from: usize) -> Option<(usize, usize)> {
    find_matches(text, needle)
        .into_iter()
        .find(|(start, _)| *start >= from)
        .or_else(|| find_matches(text, needle).into_iter().next())
}

/// Returns the last match starting before `from`, wrapping to the final match.
#[must_use]
pub fn previous_match(text: &str, needle: &str, from: usize) -> Option<(usize, usize)> {
    let matches = find_matches(text, needle);
    matches
        .iter()
        .rev()
        .find(|(start, _)| *start < from)
        .or_else(|| matches.last())
        .copied()
}

/// Replaces every literal occurrence, returning the new text and replacement count.
#[must_use]
pub fn replace_all(text: &str, needle: &str, replacement: &str) -> (String, usize) {
    if needle.is_empty() {
        return (text.to_string(), 0);
    }
    let count = text.matches(needle).count();
    (text.replace(needle, replacement), count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_links_and_rejects_unsafe_schemes() {
        let text = "See https://example.com/docs and www.example.org, plus javascript:alert(1)";
        let links = find_links(text);
        assert_eq!(links.len(), 3);
        assert!(links[0].safe);
        assert_eq!(links[0].url, "https://example.com/docs");
        assert!(links[1].safe);
        assert_eq!(links[1].url, "https://www.example.org");
        assert!(!links[2].safe);
        assert_eq!(links[2].url, "javascript:alert(1)");
    }

    #[test]
    fn safe_url_policy_is_explicit() {
        assert!(is_safe_url("https://example.com"));
        assert!(is_safe_url("http://example.com"));
        assert!(is_safe_url("mailto:person@example.com"));
        assert!(!is_safe_url("javascript:alert(1)"));
        assert!(!is_safe_url("data:text/html,hi"));
        assert!(!is_safe_url("file:///etc/passwd"));
        assert!(!is_safe_url(""));
        assert!(!is_safe_url("https://exa mple.com"));
    }

    #[test]
    fn traversal_and_replacement_are_deterministic() {
        let text = "one two one two one";
        assert_eq!(find_matches(text, "one").len(), 3);
        assert_eq!(next_match(text, "one", 5), Some((8, 11)));
        assert_eq!(previous_match(text, "one", 8), Some((0, 3)));
        assert_eq!(next_match(text, "one", 99), Some((0, 3)), "wraps forward");
        assert_eq!(
            previous_match(text, "one", 0),
            Some((16, 19)),
            "wraps backward"
        );
        let (replaced, count) = replace_all(text, "one", "1");
        assert_eq!(count, 3);
        assert_eq!(replaced, "1 two 1 two 1");
        assert_eq!(replace_all(text, "", "x"), (text.to_string(), 0));
    }
}
