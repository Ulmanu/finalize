//! Route table and destination pages for the application shell.
//!
//! Feature content (task rows, projections, editors) is added by later roadmap changes; this
//! module establishes the stable, bookmarkable destinations and the session boundary each
//! protected destination uses. Read models use blocking resources so the browser hydrates the
//! server's data. Links use native document navigation to fetch fresh server-backed projections.

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
use momentum_app::calendar::{CalendarRange, CalendarService};
use momentum_app::logbook::{LogbookAgenda, LogbookEntry, LogbookService};
use momentum_app::projects::{HeadingSummary, ProjectOrderItem, ProjectService, ProjectSummary};
use momentum_app::recurring::{RecurrenceTemplateSummary, RecurringService};
use momentum_app::search::{RecentsStore, SearchField, SearchHit, SearchService};
use momentum_app::tags::{TagService, TagSummary};
use momentum_app::todos::{Progress, TodoScope, TodoService, TodoSummary};
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
                <Route path=path!("/tomorrow") view=TomorrowPage/>
                <Route path=path!("/repeating") view=RecurringPage/>
                <Route path=path!("/projects/logged") view=LoggedProjectsPage/>
                <Route path=path!("/settings") view=SettingsPage/>
                <Route path=path!("/search") view=SearchPage/>
                <Route path=path!("/companion") view=CompanionPage/>
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
                    <a rel="external" class="button-primary" href="/signup">"Create an account"</a>
                    <a rel="external" class="button-secondary" href="/login">"Sign in"</a>
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
                    <a rel="external" href="/signup">"Create an account"</a>
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
                    <a rel="external" href="/login">"I already have an account"</a>
                </p>
            </section>
        </PublicLayout>
    }
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
    let account_for_progress = account.clone();
    let service_for_progress = service.clone();
    let today =
        SharedValue::new(|| service.as_ref().map(|service| service.local_today())).into_inner();
    let todos = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
    let progress = Resource::new_blocking(
        || (),
        move |_| {
            let service = service_for_progress.clone();
            let account = account_for_progress.clone();
            async move {
                match (service, account) {
                    (Some(service), Some(account)) => {
                        service.today_progress(account.account_id).await.ok()
                    }
                    _ => None,
                }
            }
        },
    );

    view! {
        <section class="list-page" data-testid="today-page">
            <header class="list-header">
                <h1 class="list-title">"Today"</h1>
                <p class="list-description">"Everything scheduled for today."</p>
                <p>
                    <a rel="external" href="/tomorrow" class="row-action">"Tomorrow"</a>
                </p>
                <form
                    method="post"
                    action="/todos"
                    class="add-form"
                    data-testid="today-add-form"
                >
                    <input type="hidden" name="return_to" value="/today"/>
                    <input type="hidden" name="schedule_kind" value="today"/>
                    <input
                        type="text"
                        name="title"
                        placeholder="New to-do for today"
                        aria-label="New to-do for today"
                        autocomplete="off"
                        required=true
                    />
                    <button type="submit" class="button-primary">"Add"</button>
                </form>
                <Suspense fallback=|| ()>
                {move || {
                    progress
                        .get()
                        .flatten()
                        .map(|progress| {
                            if progress.is_neutral() {
                                view! {
                                    <p class="todo-meta" data-testid="today-progress-neutral">
                                        "Nothing to complete today."
                                    </p>
                                }
                                    .into_any()
                            } else {
                                let percent = progress.percent().unwrap_or(0);
                                view! {
                                    <p class="todo-meta" data-testid="today-progress">
                                        <progress
                                            value=progress.completed.to_string()
                                            max=progress.total.to_string()
                                            aria-label=format!(
                                                "{} of {} Today items completed",
                                                progress.completed,
                                                progress.total,
                                            )
                                        ></progress>
                                        <span>
                                            {format!(
                                                "{} of {} completed ({percent}%)",
                                                progress.completed, progress.total,
                                            )}
                                        </span>
                                    </p>
                                }
                                    .into_any()
                            }
                        })
                }}
                </Suspense>
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
                                                .enumerate()
                                                .map(|(index, todo)| {
                                                    let magic_plus = MagicPlusTarget {
                                                        position: index,
                                                        position_scope: Some("today".to_string()),
                                                        context_kind: None,
                                                        context_id: None,
                                                        return_to: "/today".to_string(),
                                                        schedule_kind: Some("today".to_string()),
                                                        label: todo.title.clone(),
                                                    };
                                                    view! {
                                                        <TodoRow
                                                            todo=todo
                                                            today=today
                                                            today_order=true
                                                            magic_plus=Some(magic_plus)
                                                        />
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
                                                .enumerate()
                                                .map(|(index, todo)| {
                                                    let magic_plus = MagicPlusTarget {
                                                        position: index,
                                                        position_scope: Some("today".to_string()),
                                                        context_kind: None,
                                                        context_id: None,
                                                        return_to: "/today".to_string(),
                                                        schedule_kind: Some("evening".to_string()),
                                                        label: todo.title.clone(),
                                                    };
                                                    view! {
                                                        <TodoRow
                                                            todo=todo
                                                            today=today
                                                            today_order=true
                                                            magic_plus=Some(magic_plus)
                                                        />
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
            {today
                .map(|today| view! { <CalendarSection range_start=today range_end=today/> })}
        </section>
    }
}

/// Keyboard-invoked capture overlay: creates a fully-specified to-do without navigating away.
#[component]
pub fn QuickEntryOverlay() -> impl IntoView {
    let project_service = use_context::<Option<Arc<ProjectService>>>().flatten();
    let tag_service = use_context::<Option<Arc<TagService>>>().flatten();
    let account = use_account();
    let account_for_projects = account.clone();
    let projects = Resource::new_blocking(
        || (),
        move |_| {
            let service = project_service.clone();
            let account = account_for_projects.clone();
            async move {
                match (service, account) {
                    (Some(service), Some(account)) => service
                        .list(account.account_id)
                        .await
                        .unwrap_or_default()
                        .into_iter()
                        .map(|project| (project.id, project.title))
                        .collect::<Vec<_>>(),
                    _ => Vec::new(),
                }
            }
        },
    );
    let tags = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
    let location = leptos_router::hooks::use_location();
    let return_to = move || location.pathname.get();
    view! {
        <div
            id="quick-entry"
            class="quick-entry"
            data-testid="quick-entry"
            role="dialog"
            aria-modal="true"
            aria-label="Quick Entry"
        >
            <div class="quick-entry-panel motion-popover">
                <h2 class="section-heading">"Quick Entry"</h2>
                <Suspense fallback=|| ()>
                <form
                    method="post"
                    action="/todos/quick"
                    class="quick-entry-form"
                    data-testid="quick-entry-form"
                >
                    <input type="hidden" name="return_to" value=return_to/>
                    <input
                        type="text"
                        name="title"
                        aria-label="Title"
                        placeholder="Title"
                        autocomplete="off"
                        required=true
                    />
                    <textarea name="notes" aria-label="Notes" rows="2" placeholder="Notes"></textarea>
                    <select name="schedule_kind" aria-label="When">
                        <option value="none">"No date"</option>
                        <option value="today">"Today"</option>
                        <option value="evening">"This Evening"</option>
                        <option value="tomorrow">"Tomorrow"</option>
                        <option value="someday">"Someday"</option>
                        <option value="anytime">"Anytime"</option>
                        <option value="date">"On date"</option>
                    </select>
                    <input type="date" name="date" aria-label="Schedule date"/>
                    <input type="date" name="deadline" aria-label="Deadline"/>
                    <select name="destination" aria-label="Destination">
                        <option value="">"Inbox"</option>
                        {move || {
                            projects
                                .get()
                                .unwrap_or_default()
                                .into_iter()
                                .map(|(id, title)| {
                                    let value = format!("project:{id}");
                                    view! { <option value=value>{title}</option> }
                                })
                                .collect_view()
                        }}
                    </select>
                    <select name="tag" aria-label="Tag">
                        <option value="">"No tag"</option>
                        {move || {
                            tags.get()
                                .unwrap_or_default()
                                .into_iter()
                                .map(|tag| {
                                    view! { <option value=tag.id.to_string()>{tag.title}</option> }
                                })
                                .collect_view()
                        }}
                    </select>
                    <textarea
                        name="checklist"
                        aria-label="Checklist"
                        rows="2"
                        placeholder="Checklist (one item per line)"
                    ></textarea>
                    <div class="quick-entry-actions">
                        <button type="submit" class="button-primary">"Save"</button>
                        <a rel="external" href="#" class="row-action">"Cancel"</a>
                    </div>
                </form>
                </Suspense>
            </div>
        </div>
    }
}

/// Typed settings registry rendered with declared scopes and validated submission.
#[component]
fn PreferencesSection() -> impl IntoView {
    let store =
        use_context::<Option<Arc<dyn momentum_app::preferences::PreferenceStore>>>().flatten();
    let account = use_account();
    let stored = Resource::new_blocking(
        || (),
        move |_| {
            let store = store.clone();
            let account = account.clone();
            async move {
                match (store, account) {
                    (Some(store), Some(account)) => store
                        .load_all(account.account_id)
                        .await
                        .unwrap_or_default()
                        .into_iter()
                        .collect::<std::collections::HashMap<String, String>>(),
                    _ => std::collections::HashMap::new(),
                }
            }
        },
    );
    view! {
        <section class="preferences" data-testid="preferences">
            <h2 class="section-heading">"Preferences"</h2>
            <p class="list-description">
                "Each preference declares whether it is account-synced, device-local, or browser-local."
            </p>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let stored = stored.get().unwrap_or_default();
                    momentum_app::preferences::PreferenceDomain::ALL
                        .into_iter()
                        .map(|domain| {
                            let rows = momentum_app::preferences::registry()
                                .iter()
                                .filter(|definition| definition.domain == domain)
                                .map(|definition| {
                                    let current = momentum_app::preferences::sanitize(
                                        definition.key,
                                        stored.get(definition.key).map(String::as_str),
                                    )
                                    .unwrap_or_else(|| {
                                        momentum_app::preferences::default_value(definition)
                                    });
                                    let current_raw = current.as_str();
                                    let is_text = matches!(
                                        definition.default,
                                        momentum_app::preferences::PreferenceDefault::Text(_)
                                    );
                                    let options: Vec<String> = if is_text {
                                        Vec::new()
                                    } else if definition.choices.is_empty() {
                                        vec!["true".to_string(), "false".to_string()]
                                    } else {
                                        definition
                                            .choices
                                            .iter()
                                            .map(|choice| (*choice).to_string())
                                            .collect()
                                    };
                                    view! {
                                        <div
                                            class="preference-row"
                                            data-testid="preference"
                                            data-preference-key=definition.key
                                            data-scope=definition.scope.as_str()
                                        >
                                            <span class="task-title">{definition.title}</span>
                                            <span class="row-badge">
                                                {definition.scope.as_str()}
                                            </span>
                                            <form
                                                method="post"
                                                action="/settings/preferences"
                                                class="inline-form"
                                            >
                                                <input
                                                    type="hidden"
                                                    name="key"
                                                    value=definition.key
                                                />
                                                {if is_text {
                                                    view! {
                                                        <input
                                                            type="text"
                                                            name="value"
                                                            value=current_raw
                                                            aria-label=definition.title
                                                        />
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {
                                                        <select
                                                            name="value"
                                                            aria-label=definition.title
                                                        >
                                                            {options
                                                                .into_iter()
                                                                .map(|option| {
                                                                    let selected = option == current_raw;
                                                                    let label = option.clone();
                                                                    view! {
                                                                        <option value=option selected=selected>
                                                                            {label}
                                                                        </option>
                                                                    }
                                                                })
                                                                .collect_view()}
                                                        </select>
                                                    }
                                                        .into_any()
                                                }}
                                                <button type="submit" class="row-action">
                                                    "Save"
                                                </button>
                                            </form>
                                        </div>
                                    }
                                })
                                .collect_view();
                            view! {
                                <div class="preference-domain" data-domain=domain.as_str()>
                                    <h3 class="section-heading">{domain.as_str()}</h3>
                                    {rows}
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </Suspense>
        </section>
    }
}

/// Read-only calendar projection for one inclusive local date range.
#[component]
fn CalendarSection(
    #[prop(into)] range_start: LocalDate,
    #[prop(into)] range_end: LocalDate,
) -> impl IntoView {
    let calendar = use_context::<Option<Arc<CalendarService>>>().flatten();
    let account = use_account();
    let projection = Resource::new_blocking(
        || (),
        move |_| {
            let calendar = calendar.clone();
            let account = account.clone();
            async move {
                let (Some(calendar), Some(account)) = (calendar, account) else {
                    return momentum_app::calendar::CalendarProjection::default();
                };
                calendar
                    .events(
                        account.account_id,
                        CalendarRange::new(range_start, range_end),
                    )
                    .await
            }
        },
    );
    view! {
        <section data-testid="calendar-events">
            <h2 class="section-heading">"Calendar"</h2>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let projection = projection.get().unwrap_or_default();
                    let degraded = projection.degraded.clone();
                    let rows = projection
                        .events
                        .into_iter()
                        .map(|event| {
                            view! {
                                <li
                                    class="task-row task-row-readonly"
                                    data-testid="calendar-row"
                                    data-external="true"
                                >
                                    <span class="task-title">{event.title}</span>
                                    <span class="row-badge">{event.calendar_name}</span>
                                </li>
                            }
                        })
                        .collect_view();
                    view! {
                        {degraded
                            .map(|reason| {
                                view! {
                                    <p
                                        class="calendar-degraded"
                                        data-testid="calendar-degraded"
                                    >
                                        {reason}
                                    </p>
                                }
                            })}
                        <ul class="task-list">{rows}</ul>
                    }
                }}
            </Suspense>
        </section>
    }
}

/// Compact companion surface: Today, progress, and quick add for small screens.
#[component]
pub fn CompanionPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <CompanionView/> }}
        </Protected>
    }
}

#[component]
fn CompanionView() -> impl IntoView {
    let service = use_context::<Option<Arc<TodoService>>>().flatten();
    let account = use_account();
    let account_for_progress = account.clone();
    let service_for_progress = service.clone();
    let today =
        SharedValue::new(|| service.as_ref().map(|service| service.local_today())).into_inner();
    let todos = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
    let progress = Resource::new_blocking(
        || (),
        move |_| {
            let service = service_for_progress.clone();
            let account = account_for_progress.clone();
            async move {
                match (service, account) {
                    (Some(service), Some(account)) => {
                        service.today_progress(account.account_id).await.ok()
                    }
                    _ => None,
                }
            }
        },
    );
    view! {
        <section class="companion" data-testid="companion">
            <header class="companion-header">
                <h1 class="list-title">"Today"</h1>
                <a rel="external" href="/today" class="row-action">"Open full app"</a>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    progress
                        .get()
                        .flatten()
                        .map(|progress| {
                            view! {
                                <p
                                    class="companion-progress"
                                    data-testid="companion-progress"
                                    data-completed=progress.completed.to_string()
                                    data-total=progress.total.to_string()
                                    role="progressbar"
                                    aria-valuemin="0"
                                    aria-valuemax=progress.total.to_string()
                                    aria-valuenow=progress.completed.to_string()
                                    aria-label="Today progress"
                                >
                                    {format!(
                                        "{} of {} done",
                                        progress.completed,
                                        progress.total,
                                    )}
                                </p>
                            }
                        })
                }}
            </Suspense>
            <form
                method="post"
                action="/todos"
                class="companion-add"
                data-testid="companion-add"
            >
                <input type="hidden" name="return_to" value="/companion"/>
                <input
                    type="text"
                    name="title"
                    placeholder="Quick add"
                    aria-label="Quick add"
                    autocomplete="off"
                    required=true
                />
                <button type="submit" class="button-primary">"Add"</button>
            </form>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let items = todos.get().unwrap_or_default();
                    if items.is_empty() {
                        return view! {
                            <p class="list-empty" data-testid="companion-empty">
                                "Nothing scheduled for today."
                            </p>
                        }
                            .into_any();
                    }
                    view! {
                        <ul class="companion-list" data-testid="companion-today-list">
                            {items
                                .into_iter()
                                .map(|todo| {
                                    let complete = format!("/todos/{}/complete", todo.id);
                                    view! {
                                        <li
                                            class="companion-row"
                                            data-testid="companion-row"
                                            data-todo-id=todo.id.to_string()
                                        >
                                            <span class="task-title">{todo.title}</span>
                                            <form method="post" action=complete>
                                                <input
                                                    type="hidden"
                                                    name="return_to"
                                                    value="/companion"
                                                />
                                                <button
                                                    type="submit"
                                                    class="row-action"
                                                    aria-label="Complete to-do"
                                                >
                                                    "Done"
                                                </button>
                                            </form>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                        .into_any()
                }}
            </Suspense>
            {today
                .map(|today| view! { <CalendarSection range_start=today range_end=today/> })}
        </section>
    }
}

/// Quick Find search page.
#[component]
pub fn SearchPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <QuickFind/> }}
        </Protected>
    }
}

