//! Shared user-interface layer for Project Colony programs.
//!
//! Everything here used to be copy-pasted, re-derived, or scraped out of
//! another repository's source. A program depending on this crate gets the
//! palettes, the theme resolver, the accent overrides, the display strings and
//! the shared widgets — and, importantly, **stops having to change** when a
//! theme family is added.
//!
//! ```no_run
//! use colony_ui::{theme, widgets, Typography};
//!
//! // On startup, from the user's config:
//! theme::set_active_theme("gruvbox", "dark");
//! colony_ui::i18n::set_locale(colony_ui::i18n::Locale::Fr);
//!
//! // Then style widgets from the active palette:
//! let bg = theme::Palette::BG_PRIMARY();
//! ```
//!
//! The colours themselves live in `tokens/` at the repository root and are
//! generated into this crate — see the repository README.

pub mod i18n;
pub mod paths;
pub mod theme;
pub mod widgets;

pub use theme::{
    accent_key_to_color, active_palette, app_tint, contrast_on, contrast_ratio, effective_accent,
    hex, resolve, set_active_accent, set_active_theme, set_high_contrast, AccentOverride, ColorExt,
    Palette, ThemeFamilyMeta, ThemePalette, ThemeVariantMeta, ACCENT_OVERRIDES, FALLBACK_PALETTE,
    THEME_FAMILIES,
};

use iced::Font;

/// What a shared widget needs to know about the host program's text.
///
/// Colony scales every size through one accessor so the two independent user
/// preferences — Appearance → Typography and Accessibility → Reading — can
/// multiply. A widget in this crate cannot reach the host's state, so the host
/// passes this in.
#[derive(Debug, Clone, Copy)]
pub struct Typography {
    /// The product of every font-size preference. 1.0 is unscaled.
    pub scale: f32,
    pub regular: Font,
    pub medium: Font,
    pub bold: Font,
}

impl Typography {
    /// Scale a base size the way the host program does.
    pub fn sz(&self, base: u16) -> f32 {
        (base as f32 * self.scale).round()
    }
}

impl Default for Typography {
    fn default() -> Self {
        Typography {
            scale: 1.0,
            regular: Font::DEFAULT,
            medium: Font::DEFAULT,
            bold: Font::DEFAULT,
        }
    }
}
