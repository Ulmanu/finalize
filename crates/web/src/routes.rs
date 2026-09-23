//! Route table and destination pages for the application shell.
//!
//! Feature content (task rows, projections, editors) is added by later roadmap changes; this
//! module establishes the stable, bookmarkable destinations and the session boundary each
//! protected destination uses.

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_params_map;
use leptos_router::path;

use crate::boundaries::NotFound;
use crate::layouts::{Protected, PublicLayout};

/// Root route tree rendered inside the session provider.
#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=LandingPage/>
                <Route path=path!("/login") view=LoginPage/>
                <Route path=path!("/inbox") view=InboxPage/>
                <Route path=path!("/today") view=TodayPage/>
                <Route path=path!("/upcoming") view=UpcomingPage/>
                <Route path=path!("/anytime") view=AnytimePage/>
                <Route path=path!("/someday") view=SomedayPage/>
                <Route path=path!("/logbook") view=LogbookPage/>
                <Route path=path!("/projects") view=ProjectsPage/>
                <Route path=path!("/areas") view=AreasPage/>
                <Route path=path!("/settings") view=SettingsPage/>
                <Route path=path!("/areas/:area_id") view=AreaPage/>
                <Route path=path!("/projects/:project_id") view=ProjectPage/>
                <Route path=path!("/todos/:todo_id") view=TodoPage/>
            </Routes>
        </Router>
    }
}

/// Public landing page.
#[component]
pub fn LandingPage() -> impl IntoView {
    let product = momentum_domain::PRODUCT_NAME;
    view! {
        <PublicLayout>
            <section class="landing" data-testid="landing-page">
                <h1>{product}</h1>
                <p class="landing-tagline">
                    "A calm, keyboard-first task manager with Today, Upcoming, projects, areas, and tags."
                </p>
                <div class="landing-actions">
                    <a class="button-primary" href="/today">"Open Today"</a>
                    <a class="button-secondary" href="/login">"Sign in"</a>
                </div>
            </section>
        </PublicLayout>
    }
}

/// Sign-in route placeholder; the account-authentication change implements the form.
#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <PublicLayout>
            <section class="login-page" data-testid="login-page">
                <h1>"Sign in"</h1>
                <p>"Account sign-in is not enabled in this build yet."</p>
            </section>
        </PublicLayout>
    }
}

/// Generic built-in list page rendered inside the authenticated shell.
#[component]
fn ListPage(
    #[prop(into)] title: String,
    #[prop(into, optional)] description: String,
) -> impl IntoView {
    let description = if description.is_empty() {
        format!("The {title} list.")
    } else {
        description
    };
    view! {
        <section class="list-page" data-testid="list-page">
            <header class="list-header">
                <h1 class="list-title">{title}</h1>
                <p class="list-description">{description}</p>
            </header>
            <div class="list-empty" data-testid="list-empty">
                <p>"Nothing here yet."</p>
            </div>
        </section>
    }
}

macro_rules! list_route_page {
    ($component:ident, $title:literal, $description:literal) => {
        #[component]
        pub fn $component() -> impl IntoView {
            view! {
                <Protected>
                    {move || view! { <ListPage title=$title description=$description/> }}
                </Protected>
            }
        }
    };
}

list_route_page!(InboxPage, "Inbox", "Unprocessed captures land here.");
list_route_page!(TodayPage, "Today", "Everything scheduled for today.");
list_route_page!(
    UpcomingPage,
    "Upcoming",
    "What is scheduled for the days ahead."
);
list_route_page!(
    AnytimePage,
    "Anytime",
    "Actionable items without a future start date."
);
list_route_page!(
    SomedayPage,
    "Someday",
    "Deferred items kept out of the way."
);
list_route_page!(LogbookPage, "Logbook", "Completed and canceled items.");
list_route_page!(ProjectsPage, "Projects", "All open projects.");
list_route_page!(AreasPage, "Areas", "Ongoing areas of responsibility.");
list_route_page!(
    SettingsPage,
    "Settings",
    "Preferences and account settings."
);

/// Generic entity detail page; feature content arrives with its capability.
#[component]
fn EntityPage(#[prop(into)] kind: String, #[prop(into)] id: String) -> impl IntoView {
    let heading = format!("{kind} {id}");
    view! {
        <section class="entity-page" data-testid="entity-page">
            <header class="entity-header">
                <h1 class="entity-title">{heading}</h1>
            </header>
            <div class="entity-empty">
                <p>"This destination is addressable; its content arrives with its capability."</p>
            </div>
        </section>
    }
}

macro_rules! entity_route_page {
    ($component:ident, $kind:literal, $param:literal) => {
        #[component]
        pub fn $component() -> impl IntoView {
            let params = use_params_map();
            let id = move || params.read().get($param).unwrap_or_default();
            view! {
                <Protected>
                    {move || view! { <EntityPage kind=$kind id=id()/> }}
                </Protected>
            }
        }
    };
}

entity_route_page!(AreaPage, "Area", "area_id");
entity_route_page!(ProjectPage, "Project", "project_id");
entity_route_page!(TodoPage, "To-Do", "todo_id");
