//! Central design tokens.
//!
//! Token values live here and in `assets/design.css`. A consistency test parses the stylesheet
//! and asserts that every token below appears with the same value, so the Rust and CSS views
//! of the design system cannot drift apart.

/// A single CSS custom property and its canonical value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// Custom property name, including the leading `--`.
    pub name: &'static str,
    /// Canonical value in the light theme.
    pub value: &'static str,
}

/// Spacing scale.
pub const SPACE: &[Token] = &[
    Token {
        name: "--space-1",
        value: "4px",
    },
    Token {
        name: "--space-2",
        value: "8px",
    },
    Token {
        name: "--space-3",
        value: "12px",
    },
    Token {
        name: "--space-4",
        value: "16px",
    },
    Token {
        name: "--space-5",
        value: "24px",
    },
    Token {
        name: "--space-6",
        value: "32px",
    },
];

/// Corner radius scale.
pub const RADIUS: &[Token] = &[
    Token {
        name: "--radius-sm",
        value: "6px",
    },
    Token {
        name: "--radius-md",
        value: "10px",
    },
    Token {
        name: "--radius-lg",
        value: "16px",
    },
    Token {
        name: "--radius-pill",
        value: "999px",
    },
];

/// Typography scale and weights.
pub const TYPOGRAPHY: &[Token] = &[
    Token {
        name: "--font-size-display",
        value: "1.75rem",
    },
    Token {
        name: "--font-size-title",
        value: "1.25rem",
    },
    Token {
        name: "--font-size-body",
        value: "1rem",
    },
    Token {
        name: "--font-size-caption",
        value: "0.8125rem",
    },
    Token {
        name: "--font-weight-regular",
        value: "400",
    },
    Token {
        name: "--font-weight-medium",
        value: "500",
    },
    Token {
        name: "--font-weight-semibold",
        value: "600",
    },
];

/// Surface and text colors (light theme values; dark theme overrides the same names).
pub const COLOR: &[Token] = &[
    Token {
        name: "--color-text",
        value: "#1d1d1f",
    },
    Token {
        name: "--color-text-muted",
        value: "#6e6e73",
    },
    Token {
        name: "--color-accent",
        value: "#0a66c2",
    },
    Token {
        name: "--color-danger",
        value: "#b3261e",
    },
    Token {
        name: "--color-border",
        value: "#d8d8de",
    },
    Token {
        name: "--surface-canvas",
        value: "#f2f2f7",
    },
    Token {
        name: "--surface-raised",
        value: "#ffffff",
    },
    Token {
        name: "--surface-sunken",
        value: "#e8e8ed",
    },
    Token {
        name: "--surface-overlay",
        value: "#ffffff",
    },
];

/// Shadow elevations.
pub const SHADOW: &[Token] = &[
    Token {
        name: "--shadow-row",
        value: "0 1px 2px rgba(0, 0, 0, 0.06)",
    },
    Token {
        name: "--shadow-popover",
        value: "0 8px 24px rgba(0, 0, 0, 0.16)",
    },
    Token {
        name: "--shadow-modal",
        value: "0 16px 48px rgba(0, 0, 0, 0.24)",
    },
];

/// Icon sizes.
pub const ICON: &[Token] = &[
    Token {
        name: "--icon-sm",
        value: "14px",
    },
    Token {
        name: "--icon-md",
        value: "16px",
    },
    Token {
        name: "--icon-lg",
        value: "20px",
    },
    Token {
        name: "--icon-xl",
        value: "24px",
    },
];

/// Row metrics.
pub const ROW: &[Token] = &[
    Token {
        name: "--row-height",
        value: "44px",
    },
    Token {
        name: "--row-height-compact",
        value: "36px",
    },
    Token {
        name: "--row-inset",
        value: "12px",
    },
    Token {
        name: "--row-indent",
        value: "20px",
    },
];

/// Sidebar metrics.
pub const SIDEBAR: &[Token] = &[
    Token {
        name: "--sidebar-width",
        value: "16rem",
    },
    Token {
        name: "--sidebar-row-height",
        value: "32px",
    },
];

/// Motion durations and easing curves.
pub const MOTION: &[Token] = &[
    Token {
        name: "--motion-fast",
        value: "120ms",
    },
    Token {
        name: "--motion-base",
        value: "200ms",
    },
    Token {
        name: "--motion-slow",
        value: "320ms",
    },
    Token {
        name: "--ease-standard",
        value: "cubic-bezier(0.2, 0, 0, 1)",
    },
    Token {
        name: "--ease-decelerate",
        value: "cubic-bezier(0, 0, 0, 1)",
    },
];

/// Every token group, used by the stylesheet consistency test.
pub const ALL_TOKENS: &[&[Token]] = &[
    SPACE, RADIUS, TYPOGRAPHY, COLOR, SHADOW, ICON, ROW, SIDEBAR, MOTION,
];

/// Motion primitives exposed as CSS classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotionKind {
    /// Row/list insertion.
    #[default]
    Insert,
    /// Row/list removal or completion.
    Remove,
    /// Editor expansion.
    Editor,
    /// Popover/menu presentation.
    Popover,
    /// Drag lift/drop.
    Drag,
    /// Route navigation.
    Navigation,
}

impl MotionKind {
    /// CSS class applied to the motion wrapper.
    #[must_use]
    pub const fn class(self) -> &'static str {
        match self {
            Self::Insert => "motion-insert",
            Self::Remove => "motion-remove",
            Self::Editor => "motion-editor",
            Self::Popover => "motion-popover",
            Self::Drag => "motion-drag",
            Self::Navigation => "motion-navigation",
        }
    }

    /// Value of the `data-motion` attribute used by styles and tests.
    #[must_use]
    pub const fn data_value(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Remove => "remove",
            Self::Editor => "editor",
            Self::Popover => "popover",
            Self::Drag => "drag",
            Self::Navigation => "navigation",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_names_are_unique() {
        let mut names = std::collections::BTreeSet::new();
        for group in ALL_TOKENS {
            for token in *group {
                assert!(names.insert(token.name), "duplicate token {}", token.name);
            }
        }
    }

    #[test]
    fn motion_kinds_have_distinct_classes() {
        let mut classes = std::collections::BTreeSet::new();
        for kind in [
            MotionKind::Insert,
            MotionKind::Remove,
            MotionKind::Editor,
            MotionKind::Popover,
            MotionKind::Drag,
            MotionKind::Navigation,
        ] {
            assert!(classes.insert(kind.class()));
        }
    }
}
