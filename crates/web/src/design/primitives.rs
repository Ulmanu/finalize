//! Reusable Things-style UI primitives.
//!
//! Primitives are presentation-only: interactive controls report intent through callbacks and
//! never mutate domain state or storage. Token values come from [`crate::design::tokens`]; the
//! stylesheet consumes the same custom properties.

use leptos::prelude::*;

use crate::design::icons::{
    ArchiveIcon, CalendarIcon, CheckIcon, ChevronIcon, ClockIcon, FlagIcon, FolderIcon, InboxIcon,
    LayersIcon, NoteIcon, PlusIcon, RepeatIcon, StarIcon, TagIcon,
};
use crate::design::tokens::MotionKind;

/// Visual style of a sidebar row icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarIcon {
    /// Inbox tray.
    #[default]
    Inbox,
    /// Today star.
    Today,
    /// Upcoming clock.
    Upcoming,
    /// Anytime layers.
    Anytime,
    /// Someday/Logbook archive box.
    Archive,
    /// Project/area folder.
    Folder,
    /// Tag.
    Tag,
}

/// Checkbox control with a visible custom indicator and native input semantics.
#[component]
pub fn CheckControl(
    /// Whether the control is checked.
    #[prop(into)]
    checked: Signal<bool>,
    /// Accessible name.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    /// Disables interaction.
    #[prop(optional)]
    disabled: bool,
    /// Uses compact metrics.
    #[prop(optional)]
    compact: bool,
    /// Reports activation intent.
    #[prop(optional)]
    on_toggle: Option<Callback<()>>,
) -> impl IntoView {
    let class = if compact {
        "check-control check-control-compact"
    } else {
        "check-control"
    };
    view! {
        <label class=class data-testid="check-control">
            <input
                type="checkbox"
                class="check-input"
                checked=move || checked.get()
                disabled=disabled
                aria-label=move || label.get()
                on:change=move |_| {
                    if let Some(callback) = on_toggle {
                        callback.run(());
                    }
                }
            />
            <span class="check-visual" aria-hidden="true">
                <CheckIcon/>
            </span>
        </label>
    }
}

/// Progress ring rendered as an accessible image with a percentage label.
#[component]
pub fn ProgressRing(
    /// Completion ratio in the `0.0..=1.0` range.
    progress: f32,
    /// Overrides the default `"NN% complete"` accessible name.
    #[prop(into, optional)]
    label: MaybeProp<String>,
) -> impl IntoView {
    const RADIUS: f32 = 6.5;
    const CIRCUMFERENCE: f32 = 2.0 * std::f32::consts::PI * RADIUS;

    let clamped = progress.clamp(0.0, 1.0);
    let percent = (clamped * 100.0).round();
    let dash = CIRCUMFERENCE * clamped;
    let accessible_name = label
        .get_untracked()
        .unwrap_or_else(|| format!("{percent:.0}% complete"));
    let dash_array = format!("{dash:.2} {CIRCUMFERENCE:.2}");

    view! {
        <svg
            class="progress-ring"
            viewBox="0 0 16 16"
            role="img"
            aria-label=accessible_name
            data-testid="progress-ring"
            data-progress=format!("{clamped:.2}")
        >
            <circle
                class="progress-track"
                cx="8"
                cy="8"
                r=RADIUS
                fill="none"
                stroke-width="2"
            ></circle>
            <circle
                class="progress-value"
                cx="8"
                cy="8"
                r=RADIUS
                fill="none"
                stroke-width="2"
                stroke-dasharray=dash_array
                transform="rotate(-90 8 8)"
            ></circle>
        </svg>
    }
}

/// Section heading used inside lists (for example "This Evening").
#[component]
pub fn SectionHeading(children: Children) -> impl IntoView {
    view! {
        <h2 class="section-heading" data-testid="section-heading">
            {children()}
        </h2>
    }
}

