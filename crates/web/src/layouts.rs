//! Public and authenticated application layouts.

use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_query_map};

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
        href: "/deadlines",
        label: "Deadlines",
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
        href: "/trash",
        label: "Trash",
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

#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
fn editing_target() -> bool {
    let Some(element) = leptos::prelude::document().active_element() else {
        return false;
    };
    let tag = element.tag_name().to_lowercase();
    matches!(tag.as_str(), "input" | "textarea" | "select")
        || element.get_attribute("contenteditable").is_some()
}

/// Authenticated application layout with responsive navigation.
#[component]
pub fn AppLayout(children: ChildrenFn) -> impl IntoView {
    let product = momentum_domain::PRODUCT_NAME;
    let account = use_account();
    let signed_in_label = account.map_or_else(String::new, |account| account.email.clone());
    let account_label = signed_in_label.clone();
    let layout = LayoutState::use_state();
    let drawer_open = layout.drawer_open;
    let query = use_query_map();
    let location = use_location();
    let popout = move || query.read().get("popout").is_some();
    #[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
    let pull_state = StoredValue::new(None::<(f64, f64, std::time::Instant, bool)>);
    let replica = use_context::<Option<Arc<momentum_app::replica::OfflineCoordinator>>>().flatten();
    let pending = AsyncDerived::new(move || {
        let replica = replica.clone();
        async move {
            match replica {
                Some(replica) => replica
                    .startup()
                    .await
                    .ok()
                    .map(|startup| startup.pending.len()),
                None => None,
            }
        }
    });
    #[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
    let navigate = leptos_router::hooks::use_navigate();
    #[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
    {
        let navigate = navigate.clone();
        Effect::new(move |_| {
            use leptos::web_sys::KeyboardEvent;
            use std::cell::RefCell;
            use std::rc::Rc;
            use std::time::Instant;
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;
            let navigate = navigate.clone();
            let keydown_navigate = navigate.clone();
            let callback = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                let editing = editing_target();
                if let Some(seed) = momentum_app::keyboard::type_travel_seed(
                    &event.key(),
                    event.ctrl_key(),
                    event.meta_key(),
                    event.alt_key(),
                    editing,
                ) {
                    event.prevent_default();
                    let target = format!("/search?q={}", crate::routes::percent_encode(&seed));
                    keydown_navigate(&target, Default::default());
                }
            });
            let document = leptos::prelude::document();
            let _ = document
                .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());
            callback.forget();

            // Pull-to-search: record the touch start, navigate on a qualifying touchend.
            let pull: Rc<RefCell<Option<(f64, f64, Instant, bool)>>> = Rc::new(RefCell::new(None));
            let pull_start = Rc::clone(&pull);
            let touch_start = Closure::<dyn FnMut(web_sys::TouchEvent)>::new(
                move |event: web_sys::TouchEvent| {
                    if let Some(touch) = event.touches().item(0) {
                        let at_top = leptos::prelude::window()
                            .scroll_y()
                            .map(|y| y <= 0.0)
                            .unwrap_or(false);
                        *pull_start.borrow_mut() = Some((
                            f64::from(touch.client_y()),
                            f64::from(touch.client_x()),
                            Instant::now(),
                            at_top,
                        ));
                    }
                },
            );
            let _ = document.add_event_listener_with_callback(
                "touchstart",
                touch_start.as_ref().unchecked_ref(),
            );
            touch_start.forget();

            let pull_end = Rc::clone(&pull);
            let navigate = navigate.clone();
            let touch_end = Closure::<dyn FnMut(web_sys::TouchEvent)>::new(
                move |event: web_sys::TouchEvent| {
                    if let Some((start_y, start_x, started, at_top)) = pull_end.borrow_mut().take()
                    {
                        if let Some(touch) = event.changed_touches().item(0) {
                            let dy = f64::from(touch.client_y()) - start_y;
                            let dx = f64::from(touch.client_x()) - start_x;
                            let duration =
                                u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
                            if momentum_app::gestures::pull_to_search(at_top, dx, dy, duration) {
                                event.prevent_default();
                                navigate("/search", Default::default());
                            }
                        }
                    }
                },
            );
            let _ = document
                .add_event_listener_with_callback("touchend", touch_end.as_ref().unchecked_ref());
            touch_end.forget();
        });
    }

    view! {
        <a href="#app-main" class="skip-link">"Skip to content"</a>
        <div
            class="app-layout"
            data-testid="app-layout"
            data-layout=move || layout.mode.get().as_str()
            data-window=move || if popout() { "popout" } else { "full" }
            data-drawer=move || if drawer_open.get() { "open" } else { "closed" }
            on:keydown=move |event| {
                if event.key() == "Escape" {
                    layout.close_drawer();
                }
            }
        >
            {popout()
                .then(|| {
                    view! {
                        <header class="popout-header" data-testid="popout-header">
                            <a
                                href=move || location.pathname.get()
                                class="row-action"
                                data-testid="popout-exit"
                            >
                                "Open full app"
                            </a>
                        </header>
                    }
                })}
            {(!popout())
                .then(|| {
                    view! {
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
                    <form
                        method="get"
                        action="/search"
                        class="topbar-search"
                        data-testid="quick-find-link"
                    >
                        <input
                            type="text"
                            name="q"
                            placeholder="Quick Find"
                            aria-label="Quick Find"
                            autocomplete="off"
                        />
                    </form>
                    <a
                        href="#quick-entry"
                        class="quick-entry-trigger"
                        data-testid="quick-entry-trigger"
                        aria-keyshortcuts=momentum_app::keyboard::keys_for("quick_entry")
                            .unwrap_or_default()
                    >
                        "Quick Entry"
                    </a>
                    <a
                        href=move || format!("{}?popout=1", location.pathname.get())
                        target="_blank"
                        rel="noopener"
                        class="row-action"
                        data-testid="popout-link"
                    >
                        "Open in window"
                    </a>
                    <span class="topbar-account">{signed_in_label}</span>
                </header>
                    }
                })}

            {(!popout())
                .then(|| {
                    view! {
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
                    }
                })}

            {(!popout())
                .then(|| {
                    view! {
                <button
                    type="button"
                    class="drawer-scrim"
                    aria-label="Close navigation"
                    tabindex="-1"
                    on:click=move |_| layout.close_drawer()
                ></button>

                    }
                })}
            <main id="app-main" class="app-main motion-nav" data-testid="app-main">
                {children()}
            </main>

            {(!popout())
                .then(|| {
                    view! {
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
                    }
                })}

            {move || view! { <crate::routes::QuickEntryOverlay/> }}
            {(!popout())
                .then(|| {
                    view! {
                <footer class="app-footer" aria-label="Account">
                    {move || {
                        pending
                            .get()
                            .flatten()
                            .filter(|count| *count > 0)
                            .map(|count| {
                                view! {
                                    <span data-testid="outbox-pending">
                                        {format!("{count} pending")}
                                    </span>
                                }
                            })
                    }}
                    <span class="account-label">{account_label}</span>
                </footer>
                    }
                })}
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
