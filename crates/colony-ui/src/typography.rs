//! Text sizing: the two size preferences, their product, and `sz()`.
//!
//! Never write a raw pixel size (design/typography.md). Every size goes
//! through [`sz`], which returns `round(base × font_scale())`, and
//! [`font_scale`] is the product of two independent user preferences:
//!
//! | Preference | Where | Values |
//! |---|---|---|
//! | [`FontSize`] | Preferences → Appearance → Typography | small 0.85, default 1.0, large 1.2 |
//! | [`TextSize`] | Preferences → Accessibility → Reading | small 0.85, default 1.0, large 1.2, xlarge 1.4 |
//!
//! They multiply, so a layout has to survive both extremes: 0.7225× and 1.68×.
//!
//! Both are process-global, like the active theme, and are normally set in one
//! go by [`StandardPreferences::apply`](crate::preferences::StandardPreferences::apply).

use std::sync::atomic::{AtomicU8, Ordering};

use iced::Font;
use serde::{Deserialize, Serialize};

use crate::fonts;

/// Appearance → Typography: the base text size.
///
/// Serialized as `"small"`, `"default"`, `"large"`, the strings Colony already
/// stores, so an existing preferences file keeps its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FontSize {
    Small,
    #[default]
    Default,
    Large,
}

impl FontSize {
    /// Every value, in the order a picker lists them.
    pub const ALL: [FontSize; 3] = [FontSize::Small, FontSize::Default, FontSize::Large];

    /// The multiplier this size contributes to [`font_scale`].
    pub fn factor(self) -> f32 {
        match self {
            FontSize::Small => 0.85,
            FontSize::Default => 1.0,
            FontSize::Large => 1.2,
        }
    }

    /// The i18n key of its display name.
    pub fn label_key(self) -> &'static str {
        match self {
            FontSize::Small => "settings_font_size_small",
            FontSize::Default => "settings_font_size_default",
            FontSize::Large => "settings_font_size_large",
        }
    }
}

/// Accessibility → Reading: text scaling on top of [`FontSize`].
///
/// Serialized as `"small"`, `"default"`, `"large"`, `"xlarge"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextSize {
    Small,
    #[default]
    Default,
    Large,
    #[serde(rename = "xlarge")]
    XLarge,
}

impl TextSize {
    /// Every value, in the order a picker lists them.
    pub const ALL: [TextSize; 4] = [
        TextSize::Small,
        TextSize::Default,
        TextSize::Large,
        TextSize::XLarge,
    ];

    /// The multiplier this size contributes to [`font_scale`].
    pub fn factor(self) -> f32 {
        match self {
            TextSize::Small => 0.85,
            TextSize::Default => 1.0,
            TextSize::Large => 1.2,
            TextSize::XLarge => 1.4,
        }
    }

    /// The i18n key of its display name. The first three share
    /// [`FontSize`]'s labels: a size is called the same thing in both places.
    pub fn label_key(self) -> &'static str {
        match self {
            TextSize::Small => "settings_font_size_small",
            TextSize::Default => "settings_font_size_default",
            TextSize::Large => "settings_font_size_large",
            TextSize::XLarge => "settings_font_size_xlarge",
        }
    }
}

// Stored as the variant's discriminant, which is its index in `ALL`: the enums
// are `Copy` and tiny, and an atomic needs no lock and cannot be poisoned.
static FONT_SIZE: AtomicU8 = AtomicU8::new(1);
static TEXT_SIZE: AtomicU8 = AtomicU8::new(1);

/// Set Appearance → Typography's size.
pub fn set_font_size(size: FontSize) {
    FONT_SIZE.store(size as u8, Ordering::Relaxed);
}

/// The current Appearance → Typography size.
pub fn font_size() -> FontSize {
    FontSize::ALL
        .get(FONT_SIZE.load(Ordering::Relaxed) as usize)
        .copied()
        .unwrap_or_default()
}

/// Set Accessibility → Reading's text size.
pub fn set_text_size(size: TextSize) {
    TEXT_SIZE.store(size as u8, Ordering::Relaxed);
}

/// The current Accessibility → Reading text size.
pub fn text_size() -> TextSize {
    TextSize::ALL
        .get(TEXT_SIZE.load(Ordering::Relaxed) as usize)
        .copied()
        .unwrap_or_default()
}

