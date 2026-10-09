use iced::widget::{button, row, text};
use iced::{Alignment, Border, Color, Element, Length, Theme};

use super::icons;
use crate::theme::{contrast_on, Palette};
use crate::{fonts, Typography};

/// The program's name in the top-left corner: the way into Preferences.
///
/// design/navigation.md: the identity element is a button, it shows whether
/// Preferences is open, and the same control closes what it opened. So the host
/// passes one toggle message here and the very same one to
/// [`preferences_page`](super::preferences_page)'s Close button.
///
/// Drawn the way Colony's sidebar draws it: the name in bold at `sz(size)`
/// (Colony uses 30 in a sidebar; a tab bar wants something nearer 15), then the
/// gear glyph at `sz(14)`. While open, the gear brightens from `text_dimmer` to
/// `text_primary` and the button sits on `bg_selected`. Padding `[4, 8]`,
/// radius 8.
///
/// This is the **direct** shape: one click, straight to the page. A program
/// with several top-level destinations behind its name uses a menu instead,
/// with Preferences as one entry; a menu holding Preferences alone is the one
/// thing not to build.
pub fn identity_button<'a, M>(
    typo: &Typography,
    name: &str,
    size: u16,
    open: bool,
    on_toggle: M,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let content = row![
        text(name.to_string())
            .size(typo.sz(size))
            .font(typo.bold)
            .color(Palette::TEXT_PRIMARY()),
        text(icons::GEAR)
            .size(typo.sz(14))
            .font(fonts::glyph_font())
            .color(if open {
                Palette::TEXT_PRIMARY()
            } else {
                Palette::TEXT_DIMMER()
            }),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    button(content)
        .on_press(on_toggle)
        .padding([4, 8])
        .style(move |_theme, status| {
            let bg = match status {
                button::Status::Hovered => Palette::BG_CARD_HOVER(),
                _ if open => Palette::BG_SELECTED(),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(bg.into()),
                text_color: Palette::TEXT_PRIMARY(),
                border: Border::default().rounded(8),
                ..Default::default()
            }
        })
        .into()
}

/// The style of anything selectable in a list: a sidebar section, a
/// Preferences category. Selection is marked with **background**, never with a
/// coloured label (design/navigation.md):
///
/// - selected: `accent` background, text in [`contrast_on`] of that accent
/// - hovered: `bg_card_hover` background
/// - neither: transparent background, `text_muted` text
///
/// Radius 8. Selected text is `contrast_on(accent)` rather than
/// `text_primary`: on an accent fill, `text_primary` is below 4.5:1 on almost
/// every theme and reaches 1.01:1 on Ayu Dark, where the selected item was
/// invisible.
///
/// Exposed so a program whose items carry more than a label (an icon, a count)
/// can style its own button the same way.
pub fn selection_style(selected: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, status| {
        let accent = Palette::ACCENT();
        let (background, text_color) = match status {
            _ if selected => (accent, contrast_on(accent)),
            button::Status::Hovered | button::Status::Pressed => {
                (Palette::BG_CARD_HOVER(), Palette::TEXT_MUTED())
            }
            _ => (Color::TRANSPARENT, Palette::TEXT_MUTED()),
        };
        button::Style {
            background: Some(background.into()),
            text_color,
            border: Border::default().rounded(8),
            ..Default::default()
        }
    }
}

/// One selectable entry of a list: a sidebar section or a Preferences category.
/// Full width, padding `[8, 14]`, label at `sz(13)`, styled by
/// [`selection_style`].
pub fn nav_item<'a, M>(
    typo: &Typography,
    label: &str,
    selected: bool,
    on_press: M,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    button(text(label.to_string()).size(typo.sz(13)).font(typo.regular))
        .on_press(on_press)
        .padding([8, 14])
        .width(Length::Fill)
        .style(selection_style(selected))
        .into()
}

/// The label above a sidebar's section list. Names the list, is not clickable:
/// `sz(13)`, `text_muted`.
pub fn nav_label<'a, M: 'a>(typo: &Typography, label: &str) -> Element<'a, M> {
    text(label.to_string())
        .size(typo.sz(13))
        .font(typo.regular)
        .color(Palette::TEXT_MUTED())
        .into()
}

/// The keyboard-shortcut hint under a sidebar's section list: `sz(10)`,
/// `text_dimmest`, deliberately the quietest text in the window.
pub fn nav_hint<'a, M: 'a>(typo: &Typography, hint: &str) -> Element<'a, M> {
    text(hint.to_string())
        .size(typo.sz(10))
        .font(typo.regular)
        .color(Palette::TEXT_DIMMEST())
        .into()
}
