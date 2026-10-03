//! Leptos view tree shared by the server and the WASM client.

use leptos::prelude::*;

use crate::responsive::LayoutState;
use crate::routes::AppRoutes;
use crate::session::{provide_session, SessionBootstrap};

/// Root application component.
///
/// Provides the session signal, renders the session bootstrap element used during hydration,
/// and mounts the route tree.
#[component]
pub fn App() -> impl IntoView {
    let session = provide_session();
    LayoutState::provide();
    view! {
        <SessionBootstrap session=session.get_untracked()/>
        <AppRoutes/>
    }
}

/// Server-rendered document shell used by the Axum server.
#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    use leptos::hydration::HydrationScripts;

    let product = momentum_domain::PRODUCT_NAME;
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>{product}</title>
                <HydrationScripts options=options.clone()/>
                <link rel="stylesheet" href="/static/design.css"/>
                <link rel="stylesheet" href="/static/app.css"/>
                <link rel="manifest" href="/manifest.webmanifest"/>
                <link rel="icon" href="/static/icon.svg" type="image/svg+xml"/>
                <meta name="theme-color" content="#1d4ed8"/>
                <script inner_html=crate::pwa::REGISTRATION_SNIPPET></script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