/// Note indicator shown on rows whose item has notes.
#[component]
fn NoteIndicator(#[prop(into)] preview: String) -> impl IntoView {
    view! {
        <span class="row-badge note-indicator" title=preview.clone() aria-label=preview>
            <NoteIcon/>
        </span>
    }
}

fn date_icon(deadline: bool) -> impl IntoView {
    if deadline {
        view! { <FlagIcon/> }.into_any()
    } else {
        view! { <CalendarIcon/> }.into_any()
    }
}

/// Date/deadline badge shown on rows.
#[component]
fn DateBadge(#[prop(into)] label: String, #[prop(optional)] deadline: bool) -> impl IntoView {
    let class = if deadline {
        "row-badge date-badge date-badge-deadline"
    } else {
        "row-badge date-badge"
    };
    view! {
        <span class=class>
            {date_icon(deadline)}
            <span class="date-badge-label">{label}</span>
        </span>
    }
}

/// Tag chip shown on rows.
#[component]
fn TagChip(#[prop(into)] label: String) -> impl IntoView {
    view! {
        <span class="tag-chip">
            <TagIcon/>
            <span>{label}</span>
        </span>
    }
}

/// Task row primitive used by every list projection.
#[component]
pub fn TaskRow(
    /// Task title.
    #[prop(into)]
    title: String,
    /// Notes preview.
    #[prop(into, optional)]
    note: MaybeProp<String>,
    /// Assigned tags.
    #[prop(optional)]
    tags: Vec<String>,
    /// Schedule badge label.
    #[prop(into, optional)]
    schedule: MaybeProp<String>,
    /// Deadline badge label.
    #[prop(into, optional)]
    deadline: MaybeProp<String>,
    /// Whether the task is completed.
    #[prop(optional)]
    completed: bool,
    /// Whether the row is selected.
    #[prop(optional)]
    selected: bool,
    /// Whether the task repeats.
    #[prop(optional)]
    has_recurrence: bool,
    /// Reports completion toggle intent.
    #[prop(optional)]
    on_toggle: Option<Callback<()>>,
) -> impl IntoView {
    let class = move || {
        let mut classes = String::from("task-row");
        if completed {
            classes.push_str(" task-row-completed");
        }
        if selected {
            classes.push_str(" task-row-selected");
        }
        classes
    };
    let checkbox_label = title.clone();
    let title_for_note = title.clone();
    let toggle = on_toggle.unwrap_or_else(|| Callback::new(|_| {}));

    view! {
        <li class=class data-testid="task-row" data-selected=selected.then_some("true")>
            <CheckControl
                checked=completed
                label=checkbox_label
                on_toggle=toggle
            />
            <div class="task-row-body">
                <span class="task-title">{title}</span>
                <span class="task-row-meta">
                    {move || {
                        note.get()
                            .map(|preview| {
                                let preview = if preview.is_empty() {
                                    title_for_note.clone()
                                } else {
                                    preview
                                };
                                view! { <NoteIndicator preview=preview/> }
                            })
                    }}
                    {move || {
                        schedule.get().map(|label| view! { <DateBadge label=label/> })
                    }}
                    {move || {
                        deadline.get().map(|label| view! { <DateBadge label=label deadline=true/> })
                    }}
                    {tags
                        .iter()
                        .map(|tag| view! { <TagChip label=tag.clone()/> })
                        .collect_view()}
                    {has_recurrence
                        .then(|| {
                            view! {
                                <span class="row-badge" aria-label="Repeats">
                                    <RepeatIcon/>
                                </span>
                            }
                        })}
                </span>
            </div>
        </li>
    }
}

