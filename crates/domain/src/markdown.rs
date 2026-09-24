//! Things-style Markdown subset parser.
//!
//! Produces a structured AST so the UI layer controls escaping and link policy. Parsing never
//! mutates or copies the source into a stored rich representation.

use crate::notes::is_safe_url;

/// A link attached to an inline span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownLink {
    /// Destination URL.
    pub url: String,
    /// Whether the destination passes the safe-navigation policy.
    pub safe: bool,
}

/// One styled run of inline text.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InlineSpan {
    /// Literal text.
    pub text: String,
    /// `*emphasis*` or `_emphasis_`.
    pub emphasis: bool,
    /// `**strong**` or `__strong__`.
    pub strong: bool,
    /// `==highlight==`.
    pub highlight: bool,
    /// Inline `` `code` ``.
    pub code: bool,
    /// Optional link.
    pub link: Option<MarkdownLink>,
}

/// A parsed block-level element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    /// `# ` through `###### ` heading.
    Heading {
        /// Heading level 1-6.
        level: u8,
        /// Inline content.
        spans: Vec<InlineSpan>,
    },
    /// A paragraph of inline content.
    Paragraph {
        /// Inline content.
        spans: Vec<InlineSpan>,
    },
    /// A list item, ordered or not.
    ListItem {
        /// Whether the marker is numeric.
        ordered: bool,
        /// Original marker text (for example `-` or `1.`).
        marker: String,
        /// Inline content.
        spans: Vec<InlineSpan>,
    },
    /// A blockquote line.
    Quote {
        /// Inline content.
        spans: Vec<InlineSpan>,
    },
    /// A fenced code block.
    CodeBlock {
        /// Optional info string.
        language: Option<String>,
        /// Verbatim code text.
        text: String,
    },
}

/// Parses source text into blocks.
#[must_use]
pub fn parse_markdown(source: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut lines = source.lines().peekable();
    let mut paragraph: Vec<&str> = Vec::new();

    let flush_paragraph = |paragraph: &mut Vec<&str>, blocks: &mut Vec<Block>| {
        if !paragraph.is_empty() {
            let text = paragraph.join("\n");
            blocks.push(Block::Paragraph {
                spans: parse_inline(&text),
            });
            paragraph.clear();
        }
    };

    while let Some(line) = lines.next() {
        let trimmed = line.trim_end();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            flush_paragraph(&mut paragraph, &mut blocks);
            let fence = trimmed.get(..3).unwrap_or("```");
            let language = trimmed
                .get(3..)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string);
            let mut code = String::new();
            for code_line in lines.by_ref() {
                if code_line.trim_end().starts_with(fence) {
                    break;
                }
                if !code.is_empty() {
                    code.push('\n');
                }
                code.push_str(code_line);
            }
            blocks.push(Block::CodeBlock {
                language,
                text: code,
            });
            continue;
        }
        if trimmed.trim().is_empty() {
            flush_paragraph(&mut paragraph, &mut blocks);
            continue;
        }
        if let Some((level, text)) = parse_heading(trimmed) {
            flush_paragraph(&mut paragraph, &mut blocks);
            blocks.push(Block::Heading {
                level,
                spans: parse_inline(text),
            });
            continue;
        }
        if let Some((marker, text)) = parse_list_marker(trimmed) {
            flush_paragraph(&mut paragraph, &mut blocks);
            blocks.push(Block::ListItem {
                ordered: marker.ends_with('.'),
                marker,
                spans: parse_inline(text),
            });
            continue;
        }
        if let Some(text) = trimmed.strip_prefix('>') {
            flush_paragraph(&mut paragraph, &mut blocks);
            blocks.push(Block::Quote {
                spans: parse_inline(text.trim_start()),
            });
            continue;
        }
        paragraph.push(trimmed);
    }
    flush_paragraph(&mut paragraph, &mut blocks);
    blocks
}

