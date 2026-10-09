//! Motion: the reduced-motion preference and the effects that obey it.
//!
//! Motion is a user preference, Preferences → Accessibility → Motion, and a
//! program that animates must honour it (design/navigation.md). Rather than
//! checking two flags at every animation, ask [`effects_enabled`]: it is false
//! whenever the user turned effects off in Appearance **or** asked for reduced
//! motion in Accessibility, which is what "effects degrade to nothing" means.
//!
//! ```
//! use colony_ui::motion;
//!
//! motion::set_effects(true);
//! motion::set_reduced_motion(true);
//! assert!(!motion::effects_enabled()); // reduced motion always wins
//! ```

use std::sync::atomic::{AtomicBool, Ordering};

static REDUCED_MOTION: AtomicBool = AtomicBool::new(false);
static EFFECTS: AtomicBool = AtomicBool::new(true);

/// Set Accessibility → Motion → Reduce motion.
pub fn set_reduced_motion(enabled: bool) {
    REDUCED_MOTION.store(enabled, Ordering::Relaxed);
}

/// Whether the user asked for reduced motion.
pub fn is_reduced_motion() -> bool {
    REDUCED_MOTION.load(Ordering::Relaxed)
}

/// Set Appearance → Effects → Animations, the user's own choice. Whether
/// anything actually animates is [`effects_enabled`].
pub fn set_effects(enabled: bool) {
    EFFECTS.store(enabled, Ordering::Relaxed);
}

/// Whether to animate at all: effects on and reduced motion off. Gate every
/// transition, fade, slide and animation tick on this.
pub fn effects_enabled() -> bool {
    EFFECTS.load(Ordering::Relaxed) && !is_reduced_motion()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduced_motion_silences_effects_whatever_the_effects_toggle_says() {
        let _guard = crate::test_lock();

        for (effects, reduced, expected) in [
            (true, false, true),
            (true, true, false),
            (false, false, false),
            (false, true, false),
        ] {
            set_effects(effects);
            set_reduced_motion(reduced);
            assert_eq!(
                effects_enabled(),
                expected,
                "effects {effects}, reduced motion {reduced}"
            );
        }
        set_effects(true);
        set_reduced_motion(false);
    }
}
