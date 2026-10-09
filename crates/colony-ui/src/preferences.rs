//! The preferences every Colony program has, as one value a program embeds in
//! its own config.
//!
//! [`StandardPreferences`] holds what the mandatory Appearance and
//! Accessibility categories configure (design/settings-page.md). It knows how to
//! put itself into effect ([`StandardPreferences::apply`]), how to take a change
//! from the shared preferences widgets ([`StandardPreferences::update`]), and how
//! to survive a config file it does not fully understand.
//!
//! ```
//! use colony_ui::preferences::StandardPreferences;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Default, Serialize, Deserialize)]
//! struct MyConfig {
//!     #[serde(flatten)]
//!     standard: StandardPreferences,
//!     scan_on_startup: bool,
//! }
//!
//! // An old file, a hand-edited one, one written by a newer build: whatever is
//! // not understood falls back to its default instead of failing the load.
//! let config: MyConfig = serde_json::from_str(
//!     r#"{ "font_size": "enormous", "high_contrast": "yes", "scan_on_startup": true }"#,
//! )
//! .unwrap();
//! config.standard.apply();
//! assert!(config.scan_on_startup);
//! ```
//!
//! Where the file lives is [`paths::config_dir`](crate::paths::config_dir)'s
//! answer, not this module's: it does no I/O.

use iced::Color;
use serde::de::{Deserializer, IgnoredAny};
use serde::{Deserialize, Serialize};

use crate::typography::{self, FontSize, TextSize};
use crate::{fonts, motion, theme};

/// The user's Appearance and Accessibility choices.
///
/// Field names, and the strings the enums serialize to, are the ones Colony
/// already writes to `preferences.json`, so Colony and any program that copied
/// it can embed this struct (with `#[serde(flatten)]`) without migrating a
/// single file.
///
/// Deserializing never fails on a value: an unknown theme, a size from a newer
/// build or a field of the wrong type each fall back to their default, and the
/// rest of the file is kept. Unknown fields are ignored, which is what lets a
/// program flatten this into a struct of its own.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StandardPreferences {
    /// Theme family key, e.g. `"gruvbox"`. Kept as written even when this build
    /// does not know it: [`theme::resolve`] falls back at apply time, and a
    /// later build that does know it gets the user's choice back.
    pub selected_theme: String,
    /// Theme variant key, e.g. `"dark"`.
    pub selected_variant: String,
    /// Accent override key from [`theme::ACCENT_OVERRIDES`], e.g. `"violet"`.
    /// `None` means the theme's own accent. "Auto" is never stored here: that
    /// is [`auto_accent`](Self::auto_accent).
    pub selected_accent: Option<String>,
    /// Appearance → Colors → Auto accent from background. While on, the theme's
    /// own accent applies and `selected_accent` is remembered but not used.
    pub auto_accent: bool,
    /// Appearance → Typography → Text size.
    pub font_size: FontSize,
    /// Appearance → Effects → Animations. Silenced by `reduce_motion` whatever
    /// its value; see [`motion::effects_enabled`].
    pub animations: bool,
    /// Accessibility → Vision → High contrast.
    pub high_contrast: bool,
    /// Accessibility → Vision → Dyslexia-friendly font.
    pub dyslexia_font: bool,
    /// Accessibility → Motion → Reduce motion.
    pub reduce_motion: bool,
    /// Accessibility → Reading → Text size. Multiplies with `font_size`.
    pub text_size_a11y: TextSize,
}

impl Default for StandardPreferences {
    fn default() -> Self {
        StandardPreferences {
            selected_theme: DEFAULT_THEME.0.to_string(),
            selected_variant: DEFAULT_THEME.1.to_string(),
            selected_accent: None,
            auto_accent: false,
            font_size: FontSize::Default,
            animations: true,
            high_contrast: false,
            dyslexia_font: false,
            reduce_motion: false,
            text_size_a11y: TextSize::Default,
        }
    }
}