pub(crate) fn percent_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(char::from(*byte));
            }
            other => {
                use std::fmt::Write;
                let _ = write!(encoded, "%{other:02X}");
            }
        }
    }
    encoded
}

#[component]
fn QuickFind() -> impl IntoView {
    let todos = use_context::<Option<Arc<TodoService>>>().flatten();
    let projects = use_context::<Option<Arc<ProjectService>>>().flatten();
    let areas = use_context::<Option<Arc<AreaService>>>().flatten();
    let tags = use_context::<Option<Arc<TagService>>>().flatten();
    let recents = use_context::<Option<Arc<dyn RecentsStore>>>().flatten();
    let account = use_account();
    let query = use_query_map();
    let results = Resource::new_blocking(
        || (),
        move |_| {
            let todos = todos.clone();
            let projects = projects.clone();
            let areas = areas.clone();
            let tags = tags.clone();
            let recents = recents.clone();
            let account = account.clone();
            let needle = query.read().get("q").unwrap_or_default();
            async move {
                match (todos, projects, account) {
                    (Some(todos), Some(projects), Some(account)) => {
                        let service = SearchService::new(todos, projects, areas, tags, recents);
                        service
                            .search(account.account_id, &needle)
                            .await
                            .unwrap_or_default()
                    }
                    _ => Vec::new(),
                }
            }
        },
    );
    view! {
        <section class="list-page" data-testid="quick-find">
            <header class="list-header">
                <h1 class="list-title">"Quick Find"</h1>
                <p class="list-description">
                    "Search lists, projects, areas, tags, headings, and to-dos."
                </p>
            </header>
            <form method="get" action="/search" class="add-form" data-testid="quick-find-form">
                <input
                    id="quick-find-query"
                    type="text"
                    name="q"
                    placeholder="Search"
                    aria-label="Search"
                    autocomplete="off"
                    autofocus=true
                    on:keydown=move |event| {
                        if event.key() == "Escape" {
                            #[cfg(feature = "hydrate")]
                            {
                                if let Ok(history) = leptos::prelude::window().history() {
                                    let _ = history.back();
                                }
                            }
                            #[cfg(not(feature = "hydrate"))]
                            let _ = &event;
                        }
                    }
                />
                <button type="submit" class="button-primary">"Search"</button>
            </form>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    let hits = results.get().unwrap_or_default();
                    if hits.is_empty() {
                        return view! {
                            <div class="list-empty" data-testid="quick-find-empty">
                                <p>"No matches."</p>
                            </div>
                        }
                            .into_any();
                    }
                    view! {
                        <ul class="task-list" data-testid="quick-find-results">
                            {hits
                                .into_iter()
                                .map(|scored| {
                                    let SearchHit {
                                        title,
                                        url,
                                        kind,
                                        matched_in,
                                    } = scored.hit;
                                    let href = format!(
                                        "/search/go?url={}",
                                        percent_encode(&url),
                                    );
                                    let field_label = match matched_in {
                                        SearchField::Title => None,
                                        SearchField::Checklist => Some("Checklist"),
                                        SearchField::Notes => Some("Notes"),
                                    };
                                    view! {
                                        <li
                                            class="search-hit"
                                            data-testid="search-hit"
                                            data-kind=kind.as_str()
                                            data-score=scored.score.to_string()
                                            data-matched-in=matched_in.as_str()
                                        >
                                            <a rel="external" href=href class="task-title">{title}</a>
                                            {scored
                                                .recent
                                                .then(|| {
                                                    view! {
                                                        <span
                                                            class="row-badge"
                                                            data-testid="recent-hit"
                                                        >
                                                            "Recent"
                                                        </span>
                                                    }
                                                })}
                                            {field_label
                                                .map(|label| {
                                                    view! {
                                                        <span
                                                            class="row-badge"
                                                            data-testid="field-hit"
                                                        >
                                                            {label}
                                                        </span>
                                                    }
                                                })}
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    }
                        .into_any()
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
    let calendar = use_context::<Option<Arc<CalendarService>>>().flatten();
    let account = use_account();
    let today =
        SharedValue::new(|| service.as_ref().map(|service| service.local_today())).into_inner();
    let agenda = Resource::new_blocking(
        || (),
        move |_| {
            let service = service.clone();
            let calendar = calendar.clone();
            let account = account.clone();
            async move {
                let (Some(service), Some(account)) = (service, account) else {
                    return momentum_app::todos::UpcomingAgenda::default();
                };
                let agenda = service
                    .upcoming_agenda(account.account_id)
                    .await
                    .unwrap_or_default();
                let Some(calendar) = calendar else {
                    return agenda;
                };
                let today = service.local_today();
                let mut end = today;
                for _ in 0..60 {
                    end = end.next_day();
                }
                let projection = calendar
                    .events(account.account_id, CalendarRange::new(today, end))
                    .await;
                let rows = projection
                    .events
                    .iter()
                    .map(momentum_app::todos::CalendarRow::from_event)
                    .collect();
                agenda.with_calendar_rows(rows, projection.degraded)
            }
        },
    );

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
                                                data-date=iso.clone()
                                                data-drop-date=iso
                                            >
                                                <h2 class="section-heading">{label}</h2>
                                                <ul
                                                    class="task-list"
                                                    data-testid="upcoming-day-list"
                                                >
                                                    {day
                                                        .todos
                                                        .into_iter()
                                                        .enumerate()
                                                        .map(|(index, todo)| {
                                                            view! {
                                                                <TodoRow
                                                                    todo=todo
                                                                    today=today
                                                                    show_date_move=true
                                                                    drag_index=Some(index)
                                                                />
                                                            }
                                                        })
                                                        .collect_view()}
                                                </ul>
                                            </section>
                                        }
                                    })
                                    .collect_view()}
                                {agenda
                                    .calendar_degraded
                                    .clone()
                                    .map(|reason| {
                                        view! {
                                            <p
                                                class="calendar-degraded"
                                                data-testid="calendar-degraded"
                                            >
                                                {reason}
                                            </p>
                                        }
                                    })}
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
                                                                    data-external="true"
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
    let today =
        SharedValue::new(|| service.as_ref().map(|service| service.local_today())).into_inner();
    let agenda = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
    let labels = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

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
    let today = SharedValue::new(|| todos_service.as_ref().map(|service| service.local_today()))
        .into_inner();
    let todos = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
    let projects = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

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
                <a rel="external" class="task-title" href=open_target>{title}</a>
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
                <a rel="external" href=format!("/projects/{id}") class="row-action" aria-label="Open project">"Edit"</a>
            </div>
        </li>
    }
}
/// Settings page with the keyboard shortcut reference.
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <SettingsView/> }}
        </Protected>
    }
}

