//! Leptos view tree shared by the server and the WASM client.

use leptos::prelude::*;

/// Root application component.
///
/// Change 01 renders the application foundation; routing and the authenticated shell are
/// added by the web-application-shell change.
#[component]
pub fn App() -> impl IntoView {
    let info = momentum_app::build_info();
    view! {
        <main class="app-shell" data-testid="app-root">
            <h1 class="app-title">{info.product}</h1>
            <p class="app-tagline">"A Things-style task manager, built in Rust."</p>
            <p class="app-version" data-testid="app-version">
                {format!("v{}", info.version)}
            </p>
        </main>
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
                <link rel="stylesheet" href="/pkg/momentum-web.css"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
