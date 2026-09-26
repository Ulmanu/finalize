//! Route table and destination pages for the application shell.
//!
//! Feature content (task rows, projections, editors) is added by later roadmap changes; this
//! module establishes the stable, bookmarkable destinations and the session boundary each
//! protected destination uses.

use std::sync::Arc;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::{use_params_map, use_query_map};
use leptos_router::path;

use crate::boundaries::{Loading, NotFound};
use crate::design::CheckControl;
use crate::layouts::{Protected, PublicLayout};
use crate::session::use_account;
use momentum_app::areas::{AreaService, AreaSummary};
use momentum_app::logbook::{LogbookAgenda, LogbookEntry, LogbookService};
use momentum_app::projects::{HeadingSummary, ProjectOrderItem, ProjectService, ProjectSummary};
use momentum_app::tags::{TagService, TagSummary};
use momentum_app::todos::{TodoScope, TodoService, TodoSummary};
use momentum_domain::model::{DeadlineState, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::EntityId;

/// Root route tree rendered inside the session provider.
#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=LandingPage/>
                <Route path=path!("/login") view=LoginPage/>
                <Route path=path!("/signup") view=SignupPage/>
                <Route path=path!("/styleguide") view=StyleGuidePage/>
                <Route path=path!("/inbox") view=InboxPage/>
                <Route path=path!("/today") view=TodayPage/>
                <Route path=path!("/upcoming") view=UpcomingPage/>
                <Route path=path!("/anytime") view=AnytimePage/>
                <Route path=path!("/someday") view=SomedayPage/>
                <Route path=path!("/logbook") view=LogbookPage/>
                <Route path=path!("/projects") view=ProjectsPage/>
                <Route path=path!("/areas") view=AreasPage/>
                <Route path=path!("/tags") view=TagsPage/>
                <Route path=path!("/deadlines") view=DeadlinesPage/>
                <Route path=path!("/trash") view=TrashPage/>
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
                    <a class="button-primary" href="/signup">"Create an account"</a>
                    <a class="button-secondary" href="/login">"Sign in"</a>
                </div>
            </section>
        </PublicLayout>
    }
}

fn query_message(query: &leptos_router::params::ParamsMap) -> Option<&'static str> {
    if query.get("registered").is_some() {
        Some("Account created. Verify your email address, then sign in.")
    } else if query.get("verified").is_some() {
        Some("Email verified. Sign in to continue.")
    } else if query.get("reset").as_deref() == Some("sent") {
        Some("If that email address exists, a reset link is on its way.")
    } else if query.get("reset").as_deref() == Some("done") {
        Some("Password updated. Sign in with your new password.")
    } else if query.get("password").as_deref() == Some("changed") {
        Some("Password changed. Sign in again on this device.")
    } else if query.get("error").as_deref() == Some("unverified") {
        Some("Verify your email address before signing in.")
    } else if query.get("error").as_deref() == Some("verify") {
        Some("That verification link is invalid or has expired.")
    } else if query.get("error").as_deref() == Some("reset") {
        Some("That reset link is invalid or has expired.")
    } else if query.get("error").as_deref() == Some("unavailable") {
        Some("Authentication is not configured on this server.")
    } else if query.get("error").is_some() {
        Some("Email or password is incorrect.")
    } else {
        None
    }
}

/// Sign-in page with a native form posting to the authentication route.
#[component]
pub fn LoginPage() -> impl IntoView {
    let query = use_query_map();
    let message = move || query_message(&query.read());

    view! {
        <PublicLayout>
            <section class="login-page" data-testid="login-page">
                <h1>"Sign in"</h1>
                {move || {
                    message()
                        .map(|message| {
                            view! {
                                <p class="form-message" role="status" data-testid="login-message">
                                    {message}
                                </p>
                            }
                        })
                }}
                <form method="post" action="/auth/login" class="auth-form">
                    <label class="form-field">
                        <span>"Email"</span>
                        <input
                            type="email"
                            name="email"
                            autocomplete="email"
                            required=true
                            aria-label="Email"
                        />
                    </label>
                    <label class="form-field">
                        <span>"Password"</span>
                        <input
                            type="password"
                            name="password"
                            autocomplete="current-password"
                            required=true
                            aria-label="Password"
                        />
                    </label>
                    <button type="submit" class="button-primary">"Sign in"</button>
                </form>
                <p class="auth-secondary">
                    <a href="/signup">"Create an account"</a>
                </p>
            </section>
        </PublicLayout>
    }
}

/// Registration page with a native form posting to the authentication route.
#[component]
pub fn SignupPage() -> impl IntoView {
    let query = use_query_map();
    let message = move || match query.read().get("error").as_deref() {
        Some("taken") => Some("That email address is already registered."),
        Some("invalid") => Some("Check the email address and choose a stronger password."),
        Some("unavailable") => Some("Registration is not configured on this server."),
        _ => None,
    };

    view! {
        <PublicLayout>
            <section class="login-page" data-testid="signup-page">
                <h1>"Create your account"</h1>
                {move || {
                    message()
                        .map(|message| {
                            view! {
                                <p class="form-message" role="status" data-testid="signup-message">
                                    {message}
                                </p>
                            }
                        })
                }}
                <form method="post" action="/auth/register" class="auth-form">
                    <label class="form-field">
                        <span>"Email"</span>
                        <input
                            type="email"
                            name="email"
                            autocomplete="email"
                            required=true
                            aria-label="Email"
                        />
                    </label>
                    <label class="form-field">
                        <span>"Password"</span>
                        <input
                            type="password"
                            name="password"
                            autocomplete="new-password"
                            minlength="8"
                            required=true
                            aria-label="Password"
                        />
                    </label>
                    <label class="form-field">
                        <span>"Name (optional)"</span>
                        <input type="text" name="display_name" autocomplete="name" aria-label="Name"/>
                    </label>
                    <button type="submit" class="button-primary">"Create account"</button>
                </form>
                <p class="auth-secondary">
                    <a href="/login">"I already have an account"</a>
                </p>
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

#[component]
pub fn TodayPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <TodayList/> }}
        </Protected>
    }
}

#[component]
fn TodayList() -> impl IntoView {
    let service = use_context::<Option<Arc<TodoService>>>().flatten();
    let account = use_account();
    let today = service.as_ref().map(|service| service.local_today());
    let todos = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .list(account.account_id, TodoScope::Today)
                    .await
                    .unwrap_or_default(),
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="list-page" data-testid="today-page">
            <header class="list-header">
                <h1 class="list-title">"Today"</h1>
                <p class="list-description">"Everything scheduled for today."</p>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    todos
                        .get()
                        .map(|todos| {
                            let (evening, day): (Vec<TodoSummary>, Vec<TodoSummary>) = todos
                                .into_iter()
                                .partition(|todo| todo.schedule.is_evening());
                            view! {
                                <h2 class="section-heading">"Today"</h2>
                                {if day.is_empty() {
                                    view! {
                                        <div class="list-empty" data-testid="today-empty">
                                            <p>"Nothing scheduled for today."</p>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <ul class="task-list" data-testid="today-day-list">
                                            {day
                                                .into_iter()
                                                .map(|todo| {
                                                    view! {
                                                        <TodoRow todo=todo today=today today_order=true/>
                                                    }
                                                })
                                                .collect_view()}
                                        </ul>
                                    }
                                        .into_any()
                                }}
                                <h2 class="section-heading">"This Evening"</h2>
                                {if evening.is_empty() {
                                    view! {
                                        <div
                                            class="list-empty"
                                            data-testid="today-evening-empty"
                                        >
                                            <p>"Nothing planned for this evening."</p>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <ul
                                            class="task-list"
                                            data-testid="today-evening-list"
                                        >
                                            {evening
                                                .into_iter()
                                                .map(|todo| {
                                                    view! {
                                                        <TodoRow todo=todo today=today today_order=true/>
                                                    }
                                                })
                                                .collect_view()}
                                        </ul>
                                    }
                                        .into_any()
                                }}
                            }
                                .into_any()
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
pub fn UpcomingPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <UpcomingAgendaView/> }}
        </Protected>
    }
}

