# Design — add-calendar-integration

## Context

`UpcomingAgenda.calendar` existed as an always-empty placeholder ("empty until a calendar
integration exists") and Today rendered no calendar surface.

## Requirements Driving the Design

- `Provider-neutral calendar API`
- `Read-only projection`
- `Provider resilience`

## Architecture

- Core depends only on `CalendarProvider::events(account, range)`; provider types never cross the
  contract. `CalendarError` distinguishes auth expiry, network, and provider failures.
- `CalendarService::events` returns a projection: on success it refreshes an in-memory cache keyed
  by `(account, range)`; on failure it returns the cached events (possibly empty) with `degraded`
  and `stale` flags. Task reads never call the provider, so task data cannot be blocked.
- Today renders a `CalendarSection` for today's range; Upcoming merges rows into the agenda after
  its todo projection and surfaces the degraded banner above the calendar section. Rows are
  `task-row-readonly` with `data-external="true"` and no actions.

## Data and Persistence

- No schema: the projection cache is process-local and never authoritative; calendar events remain
  provider-owned read-only data.

## Offline and Sync

- Providers are external; the cache tolerates outages. No calendar state enters the change log.

## UI and Accessibility

- External rows are clearly marked (read-only class, external attribute, calendar-name badge) and
  excluded from task interactions; the degraded banner explains stale data.

## Error Handling

- Every provider failure degrades with a typed reason; without a cache the projection is empty and
  still non-blocking.

## Testing Strategy

- Testkit tests for range filtering/read-only flags, cache degradation across failures and
  recovery, and failure-without-cache; web tests for Today rows, Upcoming rows plus degraded
  banner, and task data surviving a network failure.

## Rollout / Migration

- None required; absent a provider the projection is simply empty.

## Open Questions the Agent Must Resolve From the Repository

- Resolved: the network adapters (Google/Microsoft/CalDAV) and OAuth are deliberately not built
  here — no credential store or HTTP client policy exists yet; the contract plus resilience plus
  UI are complete, and adapters implement `CalendarProvider` without touching core code.
