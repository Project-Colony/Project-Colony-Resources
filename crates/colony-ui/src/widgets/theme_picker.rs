use iced::widget::{button, column, container, row, text, Column, Row};
use iced::{Alignment, Border, Element, Length, Padding};

use super::icons;
use crate::theme::{contrast_on, Palette, THEME_FAMILIES};
use crate::{i18n, Typography};

/// The whole theme picker: one row per family, each showing the family's Nerd
/// Font glyph and localized name above a row of variant cards.
///
/// It renders straight from [`THEME_FAMILIES`], which is generated from
/// `tokens/`. **Adding a theme family requires no change here and no change in
/// the host program** — that is the entire point of the crate.
///
/// A card is the variant's `swatch.bg` with a bar of its `swatch.accent` across
/// it, the localized variant name underneath, and a check on the selected one.
pub fn theme_picker<'a, M, F>(
    typo: &Typography,
    selected_family: &str,
    selected_variant: &str,
    on_select: F,
) -> Element<'a, M>
where
    M: Clone + 'a,
    F: Fn(&'static str, &'static str) -> M + 'a,
{
    let mut col = Column::new().spacing(14);

    for family in THEME_FAMILIES {
        let is_selected_family = selected_family == family.key;

        let label = i18n::t(family.label_key);
        let label_text = if family.icon.is_empty() {
            label.to_string()
        } else {
            format!("{} {}", family.icon, label)
        };

        let family_label =
            text(label_text)
                .size(typo.sz(13))
                .font(typo.medium)
                .color(if is_selected_family {
                    Palette::TEXT_PRIMARY()
                } else {
                    Palette::TEXT_SECONDARY()
                });

        let mut variants = Row::new().spacing(8);
        for variant in family.variants {
            let is_active = is_selected_family && selected_variant == variant.key;
            variants = variants.push(variant_card(
                typo,
                family.key,
                variant,
                is_active,
                on_select(family.key, variant.key),
            ));
        }

        col = col.push(column![family_label, variants].spacing(6));
    }

    col.into()
}

fn variant_card<'a, M>(
    typo: &Typography,
    _family_key: &'static str,
    variant: &'static crate::theme::ThemeVariantMeta,
    is_active: bool,
    message: M,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let bg = variant.swatch_bg_color();
    let accent = variant.swatch_accent_color();

    // A bar of the accent, sitting low on a field of the background — enough
    // for the theme to be recognisable at this size.
    let accent_bar = container(text(""))
        .width(Length::Fill)
        .height(Length::Fixed(4.0))
        .style(move |_theme| container::Style {
            background: Some(accent.into()),
            border: Border::default().rounded(2),
            ..Default::default()
        });

    let swatch = container(accent_bar)
        .width(Length::Fill)
        .height(Length::Fixed(28.0))
        .padding(Padding {
            top: 20.0,
            right: 6.0,
            bottom: 4.0,
            left: 6.0,
        })
        .style(move |_theme| container::Style {
            background: Some(bg.into()),
            border: Border::default().rounded(6),
            ..Default::default()
        });

    let name = text(i18n::t(variant.label_key))
        .size(typo.sz(10))
        .font(typo.regular)
        .color(if is_active {
            Palette::TEXT_PRIMARY()
        } else {
            Palette::TEXT_MUTED()
        });

    // The mark sits on this variant's own swatch, so its legibility depends on
    // that swatch rather than on the accent of whatever theme is active — which
    // is what it used to be drawn in.
    let check: Element<'a, M> = if is_active {
        text(icons::CHECK)
            .size(typo.sz(8))
            .font(typo.regular)
            .color(contrast_on(bg))
            .into()
    } else {
        text("").size(typo.sz(8)).into()
    };

    button(
        column![
            swatch,
            container(row![name, check].spacing(4).align_y(Alignment::Center)).padding(Padding {
                top: 4.0,
                right: 0.0,
                bottom: 0.0,
                left: 2.0,
            }),
        ]
        .spacing(0)
        .width(Length::Fill),
    )
    .on_press(message)
    .padding(4)
    .width(Length::Fill)
    .style(move |_theme, status| {
        let border_color = match status {
            _ if is_active => Palette::ACCENT(),
            button::Status::Hovered => Palette::TEXT_DIMMER(),
            _ => Palette::BORDER_SUBTLE(),
        };
        button::Style {
            background: Some(Palette::BG_CARD().into()),
            text_color: Palette::TEXT_PRIMARY(),
            border: Border {
                color: border_color,
                width: if is_active { 2.0 } else { 1.0 },
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    })
    .into()
}
