//! Shared user-interface layer for Project Colony programs.
//!
//! Everything here used to be copy-pasted, re-derived, or scraped out of
//! another repository's source. A program depending on this crate gets the
//! palettes, the theme resolver, the accent overrides, the fonts, the display
//! strings, the user's standard preferences and the widgets its Preferences
//! page is built from, and, importantly, **stops having to change** when a
//! theme family is added.
//!
//! ```no_run
//! use colony_ui::preferences::StandardPreferences;
//! use colony_ui::{i18n, theme, Typography};
//!
//! // On startup, from the user's config:
//! let prefs = StandardPreferences::default(); // or deserialized from the file
//! prefs.apply();
//! i18n::set_locale(i18n::Locale::from_tag("fr"));
//!
//! // Then style widgets from the active palette, sized and set in the user's
//! // font:
//! let typo = Typography::current();
//! let title = iced::widget::text(i18n::t("settings_title"))
//!     .size(typo.sz(22))
//!     .font(typo.bold)
//!     .color(theme::Palette::TEXT_PRIMARY());
//! # let _: iced::Element<'_, ()> = title.into();
//! ```
//!
//! | Module | What it holds |
//! |---|---|
//! | [`theme`] | palettes, the active theme, accents, high contrast, contrast helpers |
//! | [`typography`] | the two text-size preferences, [`font_scale`], [`sz`], [`Typography`] |
//! | [`fonts`] | the three fonts, the accessor that honours the dyslexia toggle, the font files |
//! | [`motion`] | reduced motion and the effects that obey it |
//! | [`preferences`] | [`StandardPreferences`](preferences::StandardPreferences): load, apply, update |
//! | [`widgets`] | identity button, navigation, the Preferences page and its categories |
//! | [`i18n`] | every shared label, English and French |
//! | [`paths`] | `Colony/<Program>/` config, data and cache directories |
//!
//! The colours themselves live in `tokens/` at the repository root and are
//! generated into this crate; see the repository README.

pub mod fonts;
pub mod i18n;
pub mod motion;
pub mod paths;
pub mod preferences;
pub mod theme;
pub mod typography;
pub mod widgets;

pub use theme::{
    accent_key_to_color, active_palette, app_tint, contrast_on, contrast_ratio, effective_accent,
    hex, is_high_contrast, resolve, set_active_accent, set_active_theme, set_high_contrast,
    AccentOverride, ColorExt, Palette, ThemeFamilyMeta, ThemePalette, ThemeVariantMeta,
    ACCENT_OVERRIDES, FALLBACK_PALETTE, THEME_FAMILIES,
};

pub use fonts::{is_dyslexia_font, set_dyslexia_font, ui_font, ui_font_bold, ui_font_medium};
pub use motion::{effects_enabled, is_reduced_motion, set_effects, set_reduced_motion};
pub use typography::{
    font_scale, font_size, set_font_size, set_text_size, sz, text_size, FontSize, TextSize,
    Typography,
};

/// The globals (theme, accent, high contrast, fonts, motion, sizes) are
/// process-wide, so the unit tests that set them must not run concurrently.
#[cfg(test)]
pub(crate) fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    static GLOBALS: std::sync::Mutex<()> = std::sync::Mutex::new(());
    GLOBALS.lock().unwrap_or_else(|e| e.into_inner())
}
