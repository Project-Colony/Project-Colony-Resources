use iced::widget::{button, container, text, Row};
use iced::{Alignment, Border, Color, Element, Length};

use super::icons;
use crate::theme::{hex, Palette, ACCENT_OVERRIDES};
use crate::Typography;

/// The row of accent swatches, rendered from [`ACCENT_OVERRIDES`].
///
/// `selected` is the accent key the user picked, or `None` for auto — in which
/// case no swatch is marked and the theme's own accent applies.
///
/// This is *not* the "auto accent from background" toggle, which is a separate
/// behaviour the host draws with
/// [`functional_toggle`](super::functional_toggle). Two different notions of
/// auto, easy to conflate.
pub fn accent_picker<'a, M, F>(
    typo: &Typography,
    selected: Option<&str>,
    on_select: F,
) -> Element<'a, M>
where
    M: Clone + 'a,
    F: Fn(&'static str) -> M + 'a,
{
    let mut row = Row::new().spacing(8).align_y(Alignment::Center);

    for accent in ACCENT_OVERRIDES {
        let is_active = selected == Some(accent.key);
        let dot = hex(accent.color);

        let check: Element<'a, M> = if is_active {
            text(icons::CHECK)
                .size(typo.sz(8))
                .font(typo.regular)
                .color(Color::WHITE)
                .into()
        } else {
            text("").size(typo.sz(8)).into()
        };

        let swatch = button(
            container(check)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .on_press(on_select(accent.key))
        .width(Length::Fixed(28.0))
        .height(Length::Fixed(28.0))
        .padding(0)
        .style(move |_theme, status| {
            let border_color = match status {
                _ if is_active => Palette::TEXT_PRIMARY(),
                button::Status::Hovered => Palette::TEXT_DIMMER(),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(dot.into()),
                text_color: Color::WHITE,
                border: Border {
                    color: border_color,
                    width: if is_active { 2.0 } else { 0.0 },
                    radius: 14.0.into(),
                },
                ..Default::default()
            }
        });

        row = row.push(swatch);
    }

    row.into()
}
