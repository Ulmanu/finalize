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
use momentum_app::projects::{HeadingSummary, ProjectOrderItem, ProjectService, ProjectSummary};
use momentum_app::todos::{TodoScope, TodoService, TodoSummary};
use momentum_domain::model::Status;
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
    let up_target = format!("/todos/{id}/move-up");
    let down_target = format!("/todos/{id}/move-down");
    let schedule_target = format!("/todos/{id}/schedule");
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
                <form method="post" action=up_target>
                    <input type="hidden" name="return_to" value="/inbox"/>
                    <button type="submit" class="row-action" aria-label="Move up">"↑"</button>
                </form>
                <form method="post" action=down_target>
                    <input type="hidden" name="return_to" value="/inbox"/>
                    <button type="submit" class="row-action" aria-label="Move down">"↓"</button>
                </form>
                <form method="post" action=schedule_target class="row-schedule">
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
                                                .map(|todo| view! { <TodoRow todo=todo/> }.into_any())
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
                    {notes}
                </textarea>
            </label>
            <button type="submit" class="button-secondary">"Save notes"</button>
        </form>

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