fn parse_heading(line: &str) -> Option<(u8, &str)> {
    let hashes = line.len() - line.trim_start_matches('#').len();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let rest = line.get(hashes..)?;
    if !rest.starts_with(' ') {
        return None;
    }
    Some((u8::try_from(hashes).unwrap_or(1), rest.trim_start()))
}

fn parse_list_marker(line: &str) -> Option<(String, &str)> {
    for marker in ["- ", "* ", "+ "] {
        if let Some(rest) = line.strip_prefix(marker) {
            return Some((marker.trim_end().to_string(), rest));
        }
    }
    let digits = line.len()
        - line
            .trim_start_matches(|ch: char| ch.is_ascii_digit())
            .len();
    if digits > 0 {
        let rest = line.get(digits..)?;
        if let Some(content) = rest.strip_prefix(". ") {
            return Some((format!("{}.", line.get(..digits)?), content));
        }
    }
    None
}

/// Parses inline content into styled spans.
#[must_use]
pub fn parse_inline(text: &str) -> Vec<InlineSpan> {
    let mut spans = Vec::new();
    let mut literal = String::new();
    let mut index = 0;
    let chars: Vec<char> = text.chars().collect();

    while index < chars.len() {
        let ch = chars[index];
        if ch == '\\' && index + 1 < chars.len() {
            literal.push(chars[index + 1]);
            index += 2;
            continue;
        }
        if ch == '`' {
            if let Some(end) = find_char(&chars, index + 1, '`') {
                push_literal(&mut spans, &mut literal);
                let code: String = chars[index + 1..end].iter().collect();
                spans.push(InlineSpan {
                    text: code,
                    code: true,
                    ..InlineSpan::default()
                });
                index = end + 1;
                continue;
            }
        }
        if ch == '[' {
            if let Some((label, url, end)) = parse_link(&chars, index) {
                push_literal(&mut spans, &mut literal);
                let mut inner = parse_inline(&label);
                let link = MarkdownLink {
                    safe: is_safe_url(&url),
                    url,
                };
                if inner.is_empty() {
                    inner.push(InlineSpan {
                        text: label,
                        ..InlineSpan::default()
                    });
                }
                for span in inner {
                    spans.push(InlineSpan {
                        link: Some(link.clone()),
                        ..span
                    });
                }
                index = end;
                continue;
            }
        }
        let (marker, style): (&str, fn(&mut InlineSpan)) = if starts_with(&chars, index, "**") {
            ("**", |span: &mut InlineSpan| span.strong = true)
        } else if starts_with(&chars, index, "__") {
            ("__", |span: &mut InlineSpan| span.strong = true)
        } else if starts_with(&chars, index, "==") {
            ("==", |span: &mut InlineSpan| span.highlight = true)
        } else if ch == '*' {
            ("*", |span: &mut InlineSpan| span.emphasis = true)
        } else if ch == '_' {
            ("_", |span: &mut InlineSpan| span.emphasis = true)
        } else {
            literal.push(ch);
            index += 1;
            continue;
        };
        let marker_len = marker.chars().count();
        if let Some(close) = find_marker(&chars, index + marker_len, marker) {
            let inner: String = chars[index + marker_len..close].iter().collect();
            if !inner.is_empty() {
                push_literal(&mut spans, &mut literal);
                for mut span in parse_inline(&inner) {
                    style(&mut span);
                    spans.push(span);
                }
                index = close + marker_len;
                continue;
            }
        }
        literal.push_str(marker);
        index += marker_len;
    }
    push_literal(&mut spans, &mut literal);
    spans
}

fn push_literal(spans: &mut Vec<InlineSpan>, literal: &mut String) {
    if !literal.is_empty() {
        spans.push(InlineSpan {
            text: std::mem::take(literal),
            ..InlineSpan::default()
        });
    }
}