#[component]
fn UpcomingAgendaView() -> impl IntoView {
    let service = use_context::<Option<Arc<TodoService>>>().flatten();
    let account = use_account();
    let today = service.as_ref().map(|service| service.local_today());
    let agenda = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .upcoming_agenda(account.account_id)
                    .await
                    .unwrap_or_default(),
                _ => momentum_app::todos::UpcomingAgenda::default(),
            }
        }
    });

    view! {
        <section class="list-page" data-testid="upcoming-page">
            <header class="list-header">
                <h1 class="list-title">"Upcoming"</h1>
                <p class="list-description">"What is scheduled for the days ahead."</p>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    agenda
                        .get()
                        .map(|agenda| {
                            let is_empty = agenda.days.is_empty() && agenda.calendar.is_empty();
                            view! {
                                {if is_empty {
                                    view! {
                                        <div class="list-empty" data-testid="upcoming-empty">
                                            <p>"Nothing scheduled ahead."</p>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    ().into_any()
                                }}
                                {agenda
                                    .days
                                    .into_iter()
                                    .map(|day| {
                                        let label = day
                                            .date
                                            .as_naive()
                                            .format("%A, %-d %B %Y")
                                            .to_string();
                                        let iso = day.date.to_string();
                                        view! {
                                            <section
                                                class="upcoming-day"
                                                data-testid="upcoming-day"
                                                data-date=iso
                                            >
                                                <h2 class="section-heading">{label}</h2>
                                                <ul
                                                    class="task-list"
                                                    data-testid="upcoming-day-list"
                                                >
                                                    {day
                                                        .todos
                                                        .into_iter()
                                                        .map(|todo| {
                                                            view! {
                                                                <TodoRow
                                                                    todo=todo
                                                                    today=today
                                                                    show_date_move=true
                                                                />
                                                            }
                                                        })
                                                        .collect_view()}
                                                </ul>
                                            </section>
                                        }
                                    })
                                    .collect_view()}
                                {(!agenda.calendar.is_empty())
                                    .then(|| {
                                        view! {
                                            <section data-testid="upcoming-calendar">
                                                <h2 class="section-heading">"Calendar"</h2>
                                                <ul class="task-list">
                                                    {agenda
                                                        .calendar
                                                        .into_iter()
                                                        .map(|row| {
                                                            view! {
                                                                <li
                                                                    class="task-row task-row-readonly"
                                                                    data-testid="calendar-row"
                                                                >
                                                                    <span class="task-title">
                                                                        {row.title}
                                                                    </span>
                                                                    <span class="row-badge">
                                                                        {row.date.to_string()}
                                                                    </span>
                                                                </li>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </ul>
                                            </section>
                                        }
                                    })}
                            }
                                .into_any()
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
pub fn AnytimePage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <AnytimeAgendaView/> }}
        </Protected>
    }
}

#[component]
fn AnytimeAgendaView() -> impl IntoView {
    let service = use_context::<Option<Arc<TodoService>>>().flatten();
    let area_service = use_context::<Option<Arc<AreaService>>>().flatten();
    let project_service = use_context::<Option<Arc<ProjectService>>>().flatten();
    let account = use_account();
    let account_for_labels = account.clone();
    let today = service.as_ref().map(|service| service.local_today());
    let agenda = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .anytime_agenda(account.account_id)
                    .await
                    .unwrap_or_default(),
                _ => momentum_app::todos::AnytimeAgenda::default(),
            }
        }
    });
    let labels = AsyncDerived::new(move || {
        let areas = area_service.clone();
        let projects = project_service.clone();
        let account = account_for_labels.clone();
        async move {
            let Some(account) = account else {
                return std::collections::HashMap::new();
            };
            let mut labels: std::collections::HashMap<EntityId, String> =
                std::collections::HashMap::new();
            if let Some(areas) = areas {
                for area in areas.list(account.account_id).await.unwrap_or_default() {
                    labels.insert(area.id, area.title);
                }
            }
            if let Some(projects) = projects {
                for project in projects.list(account.account_id).await.unwrap_or_default() {
                    labels.insert(project.id, project.title);
                }
            }
            labels
        }
    });

    view! {
        <section class="list-page" data-testid="anytime-page">
            <header class="list-header">
                <h1 class="list-title">"Anytime"</h1>
                <p class="list-description">"Actionable items without a future start date."</p>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let labels = labels.get().unwrap_or_default();
                    agenda
                        .get()
                        .map(|agenda| {
                            if agenda.groups.is_empty() {
                                return view! {
                                    <div class="list-empty" data-testid="anytime-empty">
                                        <p>"Nothing actionable right now."</p>
                                    </div>
                                }
                                    .into_any();
                            }
                            view! {
                                {agenda
                                    .groups
                                    .into_iter()
                                    .map(|group| {
                                        let label = match group.context {
                                            momentum_domain::model::ContextRef::Inbox => {
                                                "Inbox".to_string()
                                            }
                                            momentum_domain::model::ContextRef::Area(id) => {
                                                labels
                                                    .get(&id)
                                                    .cloned()
                                                    .unwrap_or_else(|| "Area".to_string())
                                            }
                                            momentum_domain::model::ContextRef::Project(id) => {
                                                labels
                                                    .get(&id)
                                                    .cloned()
                                                    .unwrap_or_else(|| "Project".to_string())
                                            }
                                            momentum_domain::model::ContextRef::Heading(_) => {
                                                "Project heading".to_string()
                                            }
                                        };
                                        let context_label = match group.context {
                                            momentum_domain::model::ContextRef::Inbox => {
                                                "inbox".to_string()
                                            }
                                            momentum_domain::model::ContextRef::Area(id) => {
                                                format!("area:{id}")
                                            }
                                            momentum_domain::model::ContextRef::Project(id) => {
                                                format!("project:{id}")
                                            }
                                            momentum_domain::model::ContextRef::Heading(id) => {
                                                format!("heading:{id}")
                                            }
                                        };
                                        view! {
                                            <section
                                                class="anytime-group"
                                                data-testid="anytime-group"
                                                data-context=context_label
                                            >
                                                <h2 class="section-heading">{label}</h2>
                                                <ul
                                                    class="task-list"
                                                    data-testid="anytime-group-list"
                                                >
                                                    {group
                                                        .todos
                                                        .into_iter()
                                                        .map(|todo| {
                                                            view! { <TodoRow todo=todo today=today/> }
                                                        })
                                                        .collect_view()}
                                                </ul>
                                            </section>
                                        }
                                    })
                                    .collect_view()}
                            }
                                .into_any()
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
pub fn DeadlinesPage() -> impl IntoView {
    view! {
        <Protected>
            {move || {
                view! {
                    <ProjectionList
                        title="Deadlines"
                        description="Open items with an explicit due date."
                        scope=TodoScope::Deadlines
                        slug="deadlines"
                    />
                }
            }}
        </Protected>
    }
}

#[component]
pub fn SomedayPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <SomedayList/> }}
        </Protected>
    }
}

