//! The three fonts, and the one accessor every widget gets its font from.
//!
//! Three, and only three (design/typography.md):
//!
//! | Role | Family | Used for |
//! |---|---|---|
//! | Application | [`APP_FAMILY`] | all UI text, in Regular, Medium and Bold |
//! | Accessibility | [`DYSLEXIA_FAMILY`] | replaces the application font when the user asks |
//! | Icons | [`ICON_FAMILY`] | glyphs outside the Nerd Font range |
//!
//! The dyslexia font is a whole-application swap, not a per-widget option. A
//! program that hardcodes the application font anywhere shows a seam the moment
//! the user turns it on, so every lookup goes through [`ui_font`],
//! [`ui_font_medium`] or [`ui_font_bold`]. There is deliberately no accessor
//! for any other weight: a weight the family does not ship gets synthesized,
//! and it looks wrong next to the real ones.
//!
//! With the `fonts` feature (on by default) the files themselves are embedded
//! too, so a program registers them with iced at startup:
//!
//! ```no_run
//! # #[derive(Debug, Clone)] enum Message {}
//! # fn update(_: &mut (), _: Message) {}
//! # fn view(_: &()) -> iced::Element<'_, Message> { iced::widget::text("").into() }
//! colony_ui::fonts::BYTES
//!     .into_iter()
//!     .fold(iced::application(|| (), update, view), |app, font| app.font(font))
//!     .default_font(colony_ui::fonts::ui_font())
//!     .run()
//! # .unwrap();
//! ```

use std::sync::atomic::{AtomicBool, Ordering};

use iced::font::Weight;
use iced::Font;

/// The application font, for all UI text.
pub const APP_FAMILY: &str = "JetBrainsMono Nerd Font";

/// The dyslexia-friendly font that replaces [`APP_FAMILY`] everywhere when the
/// user turns it on.
pub const DYSLEXIA_FAMILY: &str = "OpenDyslexic";

/// The icon font, for glyphs the Nerd Font does not have.
pub const ICON_FAMILY: &str = "Font Awesome 6 Free";

/// Every font file, ready for `iced::Application::font` or
/// `iced::font::load`: JetBrainsMono Nerd Font Regular, Medium and Bold,
/// OpenDyslexic Regular, Font Awesome 6 Free Solid and Regular.
///
/// The licences travel with the crate, in `assets/fonts/` next to the files.
#[cfg(feature = "fonts")]
pub const BYTES: [&[u8]; 6] = [
    include_bytes!("../assets/fonts/JetBrainsMonoNerdFont/JetBrainsMonoNerdFont-Regular.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMonoNerdFont/JetBrainsMonoNerdFont-Medium.ttf"),
    include_bytes!("../assets/fonts/JetBrainsMonoNerdFont/JetBrainsMonoNerdFont-Bold.ttf"),
    include_bytes!("../assets/fonts/OpenDyslexic/OpenDyslexic-Regular.otf"),
    include_bytes!("../assets/fonts/FontAwesome/fa-solid-900.ttf"),
    include_bytes!("../assets/fonts/FontAwesome/fa-regular-400.ttf"),
];

static DYSLEXIA: AtomicBool = AtomicBool::new(false);

/// Swap the whole interface to the dyslexia-friendly font, or back.
pub fn set_dyslexia_font(enabled: bool) {
    DYSLEXIA.store(enabled, Ordering::Relaxed);
}

/// Whether the dyslexia-friendly font is on.
pub fn is_dyslexia_font() -> bool {
    DYSLEXIA.load(Ordering::Relaxed)
}

fn family() -> &'static str {
    if is_dyslexia_font() {
        DYSLEXIA_FAMILY
    } else {
        APP_FAMILY
    }
}

fn with_weight(weight: Weight) -> Font {
    Font {
        weight,
        ..Font::with_name(family())
    }
}

/// The interface font, Regular. OpenDyslexic while the dyslexia font is on.
pub fn ui_font() -> Font {
    with_weight(Weight::Normal)
}

/// The interface font, Medium. OpenDyslexic ships one weight, so with the
/// dyslexia font on this renders as its Regular.
pub fn ui_font_medium() -> Font {
    with_weight(Weight::Medium)
}

/// The interface font, Bold. OpenDyslexic ships one weight, so with the
/// dyslexia font on this renders as its Regular.
pub fn ui_font_bold() -> Font {
    with_weight(Weight::Bold)
}

/// The font a Nerd Font glyph (`"\u{f013}"`, a family's picker icon) is drawn
/// in: JetBrainsMono Nerd Font Regular, **whatever the dyslexia toggle says**.
///
/// OpenDyslexic has none of these glyphs, so a glyph drawn in [`ui_font`]
/// while it is on is left to the text engine's fallback, which may find it in
/// a different font or not at all. Draw glyphs in this and they render the
/// same either way. Every glyph colony-ui draws is checked against the
/// embedded file by `cargo test`.
pub fn glyph_font() -> Font {
    Font::with_name(APP_FAMILY)
}