/// Project row primitive with derived progress.
#[component]
pub fn ProjectRow(
    /// Project title.
    #[prop(into)]
    title: String,
    /// Open child count.
    #[prop(optional)]
    open_count: u32,
    /// Derived completion ratio, when known.
    #[prop(optional)]
    progress: Option<f32>,
    /// Owning area name.
    #[prop(into, optional)]
    area: MaybeProp<String>,
    /// Reports activation intent.
    #[prop(optional)]
    on_open: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <li class="project-row" data-testid="project-row">
            {progress.map(|value| view! { <ProgressRing progress=value/> })}
            <span class="project-title">{title}</span>
            {move || {
                area.get()
                    .map(|area| view! { <span class="project-area">{area}</span> })
            }}
            <span class="project-count">{format!("{open_count} open")}</span>
            <button
                type="button"
                class="row-action"
                aria-label="Open project"
                on:click=move |_| {
                    if let Some(callback) = on_open {
                        callback.run(());
                    }
                }
            >
                <ChevronIcon/>
            </button>
        </li>
    }
}

/// Sidebar row primitive for built-in lists, areas, and tags.
#[component]
pub fn SidebarRow(
    /// Visible label.
    #[prop(into)]
    label: String,
    /// Icon kind.
    #[prop(optional)]
    icon: SidebarIcon,
    /// Optional count badge.
    #[prop(optional)]
    count: Option<u32>,
    /// Marks the destination as active.
    #[prop(optional)]
    active: bool,
    /// Renders a link when provided.
    #[prop(into, optional)]
    href: MaybeProp<String>,
    /// Reports activation intent for non-link rows.
    #[prop(optional)]
    on_activate: Option<Callback<()>>,
) -> impl IntoView {
    let class = if active {
        "sidebar-row sidebar-row-active"
    } else {
        "sidebar-row"
    };
    let label_for_button = label.clone();
    let count_for_button = count;
    let label_for_link = label.clone();

    view! {
        <li class=class data-testid="sidebar-row" data-active=active.then_some("true")>
            {move || match href.get() {
                Some(href) => {
                    view! {
                        <a class="sidebar-row-link" href=href aria-current=active.then_some("page")>
                            {sidebar_icon(icon)}
                            <span class="sidebar-row-label">{label_for_link.clone()}</span>
                            {count
                                .map(|count| {
                                    view! { <span class="sidebar-row-count">{count.to_string()}</span> }
                                })}
                        </a>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <button
                            type="button"
                            class="sidebar-row-link"
                            aria-label=label_for_button.clone()
                            on:click=move |_| {
                                if let Some(callback) = on_activate {
                                    callback.run(());
                                }
                            }
                        >
                            {sidebar_icon(icon)}
                            <span class="sidebar-row-label">{label_for_button.clone()}</span>
                            {count_for_button
                                .map(|count| {
                                    view! { <span class="sidebar-row-count">{count.to_string()}</span> }
                                })}
                        </button>
                    }
                        .into_any()
                }
            }}
        </li>
    }
}

fn sidebar_icon(icon: SidebarIcon) -> impl IntoView {
    match icon {
        SidebarIcon::Inbox => view! { <InboxIcon/> }.into_any(),
        SidebarIcon::Today => view! { <StarIcon/> }.into_any(),
        SidebarIcon::Upcoming => view! { <ClockIcon/> }.into_any(),
        SidebarIcon::Anytime => view! { <LayersIcon/> }.into_any(),
        SidebarIcon::Archive => view! { <ArchiveIcon/> }.into_any(),
        SidebarIcon::Folder => view! { <FolderIcon/> }.into_any(),
        SidebarIcon::Tag => view! { <TagIcon/> }.into_any(),
    }
}