/// The product of both size preferences. 1.0 is unscaled.
pub fn font_scale() -> f32 {
    font_size().factor() * text_size().factor()
}

/// Scale a base size by the user's preferences: `round(base × font_scale())`.
///
/// ```
/// colony_ui::typography::set_font_size(colony_ui::typography::FontSize::Large);
/// colony_ui::typography::set_text_size(colony_ui::typography::TextSize::XLarge);
/// assert_eq!(colony_ui::sz(30), 50.0); // 30 × 1.68 = 50.4
/// ```
pub fn sz(base: u16) -> f32 {
    scale(base, font_scale())
}

fn scale(base: u16, factor: f32) -> f32 {
    (base as f32 * factor).round()
}

/// What a shared widget needs to know about the host program's text.
///
/// The crate's widgets take one of these rather than reading the globals
/// themselves, so a program can still draw a widget at a size or in a font of
/// its own. [`Typography::current`] builds it from the globals, which is what a
/// program normally passes.
#[derive(Debug, Clone, Copy)]
pub struct Typography {
    /// The product of every font-size preference. 1.0 is unscaled.
    pub scale: f32,
    pub regular: Font,
    pub medium: Font,
    pub bold: Font,
}

impl Typography {
    /// The user's current text settings: [`font_scale`] and the three weights
    /// of the interface font, which is OpenDyslexic while the dyslexia font is
    /// on (see [`fonts`](crate::fonts)).
    pub fn current() -> Self {
        Typography {
            scale: font_scale(),
            regular: fonts::ui_font(),
            medium: fonts::ui_font_medium(),
            bold: fonts::ui_font_bold(),
        }
    }

    /// Scale a base size the way [`sz`] does, with this typography's scale.
    pub fn sz(&self, base: u16) -> f32 {
        scale(base, self.scale)
    }
}

impl Default for Typography {
    /// Unscaled, in iced's default font. Kept for programs that do not use the
    /// Colony fonts yet; a program that does should pass
    /// [`Typography::current`].
    fn default() -> Self {
        Typography {
            scale: 1.0,
            regular: Font::DEFAULT,
            medium: Font::DEFAULT,
            bold: Font::DEFAULT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_preferences_multiply() {
        let _guard = crate::test_lock();

        set_font_size(FontSize::Large);
        set_text_size(TextSize::XLarge);
        assert!((font_scale() - 1.68).abs() < 1e-6);
        assert_eq!(sz(30), 50.0);
        assert_eq!(sz(13), 22.0);

        set_font_size(FontSize::Small);
        set_text_size(TextSize::Small);
        assert!((font_scale() - 0.7225).abs() < 1e-6);
        assert_eq!(sz(30), 22.0);
        assert_eq!(sz(10), 7.0);

        set_font_size(FontSize::Default);
        set_text_size(TextSize::Default);
        assert_eq!(font_scale(), 1.0);
        assert_eq!(sz(22), 22.0);
    }

    #[test]
    fn the_globals_round_trip_every_value() {
        let _guard = crate::test_lock();

        for size in FontSize::ALL {
            set_font_size(size);
            assert_eq!(font_size(), size);
        }
        for size in TextSize::ALL {
            set_text_size(size);
            assert_eq!(text_size(), size);
        }
        set_font_size(FontSize::Default);
        set_text_size(TextSize::Default);
    }

    #[test]
    fn current_follows_the_globals() {
        let _guard = crate::test_lock();

        set_text_size(TextSize::Large);
        let typo = Typography::current();
        assert!((typo.scale - 1.2).abs() < 1e-6);
        assert_eq!(typo.sz(10), 12.0);
        assert_eq!(typo.regular, fonts::ui_font());
        set_text_size(TextSize::Default);
    }

    #[test]
    fn sizes_serialize_as_the_strings_colony_already_stores() {
        assert_eq!(
            serde_json::to_string(&FontSize::Large).unwrap(),
            "\"large\""
        );
        assert_eq!(
            serde_json::to_string(&TextSize::XLarge).unwrap(),
            "\"xlarge\""
        );
        let parsed: TextSize = serde_json::from_str("\"small\"").unwrap();
        assert_eq!(parsed, TextSize::Small);
    }
}
