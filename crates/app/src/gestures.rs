//! Touch gesture state machines: swipe, long press, and pull-to-search classification.
//!
//! Recognition is pure arithmetic over touch deltas and durations so the hydrated client and the
//! tests share one definition. Vertical movement is treated as scrolling and never fires row
//! actions; slow gestures are treated as ordinary interactions; long press requires near-zero
//! drift so an accidental click cannot double-fire.

/// Horizontal swipe threshold in CSS pixels.
pub const SWIPE_THRESHOLD: f64 = 60.0;
/// Long-press duration threshold in milliseconds.
pub const LONG_PRESS_MS: u64 = 500;
/// Pull-to-search threshold in CSS pixels.
pub const PULL_THRESHOLD: f64 = 80.0;
/// Gestures slower than this are ordinary interactions, not swipes.
pub const GESTURE_MAX_MS: u64 = 800;
/// Maximum drift allowed for a long press.
pub const LONG_PRESS_SLOP: f64 = 10.0;

/// The action a recognized gesture performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureAction {
    /// Nothing recognized; treat as a tap/scroll.
    None,
    /// Open the item's editor (long press).
    Open,
    /// Apply the configured left-swipe action.
    SwipeLeft,
    /// Apply the configured right-swipe action.
    SwipeRight,
}

/// Configured swipe actions per direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwipeConfiguration {
    /// Action for a left swipe.
    pub left: GestureAction,
    /// Action for a right swipe.
    pub right: GestureAction,
}

/// The default swipe configuration: left selects, right opens the When controls.
#[must_use]
pub const fn default_swipe_configuration() -> SwipeConfiguration {
    SwipeConfiguration {
        left: GestureAction::SwipeLeft,
        right: GestureAction::SwipeRight,
    }
}

/// A completed touch sample relative to its start point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchSample {
    /// Horizontal movement in CSS pixels (positive is rightward).
    pub dx: f64,
    /// Vertical movement in CSS pixels (positive is downward).
    pub dy: f64,
    /// Gesture duration in milliseconds.
    pub duration_ms: u64,
}

/// Classifies a row gesture.
///
/// Long press wins when the drift stays inside the slop; otherwise a fast horizontal gesture
/// beyond the threshold becomes a swipe; everything else is `None` so scrolling and taps behave
/// normally.
#[must_use]
pub fn classify_touch(sample: TouchSample) -> GestureAction {
    if sample.duration_ms >= LONG_PRESS_MS && sample.dx.hypot(sample.dy) <= LONG_PRESS_SLOP {
        return GestureAction::Open;
    }
    if sample.duration_ms <= GESTURE_MAX_MS
        && sample.dx.abs() >= SWIPE_THRESHOLD
        && sample.dx.abs() > sample.dy.abs()
    {
        return if sample.dx < 0.0 {
            GestureAction::SwipeLeft
        } else {
            GestureAction::SwipeRight
        };
    }
    GestureAction::None
}

/// Whether a downward pull at the top boundary opens Quick Find.
#[must_use]
pub fn pull_to_search(started_at_top: bool, dx: f64, dy: f64, duration_ms: u64) -> bool {
    started_at_top
        && dy >= PULL_THRESHOLD
        && duration_ms <= GESTURE_MAX_MS
        && dx.abs() < SWIPE_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::{
        classify_touch, pull_to_search, GestureAction, TouchSample, GESTURE_MAX_MS, LONG_PRESS_MS,
        LONG_PRESS_SLOP, PULL_THRESHOLD, SWIPE_THRESHOLD,
    };

    fn sample(dx: f64, dy: f64, duration_ms: u64) -> TouchSample {
        TouchSample {
            dx,
            dy,
            duration_ms,
        }
    }

    #[test]
    fn swipes_fire_only_for_fast_horizontal_gestures() {
        assert_eq!(
            classify_touch(sample(SWIPE_THRESHOLD + 1.0, 2.0, 200)),
            GestureAction::SwipeRight
        );
        assert_eq!(
            classify_touch(sample(-SWIPE_THRESHOLD - 1.0, 2.0, 200)),
            GestureAction::SwipeLeft
        );
        // Too short, too slow, or mostly vertical: no action (scrolling wins).
        assert_eq!(
            classify_touch(sample(SWIPE_THRESHOLD - 1.0, 0.0, 200)),
            GestureAction::None
        );
        assert_eq!(
            classify_touch(sample(200.0, 0.0, GESTURE_MAX_MS + 1)),
            GestureAction::None
        );
        assert_eq!(
            classify_touch(sample(200.0, 200.0, 300)),
            GestureAction::None
        );
    }

    #[test]
    fn long_press_opens_only_within_slop() {
        assert_eq!(
            classify_touch(sample(2.0, 2.0, LONG_PRESS_MS)),
            GestureAction::Open
        );
        assert_eq!(
            classify_touch(sample(LONG_PRESS_SLOP + 1.0, 0.0, LONG_PRESS_MS)),
            GestureAction::None
        );
        assert_eq!(
            classify_touch(sample(0.0, 0.0, LONG_PRESS_MS - 1)),
            GestureAction::None
        );
        // A long press that turned into a swipe is still a swipe candidate.
        assert_eq!(
            classify_touch(sample(SWIPE_THRESHOLD + 5.0, 0.0, GESTURE_MAX_MS)),
            GestureAction::SwipeRight
        );
    }

    #[test]
    fn pull_to_search_requires_the_top_boundary() {
        assert!(pull_to_search(true, 0.0, PULL_THRESHOLD, 300));
        assert!(!pull_to_search(false, 0.0, PULL_THRESHOLD, 300));
        assert!(!pull_to_search(true, 0.0, PULL_THRESHOLD - 1.0, 300));
        assert!(!pull_to_search(
            true,
            0.0,
            PULL_THRESHOLD,
            GESTURE_MAX_MS + 1
        ));
        assert!(!pull_to_search(true, SWIPE_THRESHOLD, PULL_THRESHOLD, 300));
        assert!(!pull_to_search(true, 0.0, 20.0, 300));
    }
}
