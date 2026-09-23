//! Public and authenticated application layouts.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::session::{use_account, use_session};
use momentum_app::session::SessionState;

/// Centered layout used by landing, sign-in, and boundary pages.
#[component]
pub fn PublicLayout(children: Children) -> impl IntoView {
    let product = momentum_domain::PRODUCT_NAME;
    view! {
        <div class="public-layout" data-testid="public-layout">
            <header class="public-header">
                <a class="brand" href="/">{product}</a>
            </header>
            <main class="public-main">{children()}</main>
        </div>
    }
}

/// Navigation destination rendered in the primary sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavDestination {
    /// Route path.
    pub href: &'static str,
    /// Visible label.
    pub label: &'static str,
}

/// Built-in list destinations shown in the primary navigation.
pub const PRIMARY_DESTINATIONS: &[NavDestination] = &[
    NavDestination {
        href: "/inbox",
        label: "Inbox",
    },
    NavDestination {
        href: "/today",
        label: "Today",
    },
    NavDestination {
        href: "/upcoming",
        label: "Upcoming",
    },
    NavDestination {
        href: "/anytime",
        label: "Anytime",
    },
    NavDestination {
        href: "/someday",
        label: "Someday",
    },
    NavDestination {
        href: "/logbook",
        label: "Logbook",
    },
];

/// Authenticated application layout with primary navigation and content area.
#[component]
pub fn AppLayout(children: ChildrenFn) -> impl IntoView {
    let product = momentum_domain::PRODUCT_NAME;
    let account = use_account();
    let signed_in_label = account
        .as_ref()
        .map_or_else(String::new, |account| account.email.clone());

    view! {
        <div class="app-layout" data-testid="app-layout">
            <nav class="app-sidebar" aria-label="Primary">
                <A href="/today" attr:class="brand">{product}</A>
                <ul class="nav-list">
                    {PRIMARY_DESTINATIONS
                        .iter()
                        .map(|destination| {
                            view! {
                                <li class="nav-item">
                                    <A href=destination.href attr:class="nav-link">
                                        {destination.label}
                                    </A>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
                <div class="nav-secondary">
                    <ul class="nav-list">
                        <li class="nav-item">
                            <A href="/projects" attr:class="nav-link">"Projects"</A>
                        </li>
                        <li class="nav-item">
                            <A href="/areas" attr:class="nav-link">"Areas"</A>
                        </li>
                        <li class="nav-item">
                            <A href="/settings" attr:class="nav-link">"Settings"</A>
                        </li>
                    </ul>
                </div>
            </nav>
            <main class="app-main" data-testid="app-main">
                {children()}
            </main>
            <footer class="app-footer" aria-label="Account">
                <span class="account-label">{signed_in_label}</span>
            </footer>
        </div>
    }
}

/// Renders authenticated content in the application layout, or the unauthorized boundary.
#[component]
pub fn Protected(children: ChildrenFn) -> impl IntoView {
    let session = use_session();
    view! {
        {move || {
            let children = children.clone();
            match session.get() {
                SessionState::Authenticated(_) => {
                    view! { <AppLayout>{children()}</AppLayout> }.into_any()
                }
                SessionState::Anonymous => {
                    view! { <crate::boundaries::Unauthorized/> }.into_any()
                }
            }
        }}
    }
}
