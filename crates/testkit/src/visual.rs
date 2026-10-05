//! Deterministic visual-parity primitives: HTML normalization, baselines, and diff reports.
//!
//! Server-rendered pages are captured at fixed viewports as canonicalized structural snapshots
//! ("screenshots"): random identifiers, nonces, hydration script contents, and emails are
//! replaced with stable placeholders, attributes are sorted, and whitespace is collapsed, so the
//! same seeded data always produces the same bytes. Baselines live in a committed JSON file and
//! mismatches produce inspectable expected/actual/diff artifacts with a configurable threshold.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// A fixed viewport class in the fixture contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewportClass {
    /// Desktop viewport.
    Desktop,
    /// Tablet viewport.
    Tablet,
    /// Mobile viewport.
    Mobile,
}

impl ViewportClass {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Tablet => "tablet",
            Self::Mobile => "mobile",
        }
    }
}

/// A fixed viewport definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
    /// Width in CSS pixels.
    pub width: u32,
    /// Height in CSS pixels.
    pub height: u32,
    /// Viewport class.
    pub class: ViewportClass,
}

impl Viewport {
    /// Desktop baseline viewport.
    pub const DESKTOP: Self = Self {
        width: 1440,
        height: 900,
        class: ViewportClass::Desktop,
    };
    /// Tablet baseline viewport.
    pub const TABLET: Self = Self {
        width: 834,
        height: 1112,
        class: ViewportClass::Tablet,
    };
    /// Mobile baseline viewport.
    pub const MOBILE: Self = Self {
        width: 390,
        height: 844,
        class: ViewportClass::Mobile,
    };
}

/// One visual fixture: a page captured at a fixed viewport.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualFixture {
    /// Stable fixture name (`page-viewport`).
    pub name: String,
    /// Application path.
    pub path: String,
    /// Fixed viewport.
    pub viewport: Viewport,
}

impl VisualFixture {
    /// Builds a fixture name from its parts.
    #[must_use]
    pub fn new(path: &str, viewport: Viewport) -> Self {
        let page = path.trim_matches('/').replace('/', "-");
        let page = if page.is_empty() {
            "home".to_string()
        } else {
            page
        };
        Self {
            name: format!("{page}-{}", viewport.class.as_str()),
            path: path.to_string(),
            viewport,
        }
    }
}

/// Committed baseline snapshots.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualBaseline {
    /// Baseline format version.
    pub version: u32,
    /// Snapshots keyed by fixture name.
    pub snapshots: BTreeMap<String, String>,
}

/// Current baseline format version.
pub const BASELINE_VERSION: u32 = 1;

impl VisualBaseline {
    /// Loads a baseline from JSON, when the file exists.
    ///
    /// # Errors
    ///
    /// Propagates read/parse failures.
    pub fn load(path: &Path) -> Result<Option<Self>, String> {
        if !path.exists() {
            return Ok(None);
        }
        let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
        serde_json::from_str(&text)
            .map(Some)
            .map_err(|error| error.to_string())
    }

    /// Saves the baseline as pretty JSON.
    ///
    /// # Errors
    ///
    /// Propagates write failures.
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let text = serde_json::to_string_pretty(self).map_err(|error| error.to_string())?;
        std::fs::write(path, text).map_err(|error| error.to_string())
    }
}

/// A structural mismatch report.
#[derive(Debug, Clone, PartialEq)]
pub struct DiffReport {
    /// Fixture name.
    pub fixture: String,
    /// Fraction of differing lines (0.0..=1.0).
    pub ratio: f64,
    /// Expected snapshot lines.
    pub expected: Vec<String>,
    /// Actual snapshot lines.
    pub actual: Vec<String>,
}

impl DiffReport {
    /// A unified-style textual diff for artifacts.
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = format!("fixture: {} (diff ratio {:.4})\n", self.fixture, self.ratio);
        for line in &self.expected {
            if !self.actual.contains(line) {
                out.push_str(&format!("- {line}\n"));
            }
        }
        for line in &self.actual {
            if !self.expected.contains(line) {
                out.push_str(&format!("+ {line}\n"));
            }
        }
        out
    }
}