#[component]
fn SettingsView() -> impl IntoView {
    view! {
        <section class="list-page" data-testid="settings-page">
            <header class="list-header">
                <h1 class="list-title">"Settings"</h1>
                <p class="list-description">"Preferences and account settings."</p>
            </header>
            {move || view! { <PreferencesSection/> }}
            <section class="api-tokens" data-testid="api-tokens">
                <h2 class="section-heading">"Automation tokens"</h2>
                <p class="list-description">
                    "Tokens authenticate the /api/v1 REST surface; the plaintext is shown once."
                </p>
                {move || {
                    let service = use_context::<Option<Arc<momentum_app::api_tokens::ApiTokenService>>>()
                        .flatten();
                    let account = use_account();
                    let query = use_query_map();
                    let created = query.read().get("token").unwrap_or_default();
                    view! {
                        {(!created.is_empty())
                            .then(|| {
                                view! {
                                    <p data-testid="api-token-view-once">
                                        <strong>"Copy this token now: "</strong>
                                        <code>{created}</code>
                                    </p>
                                }
                            })}
                        {match (service, account) {
                            (Some(service), Some(account)) => {
                                let tokens = Resource::new_blocking(|| (), move |_| {
                                    let service = service.clone();
                                    async move {
                                        service.list(account.account_id).await.unwrap_or_default()
                                    }
                                });
                                view! {
                                    <Suspense fallback=|| view! { <Loading/> }>
                                        {move || {
                                            tokens
                                                .get()
                                                .unwrap_or_default()
                                                .into_iter()
                                                .map(|token| {
                                                    let revoke = format!(
                                                        "/settings/tokens/{}/revoke",
                                                        token.id,
                                                    );
                                                    let rotate = format!(
                                                        "/settings/tokens/{}/rotate",
                                                        token.id,
                                                    );
                                                    let scopes = token
                                                        .scopes
                                                        .iter()
                                                        .map(|scope| scope.as_str())
                                                        .collect::<Vec<_>>()
                                                        .join(", ");
                                                    let revoked = token.revoked_at.is_some();
                                                    view! {
                                                        <div
                                                            class="api-token-row"
                                                            data-testid="api-token"
                                                            data-token-id=token.id.to_string()
                                                        >
                                                            <span class="task-title">{token.name}</span>
                                                            <span class="row-badge">{scopes}</span>
                                                            {(revoked)
                                                                .then(|| {
                                                                    view! {
                                                                        <span class="row-badge">"Revoked"</span>
                                                                    }
                                                                })}
                                                            <form method="post" action=revoke>
                                                                <button type="submit" class="row-action">
                                                                    "Revoke"
                                                                </button>
                                                            </form>
                                                            <form method="post" action=rotate>
                                                                <button type="submit" class="row-action">
                                                                    "Rotate"
                                                                </button>
                                                            </form>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()
                                        }}
                                    </Suspense>
                                }
                                    .into_any()
                            }
                            _ => ().into_any(),
                        }}
                        <form method="post" action="/settings/tokens" data-testid="api-token-form">
                            <input
                                type="text"
                                name="name"
                                placeholder="Token name"
                                aria-label="Token name"
                                required=true
                            />
                            <select name="scopes" aria-label="Token scopes">
                                <option value="read">"Read only"</option>
                                <option value="read,write">"Read and write"</option>
                            </select>
                            <button type="submit" class="button-primary">"Create token"</button>
                        </form>
                    }
                }}
            </section>
            <section class="inbound-mail" data-testid="inbound-mail">
                <h2 class="section-heading">"Email to Inbox"</h2>
                <p class="list-description">
                    "Forward mail to your private address; subject becomes the title."
                </p>
                {move || {
                    let inbound = use_context::<Option<Arc<momentum_app::inbound::InboundService>>>()
                        .flatten();
                    let account = use_account();
                    view! {
                        {match (inbound, account) {
                            (Some(service), Some(account)) => {
                                let alias = Resource::new_blocking(|| (), move |_| {
                                    let service = service.clone();
                                    async move {
                                        service.ensure_alias(account.account_id).await.ok()
                                    }
                                });
                                view! {
                                    <Suspense fallback=|| view! { <Loading/> }>
                                        {move || {
                                            alias
                                                .get()
                                                .flatten()
                                                .map(|alias| {
                                                    view! {
                                                        <p data-testid="inbound-alias">
                                                            <code>{alias.address()}</code>
                                                            {(!alias.enabled)
                                                                .then(|| {
                                                                    view! {
                                                                        <span class="row-badge">
                                                                            "Disabled"
                                                                        </span>
                                                                    }
                                                                })}
                                                        </p>
                                                    }
                                                })
                                        }}
                                    </Suspense>
                                    <form method="post" action="/settings/inbound/rotate">
                                        <button type="submit" class="row-action">
                                            "Rotate address"
                                        </button>
                                    </form>
                                    <form method="post" action="/settings/inbound/disable">
                                        <button type="submit" class="row-action">
                                            "Disable address"
                                        </button>
                                    </form>
                                }
                                    .into_any()
                            }
                            _ => ().into_any(),
                        }}
                    }
                }}
            </section>
            <section class="keyboard-shortcuts" data-testid="shortcut-registry">
                <h2 class="section-heading">"Keyboard shortcuts"</h2>
                <p class="list-description">
                    "Bindings marked reserved fall back to a safe web equivalent."
                </p>
                {momentum_app::keyboard::registry()
                    .iter()
                    .map(|command| {
                        view! {
                            <div
                                class="shortcut-row"
                                data-testid="shortcut"
                                data-shortcut-id=command.id
                                data-keys=command.keys
                                data-category=command.category.as_str()
                            >
                                <span class="shortcut-keys">
                                    <kbd>{command.keys}</kbd>
                                </span>
                                <span class="shortcut-title">{command.title}</span>
                                {command
                                    .reserved_by
                                    .map(|reserved| {
                                        view! {
                                            <span
                                                class="row-badge"
                                                data-testid="shortcut-conflict"
                                            >
                                                {format!(
                                                    "Reserved by {reserved}; use {}",
                                                    command.alternative.unwrap_or(""),
                                                )}
                                            </span>
                                        }
                                    })}
                            </div>
                        }
                    })
                    .collect_view()}
            </section>
        </section>
    }
}

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
    let project_service = use_context::<Option<Arc<ProjectService>>>().flatten();
    let account = use_account();
    let account_for_tags = account.clone();
    let account_for_projects = account.clone();
    let today =
        SharedValue::new(|| service.as_ref().map(|service| service.local_today())).into_inner();
    let query = use_query_map();
    let todos = Resource::new_blocking(
        move || query.read().get("tags").unwrap_or_default(),
        move |tags| {
            let service = service.clone();
            let account = account.clone();
            async move {
                let selected: Vec<EntityId> = tags
                    .split(',')
                    .filter_map(|value| value.parse::<EntityId>().ok())
                    .collect();
                let todos = match (service, account) {
                    (Some(service), Some(account)) => service
                        .list(account.account_id, TodoScope::Inbox)
                        .await
                        .unwrap_or_default(),
                    _ => Vec::new(),
                };
                if selected.is_empty() {
                    todos
                } else {
                    todos
                        .into_iter()
                        .filter(|todo| {
                            selected.iter().any(|tag| {
                                todo.tags.contains(tag) || todo.inherited_tags.contains(tag)
                            })
                        })
                        .collect()
                }
            }
        },
    );
    let tags = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
    let projects = Resource::new_blocking(
        || (),
        move |_| {
            let service = project_service.clone();
            let account = account_for_projects.clone();
            async move {
                match (service, account) {
                    (Some(service), Some(account)) => {
                        service.list(account.account_id).await.unwrap_or_default()
                    }
                    _ => Vec::new(),
                }
            }
        },
    );
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
                                    <a rel="external"
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
                                        <a rel="external" href="/inbox" class="tag-chip tag-chip-clear">"Clear"</a>
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
                                let tag_choices: Vec<(EntityId, String)> = tags
                                    .get()
                                    .unwrap_or_default()
                                    .into_iter()
                                    .map(|tag| (tag.id, tag.title))
                                    .collect();
                                let rows = todos
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, todo)| {
                                        let position = format!("/todos/{}/position", todo.id);
                                        let magic_plus = MagicPlusTarget {
                                            position: index,
                                            position_scope: None,
                                            context_kind: None,
                                            context_id: None,
                                            return_to: "/inbox".to_string(),
                                            schedule_kind: None,
                                            label: todo.title.clone(),
                                        };
                                        let expand = RowEditorContext {
                                            return_to: "/inbox".to_string(),
                                            tags: tag_choices.clone(),
                                        };
                                        view! {
                                            <TodoRow
                                                todo=todo
                                                today=today
                                                drag_index=Some(index)
                                                position_action=Some(position)
                                                position_return_to="/inbox"
                                                selectable=true
                                                magic_plus=Some(magic_plus)
                                                expand=Some(expand)
                                            />
                                        }
                                    })
                                    .collect_view();
                                let tag_options = tags
                                    .get()
                                    .unwrap_or_default()
                                    .into_iter()
                                    .map(|tag| {
                                        view! {
                                            <option value=tag.id.to_string()>{tag.title}</option>
                                        }
                                    })
                                    .collect_view();
                                let project_options = projects
                                    .get()
                                    .unwrap_or_default()
                                    .into_iter()
                                    .map(|project| {
                                        let value = format!("project:{}", project.id);
                                        view! { <option value=value>{project.title}</option> }
                                    })
                                    .collect_view();
                                view! {
                                    <div
                                        class="bulk-form"
                                        data-testid="bulk-form"
                                    >
                                        <ul class="task-list" data-testid="task-list">
                                            {rows}
                                        </ul>
                                        <form id="bulk-form" method="post" action="/todos/bulk"
                                            class="bulk-actions" data-testid="bulk-actions">
                                            <input type="hidden" name="return_to" value="/inbox"/>
                                            <select name="action" aria-label="Bulk action">
                                                <option value="complete">"Complete"</option>
                                                <option value="cancel">"Cancel"</option>
                                                <option value="delete">"Delete"</option>
                                                <option value="schedule_today">"Schedule today"</option>
                                                <option value="schedule_evening">
                                                    "Schedule this evening"
                                                </option>
                                                <option value="schedule_tomorrow">
                                                    "Schedule tomorrow"
                                                </option>
                                                <option value="schedule_someday">"Someday"</option>
                                                <option value="schedule_anytime">"Anytime"</option>
                                                <option value="deadline_on">"Deadline on"</option>
                                                <option value="deadline_none">"Clear deadline"</option>
                                                <option value="move">"Move to"</option>
                                                <option value="tag">"Add tag"</option>
                                            </select>
                                            <input
                                                type="date"
                                                name="date"
                                                aria-label="Bulk date"
                                                class="bulk-date"
                                            />
                                            <select name="destination" aria-label="Bulk destination">
                                                <option value="">"Destination"</option>
                                                <option value="inbox">"Inbox"</option>
                                                {project_options}
                                            </select>
                                            <select name="tag" aria-label="Bulk tag">
                                                <option value="">"Tag"</option>
                                                {tag_options}
                                            </select>
                                            <button type="submit" class="button-primary">
                                                "Apply to selected"
                                            </button>
                                        </form>
                                    </div>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>
        </section>
    }
}

