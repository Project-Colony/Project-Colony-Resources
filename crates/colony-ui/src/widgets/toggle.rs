use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::Palette;
use crate::Typography;

/// A labelled on/off row: title, description underneath, pill toggle at the
/// right. The whole row is clickable, not just the pill.
///
/// The description is not the title said twice — it says what turning this on
/// actually does. A setting that needs a restart says so here.
pub fn functional_toggle<'a, M>(
    typo: &Typography,
    title: &str,
    description: &str,
    on: bool,
    on_toggle: M,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let track = if on {
        Palette::ACCENT()
    } else {
        Palette::BG_CARD_HOVER()
    };
    let knob_offset: f32 = if on { 16.0 } else { 2.0 };

    let knob = container(text(""))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
        .style(|_theme| container::Style {
            background: Some(Palette::TEXT_PRIMARY().into()),
            border: Border::default().rounded(7),
            ..Default::default()
        });

    let pill = container(container(knob).padding(Padding {
        top: 1.0,
        right: 0.0,
        bottom: 0.0,
        left: knob_offset,
    }))
    .width(Length::Fixed(34.0))
    .height(Length::Fixed(18.0))
    .style(move |_theme| container::Style {
        background: Some(track.into()),
        border: Border::default().rounded(9),
        ..Default::default()
    });

    button(
        row![
            column![
                text(title.to_string())
                    .size(typo.sz(13))
                    .font(typo.regular)
                    .color(Palette::TEXT_PRIMARY()),
                text(description.to_string())
                    .size(typo.sz(11))
                    .font(typo.regular)
                    .color(Palette::TEXT_DIMMER()),
            ]
            .spacing(2),
            container(text("")).width(Length::Fill),
            pill,
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .on_press(on_toggle)
    .padding([6, 4])
    .width(Length::Fill)
    .style(|_theme, status| {
        let bg = match status {
            button::Status::Hovered => Palette::BG_CARD_HOVER(),
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: Some(bg.into()),
            text_color: Palette::TEXT_PRIMARY(),
            border: Border::default().rounded(6),
            ..Default::default()
        }
    })
    .into()
}
