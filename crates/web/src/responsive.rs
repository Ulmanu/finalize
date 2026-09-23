//! Responsive layout model.
//!
//! Breakpoints are part of the design system. The server cannot know the viewport, so SSR
//! renders every layout region and CSS decides visibility; hydration resolves the mode from
//! the window width and keeps it current for behavior that depends on it.

use leptos::prelude::*;

/// Minimum viewport width (CSS pixels) treated as tablet.
pub const TABLET_MIN_WIDTH: u32 = 768;

/// Minimum viewport width (CSS pixels) treated as desktop.
pub const DESKTOP_MIN_WIDTH: u32 = 1024;

/// Canonical CSS representation of the tablet range.
pub const TABLET_MEDIA: &str = "(min-width: 48rem) and (max-width: 63.99rem)";

/// Canonical CSS representation of the mobile range.
pub const MOBILE_MEDIA: &str = "(max-width: 47.99rem)";

/// Canonical CSS representation of the desktop breakpoint.
pub const DESKTOP_MEDIA: &str = "(min-width: 64rem)";

/// Resolved layout mode for the current viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutMode {
    /// Wide screens with persistent navigation.
    #[default]
    Desktop,
    /// Medium screens with a collapsible/overlay navigation drawer.
    Tablet,
    /// Narrow screens with a single-column layout and bottom destination bar.
    Mobile,
}

impl LayoutMode {
    /// Maps a viewport width in CSS pixels to a layout mode.
    #[must_use]
    pub const fn from_width(width: u32) -> Self {
        if width >= DESKTOP_MIN_WIDTH {
            Self::Desktop
        } else if width >= TABLET_MIN_WIDTH {
            Self::Tablet
        } else {
            Self::Mobile
        }
    }

    /// Stable string used in `data-layout` attributes and tests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Tablet => "tablet",
            Self::Mobile => "mobile",
        }
    }

    /// Whether navigation is collapsed behind a toggle in this mode.
    #[must_use]
    pub const fn navigation_collapsed(self) -> bool {
        matches!(self, Self::Tablet | Self::Mobile)
    }
}

/// Reactive layout state shared by the shell.
#[derive(Debug, Clone, Copy)]
pub struct LayoutState {
    /// Resolved layout mode.
    pub mode: RwSignal<LayoutMode>,
    /// Whether the overlay navigation drawer is open.
    pub drawer_open: RwSignal<bool>,
}

impl LayoutState {
    /// Creates the state and provides it to the reactive context.
    pub fn provide() -> Self {
        let state = Self {
            mode: RwSignal::new(LayoutMode::Desktop),
            drawer_open: RwSignal::new(false),
        };
        provide_context(state);
        state.sync_with_viewport();
        state
    }

    /// Returns the layout state from context.
    ///
    /// # Panics
    ///
    /// Panics when called outside the application root, which always provides the state.
    #[must_use]
    pub fn use_state() -> Self {
        use_context::<Self>().expect("layout state is provided by App")
    }

    /// Opens or closes the navigation drawer.
    pub fn toggle_drawer(&self) {
        self.drawer_open.update(|open| *open = !*open);
    }

    /// Closes the navigation drawer.
    pub fn close_drawer(&self) {
        self.drawer_open.set(false);
    }

    /// Resolves the mode from the browser viewport and subscribes to resize events.
    ///
    /// This is a no-op during server rendering and in non-browser targets.
    pub fn sync_with_viewport(&self) {
        #[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
        {
            let mode = self.mode;
            let update = move || {
                if let Some(window) = leptos::web_sys::window() {
                    let width = window
                        .inner_width()
                        .ok()
                        .and_then(|value| value.as_f64())
                        .unwrap_or(f64::from(DESKTOP_MIN_WIDTH));
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let width = width.max(0.0) as u32;
                    mode.set(LayoutMode::from_width(width));
                }
            };
            update();
            let _ = window_event_listener(leptos::ev::resize, move |_| update());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_mapping_uses_documented_breakpoints() {
        assert_eq!(LayoutMode::from_width(320), LayoutMode::Mobile);
        assert_eq!(LayoutMode::from_width(767), LayoutMode::Mobile);
        assert_eq!(LayoutMode::from_width(768), LayoutMode::Tablet);
        assert_eq!(LayoutMode::from_width(1023), LayoutMode::Tablet);
        assert_eq!(LayoutMode::from_width(1024), LayoutMode::Desktop);
        assert_eq!(LayoutMode::from_width(2560), LayoutMode::Desktop);
    }

    #[test]
    fn collapsed_navigation_only_in_tablet_and_mobile() {
        assert!(!LayoutMode::Desktop.navigation_collapsed());
        assert!(LayoutMode::Tablet.navigation_collapsed());
        assert!(LayoutMode::Mobile.navigation_collapsed());
    }
}