/// Inline expanded editor configuration for a task row.
#[derive(Clone, PartialEq)]
pub struct RowEditorContext {
    /// Page to return to after editor commits or cancels.
    pub return_to: String,
    /// Assignable tags as (identity, title) pairs shown as progressive metadata.
    pub tags: Vec<(EntityId, String)>,
}

/// Where a Magic Plus insertion control creates its item.
#[derive(Clone, PartialEq)]
pub struct MagicPlusTarget {
    /// Zero-based insertion position among the visible rows.
    pub position: usize,
    /// Which ordering the position applies to (`today` for projection buckets).
    pub position_scope: Option<String>,
    /// Context kind for alternate creation targets (project/heading/area).
    pub context_kind: Option<String>,
    /// Context identity for alternate creation targets.
    pub context_id: Option<EntityId>,
    /// Page to return to after creation.
    pub return_to: String,
    /// Quick schedule applied on creation (`today`/`evening`/...).
    pub schedule_kind: Option<String>,
    /// Accessible label naming the following row.
    pub label: String,
}

#[component]
fn TodoRow(
    todo: TodoSummary,
    #[prop(default = None)] today: Option<LocalDate>,
    #[prop(default = false)] today_order: bool,
    #[prop(default = false)] show_date_move: bool,
    #[prop(default = None)] position_action: Option<String>,
    #[prop(default = None)] drag_index: Option<usize>,
    #[prop(into, default = String::new())] position_return_to: String,
    #[prop(default = false)] selectable: bool,
    #[prop(default = None)] magic_plus: Option<MagicPlusTarget>,
    #[prop(default = None)] expand: Option<RowEditorContext>,
) -> impl IntoView {
    let id = todo.id;
    let selected_tags = todo.tags.clone();
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
    let snoozable = todo.reminder.as_ref().is_some_and(|reminder| {
        matches!(
            reminder.state,
            momentum_domain::model::ReminderState::Delivered
                | momentum_domain::model::ReminderState::Snoozed
        )
    });
    let reminder = todo
        .reminder
        .map(|reminder| format!("Reminder: {}", reminder.at.format("%Y-%m-%d %H:%M UTC")));
    let editor_title = title.clone();

    view! {
        <li
            class=format!("{row_class} motion-insert motion-drag")
            data-testid="task-row"
            data-todo-id=id.to_string()
            draggable=drag_index.map(|_| "true")
            data-drag-index=drag_index.map(|index| index.to_string())
            data-drop-position=drag_index.map(|index| (index + 1).to_string())
        >
            {magic_plus
                .map(|target| {
                    view! {
                        <details class="insert-here" data-testid="insert-here">
                            <summary>{format!("Insert above {}", target.label)}</summary>
                            <form method="post" action="/todos" class="insert-form">
                                <input
                                    type="text"
                                    name="title"
                                    aria-label="Insert to-do title"
                                    autocomplete="off"
                                    required=true
                                />
                                <input
                                    type="hidden"
                                    name="return_to"
                                    value=target.return_to.clone()
                                />
                                <input
                                    type="hidden"
                                    name="position"
                                    value=(target.position + 1).to_string()
                                />
                                {target
                                    .position_scope
                                    .clone()
                                    .map(|scope| {
                                        view! {
                                            <input type="hidden" name="position_scope" value=scope/>
                                        }
                                    })}
                                {target
                                    .schedule_kind
                                    .clone()
                                    .map(|kind| {
                                        view! {
                                            <input type="hidden" name="schedule_kind" value=kind/>
                                        }
                                    })}
                                {target
                                    .context_kind
                                    .clone()
                                    .map(|kind| {
                                        view! {
                                            <input type="hidden" name="context_kind" value=kind/>
                                        }
                                    })}
                                {target
                                    .context_id
                                    .map(|context| {
                                        view! {
                                            <input
                                                type="hidden"
                                                name="context_id"
                                                value=context.to_string()
                                            />
                                        }
                                    })}
                                <button type="submit" class="button">"Add here"</button>
                            </form>
                        </details>
                    }
                    .into_any()
                })}
            {selectable
                .then(|| {
                    view! {
                        <input
                            type="checkbox"
                            name="ids"
                            form="bulk-form"
                            value=id.to_string()
                            class="bulk-select"
                            aria-label=format!("Select {title}")
                            data-testid="bulk-select"
                        />
                    }
                })}
            <CheckControl checked=completed label=title.clone()/>
            <div class="task-row-body">
                <a rel="external" class="task-title" href=open_target.clone()>
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
                    {reminder
                        .map(|label| {
                            view! {
                                <span class="row-badge date-badge" data-testid="reminder-badge">
                                    {label}
                                </span>
                            }
                        })}
                    {snoozable
                        .then(|| {
                            let snooze_target = format!("/todos/{id}/snooze");
                            let choices = [
                                ("ten_minutes", "10m", "Snooze 10 minutes"),
                                ("one_hour", "1h", "Snooze 1 hour"),
                                ("tomorrow", "Tomorrow", "Snooze until tomorrow"),
                            ];
                            view! {
                                <span class="row-snooze" data-testid="row-snooze">
                                    {choices
                                        .into_iter()
                                        .map(|(option, label, aria)| {
                                            view! {
                                                <form
                                                    method="post"
                                                    action=snooze_target.clone()
                                                    class="inline-form"
                                                >
                                                    <input type="hidden" name="option" value=option/>
                                                    <button
                                                        type="submit"
                                                        class="row-action"
                                                        aria-label=aria
                                                        data-testid=format!("snooze-{option}")
                                                    >
                                                        {label}
                                                    </button>
                                                </form>
                                            }
                                        })
                                        .collect_view()}
                                </span>
                            }
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
                    <button
                        type="submit"
                        class="row-action"
                        aria-label="Complete to-do"
                        aria-keyshortcuts=momentum_app::keyboard::keys_for("complete_todo")
                            .unwrap_or_default()
                    >
                        "Complete"
                    </button>
                </form>
                <form method="post" action=delete_target>
                    <button
                        type="submit"
                        class="row-action"
                        aria-label="Delete to-do"
                        aria-keyshortcuts=momentum_app::keyboard::keys_for("delete_todo")
                            .unwrap_or_default()
                    >
                        "Delete"
                    </button>
                </form>
                {position_action
                    .map(|action| {
                        let return_to = position_return_to.clone();
                        view! {
                            <form method="post" action=action class="row-schedule">
                                <input type="hidden" name="return_to" value=return_to/>
                                <input
                                    type="number"
                                    name="position"
                                    min="1"
                                    value="1"
                                    aria-label="Drop position"
                                    class="position-input"
                                />
                                <button
                                    type="submit"
                                    class="row-action"
                                    aria-label="Move to position"
                                >
                                    "Move here"
                                </button>
                            </form>
                        }
                    })}
                <a rel="external" class="row-action" href=open_target.clone() aria-label="Open to-do">"Edit"</a>
            </div>
            {expand
                .map(|editor| {
                    let title_value = editor_title.clone();
                    let notes_value = todo.notes.clone();
                    let save_target = format!("/todos/{id}");
                    let deadline_target = format!("/todos/{id}/deadline");
                    let schedule_target = format!("/todos/{id}/schedule");
                    let detail_target = format!("/todos/{id}");
                    let cancel_target = editor.return_to.clone();
                    let quick = [
                        ("today", "Today"),
                        ("evening", "This Evening"),
                        ("tomorrow", "Tomorrow"),
                        ("someday", "Someday"),
                        ("anytime", "Anytime"),
                    ];
                    let quick_forms = quick
                        .into_iter()
                        .map(|(when, label)| {
                            let return_to = editor.return_to.clone();
                            let target = schedule_target.clone();
                            view! {
                                <form method="post" action=target class="inline-form">
                                    <input type="hidden" name="when" value=when/>
                                    <input type="hidden" name="return_to" value=return_to/>
                                    <button type="submit" class="row-action">{label}</button>
                                </form>
                            }
                        })
                        .collect_view();
                    let tag_forms = editor
                        .tags
                        .into_iter()
                        .map(|(tag_id, tag_title)| {
                            let assigned = selected_tags.contains(&tag_id);
                            let action = if assigned {
                                format!("/entities/{id}/tags/{tag_id}/unassign")
                            } else {
                                format!("/entities/{id}/tags/{tag_id}/assign")
                            };
                            let label = if assigned {
                                format!("Remove {tag_title}")
                            } else {
                                format!("Tag {tag_title}")
                            };
                            let return_to = editor.return_to.clone();
                            view! {
                                <form method="post" action=action class="inline-form">
                                    <input type="hidden" name="return_to" value=return_to/>
                                    <button type="submit" class="row-action">{label}</button>
                                </form>
                            }
                        })
                        .collect_view();
                    view! {
                        <details class="row-editor motion-editor" data-testid="row-editor">
                            <summary aria-label=format!("Edit {title_value}")>"Edit"</summary>
                            <div class="editor-body">
                                <form
                                    method="post"
                                    action=save_target
                                    class="editor-form"
                                    data-testid="editor-form"
                                >
                                    <input
                                        type="hidden"
                                        name="return_to"
                                        value=editor.return_to.clone()
                                    />
                                    <input
                                        type="text"
                                        name="title"
                                        value=title_value
                                        aria-label="Title"
                                        autocomplete="off"
                                        required=true
                                    />
                                    <textarea name="notes" aria-label="Notes" rows="3">
                                        {notes_value}
                                    </textarea>
                                    <button type="submit" class="button-primary">"Save"</button>
                                    <a rel="external" href=cancel_target class="row-action">"Cancel"</a>
                                </form>
                                <div class="editor-meta" data-testid="editor-meta">
                                    <span class="editor-meta-label">"Schedule"</span>
                                    {quick_forms}
                                    <form
                                        method="post"
                                        action=deadline_target.clone()
                                        class="inline-form"
                                    >
                                        <input
                                            type="hidden"
                                            name="return_to"
                                            value=editor.return_to.clone()
                                        />
                                        <input
                                            type="date"
                                            name="date"
                                            aria-label="Deadline"
                                            data-testid="editor-deadline"
                                        />
                                        <button type="submit" class="row-action">
                                            "Set deadline"
                                        </button>
                                    </form>
                                    <span class="editor-meta-label">"When"</span>
                                    <form method="post" action=schedule_target.clone() class="inline-form">
                                        <input
                                            type="hidden"
                                            name="return_to"
                                            value=editor.return_to.clone()
                                        />
                                        <input type="hidden" name="when" value="date"/>
                                        <input
                                            type="date"
                                            name="date"
                                            aria-label="Schedule date"
                                            data-testid="editor-schedule-date"
                                        />
                                        <button type="submit" class="row-action">"When date"</button>
                                    </form>
                                    <form method="post" action=schedule_target.clone() class="inline-form">
                                        <input
                                            type="hidden"
                                            name="return_to"
                                            value=editor.return_to.clone()
                                        />
                                        <input type="hidden" name="when" value="natural"/>
                                        <input
                                            type="text"
                                            name="text"
                                            aria-label="Natural date"
                                            placeholder="tomorrow, friday, jun 20"
                                            data-testid="editor-natural-date"
                                        />
                                        <button type="submit" class="row-action">
                                            "Parse date"
                                        </button>
                                    </form>
                                    <span class="editor-meta-label">"Deadline"</span>
                                    <form method="post" action=deadline_target.clone() class="inline-form">
                                        <input
                                            type="hidden"
                                            name="return_to"
                                            value=editor.return_to.clone()
                                        />
                                        <input type="hidden" name="date" value=""/>
                                        <button type="submit" class="row-action">
                                            "Clear deadline"
                                        </button>
                                    </form>
                                    <span class="editor-meta-label">"Reminder"</span>
                                    <form
                                        method="post"
                                        action=format!("/todos/{id}/reminder")
                                        class="inline-form"
                                    >
                                        <input
                                            type="hidden"
                                            name="return_to"
                                            value=editor.return_to.clone()
                                        />
                                        <input
                                            type="datetime-local"
                                            name="at"
                                            aria-label="Reminder time"
                                            data-testid="editor-reminder"
                                        />
                                        <input
                                            type="text"
                                            name="timezone"
                                            value="UTC"
                                            aria-label="Reminder timezone"
                                            class="timezone-input"
                                        />
                                        <button type="submit" class="row-action">
                                            "Set reminder"
                                        </button>
                                    </form>
                                    <form
                                        method="post"
                                        action=format!("/todos/{id}/reminder")
                                        class="inline-form"
                                    >
                                        <input
                                            type="hidden"
                                            name="return_to"
                                            value=editor.return_to.clone()
                                        />
                                        <input type="hidden" name="at" value=""/>
                                        <button type="submit" class="row-action">
                                            "Clear reminder"
                                        </button>
                                    </form>
                                    <span class="editor-meta-label">"Tags"</span>
                                    {tag_forms}
                                    <a rel="external" href=detail_target class="row-action">
                                        "Checklist & recurrence"
                                    </a>
                                </div>
                            </div>
                        </details>
                    }
                    .into_any()
                })}
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

    let todo = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

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
                                            <a rel="external" href="/inbox">"Back to Inbox"</a>
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
    let all_tags = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

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

/// Tomorrow special view.
#[component]
pub fn TomorrowPage() -> impl IntoView {
    view! {
        <Protected>
            {move || {
                view! {
                    <ProjectionList
                        title="Tomorrow"
                        description="What is scheduled for the next day."
                        scope=TodoScope::Tomorrow
                        slug="tomorrow"
                    />
                }
            }}
        </Protected>
    }
}

/// Repeating special view: recurrence templates.
#[component]
pub fn RecurringPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <RecurringList/> }}
        </Protected>
    }
}

#[component]
fn RecurringList() -> impl IntoView {
    let service = use_context::<Option<Arc<RecurringService>>>().flatten();
    let account = use_account();
    let templates = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

    view! {
        <section class="list-page" data-testid="repeating-page">
            <header class="list-header">
                <h1 class="list-title">"Repeating"</h1>
                <p class="list-description">"Templates that generate future occurrences."</p>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    templates
                        .get()
                        .map(|templates| {
                            if templates.is_empty() {
                                view! {
                                    <div class="list-empty" data-testid="repeating-empty">
                                        <p>"No repeating items yet."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ul class="task-list" data-testid="repeating-list">
                                        {templates
                                            .into_iter()
                                            .map(|template| view! { <RecurrenceRow template=template/> })
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
fn RecurrenceRow(template: RecurrenceTemplateSummary) -> impl IntoView {
    let id = template.id;
    let title = template.title.clone();
    let next = template.next_occurrence.map_or_else(
        || "Not scheduled".to_string(),
        |date| format!("Next: {date}"),
    );
    view! {
        <li class="task-row" data-testid="repeating-row">
            <div class="task-row-body">
                <a rel="external" class="task-title" href=format!("/todos/{id}")>
                    {title}
                </a>
                <span class="task-row-meta">
                    <span class="row-badge">{next}</span>
                </span>
            </div>
        </li>
    }
}

/// Logged Projects special view.
#[component]
pub fn LoggedProjectsPage() -> impl IntoView {
    view! {
        <Protected>
            {move || view! { <LoggedProjectsList/> }}
        </Protected>
    }
}

#[component]
fn LoggedProjectsList() -> impl IntoView {
    let service = use_context::<Option<Arc<ProjectService>>>().flatten();
    let account = use_account();
    let projects = Resource::new_blocking(
        || (),
        move |_| {
            let service = service.clone();
            let account = account.clone();
            async move {
                match (service, account) {
                    (Some(service), Some(account)) => {
                        service.logged(account.account_id).await.unwrap_or_default()
                    }
                    _ => Vec::new(),
                }
            }
        },
    );

    view! {
        <section class="list-page" data-testid="logged-projects-page">
            <header class="list-header">
                <h1 class="list-title">"Logged Projects"</h1>
                <p class="list-description">"Completed and canceled projects."</p>
            </header>
            <Suspense fallback=|| view! { <Loading/> }>
                {move || {
                    projects
                        .get()
                        .map(|projects| {
                            if projects.is_empty() {
                                view! {
                                    <div
                                        class="list-empty"
                                        data-testid="logged-projects-empty"
                                    >
                                        <p>"No logged projects yet."</p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ul class="task-list" data-testid="logged-projects-list">
                                        {projects
                                            .into_iter()
                                            .map(|project| {
                                                let id = project.id;
                                                let title = project.title.clone();
                                                let status = project.status.as_str().to_string();
                                                view! {
                                                    <li
                                                        class="task-row"
                                                        data-testid="logged-project-row"
                                                    >
                                                        <div class="task-row-body">
                                                            <a rel="external"
                                                                class="task-title"
                                                                href=format!("/projects/{id}")
                                                            >
                                                                {title}
                                                            </a>
                                                            <span class="task-row-meta">
                                                                <span class="row-badge">
                                                                    {status}
                                                                </span>
                                                            </span>
                                                        </div>
                                                    </li>
                                                }
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
    let todos = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
    let projects = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
    let areas = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

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
    let agenda = Resource::new_blocking(
        || (),
        move |_| {
            let service = service.clone();
            let account = account.clone();
            async move {
                match (service, account) {
                    (Some(service), Some(account)) => service.agenda(account.account_id).await.ok(),
                    _ => None,
                }
            }
        },
    );

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
                        <a rel="external" class="task-title" href=open_target>
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
    let tags = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

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
    let areas = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

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
                <a rel="external" class="task-title" href=open_target>{title.clone()}</a>
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

    let area = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

    let loose_todos = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

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
                                                <a rel="external" href="/areas">"Back to Areas"</a>
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
    let projects = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

    view! {
        <section class="list-page" data-testid="projects-page">
            <header class="list-header">
                <h1 class="list-title">"Projects"</h1>
                <p class="list-description">"All open projects."</p>
                <p>
                    <a rel="external" href="/projects/logged" class="row-action">"Logged projects"</a>
                </p>
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
    let progress = project_progress(project.eligible_count(), project.completed_count);

    view! {
        <li class="task-row" data-testid="project-row">
            <div class="task-row-body">
                <a rel="external" class="task-title" href=open_target>{title}</a>
                <span class="task-row-meta">
                    <span class="row-badge">{counts}</span>
                    {progress}
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

fn project_progress(total: usize, completed: usize) -> AnyView {
    if total == 0 {
        return view! {
            <span class="row-badge progress-neutral" data-testid="project-progress-neutral">
                "No items"
            </span>
        }
        .into_any();
    }
    let ratio = Progress::new(completed, total);
    let percent = ratio.percent().unwrap_or(0);
    view! {
        <span class="row-badge progress-badge" data-testid="project-progress-value">
            <progress
                value=completed.to_string()
                max=total.to_string()
                aria-label=format!("{completed} of {total} completed")
            ></progress>
            <span>{format!("{percent}%")}</span>
        </span>
    }
    .into_any()
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
    let project_service_for_children = project_service.clone();
    let parsed = id.parse::<momentum_domain::EntityId>().ok();
    let today = SharedValue::new(|| {
        project_service
            .as_ref()
            .map(|service| service.local_today())
    })
    .into_inner();

    let project = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );

    let children = Resource::new_blocking(
        || (),
        move |_| {
            let service = todo_service.clone();
            let projects = project_service_for_children.clone();
            let account = account.clone();
            async move {
                match (service, account) {
                    (Some(service), Some(account)) => {
                        let Some(project_id) = parsed else {
                            return Vec::new();
                        };
                        let heading_ids: Vec<EntityId> = match projects {
                            Some(projects) => projects
                                .get(account.account_id, project_id)
                                .await
                                .ok()
                                .flatten()
                                .map(|project| {
                                    project
                                        .headings
                                        .into_iter()
                                        .map(|heading| heading.id)
                                        .collect()
                                })
                                .unwrap_or_default(),
                            None => Vec::new(),
                        };
                        service
                            .list(account.account_id, TodoScope::AllOpen)
                            .await
                            .unwrap_or_default()
                            .into_iter()
                            .filter(|todo| match todo.context {
                                momentum_domain::model::ContextRef::Project(id) => id == project_id,
                                momentum_domain::model::ContextRef::Heading(id) => {
                                    heading_ids.contains(&id)
                                }
                                _ => false,
                            })
                            .collect()
                    }
                    _ => Vec::new(),
                }
            }
        },
    );

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
                                                <a rel="external" href="/projects">"Back to Projects"</a>
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
                <form method="post" action=format!("/headings/{id}/move_project") class="row-schedule">
                    <input type="hidden" name="return_to" value=return_to.clone()/>
                    <input
                        type="text"
                        name="project"
                        placeholder="Project id"
                        aria-label="Destination project id"
                    />
                    <button type="submit" class="button-secondary">"Move heading"</button>
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
            <div class="project-progress" data-testid="project-progress">
                {project_progress(
                    project.eligible_count(),
                    project.completed_count,
                )}
            </div>
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
                                        <a rel="external"
                                            href=format!("?find={encoded}&from={prev_start}")
                                            class="row-action"
                                            aria-label="Previous match"
                                            data-testid="find-previous"
                                        >
                                            "Previous"
                                        </a>
                                        <a rel="external"
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
    let todos = Resource::new_blocking(
        || (),
        move |_| {
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
        },
    );
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
