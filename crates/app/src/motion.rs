//! Centralized motion primitives.
//!
//! Every animation in the product is declared here once: a stable id, the CSS class that carries
//! it, the motion token used, and whether it is essential. Motion is presentation-only — it
//! never alters domain state ordering and never gates command completion — and all nonessential
//! primitives are suppressed or shortened when reduced motion is active.

/// One declared motion primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotionPrimitive {
    /// Stable identifier.
    pub id: &'static str,
    /// Human-readable title.
    pub title: &'static str,
    /// CSS class carrying the animation.
    pub class_name: &'static str,
    /// Design token used for the duration (`--motion-fast`/`base`/`slow`).
    pub duration_token: &'static str,
    /// Whether the motion is essential (never suppressed by reduced motion).
    pub essential: bool,
}

const REGISTRY: &[MotionPrimitive] = &[
    MotionPrimitive {
        id: "insert",
        title: "Insertion",
        class_name: "motion-insert",
        duration_token: "--motion-base",
        essential: false,
    },
    MotionPrimitive {
        id: "remove",
        title: "Removal and completion",
        class_name: "motion-remove",
        duration_token: "--motion-base",
        essential: false,
    },
    MotionPrimitive {
        id: "editor",
        title: "Editor expansion",
        class_name: "motion-editor",
        duration_token: "--motion-base",
        essential: false,
    },
    MotionPrimitive {
        id: "popover",
        title: "Popover and overlay",
        class_name: "motion-popover",
        duration_token: "--motion-fast",
        essential: false,
    },
    MotionPrimitive {
        id: "drag",
        title: "Drag lift and drop",
        class_name: "motion-drag",
        duration_token: "--motion-fast",
        essential: false,
    },
    MotionPrimitive {
        id: "navigation",
        title: "Navigation transition",
        class_name: "motion-nav",
        duration_token: "--motion-slow",
        essential: false,
    },
];

/// Every declared motion primitive.
#[must_use]
pub const fn registry() -> &'static [MotionPrimitive] {
    REGISTRY
}

/// Looks up a primitive by id.
#[must_use]
pub fn primitive(id: &str) -> Option<&'static MotionPrimitive> {
    REGISTRY.iter().find(|primitive| primitive.id == id)
}

/// The class name for a primitive, when declared.
#[must_use]
pub fn class_for(id: &str) -> Option<&'static str> {
    primitive(id).map(|primitive| primitive.class_name)
}

/// Nonessential primitives (the set reduced motion suppresses).
#[must_use]
pub fn nonessential() -> Vec<&'static MotionPrimitive> {
    REGISTRY
        .iter()
        .filter(|primitive| !primitive.essential)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{class_for, nonessential, primitive, registry};

    #[test]
    fn registry_is_unique_and_covers_every_primitive() {
        let mut ids: Vec<&str> = registry().iter().map(|primitive| primitive.id).collect();
        let total = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), total, "duplicate motion ids");
        let mut classes: Vec<&str> = registry()
            .iter()
            .map(|primitive| primitive.class_name)
            .collect();
        classes.sort_unstable();
        classes.dedup();
        assert_eq!(classes.len(), total, "duplicate motion classes");

        for id in [
            "insert",
            "remove",
            "editor",
            "popover",
            "drag",
            "navigation",
        ] {
            assert!(primitive(id).is_some(), "missing {id}");
            assert_eq!(
                class_for(id),
                Some(primitive(id).expect("primitive").class_name)
            );
        }
        assert_eq!(class_for("missing"), None);

        for primitive in registry() {
            assert!(
                primitive.duration_token.starts_with("--motion-"),
                "{} uses an unknown token",
                primitive.id
            );
            assert!(!primitive.title.is_empty());
        }
    }

    #[test]
    fn reduced_motion_suppresses_every_nonessential_primitive() {
        assert_eq!(nonessential().len(), registry().len());
        assert!(nonessential().iter().all(|primitive| !primitive.essential));
    }
}
