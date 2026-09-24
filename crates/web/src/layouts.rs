//! Public and authenticated application layouts.

use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;

use crate::responsive::LayoutState;
use crate::session::{use_account, use_session};
use momentum_app::session::SessionState;
use momentum_app::todos::{TodoScope, TodoService};

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

/// Secondary destinations shown in the sidebar and the mobile drawer.
pub const SECONDARY_DESTINATIONS: &[NavDestination] = &[
    NavDestination {
        href: "/projects",
        label: "Projects",
    },
    NavDestination {
        href: "/areas",
        label: "Areas",
    },
    NavDestination {
        href: "/settings",
        label: "Settings",
    },
];

/// Destinations shown in the mobile bottom bar.
pub const MOBILE_BAR_DESTINATIONS: &[NavDestination] = &[
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
];

/// Authenticated application layout with responsive navigation.
#[component]
pub fn AppLayout(children: ChildrenFn) -> impl IntoView {
    let product = momentum_domain::PRODUCT_NAME;
    let account = use_account();
    let signed_in_label = account.map_or_else(String::new, |account| account.email.clone());
    let account_label = signed_in_label.clone();
    let layout = LayoutState::use_state();
    let drawer_open = layout.drawer_open;

    view! {
        <div
            class="app-layout"
            data-testid="app-layout"
            data-layout=move || layout.mode.get().as_str()
            data-drawer=move || if drawer_open.get() { "open" } else { "closed" }
            on:keydown=move |event| {
                if event.key() == "Escape" {
                    layout.close_drawer();
                }
            }
        >
            <header class="app-topbar">
                <button
                    type="button"
                    class="nav-toggle"
                    aria-label="Toggle navigation"
                    aria-controls="app-sidebar"
                    aria-expanded=move || if drawer_open.get() { "true" } else { "false" }
                    on:click=move |_| layout.toggle_drawer()
                >
                    <span class="nav-toggle-icon" aria-hidden="true"></span>
                </button>
                <A href="/today" attr:class="brand">{product}</A>
                <span class="topbar-account">{signed_in_label}</span>
            </header>

            <nav id="app-sidebar" class="app-sidebar" aria-label="Primary">
                <ul class="nav-list">
                    {PRIMARY_DESTINATIONS
                        .iter()
                        .map(|destination| {
                            let is_inbox = destination.href == "/inbox";
                            view! {
                                <li class="nav-item">
                                    <A href=destination.href attr:class="nav-link">
                                        {destination.label}
                                        {is_inbox.then(|| view! { <InboxBadge/> })}
                                    </A>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
                <div class="nav-secondary">
                    <ul class="nav-list">
                        {SECONDARY_DESTINATIONS
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
                </div>
            </nav>

            <button
                type="button"
                class="drawer-scrim"
                aria-label="Close navigation"
                tabindex="-1"
                on:click=move |_| layout.close_drawer()
            ></button>

            <main class="app-main" data-testid="app-main">
                {children()}
            </main>

            <nav class="bottom-nav" aria-label="Destinations">
                {MOBILE_BAR_DESTINATIONS
                    .iter()
                    .map(|destination| {
                        view! {
                            <A href=destination.href attr:class="bottom-nav-link">
                                {destination.label}
                            </A>
                        }
                    })
                    .collect_view()}
            </nav>

            <footer class="app-footer" aria-label="Account">
                <span class="account-label">{account_label}</span>
            </footer>
        </div>
    }
}

/// Live count of open Inbox to-dos.
#[component]
fn InboxBadge() -> impl IntoView {
    let service = use_context::<Option<Arc<TodoService>>>().flatten();
    let account = use_account();
    let count = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .list(account.account_id, TodoScope::Inbox)
                    .await
                    .map_or(0, |todos| todos.len()),
                _ => 0,
            }
        }
    });
    view! {
        <Suspense fallback=|| view! { <span class="nav-badge" aria-hidden="true"></span> }>
            {move || {
                count
                    .get()
                    .map(|count| {
                        view! {
                            <span
                                class="nav-badge"
                                data-testid="inbox-badge"
                                aria-label=format!("{count} open in Inbox")
                            >
                                {count.to_string()}
                            </span>
                        }
                    })
            }}
        </Suspense>
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