#[component]
fn SomedayList() -> impl IntoView {
    let todos_service = use_context::<Option<Arc<TodoService>>>().flatten();
    let projects_service = use_context::<Option<Arc<ProjectService>>>().flatten();
    let account = use_account();
    let account_for_projects = account.clone();
    let today = todos_service.as_ref().map(|service| service.local_today());
    let todos = AsyncDerived::new(move || {
        let service = todos_service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .list(account.account_id, TodoScope::Someday)
                    .await
                    .unwrap_or_default(),
                _ => Vec::new(),
            }
        }
    });
    let projects = AsyncDerived::new(move || {
        let service = projects_service.clone();
        let account = account_for_projects.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .someday(account.account_id)
                    .await
                    .unwrap_or_default(),
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="list-page" data-testid="someday-page">
            <header class="list-header">
                <h1 class="list-title">"Someday"</h1>
                <p class="list-description">"Deferred items kept out of the way."</p>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let todos = todos.get().unwrap_or_default();
                    let projects = projects.get().unwrap_or_default();
                    if todos.is_empty() && projects.is_empty() {
                        return view! {
                            <div class="list-empty" data-testid="someday-empty">
                                <p>"Nothing deferred right now."</p>
                            </div>
                        }
                            .into_any();
                    }
                    view! {
                        <h2 class="section-heading">"To-Dos"</h2>
                        {if todos.is_empty() {
                            view! {
                                <div class="list-empty" data-testid="someday-todos-empty">
                                    <p>"No deferred to-dos."</p>
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <ul class="task-list" data-testid="someday-todos">
                                    {todos
                                        .into_iter()
                                        .map(|todo| view! { <TodoRow todo=todo today=today/> })
                                        .collect_view()}
                                </ul>
                            }
                                .into_any()
                        }}
                        <h2 class="section-heading">"Projects"</h2>
                        {if projects.is_empty() {
                            view! {
                                <div class="list-empty" data-testid="someday-projects-empty">
                                    <p>"No deferred projects."</p>
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <ul class="task-list" data-testid="someday-projects">
                                    {projects
                                        .into_iter()
                                        .map(|project| view! { <SomedayProjectRow project=project/> })
                                        .collect_view()}
                                </ul>
                            }
                                .into_any()
                        }}
                    }
                        .into_any()
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn SomedayProjectRow(project: ProjectSummary) -> impl IntoView {
    let id = project.id;
    let title = project.title.clone();
    let activate_target = format!("/projects/{id}/schedule");
    let open_target = format!("/projects/{id}");

    view! {
        <li class="task-row" data-testid="someday-project-row">
            <div class="task-row-body">
                <a class="task-title" href=open_target>{title}</a>
                <span class="task-row-meta">
                    <span class="row-badge">"Someday"</span>
                </span>
            </div>
            <div class="task-row-actions">
                <form method="post" action=activate_target class="row-schedule">
                    <input type="hidden" name="return_to" value="/someday"/>
                    <select name="when" aria-label="Reactivate project">
                        <option value="anytime">"Anytime"</option>
                        <option value="today">"Today"</option>
                        <option value="tomorrow">"Tomorrow"</option>
                    </select>
                    <button type="submit" class="row-action" aria-label="Reactivate project">
                        "Activate"
                    </button>
                </form>
                <a href=format!("/projects/{id}") class="row-action" aria-label="Open project">"Edit"</a>
            </div>
        </li>
    }
}
list_route_page!(
    SettingsPage,
    "Settings",
    "Preferences and account settings."
);

/// Public catalog of every design-system primitive.
///
/// Used by developers, design-system tests, and (later) visual-parity fixtures.
#[component]
pub fn StyleGuidePage() -> impl IntoView {
    view! {
        <PublicLayout>
            <section class="styleguide" data-testid="styleguide">
                <h1>"Design system"</h1>
                <StyleGuideRows/>
                <StyleGuideNavigation/>
                <StyleGuideControls/>
                <StyleGuideOverlays/>
                <StyleGuideIcons/>
                <StyleGuideMotion/>
            </section>
        </PublicLayout>
    }
}

#[component]
fn StyleGuideRows() -> impl IntoView {
    use crate::design::{ProjectRow, TaskRow};

    view! {
        <div class="styleguide-section">
            <h2>"Task rows"</h2>
            <div class="styleguide-sample">
                <ul>
                    <TaskRow title="Water the plants"/>
                    <TaskRow
                        title="Draft launch checklist"
                        note="Include the rollback plan"
                        tags=vec!["Work".to_string(), "Launch".to_string()]
                        schedule="Today".to_string()
                        deadline="Friday".to_string()
                        has_recurrence=true
                    />
                    <TaskRow title="Completed example" completed=true/>
                    <TaskRow title="Selected example" selected=true/>
                </ul>
            </div>
        </div>
        <div class="styleguide-section">
            <h2>"Project rows"</h2>
            <div class="styleguide-sample">
                <ul>
                    <ProjectRow title="Website refresh" open_count=6 progress=0.4/>
                    <ProjectRow title="Quarterly report" open_count=2 progress=0.75/>
                    <ProjectRow title="No progress yet" open_count=3/>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn StyleGuideNavigation() -> impl IntoView {
    use crate::design::{SidebarIcon, SidebarRow};

    view! {
        <div class="styleguide-section">
            <h2>"Sidebar rows"</h2>
            <div class="styleguide-sample">
                <ul>
                    <SidebarRow label="Inbox" icon=SidebarIcon::Inbox count=4/>
                    <SidebarRow label="Today" icon=SidebarIcon::Today active=true count=3/>
                    <SidebarRow label="Upcoming" icon=SidebarIcon::Upcoming/>
                    <SidebarRow label="Anytime" icon=SidebarIcon::Anytime/>
                    <SidebarRow label="Someday" icon=SidebarIcon::Archive/>
                    <SidebarRow label="Logbook" icon=SidebarIcon::Archive/>
                    <SidebarRow label="Work" icon=SidebarIcon::Folder/>
                    <SidebarRow label="Errands" icon=SidebarIcon::Tag/>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn StyleGuideControls() -> impl IntoView {
    use crate::design::{AddControl, CheckControl, DateControl, ProgressRing, SectionHeading};

    view! {
        <div class="styleguide-section">
            <h2>"Check controls and progress"</h2>
            <div class="styleguide-sample styleguide-row">
                <CheckControl checked=false label="Unchecked"/>
                <CheckControl checked=true label="Checked"/>
                <CheckControl checked=false label="Disabled" disabled=true/>
                <ProgressRing progress=0.0/>
                <ProgressRing progress=0.25/>
                <ProgressRing progress=0.5/>
                <ProgressRing progress=1.0/>
            </div>
        </div>
        <div class="styleguide-section">
            <h2>"Section headings"</h2>
            <div class="styleguide-sample">
                <SectionHeading>"This Evening"</SectionHeading>
            </div>
        </div>
        <div class="styleguide-section">
            <h2>"Date and add controls"</h2>
            <div class="styleguide-sample styleguide-row">
                <DateControl label="Schedule" value="Today"/>
                <DateControl label="Deadline" value="Friday" deadline=true/>
                <AddControl label="Add to-do"/>
            </div>
        </div>
    }
}

#[component]
fn StyleGuideOverlays() -> impl IntoView {
    use crate::design::{ContextMenu, ContextMenuItem, Popover};

    view! {
        <div class="styleguide-section">
            <h2>"Popover"</h2>
            <div class="styleguide-sample">
                <Popover id="styleguide-popover" trigger_label="When">
                    <p>"Today"</p>
                    <p>"This Evening"</p>
                </Popover>
            </div>
        </div>
        <div class="styleguide-section">
            <h2>"Context menu"</h2>
            <div class="styleguide-sample">
                <ContextMenu
                    label="Task actions"
                    items=vec![
                        ContextMenuItem::new("Complete"),
                        ContextMenuItem::new("Move to…"),
                        ContextMenuItem::new("Delete").danger(),
                        ContextMenuItem::new("Unavailable").disabled(),
                    ]
                />
            </div>
        </div>
    }
}

#[component]
fn StyleGuideIcons() -> impl IntoView {
    use crate::design::icons::*;

    view! {
        <div class="styleguide-section">
            <h2>"Icons"</h2>
            <div class="styleguide-sample styleguide-row">
                <CheckIcon/>
                <PlusIcon/>
                <CalendarIcon/>
                <FlagIcon/>
                <TagIcon/>
                <NoteIcon/>
                <RepeatIcon/>
                <TrashIcon/>
                <ChevronIcon/>
                <InboxIcon/>
                <StarIcon/>
                <ClockIcon/>
                <LayersIcon/>
                <ArchiveIcon/>
                <FolderIcon/>
            </div>
        </div>
    }
}

#[component]
fn StyleGuideMotion() -> impl IntoView {
    use crate::design::{Motion, MotionKind};

    view! {
        <div class="styleguide-section">
            <h2>"Motion primitives"</h2>
            <div class="styleguide-sample">
                <Motion kind=MotionKind::Insert>"Insert"</Motion>
                <Motion kind=MotionKind::Editor>"Editor"</Motion>
                <Motion kind=MotionKind::Popover>"Popover"</Motion>
                <Motion kind=MotionKind::Drag>"Drag"</Motion>
                <Motion kind=MotionKind::Navigation>"Navigation"</Motion>
                <Motion kind=MotionKind::Remove>"Remove"</Motion>
            </div>
        </div>
    }
}

/// Inbox destination with the real to-do list.
#[component]
pub fn InboxPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <InboxList/> }}
        </Protected>
    }
}

#[component]
fn InboxList() -> impl IntoView {
    let service = use_context::<Option<Arc<TodoService>>>().flatten();
    let tag_service = use_context::<Option<Arc<TagService>>>().flatten();
    let account = use_account();
    let account_for_tags = account.clone();
    let today = service.as_ref().map(|service| service.local_today());
    let query = use_query_map();
    let todos = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .list(account.account_id, TodoScope::Inbox)
                    .await
                    .unwrap_or_default(),
                _ => Vec::new(),
            }
        }
    });
    let tags = AsyncDerived::new(move || {
        let service = tag_service.clone();
        let account = account_for_tags.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => {
                    service.list(account.account_id).await.unwrap_or_default()
                }
                _ => Vec::new(),
            }
        }
    });
    let selected_tags = move || -> Vec<EntityId> {
        query
            .read()
            .get("tags")
            .unwrap_or_default()
            .split(',')
            .filter_map(|value| value.parse::<EntityId>().ok())
            .collect()
    };

    view! {
        <section class="list-page" data-testid="list-page">
            <header class="list-header">
                <h1 class="list-title">"Inbox"</h1>
                <p class="list-description">"Unprocessed captures land here."</p>
            </header>
            <div class="tag-filter" data-testid="tag-filter">
                <Suspense fallback=|| view! { <Loading/> }>
                    {move || {
                        let selected = selected_tags();
                        let all = tags.get().unwrap_or_default();
                        if all.is_empty() {
                            return ().into_any();
                        }
                        let links = all
                            .into_iter()
                            .map(|tag| {
                                let mut next = selected.clone();
                                let active = next.contains(&tag.id);
                                if active {
                                    next.retain(|id| *id != tag.id);
                                } else {
                                    next.push(tag.id);
                                }
                                let href = if next.is_empty() {
                                    "/inbox".to_string()
                                } else {
                                    format!(
                                        "/inbox?tags={}",
                                        next.iter()
                                            .map(ToString::to_string)
                                            .collect::<Vec<_>>()
                                            .join(","),
                                    )
                                };
                                let label = if active {
                                    format!("Remove filter: {}", tag.title)
                                } else {
                                    format!("Filter by {}", tag.title)
                                };
                                view! {
                                    <a
                                        href=href
                                        class=if active { "tag-chip tag-chip-active" } else { "tag-chip" }
                                        aria-label=label
                                        data-testid="tag-filter-link"
                                    >
                                        {tag.title}
                                    </a>
                                }
                            })
                            .collect_view();
                        view! {
                            {links}
                            {(!selected.is_empty())
                                .then(|| {
                                    view! {
                                        <a href="/inbox" class="tag-chip tag-chip-clear">"Clear"</a>
                                    }
                                })}
                        }
                            .into_any()
                    }}
                </Suspense>
            </div>
            <form method="post" action="/todos" class="add-form" data-testid="add-todo-form">
                <input type="hidden" name="return_to" value="/inbox"/>
                <input
                    id="new-todo-title"
                    type="text"
                    name="title"
                    placeholder="New to-do"
                    aria-label="New to-do"
                    autocomplete="off"
                    required=true
                />
                <button type="submit" class="button-primary">"Add"</button>
            </form>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let selected = selected_tags();
                    todos
                        .get()
                        .map(|todos| {
                            let todos: Vec<TodoSummary> = if selected.is_empty() {
                                todos
                            } else {
                                todos
                                    .into_iter()
                                    .filter(|todo| {
                                        selected.iter().any(|tag| {
                                            todo.tags.contains(tag)
                                                || todo.inherited_tags.contains(tag)
                                        })
                                    })
                                    .collect()
                            };
                            if todos.is_empty() {
                                view! {
                                    <div class="list-empty" data-testid="list-empty">
                                        <p>"Nothing here yet."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ul class="task-list" data-testid="task-list">
                                        {todos
                                            .into_iter()
                                            .map(|todo| view! { <TodoRow todo=todo today=today/> })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn TodoRow(
    todo: TodoSummary,
    #[prop(default = None)] today: Option<LocalDate>,
    #[prop(default = false)] today_order: bool,
    #[prop(default = false)] show_date_move: bool,
) -> impl IntoView {
    let id = todo.id;
    let title = todo.title.clone();
    let completed = todo.status == Status::Completed;
    let row_class = if completed {
        "task-row task-row-completed"
    } else {
        "task-row"
    };
    let toggle_target = format!("/todos/{id}/complete");
    let open_target = format!("/todos/{id}");
    let delete_target = format!("/todos/{id}/delete");
    let up_target = if today_order {
        format!("/todos/{id}/today-up")
    } else {
        format!("/todos/{id}/move-up")
    };
    let down_target = if today_order {
        format!("/todos/{id}/today-down")
    } else {
        format!("/todos/{id}/move-down")
    };
    let schedule_target = format!("/todos/{id}/schedule");
    let schedule = todo.schedule.date().map(|date| format!("When: {date}"));
    let deadline = todo.deadline.map(|deadline| {
        let state = today.map(|today| deadline.state(today));
        let label = match state {
            Some(DeadlineState::Overdue) => format!("Overdue: {}", deadline.date),
            Some(DeadlineState::DueToday) => format!("Due today: {}", deadline.date),
            _ => format!("Due: {}", deadline.date),
        };
        (label, matches!(state, Some(DeadlineState::Overdue)))
    });
    let note = (!todo.notes.is_empty()).then(|| todo.notes.clone());

    view! {
        <li class=row_class data-testid="task-row" data-todo-id=id.to_string()>
            <CheckControl checked=completed label=title.clone()/>
            <div class="task-row-body">
                <a class="task-title" href=open_target.clone()>
                    {title}
                </a>
                <span class="task-row-meta">
                    {note
                        .map(|note| {
                            let title = note.clone();
                            view! {
                                <span class="row-badge note-indicator" title=title>{note}</span>
                            }
                        })}
                    {schedule
                        .map(|label| view! { <span class="row-badge date-badge">{label}</span> })}
                    {deadline
                        .map(|(label, overdue)| {
                            let class = if overdue {
                                "row-badge date-badge date-badge-deadline deadline-overdue"
                            } else {
                                "row-badge date-badge date-badge-deadline"
                            };
                            view! { <span class=class data-testid="deadline-badge">{label}</span> }
                        })}
                </span>
            </div>
            <div class="task-row-actions">
                <form method="post" action=up_target>
                    <input type="hidden" name="return_to" value="/inbox"/>
                    <button type="submit" class="row-action" aria-label="Move up">"↑"</button>
                </form>
                <form method="post" action=down_target>
                    <input type="hidden" name="return_to" value="/inbox"/>
                    <button type="submit" class="row-action" aria-label="Move down">"↓"</button>
                </form>
                <form method="post" action=schedule_target.clone() class="row-schedule">
                    <input type="hidden" name="return_to" value="/inbox"/>
                    <select name="when" aria-label="Schedule">
                        <option value="today">"Today"</option>
                        <option value="tomorrow">"Tomorrow"</option>
                        <option value="someday">"Someday"</option>
                        <option value="anytime">"Anytime"</option>
                    </select>
                    <button type="submit" class="row-action" aria-label="Apply schedule">
                        "When"
                    </button>
                </form>
                {show_date_move
                    .then(|| {
                        view! {
                            <form method="post" action=schedule_target class="row-schedule">
                                <input type="hidden" name="when" value="date"/>
                                <input type="hidden" name="return_to" value="/upcoming"/>
                                <input type="date" name="date" aria-label="Move to date"/>
                                <button type="submit" class="row-action" aria-label="Move to date">
                                    "Move"
                                </button>
                            </form>
                        }
                    })}
                <form method="post" action=toggle_target>
                    <button type="submit" class="row-action" aria-label="Complete to-do">
                        "Complete"
                    </button>
                </form>
                <form method="post" action=delete_target>
                    <button type="submit" class="row-action" aria-label="Delete to-do">"Delete"</button>
                </form>
                <a class="row-action" href=open_target aria-label="Open to-do">"Edit"</a>
            </div>
        </li>
    }
}

/// To-do editor destination with lifecycle actions.
#[component]
pub fn TodoPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("todo_id").unwrap_or_default();
    view! {
        <Protected>
            {move || view! { <TodoEditor id=id()/> }}
        </Protected>
    }
}

#[component]
fn TodoEditor(#[prop(into)] id: String) -> impl IntoView {
    let service = use_context::<Option<Arc<TodoService>>>().flatten();
    let account = use_account();
    let parsed = id.parse::<momentum_domain::EntityId>().ok();

    let todo = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account, parsed) {
                (Some(service), Some(account), Some(id)) => {
                    service.get(account.account_id, id).await.ok().flatten()
                }
                _ => None,
            }
        }
    });

    view! {
        <Suspense fallback=|| view! { <Loading/> }>
            {move || {
                todo.get()
                    .map(|todo| {
                        todo.map_or_else(
                            || {
                                view! {
                                    <section class="entity-page" data-testid="todo-not-found">
                                        <h1>"To-Do not found"</h1>
                                        <p>
                                            <a href="/inbox">"Back to Inbox"</a>
                                        </p>
                                    </section>
                                }
                                    .into_any()
                            },
                            |todo| view! { <TodoEditorBody todo=todo/> }.into_any(),
                        )
                    })
            }}
        </Suspense>
    }
}

#[component]
fn TodoEditorBody(todo: TodoSummary) -> impl IntoView {
    let id = todo.id;
    let title = todo.title.clone();
    let notes = todo.notes.clone();
    let update_target = format!("/todos/{id}");
    let complete_target = format!("/todos/{id}/complete");
    let cancel_target = format!("/todos/{id}/cancel");
    let reopen_target = format!("/todos/{id}/reopen");
    let duplicate_target = format!("/todos/{id}/duplicate");
    let delete_target = format!("/todos/{id}/delete");
    let restore_target = format!("/todos/{id}/restore");

    let is_deleted = todo.status == Status::Deleted;
    let is_terminal = todo.status.is_terminal();
    let schedule = todo
        .schedule
        .date()
        .map_or_else(|| "Anytime".to_string(), |date| date.to_string());
    let deadline = todo
        .deadline
        .map_or_else(|| "None".to_string(), |deadline| deadline.date.to_string());

    view! {
        <section class="entity-page" data-testid="todo-editor">
            <header class="entity-header">
                <h1 class="entity-title" data-testid="todo-title">{title.clone()}</h1>
                <p class="todo-meta">
                    <span data-testid="todo-status">{format!("Status: {}", todo.status.as_str())}</span>
                    " · "
                    <span data-testid="todo-schedule">{format!("When: {schedule}")}</span>
                    " · "
                    <span data-testid="todo-deadline">{format!("Deadline: {deadline}")}</span>
                </p>
            </header>

            <form method="post" action=update_target class="auth-form">
                <label class="form-field">
                    <span>"Title"</span>
                    <input
                        type="text"
                        name="title"
                        value=title
                        aria-label="Title"
                        autocomplete="off"
                    />
                </label>
                <label class="form-field">
                    <span>"Notes"</span>
                    <textarea name="notes" rows="6" aria-label="Notes">{notes}</textarea>
                </label>
                <button type="submit" class="button-primary">"Save"</button>
            </form>

            <NotesTools
                notes=todo.notes.clone()
                replace_action=format!("/todos/{id}/notes/replace")
                return_to=format!("/todos/{id}")
            />

            <form method="post" action=format!("/todos/{id}/schedule") class="auth-form">
                <input type="hidden" name="return_to" value=format!("/todos/{id}")/>
                <label class="form-field">
                    <span>"When"</span>
                    <select name="when" aria-label="When">
                        <option value="today">"Today"</option>
                        <option value="evening">"This Evening"</option>
                        <option value="tomorrow">"Tomorrow"</option>
                        <option value="someday">"Someday"</option>
                        <option value="anytime">"Anytime"</option>
                        <option value="date">"On a date"</option>
                    </select>
                </label>
                <label class="form-field">
                    <span>"Date (for On a date)"</span>
                    <input type="date" name="date" aria-label="Schedule date"/>
                </label>
                <button type="submit" class="button-secondary">"Apply schedule"</button>
            </form>

            <form method="post" action=format!("/todos/{id}/deadline") class="auth-form">
                <input type="hidden" name="return_to" value=format!("/todos/{id}")/>
                <label class="form-field">
                    <span>"Deadline"</span>
                    <input type="date" name="date" aria-label="Deadline date"/>
                </label>
                <button type="submit" class="button-secondary">"Apply deadline"</button>
            </form>

            <form method="post" action=format!("/todos/{id}/move") class="auth-form">
                <input type="hidden" name="return_to" value=format!("/todos/{id}")/>
                <label class="form-field">
                    <span>"Destination"</span>
                    <select name="context_kind" aria-label="Destination">
                        <option value="inbox">"Inbox"</option>
                        <option value="project">"Project"</option>
                        <option value="area">"Area"</option>
                        <option value="heading">"Heading"</option>
                    </select>
                </label>
                <label class="form-field">
                    <span>"Destination id"</span>
                    <input type="text" name="context_id" aria-label="Destination id"/>
                </label>
                <button type="submit" class="button-secondary">"Move"</button>
            </form>

            <div class="todo-actions">
                <form method="post" action=complete_target>
                    <button type="submit" class="button-secondary">"Complete"</button>
                </form>
                <form method="post" action=cancel_target>
                    <button type="submit" class="button-secondary">"Cancel"</button>
                </form>
                <form method="post" action=reopen_target>
                    <button type="submit" class="button-secondary">"Reopen"</button>
                </form>
                <form method="post" action=duplicate_target>
                    <button type="submit" class="button-secondary">"Duplicate"</button>
                </form>
                {is_deleted
                    .then(|| {
                        view! {
                            <form method="post" action=restore_target>
                                <button type="submit" class="button-secondary">"Restore"</button>
                            </form>
                        }
                    })}
                {(!is_deleted)
                    .then(|| {
                        view! {
                            <form method="post" action=delete_target>
                                <button type="submit" class="button-secondary">"Delete"</button>
                            </form>
                        }
                    })}
            </div>
            <h2 class="section-heading">"Tags"</h2>
            <TagAssignments todo_id=id tags=todo.tags.clone()/>
            <h2 class="section-heading">"Checklist"</h2>
            <ChecklistEditor todo=todo.clone()/>
            {is_terminal
                .then(|| {
                    view! {
                        <p class="todo-note">
                            "This to-do is logged in the Logbook until reopened."
                        </p>
                    }
                })}
        </section>
    }
}

#[component]
fn TagAssignments(todo_id: EntityId, tags: Vec<EntityId>) -> impl IntoView {
    let tag_service = use_context::<Option<Arc<TagService>>>().flatten();
    let account = use_account();
    let all_tags = AsyncDerived::new(move || {
        let service = tag_service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => {
                    service.list(account.account_id).await.unwrap_or_default()
                }
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="tag-assignments" data-testid="tag-assignments">
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let assigned = tags.clone();
                    all_tags
                        .get()
                        .map(|all| {
                            all.into_iter()
                                .map(|tag| {
                                    let is_assigned = assigned.contains(&tag.id);
                                    let action = if is_assigned {
                                        format!("/entities/{todo_id}/tags/{}/unassign", tag.id)
                                    } else {
                                        format!("/entities/{todo_id}/tags/{}/assign", tag.id)
                                    };
                                    let label = if is_assigned {
                                        format!("Remove tag {}", tag.title)
                                    } else {
                                        format!("Assign tag {}", tag.title)
                                    };
                                    let button = if is_assigned { "Remove" } else { "Assign" };
                                    view! {
                                        <form method="post" action=action class="row-schedule">
                                            <input
                                                type="hidden"
                                                name="return_to"
                                                value=format!("/todos/{todo_id}")
                                            />
                                            <span class="tag-name">{tag.title}</span>
                                            <button type="submit" class="button-secondary" aria-label=label>
                                                {button}
                                            </button>
                                        </form>
                                    }
                                })
                                .collect_view()
                        })
                }}
            </Suspense>
        </section>
    }
}

