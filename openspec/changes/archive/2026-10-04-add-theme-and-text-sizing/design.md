# Design — add-theme-and-text-sizing

## Context

The design tokens already support `prefers-color-scheme` and `prefers-reduced-motion`; the
settings registry (63) already declares `appearance.theme`. This change completes the appearance
domain with text size and motion, and adds explicit theme overrides.

## Requirements Driving the Design

- `Theme choice`
- `Text size`
- `Contrast/reduced motion compatibility`

## Architecture

- Explicit theme: `[data-theme="dark"]` re-declares the dark tokens and `[data-theme="light"]`
  pins `color-scheme`; the system media query is scoped to `:root:not([data-theme="light"])` so a
  forced light theme wins over a dark OS preference, while System follows the OS untouched.
- Text size: `html[data-text-size="large"|"extra_large"]` scales the root font size; all type
  tokens are rem-based so layouts scale without structural changes.
- Accessibility: `prefers-contrast: more` strengthens borders and muted text for both themes, the
  existing `prefers-reduced-motion` queries remain, and `[data-reduced-motion="true"]` disables
  transitions/animations when the user opts in beyond the system setting.
- Preferences are browser-local and validated by the registry; the hydrated client applies them
  by setting `data-theme`, `data-text-size`, and `data-reduced-motion` on the root.

## Data and Persistence

- None server-side for appearance; browser-local storage is the client's job.

## Offline and Sync

- Not applicable; appearance is device/browser state.

## UI and Accessibility

- Settings exposes labeled selects; contrast and motion preferences remain compatible with
  system accessibility settings rather than overriding them.

## Error Handling

- Invalid values are rejected by the registry; unknown persisted values fall back to defaults.

## Testing Strategy

- Existing registry unit tests cover the new entries; web tests assert the rendered preference
  keys/scopes and the presence of theme, text-size, contrast, and reduced-motion rules in the
  served stylesheets, plus browser-scope acknowledgement without server storage.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: browser-local appearance preferences are applied by the hydrated client via root
  attributes; the server ships the complete CSS contract so the client needs no style logic.
