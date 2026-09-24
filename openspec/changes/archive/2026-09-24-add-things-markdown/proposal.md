# Proposal

## Why

Things-style notes are edited as raw Markdown but read as formatted text. The rendering must be
derived from the stored source so nothing is ever lost or destructively converted.

## What Changes

- Add a pure Rust Markdown subset parser producing a structured block/inline AST: emphasis,
  strong, highlight, headings, lists, quotes, code (fenced and inline), and links.
- Reuse the notes safe-URL policy so rendered links can never navigate to untrusted schemes.
- Render a syntax-visible preview under the notes textarea on to-do and project pages while the
  raw source stays editable.

## Capabilities

### New Capabilities

- `features/markdown`: supported Markdown subset rendering with syntax-visible editing and
  lossless storage.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (markdown parser + tests), `crates/web` (preview component).
- No schema change: rendering is derived from stored note text on every read.
- Non-goals: full CommonMark, HTML passthrough, WYSIWYG editing.
