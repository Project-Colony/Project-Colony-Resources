use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use super::icons;
use crate::theme::Palette;
use crate::Typography;

/// A collapsible section: a flat header row that toggles its body.
///
/// This is what keeps a preferences category readable — on arrival the user
/// sees a short list of closed rows rather than every control at once. The
/// header is deliberately **flat**: no card, no background except on hover, so
/// the page reads as a list and not as a stack of boxes.
///
/// The whole header is the target, not just the chevron.
///
/// The host owns the expanded state — usually a `HashSet<String>` of open
/// section keys — because it is the host that persists it.
pub fn collapsible_section<'a, M>(
    typo: &Typography,
    title: &str,
    expanded: bool,
    on_toggle: M,
    content: Element<'a, M>,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let chevron = if expanded {
        icons::CHEVRON_DOWN
    } else {
        icons::CHEVRON_RIGHT
    };

    let header = button(
        row![
            text(title.to_string())
                .size(typo.sz(15))
                .font(typo.bold)
                .color(Palette::TEXT_PRIMARY()),
            container(text("")).width(Length::Fill),
            text(chevron)
                .size(typo.sz(9))
                .font(typo.regular)
                .color(Palette::TEXT_DIMMER()),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .on_press(on_toggle)
    .padding([12, 4])
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
    });

    if !expanded {
        return header.into();
    }

    let divider = container(text(""))
        .width(Length::Fill)
        .height(1)
        .style(|_theme| container::Style {
            background: Some(Palette::DIVIDER().into()),
            ..Default::default()
        });

    let body = container(content)
        .padding(Padding {
            top: 12.0,
            right: 4.0,
            bottom: 4.0,
            left: 4.0,
        })
        .width(Length::Fill);

    column![header, divider, body].spacing(0).into()
}