/// Compares a baseline snapshot with an actual snapshot.
///
/// Returns a report when the differing-line ratio exceeds `threshold`.
#[must_use]
pub fn compare(fixture: &str, expected: &str, actual: &str, threshold: f64) -> Option<DiffReport> {
    if expected == actual {
        return None;
    }
    let expected_lines: Vec<String> = expected.lines().map(str::to_string).collect();
    let actual_lines: Vec<String> = actual.lines().map(str::to_string).collect();
    let total = expected_lines.len().max(actual_lines.len()).max(1);
    let differing = expected_lines
        .iter()
        .filter(|line| !actual_lines.contains(line))
        .count()
        + actual_lines
            .iter()
            .filter(|line| !expected_lines.contains(line))
            .count();
    let ratio = (differing as f64) / (total as f64);
    (ratio > threshold).then(|| DiffReport {
        fixture: fixture.to_string(),
        ratio,
        expected: expected_lines,
        actual: actual_lines,
    })
}

/// Writes expected/actual/diff artifacts for one failure.
///
/// # Errors
///
/// Propagates filesystem failures.
pub fn write_artifacts(dir: &Path, report: &DiffReport) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    std::fs::write(
        dir.join(format!("{}.expected.html", report.fixture)),
        report.expected.join("\n"),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        dir.join(format!("{}.actual.html", report.fixture)),
        report.actual.join("\n"),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        dir.join(format!("{}.diff.txt", report.fixture)),
        report.render(),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

/// Canonicalizes server-rendered HTML into a deterministic structural snapshot.
#[must_use]
pub fn normalize_html(body: &str) -> String {
    // 1. Drop script contents entirely (hydration bootstrap, resource bookkeeping).
    let mut text = strip_elements(body, "<script", "</script>");
    // 2. Replace dynamic identifiers and emails with placeholders.
    text = replace_uuids(&text);
    text = replace_emails(&text);
    text = replace_hex_tokens(&text);
    text = replace_attribute(&text, "nonce");
    text = replace_attribute(&text, "crossorigin");
    // 3. Sort attributes within each tag.
    text = sort_attributes(&text);
    // 4. Collapse whitespace between tags and trim.
    text = collapse_whitespace(&text);
    text.trim().to_string()
}

fn strip_elements(body: &str, open: &str, close: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut rest = body;
    while let Some(start) = rest.find(open) {
        out.push_str(&rest[..start]);
        let Some(end) = rest[start..].find(close) else {
            out.push_str(&rest[start..]);
            return out;
        };
        out.push_str("<script></script>");
        rest = &rest[start + end + close.len()..];
    }
    out.push_str(rest);
    out
}

fn replace_uuids(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if is_uuid_at(&chars, index) {
            out.push_str("[id]");
            index += 36;
        } else {
            out.push(chars[index]);
            index += 1;
        }
    }
    out
}

fn is_uuid_at(chars: &[char], index: usize) -> bool {
    if index + 36 > chars.len() {
        return false;
    }
    for offset in 0..36 {
        let character = chars[index + offset];
        match offset {
            8 | 13 | 18 | 23 => {
                if character != '-' {
                    return false;
                }
            }
            _ => {
                if !character.is_ascii_hexdigit() {
                    return false;
                }
            }
        }
    }
    true
}

fn replace_emails(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut index = 0;
    while index < chars.len() {
        if is_email_char(chars[index]) {
            let mut at = index;
            while at < chars.len() && is_email_char(chars[at]) {
                at += 1;
            }
            if at < chars.len() && chars[at] == '@' {
                let mut end = at + 1;
                while end < chars.len() && is_email_char(chars[end]) {
                    end += 1;
                }
                if chars[at + 1..end].contains(&'.') {
                    out.push_str("[email]");
                    index = end;
                    continue;
                }
            }
        }
        out.push(chars[index]);
        index += 1;
    }
    out
}

fn is_email_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '%' | '+' | '-')
}

fn replace_hex_tokens(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut index = 0;
    while index < chars.len() {
        if chars[index].is_ascii_hexdigit() {
            let mut end = index;
            while end < chars.len() && chars[end].is_ascii_hexdigit() {
                end += 1;
            }
            if end - index >= 32 {
                out.push_str("[token]");
                index = end;
                continue;
            }
        }
        out.push(chars[index]);
        index += 1;
    }
    out
}

