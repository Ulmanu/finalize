//! Original inline-SVG icons for Momentum.
//!
//! Every icon is defined here in Rust source. No third-party or Cultured Code icon assets are
//! used. Icons are decorative by default (`aria-hidden`); interactive controls provide their
//! own accessible names.

use leptos::prelude::*;

macro_rules! icon_component {
    ($(#[$meta:meta])* $name:ident, $path:literal) => {
        $(#[$meta])*
        #[component]
        pub fn $name() -> impl IntoView {
            view! {
                <svg
                    class="icon icon-md"
                    viewBox="0 0 16 16"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.6"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                    focusable="false"
                >
                    <path d=$path></path>
                </svg>
            }
        }
    };
}

icon_component!(
    /// Checkmark used by completion controls.
    CheckIcon,
    "M3 8.5 6.2 11.7 13 4.5"
);
icon_component!(
    /// Plus used by add controls.
    PlusIcon,
    "M8 3v10M3 8h10"
);
icon_component!(
    /// Calendar used by schedule/date controls.
    CalendarIcon,
    "M3.5 4.5h9v8h-9zM3.5 7h9M5.5 3v3M10.5 3v3"
);
icon_component!(
    /// Flag used by deadlines.
    FlagIcon,
    "M4 2.5v11M4 3.5h7.5l-1.5 2.5 1.5 2.5H4"
);
icon_component!(
    /// Tag used by tag chips and tag destinations.
    TagIcon,
    "M2.5 7.2 7.2 2.5h5.3v5.3l-4.7 4.7zM10.5 5.5h.01"
);
icon_component!(
    /// Note lines used by note indicators.
    NoteIcon,
    "M3.5 4h9M3.5 7h9M3.5 10h6"
);
icon_component!(
    /// Repeating arrows used by recurrence indicators.
    RepeatIcon,
    "M4 6.5A3.5 3.5 0 0 1 7.5 3H12M12 3l-2-2M12 9.5A3.5 3.5 0 0 1 8.5 13H4M4 13l2 2"
);
icon_component!(
    /// Trash used by deletion affordances.
    TrashIcon,
    "M3.5 5h9M6 5V3.5h4V5M5 5l.5 8h5L11 5"
);
icon_component!(
    /// Chevron used for disclosure and sidebar disclosure.
    ChevronIcon,
    "M6 4l4 4-4 4"
);
icon_component!(
    /// Inbox tray used by the Inbox destination.
    InboxIcon,
    "M2.5 9.5 4 3.5h8l1.5 6v3h-11zM2.5 9.5h3l1 1.5h3l1-1.5h3"
);
icon_component!(
    /// Star used by the Today destination.
    StarIcon,
    "M8 2.5l1.7 3.6 3.8.5-2.8 2.7.7 3.9L8 11.3l-3.4 1.9.7-3.9L2.5 6.6l3.8-.5z"
);
icon_component!(
    /// Clock used by Upcoming.
    ClockIcon,
    "M8 2.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11zM8 5v3.2l2.2 1.3"
);
icon_component!(
    /// Layers used by Anytime.
    LayersIcon,
    "M8 2.5 14 6l-6 3.5L2 6zM2.5 9.5 8 12.7l5.5-3.2"
);
icon_component!(
    /// Archive box used by Someday and Logbook.
    ArchiveIcon,
    "M2.5 3.5h11v3h-11zM4 6.5v6h8v-6M6.5 9h3"
);
icon_component!(
    /// Folder used by projects and areas.
    FolderIcon,
    "M2.5 4h4l1.2 1.5h5.8v7h-11z"
);