/// Font Awesome, Solid: the filled glyphs, and the only style that has most of
/// them.
pub fn icon_font() -> Font {
    Font {
        weight: Weight::Black,
        ..Font::with_name(ICON_FAMILY)
    }
}

/// Font Awesome, Regular: the outlined variants of the glyphs that have one.
pub fn icon_font_regular() -> Font {
    Font::with_name(ICON_FAMILY)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::font::Family;

    #[test]
    fn the_dyslexia_toggle_swaps_every_weight() {
        let _guard = crate::test_lock();

        set_dyslexia_font(false);
        for font in [ui_font(), ui_font_medium(), ui_font_bold()] {
            assert_eq!(font.family, Family::Name(APP_FAMILY));
        }
        assert_eq!(ui_font_bold().weight, Weight::Bold);

        set_dyslexia_font(true);
        for font in [ui_font(), ui_font_medium(), ui_font_bold()] {
            assert_eq!(font.family, Family::Name(DYSLEXIA_FAMILY));
        }
        set_dyslexia_font(false);
    }

    #[test]
    fn icons_never_follow_the_dyslexia_toggle() {
        let _guard = crate::test_lock();

        set_dyslexia_font(true);
        assert_eq!(icon_font().family, Family::Name(ICON_FAMILY));
        assert_eq!(glyph_font().family, Family::Name(APP_FAMILY));
        assert_eq!(glyph_font().weight, Weight::Normal);
        set_dyslexia_font(false);
    }

    /// design/typography.md: a codepoint that is not in JetBrainsMono Nerd
    /// Font is a tofu box on every user's machine. Holds every glyph the
    /// widgets draw and every theme family's picker icon to the embedded file,
    /// so a new family with a bad codepoint fails here rather than on screen.
    #[cfg(feature = "fonts")]
    #[test]
    fn every_glyph_colony_ui_draws_is_in_the_nerd_font() {
        use crate::widgets::icons;

        let face = ttf_parser::Face::parse(BYTES[0], 0).expect("the Regular file parses");
        let glyphs = [
            ("CHEVRON_DOWN", icons::CHEVRON_DOWN),
            ("CHEVRON_RIGHT", icons::CHEVRON_RIGHT),
            ("CHECK", icons::CHECK),
            ("GEAR", icons::GEAR),
            ("CLOSE", icons::CLOSE),
        ]
        .into_iter()
        .chain(
            crate::THEME_FAMILIES
                .iter()
                .filter(|family| !family.icon.is_empty())
                .map(|family| (family.key, family.icon)),
        );
        for (name, glyph) in glyphs {
            for c in glyph.chars() {
                assert!(
                    face.glyph_index(c).is_some(),
                    "{name}: U+{:04X} is not in {APP_FAMILY}",
                    c as u32
                );
            }
        }
    }

    /// The bytes are what iced is handed: each file has to be a real font, not
    /// a Git LFS pointer or an HTML error page that happened to be saved.
    #[cfg(feature = "fonts")]
    #[test]
    fn every_embedded_file_is_a_font() {
        for bytes in BYTES {
            let magic = &bytes[..4];
            assert!(
                magic == [0, 1, 0, 0] || magic == b"OTTO",
                "not a TrueType or OpenType file: {magic:?}"
            );
            assert!(bytes.len() > 10_000);
        }
    }

    /// A font is looked up by typographic family name and weight, so an
    /// accessor that drifts from the file it means silently draws in another
    /// font, or in a weight the text engine synthesizes.
    #[cfg(feature = "fonts")]
    #[test]
    fn the_accessors_name_the_embedded_files() {
        use ttf_parser::name_id::TYPOGRAPHIC_FAMILY;

        let _guard = crate::test_lock();
        set_dyslexia_font(false);
        let expected = [
            ui_font(),
            ui_font_medium(),
            ui_font_bold(),
            Font {
                weight: Weight::Normal,
                ..Font::with_name(DYSLEXIA_FAMILY)
            },
            icon_font(),
            icon_font_regular(),
        ];
        for (bytes, font) in BYTES.into_iter().zip(expected) {
            let face = ttf_parser::Face::parse(bytes, 0).expect("every file parses");
            let iced::font::Family::Name(family) = font.family else {
                panic!("{font:?} is not a named family");
            };
            assert!(
                face.names()
                    .into_iter()
                    .any(|name| name.name_id == TYPOGRAPHIC_FAMILY
                        && name.to_string().as_deref() == Some(family)),
                "no embedded file is {family}"
            );
            let weight = match font.weight {
                Weight::Normal => 400,
                Weight::Medium => 500,
                Weight::Bold => 700,
                Weight::Black => 900,
                other => panic!("{other:?} is not a weight colony-ui uses"),
            };
            assert_eq!(face.weight().to_number(), weight, "{family} {weight}");
        }
    }
}
