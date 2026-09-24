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
use momentum_app::todos::{TodoScope, TodoService, TodoSummary};
use momentum_domain::model::Status;

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
    let account = use_account();
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

    view! {
        <section class="list-page" data-testid="list-page">
            <header class="list-header">
                <h1 class="list-title">"Inbox"</h1>
                <p class="list-description">"Unprocessed captures land here."</p>
            </header>
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
                    todos
                        .get()
                        .map(|todos| {
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

#[component]
fn TodoRow(todo: TodoSummary) -> impl IntoView {
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
    let schedule = todo.schedule.date().map(|date| format!("When: {date}"));
    let deadline = todo
        .deadline
        .map(|deadline| format!("Due: {}", deadline.date));
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
                        .map(|label| {
                            view! {
                                <span class="row-badge date-badge date-badge-deadline">{label}</span>
                            }
                        })}
                </span>
            </div>
            <div class="task-row-actions">
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
