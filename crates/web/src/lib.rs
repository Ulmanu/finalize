//! Momentum web application: Leptos views, Axum server, and WASM client.
//!
//! The same view tree in [`app`] is compiled for two targets:
//!
//! * `ssr` (default) — server-side rendering used by the Axum server binary.
//! * `hydrate` — the Rust/WASM client that takes over the server-rendered markup.
//!
//! No handwritten JavaScript or TypeScript exists in this crate; the only browser script is
//! the WASM bootstrap emitted by the Rust toolchain.

#![forbid(unsafe_code)]
#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
pub mod api_routes;
pub mod app;
#[cfg(feature = "ssr")]
pub mod area_routes;
#[cfg(feature = "ssr")]
pub mod auth_routes;
pub mod boundaries;
#[cfg(feature = "ssr")]
pub mod capture_routes;
#[cfg(feature = "ssr")]
pub mod command_routes;
pub mod design;
#[cfg(feature = "ssr")]
pub mod export_import_routes;
#[cfg(feature = "ssr")]
pub mod inbound_routes;
pub mod layouts;
#[cfg(feature = "ssr")]
pub mod logbook_routes;
#[cfg(feature = "ssr")]
pub mod notes_routes;
#[cfg(feature = "ssr")]
pub mod preferences_routes;
#[cfg(feature = "ssr")]
pub mod project_routes;
#[cfg(feature = "ssr")]
pub mod push_routes;
pub mod pwa;
pub mod responsive;
pub mod routes;
#[cfg(feature = "ssr")]
pub mod search_routes;
#[cfg(feature = "ssr")]
pub mod security;
pub mod session;
#[cfg(feature = "ssr")]
pub mod share_routes;
#[cfg(feature = "ssr")]
pub mod sync_routes;
#[cfg(feature = "ssr")]
pub mod tag_routes;
#[cfg(feature = "ssr")]
pub mod todo_routes;

#[cfg(feature = "ssr")]
pub mod server;

/// Browser entry point for the WASM client.
///
/// This is compiled only for `wasm32-unknown-unknown` with the `hydrate` feature, so native
/// `--all-features` builds stay linkable.
#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
