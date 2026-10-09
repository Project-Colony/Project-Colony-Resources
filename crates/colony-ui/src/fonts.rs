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
        set_dyslexia_font(false);
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
}
