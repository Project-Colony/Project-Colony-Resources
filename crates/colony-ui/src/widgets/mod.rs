//! The widgets every Colony program's preferences page is built from.
//!
//! Ported from Colony's `src/ui/settings.rs`, which is where they were first
//! written and where they were being copied from by hand. They are generic over
//! the host's message type: you pass the message to emit, the crate draws the
//! control.
//!
//! Sizes go through [`Typography`](crate::Typography) rather than being
//! hardcoded, so the host's two font-scale preferences reach them.

mod accent_picker;
mod collapsible;
mod theme_picker;
mod toggle;

pub use accent_picker::accent_picker;
pub use collapsible::collapsible_section;
pub use theme_picker::theme_picker;
pub use toggle::functional_toggle;

/// Glyphs the shared widgets draw, in the Nerd Font range that
/// `JetBrainsMono Nerd Font` covers. Exposed so a program's own widgets can use
/// the same ones rather than picking a near-miss.
pub mod icons {
    /// Section expanded.
    pub const CHEVRON_DOWN: &str = "\u{f078}";
    /// Section collapsed.
    pub const CHEVRON_RIGHT: &str = "\u{f054}";
    /// Selected.
    pub const CHECK: &str = "\u{f00c}";
    /// Opens the preferences, next to the program name.
    pub const GEAR: &str = "\u{f013}";
}
