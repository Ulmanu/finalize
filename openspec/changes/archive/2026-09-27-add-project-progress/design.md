# Design — add-project-progress

## Context

`ProjectSummary::progress()` existed but counted canceled children as completed and had no
neutral-state UI; Today had no progress read at all.

## Requirements Driving the Design

- `Project progress`
- `Today progress`
- `Edge-case policy`

## Architecture

- Eligibility: project progress counts direct project-context children with status Open or
  Completed; canceled and deleted children are excluded from both sides of the ratio. Zero
  eligible children → `None`/neutral.
- `Progress { completed, total }` (application layer) centralizes `ratio`, `percent`, and
  `is_neutral` so every surface uses identical rounding and neutral semantics.
- `TodoScope::TodayAll` reads open + completed date-scheduled items on or before the local
  today; `TodoService::today_progress` derives the indicator and excludes canceled/deleted/
  future/Someday rows by construction.
- The UI renders native `<progress value max>` elements with text and aria labels, so the
  indicator is accessible without custom ARIA or handwritten script.

## Data and Persistence

- None; progress is always derived on read.

## Offline and Sync

- Read-only change; projection-only UI state is never persisted.

## UI and Accessibility

- `<progress>` plus a percentage label per surface; neutral states render explicit text.

## Error Handling

- Storage failures surface as absent indicators (existing empty-list behavior).

## Testing Strategy

- Testkit tests for derived project ratios, canceled/deleted exclusion, zero-item neutrality,
  and Today progress updates including reopen.
- Web tests for project/Today indicators and neutral states.

## Rollout / Migration

- None required.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: canceled work is excluded from progress rather than counted as done, matching the
  distinction established by the completion capability.