fn replace_attribute(text: &str, attribute: &str) -> String {
    let needle = format!("{attribute}=\"");
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(&needle) {
        out.push_str(&rest[..start]);
        let after = &rest[start + needle.len()..];
        let Some(end) = after.find('"') else {
            out.push_str(&rest[start..]);
            return out;
        };
        out.push_str(&format!("{attribute}=\"[value]\""));
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

fn sort_attributes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('<') {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        let Some(end) = find_tag_end(after) else {
            out.push_str(after);
            return out;
        };
        let tag = &after[..=end];
        out.push_str(&canonical_tag(tag));
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

/// Finds the closing `>` of a tag, ignoring `>` characters inside quoted values.
fn find_tag_end(text: &str) -> Option<usize> {
    let mut in_quotes = false;
    for (index, character) in text.char_indices() {
        match character {
            '"' => in_quotes = !in_quotes,
            '>' if !in_quotes => return Some(index),
            _ => {}
        }
    }
    None
}

fn canonical_tag(tag: &str) -> String {
    if tag.starts_with("</") || tag.starts_with("<!") || tag.starts_with("<?") {
        return tag.to_string();
    }
    let inner = tag
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim_end_matches('/');
    let Some(space) = inner.find(char::is_whitespace) else {
        return format!("<{inner}>");
    };
    let name = &inner[..space];
    let mut attributes: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for character in inner[space..].chars() {
        match character {
            '"' => {
                in_quotes = !in_quotes;
                current.push(character);
            }
            whitespace if whitespace.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    attributes.push(std::mem::take(&mut current));
                }
            }
            other => current.push(other),
        }
    }
    if !current.is_empty() {
        attributes.push(current);
    }
    attributes.sort();
    let self_closing = tag.trim_end().ends_with("/>");
    format!(
        "<{name}{}{}{}>",
        if attributes.is_empty() { "" } else { " " },
        attributes.join(" "),
        if self_closing { "/" } else { "" }
    )
}

fn collapse_whitespace(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last_space = false;
    for character in text.chars() {
        if character.is_whitespace() {
            if !last_space {
                out.push(' ');
                last_space = true;
            }
        } else {
            out.push(character);
            last_space = false;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        compare, normalize_html, replace_emails, write_artifacts, Viewport, VisualBaseline,
        VisualFixture, BASELINE_VERSION,
    };

    #[test]
    fn normalization_is_deterministic_and_strips_dynamic_values() {
        let html = "<div id=\"a\" class=\"b\" nonce=\"abc123\" data-x=\"1\">\
            <script nonce=\"abc123\">var x = 1;</script>\
            <span class=\"c\" id=\"d\">user@example.com</span>\
            <span>01a0e396-56d6-702b-966f-a2df72f8467e</span>\
            <span>0123456789abcdef0123456789abcdef</span>\
            </div>";
        let first = normalize_html(html);
        let second = normalize_html(html);
        assert_eq!(first, second);
        assert!(first.contains("<script></script>"), "{first}");
        assert!(first.contains("[email]"), "{first}");
        assert!(first.contains("[id]"), "{first}");
        assert!(first.contains("[token]"), "{first}");
        assert!(!first.contains("abc123"), "{first}");
        // Attributes are sorted alphabetically within the tag.
        let class_at = first.find("class=\"b\"").expect("class");
        let id_at = first.find("id=\"a\"").expect("id");
        assert!(class_at < id_at, "{first}");
    }

    #[test]
    fn compare_and_artifacts_report_inspectable_differences() {
        let expected = "<p>same</p>\n<p>old</p>";
        let actual = "<p>same</p>\n<p>new</p>";
        assert!(compare("fixture", expected, expected, 0.0).is_none());
        let report = compare("fixture", expected, actual, 0.0).expect("diff");
        assert!(report.ratio > 0.0);
        assert!(
            report.render().contains("- <p>old</p>"),
            "{}",
            report.render()
        );
        assert!(
            report.render().contains("+ <p>new</p>"),
            "{}",
            report.render()
        );

        let dir = std::env::temp_dir().join(format!("visual-artifacts-{}", std::process::id()));
        write_artifacts(&dir, &report).expect("artifacts");
        assert!(dir.join("fixture.diff.txt").exists());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn baselines_round_trip_and_fixtures_name_viewports() {
        let mut baseline = VisualBaseline {
            version: BASELINE_VERSION,
            snapshots: Default::default(),
        };
        baseline
            .snapshots
            .insert("inbox-desktop".to_string(), "<html></html>".to_string());
        let path =
            std::env::temp_dir().join(format!("visual-baseline-{}.json", std::process::id()));
        baseline.save(&path).expect("save");
        let loaded = VisualBaseline::load(&path)
            .expect("load")
            .expect("baseline");
        assert_eq!(loaded, baseline);
        let _ = std::fs::remove_file(path);

        let fixture = VisualFixture::new("/inbox", Viewport::MOBILE);
        assert_eq!(fixture.name, "inbox-mobile");
        assert_eq!(fixture.viewport.width, 390);
        let _ = replace_emails("x y@z.example");
    }
}
