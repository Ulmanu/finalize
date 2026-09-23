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

pub mod app;

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
