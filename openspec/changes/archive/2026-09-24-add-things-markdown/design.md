# Design — add-things-markdown

## Context

Notes are plain text (change 18). Rendering must be deterministic, testable without a browser,
and must never mutate stored text, so parsing lives in `momentum_domain::markdown` and the web
layer only maps the AST to accessible elements.

## Requirements Driving the Design

- `Supported Markdown subset`
- `Syntax-visible editing`
- `Lossless storage`

## Architecture

- `markdown::parse_markdown(&str) -> Vec<Block>` where blocks are headings, paragraphs, list
  items, quotes, and fenced code blocks; inline content is `Vec<InlineSpan>` carrying emphasis,
  strong, highlight, code, and optional link flags.
- Inline parsing is a deterministic single pass with backslash escapes, code spans taking
  precedence, and recursive emphasis/strong/highlight inside link text.
- Links reuse `notes::is_safe_url`: safe links render as anchors with `noopener noreferrer`;
  unsafe schemes render as inert text with a blocked marker.
- The textarea keeps the raw source; the preview is derived and read-only, satisfying
  syntax-visible editing and lossless storage simultaneously.

## Data and Persistence

- None; no schema change and no new commands.

## Offline and Sync

- Not applicable: rendering is a pure projection of stored text.

## UI and Accessibility

- Semantic elements (`h3`-level headings, `ul`/`ol`, `blockquote`, `pre`/`code`, `strong`,
  `em`, `mark`, `code`) with design-system classes; preview is labeled for assistive tech.

## Error Handling

- Malformed syntax degrades to literal text; no panics, no partial state.

## Testing Strategy

- Domain unit tests for each supported construct, escaping, malformed input, and source
  preservation.
- Web tests that preview formatting renders, raw syntax stays in the textarea, and unsafe links
  never become hrefs.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: a structured AST (not an HTML string) is produced so the Leptos layer keeps control
  of escaping and link policy.