fn starts_with(chars: &[char], index: usize, marker: &str) -> bool {
    let marker: Vec<char> = marker.chars().collect();
    index + marker.len() <= chars.len() && chars[index..index + marker.len()] == marker[..]
}

fn find_char(chars: &[char], from: usize, target: char) -> Option<usize> {
    chars
        .iter()
        .enumerate()
        .skip(from)
        .find(|(_, ch)| **ch == target)
        .map(|(index, _)| index)
}

fn find_marker(chars: &[char], from: usize, marker: &str) -> Option<usize> {
    let marker: Vec<char> = marker.chars().collect();
    if marker.is_empty() {
        return None;
    }
    let mut index = from;
    while index + marker.len() <= chars.len() {
        if chars[index..index + marker.len()] == marker[..] {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn parse_link(chars: &[char], start: usize) -> Option<(String, String, usize)> {
    let close_bracket = find_char(chars, start + 1, ']')?;
    if chars.get(close_bracket + 1) != Some(&'(') {
        return None;
    }
    let close_paren = find_char(chars, close_bracket + 2, ')')?;
    let label: String = chars[start + 1..close_bracket].iter().collect();
    let url: String = chars[close_bracket + 2..close_paren].iter().collect();
    Some((label, url.trim().to_string(), close_paren + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_block_constructs() {
        let source = "# Heading\n\nA paragraph.\n\n- one\n- two\n\n1. first\n\n> quote\n\n```rust\nfn main() {}\n```";
        let blocks = parse_markdown(source);
        assert!(matches!(&blocks[0], Block::Heading { level: 1, .. }));
        assert!(matches!(&blocks[1], Block::Paragraph { .. }));
        assert!(matches!(&blocks[2], Block::ListItem { ordered: false, .. }));
        assert!(
            matches!(&blocks[4], Block::ListItem { ordered: true, marker, .. } if marker == "1.")
        );
        assert!(matches!(&blocks[5], Block::Quote { .. }));
        assert!(
            matches!(&blocks[6], Block::CodeBlock { language, text } if language.as_deref() == Some("rust") && text == "fn main() {}")
        );
    }

    #[test]
    fn parses_inline_styles_and_links() {
        let spans = parse_inline(
            "**bold** and *em* and ==mark== and `code` and [docs](https://example.com)",
        );
        assert!(spans.iter().any(|span| span.strong && span.text == "bold"));
        assert!(spans.iter().any(|span| span.emphasis && span.text == "em"));
        assert!(spans
            .iter()
            .any(|span| span.highlight && span.text == "mark"));
        assert!(spans.iter().any(|span| span.code && span.text == "code"));
        let link = spans
            .iter()
            .find(|span| span.link.is_some())
            .expect("link span");
        assert_eq!(link.link.as_ref().expect("link").url, "https://example.com");
        assert!(link.link.as_ref().expect("link").safe);
    }

    #[test]
    fn unsafe_links_and_malformed_syntax_degrade_safely() {
        let spans = parse_inline("[click](javascript:alert(1)) and **unclosed");
        let link = spans
            .iter()
            .find(|span| span.link.is_some())
            .expect("link span");
        assert!(!link.link.as_ref().expect("link").safe);
        assert!(
            spans.iter().any(|span| span.text.contains("**unclosed")),
            "malformed markers stay literal: {spans:?}"
        );
        assert_eq!(
            parse_inline("escaped \\*not emphasis\\*"),
            vec![InlineSpan {
                text: "escaped *not emphasis*".to_string(),
                ..InlineSpan::default()
            }]
        );
    }

    #[test]
    fn parsing_is_lossless_for_literal_text() {
        let source = "plain text with #hash and - dash";
        let blocks = parse_markdown(source);
        let rendered: String = match &blocks[0] {
            Block::Paragraph { spans } => spans.iter().map(|span| span.text.as_str()).collect(),
            other => panic!("expected paragraph, got {other:?}"),
        };
        assert_eq!(rendered, "plain text with #hash and - dash");
    }
}