/// Recoverable Trash with restore and permanent deletion.
#[component]
pub fn TrashPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <TrashView/> }}
        </Protected>
    }
}

#[component]
fn TrashView() -> impl IntoView {
    let todo_service = use_context::<Option<Arc<TodoService>>>().flatten();
    let project_service = use_context::<Option<Arc<ProjectService>>>().flatten();
    let area_service = use_context::<Option<Arc<AreaService>>>().flatten();
    let account = use_account();
    let account_for_projects = account.clone();
    let account_for_areas = account.clone();
    let todos = AsyncDerived::new(move || {
        let service = todo_service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .list(account.account_id, TodoScope::Trash)
                    .await
                    .unwrap_or_default(),
                _ => Vec::new(),
            }
        }
    });
    let projects = AsyncDerived::new(move || {
        let service = project_service.clone();
        let account = account_for_projects.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => {
                    service.trash(account.account_id).await.unwrap_or_default()
                }
                _ => Vec::new(),
            }
        }
    });
    let areas = AsyncDerived::new(move || {
        let service = area_service.clone();
        let account = account_for_areas.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => {
                    service.trash(account.account_id).await.unwrap_or_default()
                }
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="list-page" data-testid="trash-page">
            <header class="list-header">
                <h1 class="list-title">"Trash"</h1>
                <p class="list-description">"Deleted items stay recoverable until you empty them."</p>
            </header>
            <form method="post" action="/trash/empty" data-testid="empty-trash-form">
                <input type="hidden" name="return_to" value="/trash"/>
                <button type="submit" class="button-secondary">"Empty Trash"</button>
            </form>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let todos = todos.get().unwrap_or_default();
                    let projects = projects.get().unwrap_or_default();
                    let areas = areas.get().unwrap_or_default();
                    if todos.is_empty() && projects.is_empty() && areas.is_empty() {
                        return view! {
                            <div class="list-empty" data-testid="trash-empty">
                                <p>"Trash is empty."</p>
                            </div>
                        }
                            .into_any();
                    }
                    view! {
                        <h2 class="section-heading">"To-Dos"</h2>
                        {trash_list(
                            "trash-todos",
                            todos
                                .into_iter()
                                .map(|todo| {
                                    let title = todo.title.clone();
                                    view! {
                                        <TrashRow
                                            title=title
                                            restore_action=format!("/todos/{}/restore", todo.id)
                                            purge_action=format!("/todos/{}/purge", todo.id)
                                        />
                                    }
                                })
                                .collect_view(),
                        )}
                        <h2 class="section-heading">"Projects"</h2>
                        {trash_list(
                            "trash-projects",
                            projects
                                .into_iter()
                                .map(|project| {
                                    let title = project.title.clone();
                                    view! {
                                        <TrashRow
                                            title=title
                                            restore_action=format!("/projects/{}/restore", project.id)
                                            purge_action=format!("/projects/{}/purge", project.id)
                                        />
                                    }
                                })
                                .collect_view(),
                        )}
                        <h2 class="section-heading">"Areas"</h2>
                        {trash_list(
                            "trash-areas",
                            areas
                                .into_iter()
                                .map(|area| {
                                    let title = area.title.clone();
                                    view! {
                                        <TrashRow
                                            title=title
                                            restore_action=format!("/areas/{}/restore", area.id)
                                            purge_action=format!("/areas/{}/purge", area.id)
                                        />
                                    }
                                })
                                .collect_view(),
                        )}
                    }
                        .into_any()
                }}
            </Suspense>
        </section>
    }
}