/// The theme a fresh install starts on, and the one an unknown theme falls
/// back to: [`theme::FALLBACK_PALETTE`] is Gruvbox dark.
pub const DEFAULT_THEME: (&str, &str) = ("gruvbox", "dark");

/// One change a shared preferences widget asks for. The host wraps it in its
/// own message and hands it back to [`StandardPreferences::update`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    /// A theme picker card: family and variant keys.
    Theme {
        family: &'static str,
        variant: &'static str,
    },
    /// An accent swatch. Picking one turns auto accent off.
    Accent(&'static str),
    AutoAccent(bool),
    FontSize(FontSize),
    Animations(bool),
    HighContrast(bool),
    DyslexiaFont(bool),
    ReduceMotion(bool),
    TextSize(TextSize),
}

impl StandardPreferences {
    /// Put every preference into effect: the active theme, the accent override,
    /// high contrast, the dyslexia font, reduced motion, effects and both text
    /// sizes. Call it once at startup after loading the config; [`update`]
    /// calls it after every change.
    ///
    /// [`update`]: Self::update
    pub fn apply(&self) {
        theme::set_active_theme(&self.selected_theme, &self.selected_variant);
        theme::set_active_accent(self.accent_override());
        theme::set_high_contrast(self.high_contrast);
        fonts::set_dyslexia_font(self.dyslexia_font);
        motion::set_reduced_motion(self.reduce_motion);
        motion::set_effects(self.animations);
        typography::set_font_size(self.font_size);
        typography::set_text_size(self.text_size_a11y);
    }

    /// Record a change and apply it. The host then saves its config: there is
    /// no Save button, a change applies the moment it is made.
    pub fn update(&mut self, change: Change) {
        match change {
            Change::Theme { family, variant } => {
                self.selected_theme = family.to_string();
                self.selected_variant = variant.to_string();
            }
            Change::Accent(key) => {
                self.selected_accent = Some(key.to_string());
                self.auto_accent = false;
            }
            Change::AutoAccent(on) => self.auto_accent = on,
            Change::FontSize(size) => self.font_size = size,
            Change::Animations(on) => self.animations = on,
            Change::HighContrast(on) => self.high_contrast = on,
            Change::DyslexiaFont(on) => self.dyslexia_font = on,
            Change::ReduceMotion(on) => self.reduce_motion = on,
            Change::TextSize(size) => self.text_size_a11y = size,
        }
        self.apply();
    }

    /// The accent override in effect: `None` while auto accent is on, when no
    /// accent was picked, or when the stored key is not a shipped accent.
    pub fn accent_override(&self) -> Option<Color> {
        if self.auto_accent {
            return None;
        }
        self.selected_accent
            .as_deref()
            .and_then(theme::accent_key_to_color)
    }

    /// The accent key the accent picker marks, or `None` for the theme's own.
    pub fn picked_accent(&self) -> Option<&str> {
        if self.auto_accent {
            None
        } else {
            self.selected_accent.as_deref()
        }
    }
}

impl<'de> Deserialize<'de> for StandardPreferences {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Every field optional and individually lenient, then laid over the
        // defaults. A field that fails to parse is a field the user did not
        // set, not a reason to throw away the whole file.
        #[derive(Default, Deserialize)]
        #[serde(default)]
        struct Raw {
            #[serde(deserialize_with = "lenient")]
            selected_theme: Option<String>,
            #[serde(deserialize_with = "lenient")]
            selected_variant: Option<String>,
            #[serde(deserialize_with = "lenient")]
            selected_accent: Option<Option<String>>,
            #[serde(deserialize_with = "lenient")]
            auto_accent: Option<bool>,
            #[serde(deserialize_with = "lenient")]
            font_size: Option<FontSize>,
            #[serde(deserialize_with = "lenient")]
            animations: Option<bool>,
            #[serde(deserialize_with = "lenient")]
            high_contrast: Option<bool>,
            #[serde(deserialize_with = "lenient")]
            dyslexia_font: Option<bool>,
            #[serde(deserialize_with = "lenient")]
            reduce_motion: Option<bool>,
            #[serde(deserialize_with = "lenient")]
            text_size_a11y: Option<TextSize>,
        }

