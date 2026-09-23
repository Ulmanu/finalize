# Proposal

## Why

The workspace foundation renders a single page. Every later capability needs stable,
bookmarkable destinations and a shell that survives direct loads and refreshes with
server-rendered content that becomes interactive in Rust/WASM. Without explicit loading,
not-found, unauthorized, and error boundaries, later features would each invent their own
partial behavior.

## What Changes

- Add Leptos Router routing with stable routes for Inbox, Today, Upcoming, Anytime, Someday,
  Logbook, Tomorrow/Deadlines/Repeating placeholders are deferred to their capability; the
  shell covers built-in lists, All Projects/Areas overview, individual projects/areas/to-dos,
  and a public landing plus sign-in route placeholder.
- Add public and authenticated layouts: a public layout for landing/sign-in, and an
  application layout with primary navigation used by protected routes.
- Add a session boundary: routes requiring an account render an unauthorized boundary when no
  authenticated session is present; session resolution is a provider-injected contract so the
  account-authentication change can supply the real cookie-backed resolver.
- Add loading, not-found, unauthorized, and recoverable error boundaries with accessible
  semantics.
- Wire Axum SSR route generation (`generate_route_list` + `leptos_routes_with_context`) so
  direct loads render the same logical destination and hydrate without duplicating state.

## Capabilities

### New Capabilities

- `app/shell`: route-addressable application shell with SSR/hydration continuity, public and
  authenticated layouts, session boundary, and loading/not-found/unauthorized/error boundaries.

### Modified Capabilities

(none)

## Impact

- `crates/web`: router, layouts, boundary components, session context, server route wiring.
- `crates/app`: session state contract (anonymous/authenticated) and account identity type
  reused by the authentication change.
- `crates/domain`: `AccountId` identity newtype.
- No persistence; no migrations. Auth credentials and cookie parsing remain out of scope
  (change 06).
- Non-goals: no handwritten JS/TS; no real authentication yet; no feature list content yet;
  no Cultured Code branding/assets.