fn trash_list(testid: &str, rows: impl IntoView + 'static) -> AnyView {
    let testid = testid.to_string();
    view! { <ul class="task-list" data-testid=testid>{rows}</ul> }.into_any()
}

#[component]
fn TrashRow(
    #[prop(into)] title: String,
    #[prop(into)] restore_action: String,
    #[prop(into)] purge_action: String,
) -> impl IntoView {
    view! {
        <li class="task-row" data-testid="trash-row">
            <div class="task-row-body">
                <span class="task-title">{title}</span>
            </div>
            <div class="task-row-actions">
                <form method="post" action=restore_action>
                    <input type="hidden" name="return_to" value="/trash"/>
                    <button type="submit" class="button-secondary">"Restore"</button>
                </form>
                <form method="post" action=purge_action>
                    <input type="hidden" name="return_to" value="/trash"/>
                    <button type="submit" class="button-secondary">"Delete permanently"</button>
                </form>
            </div>
        </li>
    }
}

/// Logbook history with logging policy and explicit logging.
#[component]
pub fn LogbookPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <LogbookView/> }}
        </Protected>
    }
}

#[component]
fn LogbookView() -> impl IntoView {
    let service = use_context::<Option<Arc<LogbookService>>>().flatten();
    let account = use_account();
    let agenda = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service.agenda(account.account_id).await.ok(),
                _ => None,
            }
        }
    });

    view! {
        <section class="list-page" data-testid="logbook-page">
            <header class="list-header">
                <h1 class="list-title">"Logbook"</h1>
                <p class="list-description">"Completed and canceled items."</p>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    agenda
                        .get()
                        .map(|agenda| {
                            agenda.map_or_else(
                                || view! { <div></div> }.into_any(),
                                |agenda| view! { <LogbookBody agenda=agenda/> }.into_any(),
                            )
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn LogbookBody(agenda: LogbookAgenda) -> impl IntoView {
    let policy = agenda.policy;
    let is_manual = policy == momentum_domain::model::LoggingPolicy::Manual;
    let is_empty = agenda.days.is_empty() && agenda.pending.is_empty();

    view! {
        <form
            method="post"
            action="/logbook/policy"
            class="row-schedule"
            data-testid="logging-policy-form"
        >
            <input type="hidden" name="return_to" value="/logbook"/>
            <select name="policy" aria-label="Logging policy">
                <option value="automatic">"Automatic"</option>
                <option value="manual">"Manual"</option>
            </select>
            <button type="submit" class="button-secondary">"Apply policy"</button>
        </form>
        <p class="todo-meta" data-testid="logging-policy">
            {format!("Logging policy: {}", policy.as_str())}
        </p>
        {(!agenda.pending.is_empty())
            .then(|| {
                view! {
                    <section data-testid="logbook-pending">
                        <h2 class="section-heading">"Pending logging"</h2>
                        <ul class="task-list" data-testid="logbook-pending-list">
                            {agenda
                                .pending
                                .into_iter()
                                .map(|entry| {
                                    view! { <LogbookEntryRow entry=entry manual=is_manual/> }
                                })
                                .collect_view()}
                        </ul>
                    </section>
                }
            })}
        {if is_empty {
            view! {
                <div class="list-empty" data-testid="logbook-empty">
                    <p>"Nothing logged yet."</p>
                </div>
            }
                .into_any()
        } else {
            view! {
                {agenda
                    .days
                    .into_iter()
                    .map(|day| {
                        let label = day.date.as_naive().format("%A, %-d %B %Y").to_string();
                        let iso = day.date.to_string();
                        view! {
                            <section
                                class="logbook-day"
                                data-testid="logbook-day"
                                data-date=iso
                            >
                                <h2 class="section-heading">{label}</h2>
                                <ul class="task-list" data-testid="logbook-day-list">
                                    {day
                                        .entries
                                        .into_iter()
                                        .map(|entry| {
                                            view! { <LogbookEntryRow entry=entry manual=is_manual/> }
                                        })
                                        .collect_view()}
                                </ul>
                            </section>
                        }
                    })
                    .collect_view()}
            }
                .into_any()
        }}
    }
}

#[component]
fn LogbookEntryRow(entry: LogbookEntry, manual: bool) -> impl IntoView {
    match entry {
        LogbookEntry::Todo(todo) => view! { <TodoRow todo=todo/> }.into_any(),
        LogbookEntry::Project(project) => {
            let id = project.id;
            let title = project.title.clone();
            let status = project.status.as_str().to_string();
            let mut instants = Vec::new();
            if let Some(completed) = project.completed_at {
                instants.push(format!("Completed {}", completed.format("%Y-%m-%d %H:%M")));
            }
            if let Some(canceled) = project.canceled_at {
                instants.push(format!("Canceled {}", canceled.format("%Y-%m-%d %H:%M")));
            }
            if let Some(logged) = project.logged_at {
                instants.push(format!("Logged {}", logged.format("%Y-%m-%d %H:%M")));
            }
            let log_target = format!("/projects/{id}/log");
            let open_target = format!("/projects/{id}");
            view! {
                <li class="task-row" data-testid="logbook-project-row">
                    <div class="task-row-body">
                        <a class="task-title" href=open_target>
                            {title}
                        </a>
                        <span class="task-row-meta">
                            <span class="row-badge">{status}</span>
                            {instants
                                .into_iter()
                                .map(|instant| view! { <span class="row-badge">{instant}</span> })
                                .collect_view()}
                        </span>
                    </div>
                    <div class="task-row-actions">
                        {(manual && project.logged_at.is_none())
                            .then(|| {
                                view! {
                                    <form method="post" action=log_target>
                                        <input type="hidden" name="return_to" value="/logbook"/>
                                        <button type="submit" class="button-secondary">"Log"</button>
                                    </form>
                                }
                            })}
                    </div>
                </li>
            }
                .into_any()
        }
    }
}

/// Tags management page.
#[component]
pub fn TagsPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <TagsList/> }}
        </Protected>
    }
}

