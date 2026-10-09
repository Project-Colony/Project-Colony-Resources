use iced::widget::{button, row, text};
use iced::{Alignment, Border, Element};

use super::icons;
use crate::theme::Palette;
use crate::{fonts, Typography};

/// A dismissible confirmation: the message in `success` on `success_bg`, then
/// a close glyph in `text_dimmer`. The whole toast is the dismiss target.
/// Padding `[8, 16]`, radius 8, as Colony draws its notifications.
///
/// This is what Appearance raises after a theme change (design/settings-page.md):
/// the change was immediate, the toast only makes it noticeable.
///
/// ```
/// # use colony_ui::{i18n, widgets, Typography};
/// # #[derive(Clone)] enum Message { DismissToast }
/// let toast: iced::Element<'_, Message> = widgets::toast(
///     &Typography::current(),
///     i18n::t("theme_applied"),
///     Message::DismissToast,
/// );
/// ```
///
/// The host decides where it sits (Colony stacks them bottom-left, over the
/// page) and removes it on `on_dismiss`. A fade in or out is an effect: run it
/// only while [`effects_enabled`](crate::effects_enabled).
pub fn toast<'a, M>(typo: &Typography, message: &str, on_dismiss: M) -> Element<'a, M>
where
    M: Clone + 'a,
{
    button(
        row![
            text(message.to_string())
                .size(typo.sz(13))
                .font(typo.regular)
                .color(Palette::SUCCESS()),
            text(icons::CLOSE)
                .size(typo.sz(12))
                .font(fonts::glyph_font())
                .color(Palette::TEXT_DIMMER()),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
    )
    .on_press(on_dismiss)
    .padding([8, 16])
    .style(|_theme, _status| button::Style {
        background: Some(Palette::SUCCESS_BG().into()),
        text_color: Palette::TEXT_PRIMARY(),
        border: Border::default().rounded(8),
        ..Default::default()
    })
    .into()
}