        let raw = lenient::<_, Raw>(deserializer)?.unwrap_or_default();
        let d = StandardPreferences::default();
        Ok(StandardPreferences {
            selected_theme: raw.selected_theme.unwrap_or(d.selected_theme),
            selected_variant: raw.selected_variant.unwrap_or(d.selected_variant),
            selected_accent: raw.selected_accent.unwrap_or(d.selected_accent),
            auto_accent: raw.auto_accent.unwrap_or(d.auto_accent),
            font_size: raw.font_size.unwrap_or(d.font_size),
            animations: raw.animations.unwrap_or(d.animations),
            high_contrast: raw.high_contrast.unwrap_or(d.high_contrast),
            dyslexia_font: raw.dyslexia_font.unwrap_or(d.dyslexia_font),
            reduce_motion: raw.reduce_motion.unwrap_or(d.reduce_motion),
            text_size_a11y: raw.text_size_a11y.unwrap_or(d.text_size_a11y),
        })
    }
}

/// Parse a `T`, or `None` if the value is anything else. The value is consumed
/// either way, so the document around it still parses. Needs a self-describing
/// format (JSON, TOML, YAML, RON), which is what a config file is.
fn lenient<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Loose<T> {
        Valid(T),
        Invalid(IgnoredAny),
    }

    Ok(match Loose::<T>::deserialize(deserializer)? {
        Loose::Valid(value) => Some(value),
        Loose::Invalid(_) => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(json: &str) -> StandardPreferences {
        serde_json::from_str(json).expect("lenient parsing never fails on a value")
    }

    #[test]
    fn an_empty_file_is_the_defaults() {
        assert_eq!(parse("{}"), StandardPreferences::default());
    }

    #[test]
    fn defaults_are_the_documented_ones() {
        let d = StandardPreferences::default();
        assert_eq!(
            (d.selected_theme.as_str(), d.selected_variant.as_str()),
            DEFAULT_THEME
        );
        assert_eq!(d.selected_accent, None, "the theme's own accent");
        assert!(!d.auto_accent, "auto accent is off by default");
        assert!(d.animations);
        assert!(!d.high_contrast && !d.dyslexia_font && !d.reduce_motion);
        assert_eq!(d.font_size, FontSize::Default);
        assert_eq!(d.text_size_a11y, TextSize::Default);
    }

    #[test]
    fn reads_what_colony_already_writes() {
        let p = parse(
            r#"{
                "selected_section": 2,
                "selected_theme": "catppuccin",
                "selected_variant": "mocha",
                "selected_accent": "violet",
                "auto_accent": false,
                "font_size": "large",
                "animations": false,
                "high_contrast": true,
                "text_size_a11y": "xlarge",
                "reduce_motion": true,
                "keyboard_nav": true,
                "dyslexia_font": true
            }"#,
        );
        assert_eq!(p.selected_theme, "catppuccin");
        assert_eq!(p.selected_variant, "mocha");
        assert_eq!(p.selected_accent.as_deref(), Some("violet"));
        assert_eq!(p.font_size, FontSize::Large);
        assert!(!p.animations);
        assert!(p.high_contrast && p.reduce_motion && p.dyslexia_font);
        assert_eq!(p.text_size_a11y, TextSize::XLarge);
    }

    #[test]
    fn a_bad_value_falls_back_and_keeps_the_rest() {
        let p = parse(
            r#"{
                "selected_theme": 42,
                "selected_accent": ["violet"],
                "font_size": "enormous",
                "text_size_a11y": null,
                "high_contrast": "yes",
                "animations": false,
                "reduce_motion": true
            }"#,
        );
        let d = StandardPreferences::default();
        assert_eq!(p.selected_theme, d.selected_theme);
        assert_eq!(p.selected_accent, None);
        assert_eq!(p.font_size, FontSize::Default);
        assert_eq!(p.text_size_a11y, TextSize::Default);
        assert!(!p.high_contrast);
        // The valid neighbours survive.
        assert!(!p.animations);
        assert!(p.reduce_motion);
    }

    #[test]
    fn not_even_an_object_is_still_the_defaults() {
        assert_eq!(parse("[1, 2, 3]"), StandardPreferences::default());
        assert_eq!(parse("\"gruvbox\""), StandardPreferences::default());
    }

    #[test]
    fn it_round_trips_and_flattens_into_a_program_config() {
        let _guard = crate::test_lock();

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Config {
            #[serde(flatten)]
            standard: StandardPreferences,
            language: String,
        }

        let mut standard = StandardPreferences::default();
        standard.update(Change::Accent("amber"));
        standard.update(Change::TextSize(TextSize::Large));
        let config = Config {
            standard,
            language: "fr".into(),
        };

        StandardPreferences::default().apply();

        let json = serde_json::to_string(&config).unwrap();
        let back: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(back, config);

        // A bad value inside a flattened struct does not take the host's own
        // fields down with it.
        let back: Config = serde_json::from_str(
            r#"{ "font_size": 7, "language": "fr", "selected_theme": "nord" }"#,
        )
        .unwrap();
        assert_eq!(back.language, "fr");
        assert_eq!(back.standard.selected_theme, "nord");
        assert_eq!(back.standard.font_size, FontSize::Default);
    }

    #[test]
    fn auto_is_a_behaviour_never_a_stored_colour() {
        let _guard = crate::test_lock();

        let mut p = StandardPreferences::default();
        p.update(Change::Accent("violet"));
        assert_eq!(p.picked_accent(), Some("violet"));
        assert!(p.accent_override().is_some());

        p.update(Change::AutoAccent(true));
        assert_eq!(p.accent_override(), None);
        assert_eq!(p.picked_accent(), None);
        // The pick is remembered for when auto goes off again.
        assert_eq!(p.selected_accent.as_deref(), Some("violet"));

        // Picking a swatch is an explicit choice, so it ends auto.
        p.update(Change::Accent("green"));
        assert!(!p.auto_accent);

        // An accent this build does not ship is the theme's own, not a crash.
        p.selected_accent = Some("auto".into());
        assert_eq!(p.accent_override(), None);

        StandardPreferences::default().apply();
    }

    #[test]
    fn apply_sets_every_global() {
        let _guard = crate::test_lock();

        let p = StandardPreferences {
            selected_theme: "catppuccin".into(),
            selected_variant: "latte".into(),
            selected_accent: Some("violet".into()),
            auto_accent: false,
            font_size: FontSize::Large,
            animations: true,
            high_contrast: true,
            dyslexia_font: true,
            reduce_motion: true,
            text_size_a11y: TextSize::XLarge,
        };
        p.apply();

        assert_eq!(
            theme::active_palette(),
            theme::ThemePalette::CATPPUCCIN_LATTE.with_high_contrast()
        );
        assert_eq!(
            theme::effective_accent(),
            theme::accent_key_to_color("violet").unwrap()
        );
        assert!(theme::is_high_contrast());
        assert!(fonts::is_dyslexia_font());
        assert!(motion::is_reduced_motion());
        assert!(!motion::effects_enabled(), "reduced motion wins");
        assert_eq!(typography::font_size(), FontSize::Large);
        assert_eq!(typography::text_size(), TextSize::XLarge);

        StandardPreferences::default().apply();
        assert!(!theme::is_high_contrast());
        assert!(!fonts::is_dyslexia_font());
        assert!(motion::effects_enabled());
        assert_eq!(theme::active_accent(), None);
        assert_eq!(typography::font_scale(), 1.0);
    }

    #[test]
    fn an_unknown_theme_is_kept_but_renders_the_fallback() {
        let _guard = crate::test_lock();

        let p = parse(r#"{ "selected_theme": "from_the_future", "selected_variant": "x" }"#);
        assert_eq!(p.selected_theme, "from_the_future");
        p.apply();
        assert_eq!(theme::active_palette(), theme::FALLBACK_PALETTE);
        StandardPreferences::default().apply();
    }
}