#[component]
fn TagsList() -> impl IntoView {
    let service = use_context::<Option<Arc<TagService>>>().flatten();
    let account = use_account();
    let tags = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => {
                    service.list(account.account_id).await.unwrap_or_default()
                }
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="list-page" data-testid="tags-page">
            <header class="list-header">
                <h1 class="list-title">"Tags"</h1>
                <p class="list-description">"Cross-cutting context with nesting."</p>
            </header>
            <form method="post" action="/tags" class="add-form" data-testid="add-tag-form">
                <input type="hidden" name="return_to" value="/tags"/>
                <input
                    type="text"
                    name="title"
                    placeholder="New tag"
                    aria-label="New tag"
                    autocomplete="off"
                    required=true
                />
                <input
                    type="text"
                    name="parent"
                    placeholder="Parent tag id (optional)"
                    aria-label="Parent tag id"
                    autocomplete="off"
                />
                <button type="submit" class="button-primary">"Add"</button>
            </form>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    tags.get()
                        .map(|tags| {
                            if tags.is_empty() {
                                view! {
                                    <div class="list-empty" data-testid="tags-empty">
                                        <p>"No tags yet."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                let all = tags.clone();
                                view! {
                                    <ul class="task-list" data-testid="tag-list">
                                        {tags
                                            .into_iter()
                                            .map(|tag| {
                                                view! { <TagRow tag=tag all=all.clone()/> }
                                            })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn TagRow(tag: TagSummary, all: Vec<TagSummary>) -> impl IntoView {
    let id = tag.id;
    let title = tag.title.clone();
    let parent_title = tag
        .parent
        .and_then(|parent| {
            all.iter()
                .find(|candidate| candidate.id == parent)
                .map(|candidate| candidate.title.clone())
        })
        .unwrap_or_else(|| "Top level".to_string());
    let counts = format!("{} tagged", tag.assigned_count);
    let rename_target = format!("/tags/{id}/rename");
    let move_target = format!("/tags/{id}/move");
    let up_target = format!("/tags/{id}/move_up");
    let down_target = format!("/tags/{id}/move_down");
    let delete_target = format!("/tags/{id}/delete");
    let parents = all
        .into_iter()
        .filter(|candidate| candidate.id != id)
        .collect::<Vec<_>>();

    view! {
        <li class="task-row" data-testid="tag-row">
            <div class="task-row-body">
                <span class="task-title" data-testid="tag-title">{title.clone()}</span>
                <span class="task-row-meta">
                    <span class="row-badge">{parent_title}</span>
                    <span class="row-badge">{counts}</span>
                </span>
            </div>
            <div class="task-row-actions">
                <form method="post" action=rename_target class="row-schedule">
                    <input type="hidden" name="return_to" value="/tags"/>
                    <input type="text" name="title" value=title aria-label="Tag title"/>
                    <button type="submit" class="button-secondary">"Rename"</button>
                </form>
                <form method="post" action=move_target class="row-schedule">
                    <input type="hidden" name="return_to" value="/tags"/>
                    <select name="parent" aria-label="Parent tag">
                        <option value="">"Top level"</option>
                        {parents
                            .into_iter()
                            .map(|candidate| {
                                let selected = Some(candidate.id) == tag.parent;
                                view! {
                                    <option value=candidate.id.to_string() selected=selected>
                                        {candidate.title}
                                    </option>
                                }
                            })
                            .collect_view()}
                    </select>
                    <button type="submit" class="button-secondary">"Move"</button>
                </form>
                <form method="post" action=up_target>
                    <input type="hidden" name="return_to" value="/tags"/>
                    <button type="submit" class="row-action" aria-label="Move tag up">
                        "\u{2191}"
                    </button>
                </form>
                <form method="post" action=down_target>
                    <input type="hidden" name="return_to" value="/tags"/>
                    <button type="submit" class="row-action" aria-label="Move tag down">
                        "\u{2193}"
                    </button>
                </form>
                <form method="post" action=delete_target>
                    <input type="hidden" name="return_to" value="/tags"/>
                    <button type="submit" class="button-secondary">"Delete"</button>
                </form>
            </div>
        </li>
    }
}

#[component]
fn ChecklistEditor(todo: TodoSummary) -> impl IntoView {
    let id = todo.id;
    let items = todo.checklist.clone();
    let return_to = format!("/todos/{id}");

    view! {
        <section class="checklist" data-testid="checklist-editor">
            <ul class="checklist-items" data-testid="checklist-items">
                {items
                    .into_iter()
                    .map(|item| {
                        let item_id = item.id;
                        let toggle_target = format!("/todos/{id}/checklist/{item_id}/toggle");
                        let rename_target = format!("/todos/{id}/checklist/{item_id}/rename");
                        let up_target = format!("/todos/{id}/checklist/{item_id}/move_up");
                        let down_target = format!("/todos/{id}/checklist/{item_id}/move_down");
                        let remove_target = format!("/todos/{id}/checklist/{item_id}/remove");
                        let next_completed = if item.completed { "0" } else { "1" };
                        let marker = if item.completed { "\u{2611}" } else { "\u{2610}" };
                        let toggle_label = if item.completed {
                            "Mark checklist item incomplete"
                        } else {
                            "Mark checklist item complete"
                        };
                        view! {
                            <li class="checklist-item" data-testid="checklist-item">
                                <form method="post" action=toggle_target class="row-schedule">
                                    <input type="hidden" name="return_to" value=return_to.clone()/>
                                    <input type="hidden" name="completed" value=next_completed/>
                                    <button type="submit" class="row-action" aria-label=toggle_label>
                                        {marker}
                                    </button>
                                </form>
                                <form method="post" action=rename_target class="row-schedule">
                                    <input type="hidden" name="return_to" value=return_to.clone()/>
                                    <input
                                        type="text"
                                        name="title"
                                        value=item.title
                                        aria-label="Checklist item"
                                    />
                                    <button type="submit" class="button-secondary">"Rename"</button>
                                </form>
                                <form method="post" action=up_target>
                                    <input type="hidden" name="return_to" value=return_to.clone()/>
                                    <button
                                        type="submit"
                                        class="row-action"
                                        aria-label="Move checklist item up"
                                    >
                                        "\u{2191}"
                                    </button>
                                </form>
                                <form method="post" action=down_target>
                                    <input type="hidden" name="return_to" value=return_to.clone()/>
                                    <button
                                        type="submit"
                                        class="row-action"
                                        aria-label="Move checklist item down"
                                    >
                                        "\u{2193}"
                                    </button>
                                </form>
                                <form method="post" action=remove_target>
                                    <input type="hidden" name="return_to" value=return_to.clone()/>
                                    <button type="submit" class="row-action">"Remove"</button>
                                </form>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
            <form
                method="post"
                action=format!("/todos/{id}/checklist")
                class="add-form"
                data-testid="add-checklist-item-form"
            >
                <input type="hidden" name="return_to" value=return_to.clone()/>
                <input
                    type="text"
                    name="title"
                    placeholder="New checklist item"
                    aria-label="New checklist item"
                    autocomplete="off"
                    required=true
                />
                <button type="submit" class="button-primary">"Add item"</button>
            </form>
            <form
                method="post"
                action=format!("/todos/{id}/checklist/expand")
                class="auth-form"
                data-testid="expand-checklist-form"
            >
                <input type="hidden" name="return_to" value=return_to/>
                <label class="form-field">
                    <span>"Paste lines"</span>
                    <textarea name="text" rows="4" aria-label="Paste checklist lines"></textarea>
                </label>
                <button type="submit" class="button-secondary">"Add pasted lines"</button>
            </form>
        </section>
    }
}

/// Areas overview with create/delete controls.
#[component]
pub fn AreasPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <AreasList/> }}
        </Protected>
    }
}

#[component]
fn AreasList() -> impl IntoView {
    let service = use_context::<Option<Arc<AreaService>>>().flatten();
    let account = use_account();
    let areas = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => {
                    service.list(account.account_id).await.unwrap_or_default()
                }
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="list-page" data-testid="areas-page">
            <header class="list-header">
                <h1 class="list-title">"Areas"</h1>
                <p class="list-description">"Ongoing responsibilities."</p>
            </header>
            <form method="post" action="/areas" class="add-form" data-testid="add-area-form">
                <input
                    type="text"
                    name="title"
                    placeholder="New area"
                    aria-label="New area"
                    autocomplete="off"
                    required=true
                />
                <button type="submit" class="button-primary">"Add"</button>
            </form>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    areas
                        .get()
                        .map(|areas| {
                            if areas.is_empty() {
                                view! {
                                    <div class="list-empty" data-testid="areas-empty">
                                        <p>"No areas yet."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ul class="task-list" data-testid="area-list">
                                        {areas
                                            .into_iter()
                                            .map(|area| view! { <AreaRow area=area/> })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AreaRow(area: AreaSummary) -> impl IntoView {
    let id = area.id;
    let title = area.title.clone();
    let counts = format!(
        "{} projects · {} to-dos",
        area.project_count, area.todo_count
    );
    let open_target = format!("/areas/{id}");
    let rename_target = format!("/areas/{id}/rename");
    let delete_target = format!("/areas/{id}/delete");

    view! {
        <li class="task-row" data-testid="area-row">
            <div class="task-row-body">
                <a class="task-title" href=open_target>{title.clone()}</a>
                <span class="task-row-meta">
                    <span class="row-badge">{counts}</span>
                </span>
            </div>
            <div class="task-row-actions">
                <form method="post" action=rename_target class="row-schedule">
                    <input type="text" name="title" value=title aria-label="Rename area"/>
                    <button type="submit" class="row-action" aria-label="Rename area">"Rename"</button>
                </form>
                <form method="post" action=delete_target class="row-schedule">
                    <select name="policy" aria-label="Deletion policy">
                        <option value="detach">"Keep contents"</option>
                        <option value="trash">"Trash contents"</option>
                    </select>
                    <button type="submit" class="row-action" aria-label="Delete area">"Delete"</button>
                </form>
            </div>
        </li>
    }
}

/// Area detail listing projects and loose to-dos.
#[component]
pub fn AreaPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("area_id").unwrap_or_default();
    view! {
        <Protected>
            {move || view! { <AreaDetail id=id()/> }}
        </Protected>
    }
}

#[component]
fn AreaDetail(#[prop(into)] id: String) -> impl IntoView {
    let area_service = use_context::<Option<Arc<AreaService>>>().flatten();
    let todo_service = use_context::<Option<Arc<TodoService>>>().flatten();
    let account = use_account();
    let account_for_area = account.clone();
    let parsed = id.parse::<momentum_domain::EntityId>().ok();

    let area = AsyncDerived::new(move || {
        let service = area_service.clone();
        let account = account_for_area.clone();
        async move {
            match (service, account, parsed) {
                (Some(service), Some(account), Some(id)) => {
                    service.get(account.account_id, id).await.ok().flatten()
                }
                _ => None,
            }
        }
    });

    let loose_todos = AsyncDerived::new(move || {
        let service = todo_service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .list(account.account_id, TodoScope::AllOpen)
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|todo| {
                        parsed.is_some_and(|id| {
                            todo.context == momentum_domain::model::ContextRef::Area(id)
                        })
                    })
                    .collect(),
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="entity-page" data-testid="area-detail">
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    area.get()
                        .map(|area| {
                            area.map_or_else(
                                || {
                                    view! {
                                        <header class="entity-header">
                                            <h1 class="entity-title">"Area not found"</h1>
                                            <p>
                                                <a href="/areas">"Back to Areas"</a>
                                            </p>
                                        </header>
                                    }
                                        .into_any()
                                },
                                |area| {
                                    view! {
                                        <header class="entity-header">
                                            <h1 class="entity-title" data-testid="area-title">
                                                {area.title.clone()}
                                            </h1>
                                            <p class="todo-meta">
                                                {format!(
                                                    "{} projects · {} loose to-dos",
                                                    area.project_count,
                                                    area.todo_count,
                                                )}
                                            </p>
                                        </header>
                                    }
                                        .into_any()
                                },
                            )
                        })
                }}
            </Suspense>
            <h2 class="section-heading">"Loose to-dos"</h2>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    loose_todos
                        .get()
                        .map(|todos| {
                            if todos.is_empty() {
                                view! {
                                    <div class="list-empty" data-testid="area-loose-empty">
                                        <p>"No loose to-dos."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ul class="task-list" data-testid="area-loose-list">
                                        {todos
                                            .into_iter()
                                            .map(|todo| view! { <TodoRow todo=todo/> })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </section>
    }
}

/// Projects overview with create controls and derived progress.
#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <ProjectsList/> }}
        </Protected>
    }
}

#[component]
fn ProjectsList() -> impl IntoView {
    let service = use_context::<Option<Arc<ProjectService>>>().flatten();
    let account = use_account();
    let projects = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => {
                    service.list(account.account_id).await.unwrap_or_default()
                }
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="list-page" data-testid="projects-page">
            <header class="list-header">
                <h1 class="list-title">"Projects"</h1>
                <p class="list-description">"All open projects."</p>
            </header>
            <form method="post" action="/projects" class="add-form" data-testid="add-project-form">
                <input
                    type="text"
                    name="title"
                    placeholder="New project"
                    aria-label="New project"
                    autocomplete="off"
                    required=true
                />
                <button type="submit" class="button-primary">"Add"</button>
            </form>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    projects
                        .get()
                        .map(|projects| {
                            if projects.is_empty() {
                                view! {
                                    <div class="list-empty" data-testid="projects-empty">
                                        <p>"No projects yet."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ul class="task-list" data-testid="project-list">
                                        {projects
                                            .into_iter()
                                            .map(|project| view! { <ProjectListRow project=project/> })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn ProjectListRow(project: ProjectSummary) -> impl IntoView {
    let id = project.id;
    let title = project.title.clone();
    let counts = format!(
        "{} open · {} done",
        project.open_count, project.completed_count
    );
    let open_target = format!("/projects/{id}");

    view! {
        <li class="task-row" data-testid="project-row">
            <div class="task-row-body">
                <a class="task-title" href=open_target>{title}</a>
                <span class="task-row-meta">
                    <span class="row-badge">{counts}</span>
                    {project
                        .schedule
                        .date()
                        .map(|date| {
                            view! {
                                <span class="row-badge date-badge">{format!("When: {date}")}</span>
                            }
                        })}
                </span>
            </div>
        </li>
    }
}

/// Project detail with notes, lifecycle controls, and direct to-dos.
#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("project_id").unwrap_or_default();
    view! {
        <Protected>
            {move || view! { <ProjectDetail id=id()/> }}
        </Protected>
    }
}

#[component]
fn ProjectDetail(#[prop(into)] id: String) -> impl IntoView {
    let project_service = use_context::<Option<Arc<ProjectService>>>().flatten();
    let todo_service = use_context::<Option<Arc<TodoService>>>().flatten();
    let account = use_account();
    let account_for_project = account.clone();
    let parsed = id.parse::<momentum_domain::EntityId>().ok();
    let today = project_service
        .as_ref()
        .map(|service| service.local_today());

    let project = AsyncDerived::new(move || {
        let service = project_service.clone();
        let account = account_for_project.clone();
        async move {
            match (service, account, parsed) {
                (Some(service), Some(account), Some(id)) => {
                    service.get(account.account_id, id).await.ok().flatten()
                }
                _ => None,
            }
        }
    });

    let children = AsyncDerived::new(move || {
        let service = todo_service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .list(account.account_id, TodoScope::AllOpen)
                    .await
                    .unwrap_or_default(),
                _ => Vec::new(),
            }
        }
    });

    view! {
        <section class="entity-page" data-testid="project-detail">
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    project
                        .get()
                        .map(|project| {
                            project.map_or_else(
                                || {
                                    view! {
                                        <header class="entity-header">
                                            <h1 class="entity-title">"Project not found"</h1>
                                            <p>
                                                <a href="/projects">"Back to Projects"</a>
                                            </p>
                                        </header>
                                    }
                                        .into_any()
                                },
                                |project| view! { <ProjectDetailBody project=project/> }.into_any(),
                            )
                        })
                }}
            </Suspense>
            <h2 class="section-heading">"Headings"</h2>
            <form method="post" action="/headings" class="add-form" data-testid="add-heading-form">
                <input type="hidden" name="project_id" value=id.clone()/>
                <input type="hidden" name="return_to" value=format!("/projects/{id}")/>
                <input
                    type="text"
                    name="title"
                    placeholder="New heading"
                    aria-label="New heading"
                    autocomplete="off"
                    required=true
                />
                <button type="submit" class="button-secondary">"Add heading"</button>
            </form>
            <h2 class="section-heading">"To-Dos"</h2>
            <form method="post" action="/todos" class="add-form">
                <input type="hidden" name="return_to" value=format!("/projects/{id}")/>
                <input type="hidden" name="context_kind" value="project"/>
                <input type="hidden" name="context_id" value=id.clone()/>
                <input
                    type="text"
                    name="title"
                    placeholder="New to-do in this project"
                    aria-label="New to-do in this project"
                    autocomplete="off"
                    required=true
                />
                <button type="submit" class="button-primary">"Add"</button>
            </form>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let Some(Some(project)) = project.get() else {
                        return ().into_any();
                    };
                    let todos = children.get().unwrap_or_default();
                    let by_id: std::collections::HashMap<EntityId, TodoSummary> =
                        todos.into_iter().map(|todo| (todo.id, todo)).collect();
                    let archived: Vec<HeadingSummary> =
                        project.headings.iter().filter(|heading| heading.archived).cloned().collect();
                    let return_to = format!("/projects/{id}");
                    view! {
                        <div data-testid="project-structure">
                            {project
                                .order
                                .iter()
                                .map(|item| {
                                    match item {
                                        ProjectOrderItem::Heading(heading_id) => {
                                            project
                                                .headings
                                                .iter()
                                                .find(|heading| heading.id == *heading_id)
                                                .map(|heading| {
                                                    let heading_todos = heading
                                                        .todo_ids
                                                        .iter()
                                                        .filter_map(|todo_id| by_id.get(todo_id).cloned())
                                                        .collect::<Vec<_>>();
                                                    view! {
                                                        <HeadingSection
                                                            heading=heading.clone()
                                                            todos=heading_todos
                                                            return_to=return_to.clone()
                                                        />
                                                    }
                                                        .into_any()
                                                })
                                        }
                                        ProjectOrderItem::Todo(todo_id) => {
                                            by_id
                                                .get(todo_id)
                                                .cloned()
                                                .map(|todo| {
                                                    view! { <TodoRow todo=todo today=today/> }.into_any()
                                                })
                                        }
                                    }
                                })
                                .collect_view()}
                            {(!archived.is_empty())
                                .then(|| {
                                    view! {
                                        <section
                                            class="archived-headings"
                                            data-testid="archived-headings"
                                        >
                                            <h3 class="section-heading">"Archived headings"</h3>
                                            {archived
                                                .into_iter()
                                                .map(|heading| {
                                                    let restore = format!("/headings/{}/restore", heading.id);
                                                    view! {
                                                        <form method="post" action=restore class="row-schedule">
                                                            <input
                                                                type="hidden"
                                                                name="return_to"
                                                                value=return_to.clone()
                                                            />
                                                            <span>{heading.title}</span>
                                                            <button type="submit" class="button-secondary">
                                                                "Restore"
                                                            </button>
                                                        </form>
                                                    }
                                                })
                                                .collect_view()}
                                        </section>
                                    }
                                })}
                        </div>
                    }
                        .into_any()
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn HeadingSection(
    heading: HeadingSummary,
    todos: Vec<TodoSummary>,
    #[prop(into)] return_to: String,
) -> impl IntoView {
    let id = heading.id;
    let title = heading.title.clone();
    let rename_target = format!("/headings/{id}/rename");
    let up_target = format!("/headings/{id}/move_up");
    let down_target = format!("/headings/{id}/move_down");
    let archive_target = format!("/headings/{id}/archive");

    view! {
        <section class="heading-section" data-testid="heading-section">
            <div class="heading-row">
                <h3 class="heading-title" data-testid="heading-title">
                    {title.clone()}
                </h3>
                <form method="post" action=rename_target class="row-schedule">
                    <input type="hidden" name="return_to" value=return_to.clone()/>
                    <input type="text" name="title" value=title aria-label="Heading title"/>
                    <button type="submit" class="button-secondary">"Rename"</button>
                </form>
                <form method="post" action=up_target>
                    <input type="hidden" name="return_to" value=return_to.clone()/>
                    <button type="submit" class="button-secondary" aria-label="Move heading up">
                        "Up"
                    </button>
                </form>
                <form method="post" action=down_target>
                    <input type="hidden" name="return_to" value=return_to.clone()/>
                    <button type="submit" class="button-secondary" aria-label="Move heading down">
                        "Down"
                    </button>
                </form>
                <form method="post" action=archive_target>
                    <input type="hidden" name="return_to" value=return_to.clone()/>
                    <button type="submit" class="button-secondary">"Archive"</button>
                </form>
            </div>
            <form method="post" action="/todos" class="add-form">
                <input type="hidden" name="return_to" value=return_to.clone()/>
                <input type="hidden" name="context_kind" value="heading"/>
                <input type="hidden" name="context_id" value=id.to_string()/>
                <input
                    type="text"
                    name="title"
                    placeholder="New to-do in this heading"
                    aria-label="New to-do in this heading"
                    autocomplete="off"
                    required=true
                />
                <button type="submit" class="button-primary">"Add"</button>
            </form>
            <ul class="task-list" data-testid="heading-children">
                {todos.into_iter().map(|todo| view! { <TodoRow todo=todo/> }).collect_view()}
            </ul>
        </section>
    }
}

#[component]
fn ProjectDetailBody(project: ProjectSummary) -> impl IntoView {
    let id = project.id;
    let title = project.title.clone();
    let notes = project.notes.clone();
    let rename_target = format!("/projects/{id}/rename");
    let notes_target = format!("/projects/{id}/notes");
    let complete_target = format!("/projects/{id}/complete");
    let cancel_target = format!("/projects/{id}/cancel");
    let reopen_target = format!("/projects/{id}/reopen");
    let schedule_target = format!("/projects/{id}/schedule");
    let delete_target = format!("/projects/{id}/delete");
    let schedule = project
        .schedule
        .date()
        .map_or_else(|| "Anytime".to_string(), |date| date.to_string());
    let deadline = project
        .deadline
        .map_or_else(|| "None".to_string(), |deadline| deadline.date.to_string());
    let counts = format!(
        "{} open · {} done",
        project.open_count, project.completed_count
    );

    view! {
        <header class="entity-header">
            <h1 class="entity-title" data-testid="project-title">
                {title.clone()}
            </h1>
            <p class="todo-meta">
                <span data-testid="project-status">
                    {format!("Status: {}", project.status.as_str())}
                </span>
                " · "
                <span>{format!("When: {schedule}")}</span>
                " · "
                <span>{format!("Deadline: {deadline}")}</span>
                " · "
                <span data-testid="project-counts">{counts}</span>
            </p>
        </header>

        <form method="post" action=rename_target class="auth-form">
            <label class="form-field">
                <span>"Title"</span>
                <input type="text" name="title" value=title aria-label="Project title"/>
            </label>
            <button type="submit" class="button-secondary">"Rename"</button>
        </form>

        <form method="post" action=notes_target class="auth-form">
            <label class="form-field">
                <span>"Notes"</span>
                <textarea name="notes" rows="5" aria-label="Project notes">
                    {notes.clone()}
                </textarea>
            </label>
            <button type="submit" class="button-secondary">"Save notes"</button>
        </form>

        <NotesTools
            notes=notes.clone()
            replace_action=format!("/projects/{id}/notes/replace")
            return_to=format!("/projects/{id}")
        />

        <form method="post" action=schedule_target class="auth-form">
            <label class="form-field">
                <span>"When"</span>
                <select name="when" aria-label="Project when">
                    <option value="today">"Today"</option>
                    <option value="evening">"This Evening"</option>
                    <option value="tomorrow">"Tomorrow"</option>
                    <option value="someday">"Someday"</option>
                    <option value="anytime">"Anytime"</option>
                    <option value="date">"On a date"</option>
                </select>
            </label>
            <label class="form-field">
                <span>"Date (for On a date)"</span>
                <input type="date" name="date" aria-label="Project schedule date"/>
            </label>
            <button type="submit" class="button-secondary">"Apply schedule"</button>
        </form>

        <form method="post" action=format!("/projects/{id}/deadline") class="auth-form">
            <input type="hidden" name="return_to" value=format!("/projects/{id}")/>
            <label class="form-field">
                <span>"Deadline"</span>
                <input type="date" name="date" aria-label="Project deadline date"/>
            </label>
            <button type="submit" class="button-secondary">"Apply deadline"</button>
        </form>

        <div class="todo-actions">
            <form method="post" action=complete_target class="row-schedule">
                <select name="policy" aria-label="Completion policy">
                    <option value="reject">"Only when empty"</option>
                    <option value="cascade">"Complete children"</option>
                </select>
                <button type="submit" class="button-secondary">"Complete"</button>
            </form>
            <form method="post" action=cancel_target>
                <button type="submit" class="button-secondary">"Cancel"</button>
            </form>
            <form method="post" action=reopen_target>
                <button type="submit" class="button-secondary">"Reopen"</button>
            </form>
            <form method="post" action=delete_target class="row-schedule">
                <select name="policy" aria-label="Deletion policy">
                    <option value="detach">"Keep children"</option>
                    <option value="trash">"Trash children"</option>
                </select>
                <button type="submit" class="button-secondary">"Delete"</button>
            </form>
        </div>
    }
}

fn encode_query(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

/// Notes tools: detected links, find traversal, and deterministic replacement.
#[component]
fn NotesTools(
    #[prop(into)] notes: String,
    #[prop(into)] replace_action: String,
    #[prop(into)] return_to: String,
) -> impl IntoView {
    let query = use_query_map();
    let link_source = notes.clone();
    let links = momentum_domain::notes::find_links(&link_source);
    let find_term = query.read().get("find").unwrap_or_default();
    let from: usize = query
        .read()
        .get("from")
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);
    let matches = momentum_domain::notes::find_matches(&notes, &find_term);
    let current = matches
        .iter()
        .position(|(start, _)| *start >= from)
        .unwrap_or(0);
    let (prev_start, next_start) = match matches.get(current) {
        Some((start, end)) => {
            let previous = momentum_domain::notes::previous_match(&notes, &find_term, *start)
                .map_or(*start, |(start, _)| start);
            (previous, *end)
        }
        None => (0, 0),
    };
    let encoded = encode_query(&find_term);
    let result_label = if find_term.is_empty() {
        String::new()
    } else if matches.is_empty() {
        "No matches".to_string()
    } else {
        format!("{}/{}", current + 1, matches.len())
    };
    let find_value = find_term.clone();

    view! {
        <section class="notes-tools" data-testid="notes-tools">
            {(!links.is_empty())
                .then(|| {
                    view! {
                        <ul class="note-links" data-testid="note-links">
                            {links
                                .into_iter()
                                .map(|link| {
                                    if link.safe {
                                        let href = link.url.clone();
                                        view! {
                                            <li>
                                                <a
                                                    href=href
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    data-testid="note-link"
                                                >
                                                    {link.url}
                                                </a>
                                            </li>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <li>
                                                <span
                                                    class="note-link-blocked"
                                                    data-testid="blocked-link"
                                                    aria-label="Blocked unsafe link"
                                                >
                                                    {format!("{} (blocked scheme)", link.url)}
                                                </span>
                                            </li>
                                        }
                                            .into_any()
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                })}
            <MarkdownPreview notes=link_source.clone()/>
            <form method="get" action="" class="row-schedule" data-testid="find-form">
                <input
                    type="text"
                    name="find"
                    value=find_value
                    placeholder="Find in notes"
                    aria-label="Find in notes"
                    autocomplete="off"
                />
                <button type="submit" class="button-secondary">"Find"</button>
            </form>
            {(!result_label.is_empty())
                .then(|| {
                    view! {
                        <div class="find-results" data-testid="find-results">
                            <span aria-live="polite">{result_label}</span>
                            {(!matches.is_empty())
                                .then(|| {
                                    view! {
                                        <a
                                            href=format!("?find={encoded}&from={prev_start}")
                                            class="row-action"
                                            aria-label="Previous match"
                                            data-testid="find-previous"
                                        >
                                            "Previous"
                                        </a>
                                        <a
                                            href=format!("?find={encoded}&from={next_start}")
                                            class="row-action"
                                            aria-label="Next match"
                                            data-testid="find-next"
                                        >
                                            "Next"
                                        </a>
                                    }
                                })}
                        </div>
                    }
                })}
            {(!find_term.is_empty())
                .then(|| {
                    view! {
                        <form
                            method="post"
                            action=replace_action.clone()
                            class="row-schedule"
                            data-testid="replace-form"
                        >
                            <input type="hidden" name="find" value=find_term.clone()/>
                            <input type="hidden" name="return_to" value=return_to.clone()/>
                            <input
                                type="text"
                                name="replace"
                                placeholder="Replace with"
                                aria-label="Replace with"
                                autocomplete="off"
                            />
                            <button type="submit" class="button-secondary">"Replace all"</button>
                        </form>
                    }
                })}
        </section>
    }
}

fn render_span(span: momentum_domain::markdown::InlineSpan) -> AnyView {
    let mut node: AnyView = view! { {span.text.clone()} }.into_any();
    if span.code {
        node = view! { <code class="md-inline-code">{node}</code> }.into_any();
    }
    if span.highlight {
        node = view! { <mark class="md-highlight">{node}</mark> }.into_any();
    }
    if span.emphasis {
        node = view! { <em>{node}</em> }.into_any();
    }
    if span.strong {
        node = view! { <strong>{node}</strong> }.into_any();
    }
    if let Some(link) = span.link {
        node = if link.safe {
            let url = link.url;
            view! {
                <a
                    class="md-link"
                    href=url
                    target="_blank"
                    rel="noopener noreferrer"
                    data-testid="markdown-link"
                >
                    {node}
                </a>
            }
            .into_any()
        } else {
            view! {
                <span
                    class="md-blocked"
                    data-testid="markdown-blocked-link"
                    aria-label="Blocked unsafe link"
                >
                    {node}
                </span>
            }
            .into_any()
        };
    }
    node
}

fn render_spans(spans: Vec<momentum_domain::markdown::InlineSpan>) -> AnyView {
    spans.into_iter().map(render_span).collect_view().into_any()
}

/// Renders the supported Markdown subset derived from the stored source.
#[component]
fn MarkdownPreview(#[prop(into)] notes: String) -> impl IntoView {
    let blocks = momentum_domain::markdown::parse_markdown(&notes);
    let mut nodes: Vec<AnyView> = Vec::new();
    let mut list: Vec<(bool, String, Vec<momentum_domain::markdown::InlineSpan>)> = Vec::new();

    fn flush_list(
        list: &mut Vec<(bool, String, Vec<momentum_domain::markdown::InlineSpan>)>,
        nodes: &mut Vec<AnyView>,
    ) {
        if list.is_empty() {
            return;
        }
        let ordered = list[0].0;
        let items: Vec<AnyView> = list
            .drain(..)
            .map(|(_, marker, spans)| {
                view! {
                    <li class="md-list-item" data-marker=marker>
                        {render_spans(spans)}
                    </li>
                }
                .into_any()
            })
            .collect();
        if ordered {
            nodes.push(view! { <ol class="md-list">{items}</ol> }.into_any());
        } else {
            nodes.push(view! { <ul class="md-list">{items}</ul> }.into_any());
        }
    }

    for block in blocks {
        match block {
            momentum_domain::markdown::Block::ListItem {
                ordered,
                marker,
                spans,
            } => {
                if list.last().is_some_and(|(kind, _, _)| *kind != ordered) {
                    flush_list(&mut list, &mut nodes);
                }
                list.push((ordered, marker, spans));
            }
            momentum_domain::markdown::Block::Heading { level, spans } => {
                flush_list(&mut list, &mut nodes);
                let class = format!("md-heading md-heading-{level}");
                nodes.push(
                    view! {
                        <h3 class=class data-level=level.to_string()>
                            {render_spans(spans)}
                        </h3>
                    }
                    .into_any(),
                );
            }
            momentum_domain::markdown::Block::Paragraph { spans } => {
                flush_list(&mut list, &mut nodes);
                nodes.push(view! { <p class="md-paragraph">{render_spans(spans)}</p> }.into_any());
            }
            momentum_domain::markdown::Block::Quote { spans } => {
                flush_list(&mut list, &mut nodes);
                nodes.push(
                    view! { <blockquote class="md-quote">{render_spans(spans)}</blockquote> }
                        .into_any(),
                );
            }
            momentum_domain::markdown::Block::CodeBlock { language, text } => {
                flush_list(&mut list, &mut nodes);
                let label = language.clone().unwrap_or_else(|| "code".to_string());
                nodes.push(
                    view! {
                        <pre class="md-code" aria-label=format!("Code block ({label})")>
                            <code>{text}</code>
                        </pre>
                    }
                    .into_any(),
                );
            }
        }
    }
    flush_list(&mut list, &mut nodes);

    view! {
        <section
            class="markdown-preview"
            data-testid="markdown-preview"
            aria-label="Formatted note preview"
        >
            {nodes}
        </section>
    }
}

/// A schedule-driven projection list.
#[component]
fn ProjectionList(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    scope: TodoScope,
    #[prop(into)] slug: String,
) -> impl IntoView {
    let service = use_context::<Option<Arc<TodoService>>>().flatten();
    let account = use_account();
    let todos = AsyncDerived::new(move || {
        let service = service.clone();
        let account = account.clone();
        async move {
            match (service, account) {
                (Some(service), Some(account)) => service
                    .list(account.account_id, scope)
                    .await
                    .unwrap_or_default(),
                _ => Vec::new(),
            }
        }
    });
    let page_testid = format!("{slug}-page");
    let list_testid = format!("{slug}-list");
    let empty_testid = format!("{slug}-empty");

    view! {
        <section class="list-page" data-testid=page_testid>
            <header class="list-header">
                <h1 class="list-title">{title}</h1>
                <p class="list-description">{description}</p>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    todos
                        .get()
                        .map(|todos| {
                            if todos.is_empty() {
                                view! {
                                    <div class="list-empty" data-testid=empty_testid.clone()>
                                        <p>"Nothing here yet."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ul class="task-list" data-testid=list_testid.clone()>
                                        {todos
                                            .into_iter()
                                            .map(|todo| view! { <TodoRow todo=todo/> })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </section>
    }
}