/// Popover primitive with `aria-expanded`/`aria-controls` and Escape dismissal.
#[component]
pub fn Popover(
    /// DOM id of the panel; must be unique on the page.
    #[prop(into)]
    id: String,
    /// Trigger label.
    #[prop(into)]
    trigger_label: String,
    /// Whether the panel starts open.
    #[prop(optional)]
    initially_open: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(initially_open);
    let panel_id = format!("{id}-panel");
    let trigger_id = format!("{id}-trigger");
    let panel_id_for_trigger = panel_id.clone();

    view! {
        <div
            class="popover"
            data-testid="popover"
            data-state=move || if open.get() { "open" } else { "closed" }
            on:keydown=move |event| {
                if event.key() == "Escape" {
                    open.set(false);
                }
            }
        >
            <button
                type="button"
                id=trigger_id
                class="popover-trigger"
                aria-haspopup="dialog"
                aria-expanded=move || if open.get() { "true" } else { "false" }
                aria-controls=panel_id_for_trigger
                on:click=move |_| open.update(|open| *open = !*open)
            >
                {trigger_label}
            </button>
            <div
                id=panel_id
                class="popover-panel"
                role="dialog"
                aria-label="Options"
                hidden=move || !open.get()
            >
                {children()}
            </div>
        </div>
    }
}

/// One entry in a context menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextMenuItem {
    /// Visible label.
    pub label: String,
    /// Whether the item is disabled.
    pub disabled: bool,
    /// Whether the item is destructive.
    pub danger: bool,
}

impl ContextMenuItem {
    /// Creates an enabled item.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            danger: false,
        }
    }

    /// Marks the item destructive.
    #[must_use]
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    /// Marks the item disabled.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Context menu primitive exposing `menu`/`menuitem` semantics.
#[component]
pub fn ContextMenu(
    /// Menu label.
    #[prop(into)]
    label: String,
    /// Menu entries.
    items: Vec<ContextMenuItem>,
    /// Reports the selected entry index.
    #[prop(optional)]
    on_select: Option<Callback<usize>>,
) -> impl IntoView {
    view! {
        <div class="context-menu" role="menu" aria-label=label data-testid="context-menu">
            {items
                .into_iter()
                .enumerate()
                .map(|(index, item)| {
                    let class = if item.danger {
                        "context-menu-item context-menu-item-danger"
                    } else {
                        "context-menu-item"
                    };
                    let label = item.label.clone();
                    view! {
                        <button
                            type="button"
                            role="menuitem"
                            class=class
                            tabindex="-1"
                            disabled=item.disabled
                            on:click=move |_| {
                                if let Some(callback) = on_select {
                                    callback.run(index);
                                }
                            }
                        >
                            {label}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Date control button used by schedule/deadline/reminder pickers.
#[component]
pub fn DateControl(
    /// Accessible name describing what the control changes.
    #[prop(into)]
    label: String,
    /// Displayed value.
    #[prop(into)]
    value: String,
    /// Whether this represents a deadline.
    #[prop(optional)]
    deadline: bool,
    /// Reports activation intent.
    #[prop(optional)]
    on_click: Option<Callback<()>>,
) -> impl IntoView {
    let class = if deadline {
        "date-control date-control-deadline"
    } else {
        "date-control"
    };
    let icon = if deadline {
        view! { <FlagIcon/> }.into_any()
    } else {
        view! { <CalendarIcon/> }.into_any()
    };
    view! {
        <button
            type="button"
            class=class
            aria-label=label
            data-testid="date-control"
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            {icon}
            <span>{value}</span>
        </button>
    }
}

/// Add control used by Magic Plus and list footers.
#[component]
pub fn AddControl(
    /// Accessible name.
    #[prop(into)]
    label: String,
    /// Reports activation intent.
    #[prop(optional)]
    on_click: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="add-control"
            aria-label=label
            data-testid="add-control"
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            <PlusIcon/>
        </button>
    }
}

/// Motion wrapper applying a centralized motion primitive.
#[component]
pub fn Motion(
    /// Motion primitive kind.
    #[prop(optional)]
    kind: MotionKind,
    children: Children,
) -> impl IntoView {
    let class = format!("motion {}", kind.class());
    view! {
        <div class=class data-motion=kind.data_value()>
            {children()}
        </div>
    }
}
