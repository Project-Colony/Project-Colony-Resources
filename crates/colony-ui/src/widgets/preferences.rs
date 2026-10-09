use std::collections::HashSet;
use std::fmt;

use iced::widget::overlay::menu;
use iced::widget::{button, column, container, pick_list, row, scrollable, space, text, Column};
use iced::{Alignment, Border, Color, Element, Length, Padding, Shadow};

use super::{accent_picker, collapsible_section, functional_toggle, nav_item, theme_picker};
use crate::preferences::{Change, StandardPreferences};
use crate::theme::{self, contrast_on, Palette};
use crate::typography::{FontSize, TextSize};
use crate::{fonts, i18n, Typography};

/// The i18n keys of the three categories every Preferences page opens with,
/// in their fixed order: General, Appearance, Accessibility. A program's own
/// categories follow them, and [`ABOUT_CATEGORY`] comes last when there is one.
/// Do not reorder, rename or repurpose these three.
pub const STANDARD_CATEGORIES: [&str; 3] = [
    "settings_cat_general",
    "settings_cat_appearance",
    "settings_cat_accessibility",
];

/// The i18n key of About, the last category when a program has one.
pub const ABOUT_CATEGORY: &str = "settings_cat_about";

/// The Preferences page header: the title, a spacer, and Close.
///
/// Title `Preferences` at `sz(22)` bold `text_primary`. Close at `sz(13)`
/// `text_muted`, padding `[6, 14]`, radius 6, `bg_card_hover` on hover.
/// `on_close` must be the same message the
/// [`identity_button`](super::identity_button) sends: two ways out, one
/// message.
pub fn preferences_header<'a, M>(typo: &Typography, on_close: M) -> Element<'a, M>
where
    M: Clone + 'a,
{
    row![
        text(i18n::t("settings_title"))
            .size(typo.sz(22))
            .font(typo.bold)
            .color(Palette::TEXT_PRIMARY()),
        space().width(Length::Fill),
        button(
            text(i18n::t("settings_close"))
                .size(typo.sz(13))
                .font(typo.regular)
        )
        .on_press(on_close)
        .padding([6, 14])
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => Palette::BG_CARD_HOVER(),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(bg.into()),
                text_color: Palette::TEXT_MUTED(),
                border: Border::default().rounded(6),
                ..Default::default()
            }
        }),
    ]
    .align_y(Alignment::Center)
    .into()
}

/// The whole Preferences page: [`preferences_header`] over the category list
/// on the left and the selected category's content on the right, scrolling.
///
/// It **replaces the content area**: the host shows it where its main content
/// normally goes, and keeps its own chrome (sidebar, top bar) around it. Not a
/// modal, not a window, not a popover.
pub fn preferences_page<'a, M>(
    typo: &Typography,
    categories: impl Into<Element<'a, M>>,
    content: impl Into<Element<'a, M>>,
    on_close: M,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let content = scrollable(container(content).padding(Padding {
        top: 0.0,
        right: 24.0,
        bottom: 24.0,
        left: 0.0,
    }))
    .height(Length::Fill);

    let body = row![categories.into(), container(content).width(Length::Fill)];

    container(
        column![preferences_header(typo, on_close), body]
            .spacing(16)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .style(|_theme| container::Style {
        background: Some(Palette::BG_PRIMARY().into()),
        ..Default::default()
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// The category list on the left of the page: one [`nav_item`] per label,
/// `on_select` receiving the index. Start the labels with
/// [`STANDARD_CATEGORIES`].
///
/// It is 180 wide at a text scale of 1.0 and grows with the scale, so the
/// longest standard label, "Accessibility", fits on one line from 0.7225x to
/// 1.68x, OpenDyslexic included.
///
/// ```
/// # use colony_ui::{i18n, Typography};
/// # use colony_ui::widgets::{category_list, STANDARD_CATEGORIES};
/// # #[derive(Clone)] enum Message { Category(usize) }
/// let labels = STANDARD_CATEGORIES
///     .iter()
///     .map(|key| i18n::t(key))
///     .chain(["Audio", i18n::t("settings_cat_about")]);
/// let list: iced::Element<'_, Message> =
///     category_list(&Typography::current(), labels, 0, Message::Category);
/// ```
pub fn category_list<'a, M, L>(
    typo: &Typography,
    labels: impl IntoIterator<Item = L>,
    selected: usize,
    on_select: impl Fn(usize) -> M,
) -> Element<'a, M>
where
    M: Clone + 'a,
    L: AsRef<str>,
{
    let items = labels
        .into_iter()
        .enumerate()
        .map(|(i, label)| nav_item(typo, label.as_ref(), i == selected, on_select(i)));

    container(Column::with_children(items).spacing(2))
        .width(Length::Fixed(CATEGORY_LIST_WIDTH * typo.scale))
        .padding(Padding {
            top: 0.0,
            right: 16.0,
            bottom: 0.0,
            left: 0.0,
        })
        .into()
}

/// The category list's width at a text scale of 1.0. OpenDyslexic sets
/// "Accessibility" at 9.3 em; at `sz(13)` that plus the item's padding and the
/// list's gutter needs 165.
const CATEGORY_LIST_WIDTH: f32 = 180.0;

/// What every category opens with: its heading at `sz(18)` bold and a
/// one-line description under it in `text_muted`.
///
/// Returns the column the category's sections go into, so the host pushes them
/// straight after:
///
/// ```
/// # use colony_ui::{i18n, Typography};
/// # use colony_ui::widgets::category_heading;
/// # #[derive(Clone)] enum Message {}
/// # let section: iced::Element<'_, Message> = iced::widget::text("").into();
/// let page: iced::widget::Column<'_, Message> =
///     category_heading(&Typography::current(), "Storage", "Where files go.").push(section);
/// ```
pub fn category_heading<'a, M: 'a>(
    typo: &Typography,
    title: &str,
    description: &str,
) -> Column<'a, M> {
    let heading = column![
        text(title.to_string())
            .size(typo.sz(18))
            .font(typo.bold)
            .color(Palette::TEXT_PRIMARY()),
        text(description.to_string())
            .size(typo.sz(12))
            .font(typo.regular)
            .color(Palette::TEXT_MUTED()),
    ]
    .spacing(4)
    .padding(Padding {
        top: 0.0,
        right: 0.0,
        bottom: 14.0,
        left: 0.0,
    });

    column![heading].spacing(6)
}

/// The first line inside a section: what the section's settings do, in
/// `text_muted` at `sz(12)`. It explains the consequence, not the name again,
/// and a setting that needs a restart says so here.
pub fn section_description<'a, M: 'a>(typo: &Typography, description: &str) -> Element<'a, M> {
    container(
        text(description.to_string())
            .size(typo.sz(12))
            .font(typo.regular)
            .color(Palette::TEXT_MUTED()),
    )
    .padding(Padding {
        top: 0.0,
        right: 0.0,
        bottom: 12.0,
        left: 0.0,
    })
    .into()
}

/// A setting with a control on the right: title at `sz(13)` `text_primary`,
/// description under it at `sz(11)` `text_dimmer`, then the control.
pub fn setting_row<'a, M: 'a>(
    typo: &Typography,
    title: &str,
    description: &str,
    control: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
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
        .spacing(2)
        .width(Length::Fill),
        control.into(),
    ]
    .spacing(10)
    .padding([6, 4])
    .align_y(Alignment::Center)
    .into()
}

/// A drop-down styled from the palette: `bg_card` field, `border_subtle` edge,
/// the open list's highlighted row on the accent with [`contrast_on`] text.
///
/// `options` pairs each value with its display label; `on_select` receives the
/// value, never the label, so a translated label can never end up in a config
/// file.
pub fn dropdown<'a, T, M>(
    typo: &Typography,
    options: impl IntoIterator<Item = (T, String)>,
    selected: &T,
    on_select: impl Fn(T) -> M + 'a,
) -> Element<'a, M>
where
    T: Clone + PartialEq + 'a,
    M: Clone + 'a,
{
    let options: Vec<Choice<T>> = options
        .into_iter()
        .map(|(value, label)| Choice { value, label })
        .collect();
    let current = options.iter().find(|c| &c.value == selected).cloned();

    pick_list(options, current, move |choice: Choice<T>| {
        on_select(choice.value)
    })
    .text_size(typo.sz(12))
    .font(typo.regular)
    .padding([4, 10])
    .style(|_theme, status| {
        let background = match status {
            pick_list::Status::Active => Palette::BG_CARD(),
            pick_list::Status::Hovered | pick_list::Status::Opened { .. } => {
                Palette::BG_CARD_HOVER()
            }
        };
        pick_list::Style {
            text_color: Palette::TEXT_SECONDARY(),
            placeholder_color: Palette::TEXT_DIMMER(),
            handle_color: Palette::TEXT_DIMMER(),
            background: background.into(),
            border: Border {
                color: Palette::BORDER_SUBTLE(),
                width: 1.0,
                radius: 6.0.into(),
            },
        }
    })
    .menu_style(|_theme| menu::Style {
        background: Palette::BG_CARD().into(),
        border: Border {
            color: Palette::BORDER_SUBTLE(),
            width: 1.0,
            radius: 6.0.into(),
        },
        text_color: Palette::TEXT_PRIMARY(),
        selected_text_color: contrast_on(Palette::ACCENT()),
        selected_background: Palette::ACCENT().into(),
        shadow: Shadow::default(),
    })
    .into()
}

/// A pick-list entry: compared by value, displayed by label.
#[derive(Debug, Clone)]
struct Choice<T> {
    value: T,
    label: String,
}

impl<T: PartialEq> PartialEq for Choice<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> fmt::Display for Choice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label)
    }
}

// Section keys, the same ones Colony keeps in its expanded-sections set.
const THEME: &str = "theme";
const COLORS: &str = "colors";
const TYPOGRAPHY: &str = "typography";
const EFFECTS: &str = "effects";
const PREVIEW: &str = "preview";
const VISION: &str = "vision";
const MOTION: &str = "motion";
const READING: &str = "reading";

/// The ready-made content of the mandatory categories, built from the user's
/// [`StandardPreferences`].
///
/// - `on_change` wraps a [`Change`] in the host's message; the host hands it
///   back to [`StandardPreferences::update`] and saves.
/// - `on_section` wraps a section key in the host's message; the host toggles
///   it in `expanded`, the set of open sections. Every section arrives closed
///   and stays as the user left it while the page is open.
///
/// ```
/// # use std::collections::HashSet;
/// # use colony_ui::preferences::{Change, StandardPreferences};
/// # use colony_ui::widgets::PreferencesView;
/// # use colony_ui::Typography;
/// #[derive(Debug, Clone)]
/// enum Message {
///     Preference(Change),
///     ToggleSection(&'static str),
/// }
///
/// let prefs = StandardPreferences::default();
/// let expanded = HashSet::new();
/// let view = PreferencesView::new(
///     Typography::current(),
///     &prefs,
///     &expanded,
///     Message::Preference,
///     Message::ToggleSection,
/// );
/// let appearance: iced::Element<'_, Message> = view.appearance().into();
/// ```
///
/// A category the kit does not cover entirely composes the parts: General is
/// [`general`](Self::general) plus the program's own sections, and a program
/// with a Navigation section in Accessibility builds
/// [`accessibility_heading`](Self::accessibility_heading), then Vision, Motion,
/// its Navigation, then Reading.
pub struct PreferencesView<'p, F, S> {
    typo: Typography,
    prefs: &'p StandardPreferences,
    expanded: &'p HashSet<String>,
    on_change: F,
    on_section: S,
}

impl<'p, F, S, M> PreferencesView<'p, F, S>
where
    F: Fn(Change) -> M + Clone,
    S: Fn(&'static str) -> M,
    M: Clone,
{
    pub fn new(
        typo: Typography,
        prefs: &'p StandardPreferences,
        expanded: &'p HashSet<String>,
        on_change: F,
        on_section: S,
    ) -> Self {
        PreferencesView {
            typo,
            prefs,
            expanded,
            on_change,
            on_section,
        }
    }

    /// General's heading and its contract, "Preferences are saved
    /// automatically." Push the program's own sections (startup, updates)
    /// onto it.
    pub fn general<'a>(&self) -> Column<'a, M>
    where
        M: 'a,
    {
        category_heading(
            &self.typo,
            i18n::t("settings_cat_general"),
            i18n::t("settings_general_desc"),
        )
    }

    /// Appearance, complete: Theme, Colors & accents, Typography, Effects,
    /// Preview.
    pub fn appearance<'a>(&self) -> Column<'a, M>
    where
        F: 'a,
        M: 'a,
    {
        category_heading(
            &self.typo,
            i18n::t("settings_cat_appearance"),
            i18n::t("settings_appearance_desc"),
        )
        .push(self.theme_section())
        .push(self.colors_section())
        .push(self.typography_section())
        .push(self.effects_section())
        .push(self.preview_section())
    }

    /// Accessibility, complete: Vision, Motion, Reading.
    pub fn accessibility<'a>(&self) -> Column<'a, M>
    where
        F: 'a,
        M: 'a,
    {
        self.accessibility_heading()
            .push(self.vision_section())
            .push(self.motion_section())
            .push(self.reading_section())
    }

    /// Accessibility's heading alone, for a program that adds sections of its
    /// own between the standard ones.
    pub fn accessibility_heading<'a>(&self) -> Column<'a, M>
    where
        M: 'a,
    {
        category_heading(
            &self.typo,
            i18n::t("settings_cat_accessibility"),
            i18n::t("settings_accessibility_desc"),
        )
    }

    /// Appearance → Theme: the family/variant picker, rendered from
    /// [`THEME_FAMILIES`](crate::THEME_FAMILIES). The host raises its
    /// `theme_applied` toast when it receives [`Change::Theme`].
    pub fn theme_section<'a>(&self) -> Element<'a, M>
    where
        F: 'a,
        M: 'a,
    {
        let on_change = self.on_change.clone();
        let picker = theme_picker(
            &self.typo,
            &self.prefs.selected_theme,
            &self.prefs.selected_variant,
            move |family, variant| on_change(Change::Theme { family, variant }),
        );
        self.section(
            THEME,
            "settings_section_theme",
            "settings_theme_desc",
            vec![picker],
        )
    }

    /// Appearance → Colors & accents: the eight swatches, then the separate
    /// auto accent toggle.
    pub fn colors_section<'a>(&self) -> Element<'a, M>
    where
        F: 'a,
        M: 'a,
    {
        let on_change = self.on_change.clone();
        let swatches = accent_picker(&self.typo, self.prefs.picked_accent(), move |key| {
            on_change(Change::Accent(key))
        });
        let auto = functional_toggle(
            &self.typo,
            i18n::t("settings_auto_accent"),
            i18n::t("settings_auto_accent_desc"),
            self.prefs.auto_accent,
            (self.on_change)(Change::AutoAccent(!self.prefs.auto_accent)),
        );
        self.section(
            COLORS,
            "settings_section_colors",
            "settings_colors_desc",
            vec![swatches, space().height(12).into(), auto],
        )
    }

    /// Appearance → Typography: the font family in use, and the base size.
    ///
    /// The family is shown, not picked: the three-font rule leaves exactly one
    /// choice, the dyslexia-friendly font, and that lives in Accessibility.
    pub fn typography_section<'a>(&self) -> Element<'a, M>
    where
        F: 'a,
        M: 'a,
    {
        let family = if self.prefs.dyslexia_font {
            fonts::DYSLEXIA_FAMILY
        } else {
            fonts::APP_FAMILY
        };
        let family_row = setting_row(
            &self.typo,
            i18n::t("settings_font_family"),
            i18n::t("settings_font_family_desc"),
            text(family)
                .size(self.typo.sz(12))
                .font(self.typo.regular)
                .color(Palette::TEXT_SECONDARY()),
        );

        let on_change = self.on_change.clone();
        let size = setting_row(
            &self.typo,
            i18n::t("settings_font_size"),
            i18n::t("settings_font_size_desc"),
            dropdown(
                &self.typo,
                FontSize::ALL.map(|s| (s, i18n::t(s.label_key()).to_string())),
                &self.prefs.font_size,
                move |s| on_change(Change::FontSize(s)),
            ),
        );

        self.section(
            TYPOGRAPHY,
            "settings_section_typography",
            "settings_typography_desc",
            vec![family_row, size],
        )
    }

    /// Appearance → Effects: the animations toggle. Its description says that
    /// Reduce motion overrides it, which [`effects_enabled`](crate::effects_enabled)
    /// enforces.
    pub fn effects_section<'a>(&self) -> Element<'a, M>
    where
        M: 'a,
    {
        let toggle = functional_toggle(
            &self.typo,
            i18n::t("settings_animations"),
            i18n::t("settings_animations_desc"),
            self.prefs.animations,
            (self.on_change)(Change::Animations(!self.prefs.animations)),
        );
        self.section(
            EFFECTS,
            "settings_section_effects",
            "settings_effects_desc",
            vec![toggle],
        )
    }

    /// Appearance → Preview: a card drawn with the live palette, accent, font
    /// and size, so a theme can be judged without leaving the page.
    pub fn preview_section<'a>(&self) -> Element<'a, M>
    where
        M: 'a,
    {
        let typo = &self.typo;
        let theme_name = match theme::family(&self.prefs.selected_theme) {
            Some(family) => {
                let variant = family
                    .variant(&self.prefs.selected_variant)
                    .map(|v| i18n::t(v.label_key))
                    .unwrap_or_default();
                format!("{} · {}", i18n::t(family.label_key), variant)
            }
            None => self.prefs.selected_theme.clone(),
        };
        let accent_name = self
            .prefs
            .picked_accent()
            .and_then(|key| theme::ACCENT_OVERRIDES.iter().find(|a| a.key == key))
            .map(|a| i18n::t(a.label_key))
            .unwrap_or_else(|| i18n::t("settings_default_accent"));

        let accent = Palette::ACCENT();
        let sample_button = container(
            text(i18n::t("settings_preview_button"))
                .size(typo.sz(12))
                .font(typo.medium)
                .color(contrast_on(accent)),
        )
        .padding([6, 14])
        .style(move |_theme| container::Style {
            background: Some(accent.into()),
            border: Border::default().rounded(6),
            ..Default::default()
        });

        let card = container(
            column![
                text(i18n::t("settings_preview_card"))
                    .size(typo.sz(14))
                    .font(typo.bold)
                    .color(Palette::TEXT_PRIMARY()),
                text(i18n::t("settings_preview_body"))
                    .size(typo.sz(12))
                    .font(typo.regular)
                    .color(Palette::TEXT_SECONDARY()),
                text(format!("{theme_name} · {accent_name}"))
                    .size(typo.sz(11))
                    .font(typo.regular)
                    .color(Palette::TEXT_MUTED()),
                sample_button,
            ]
            .spacing(8),
        )
        .padding(16)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Palette::BG_CARD().into()),
            border: Border {
                color: Palette::BORDER_SUBTLE(),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        self.section(
            PREVIEW,
            "settings_section_preview",
            "settings_preview_desc",
            vec![card.into()],
        )
    }

    /// Accessibility → Vision: high contrast, derived from the active palette,
    /// and the dyslexia-friendly font.
    pub fn vision_section<'a>(&self) -> Element<'a, M>
    where
        M: 'a,
    {
        let contrast = functional_toggle(
            &self.typo,
            i18n::t("settings_high_contrast"),
            i18n::t("settings_high_contrast_desc"),
            self.prefs.high_contrast,
            (self.on_change)(Change::HighContrast(!self.prefs.high_contrast)),
        );
        let dyslexia = functional_toggle(
            &self.typo,
            i18n::t("settings_dyslexia_font"),
            i18n::t("settings_dyslexia_font_desc"),
            self.prefs.dyslexia_font,
            (self.on_change)(Change::DyslexiaFont(!self.prefs.dyslexia_font)),
        );
        self.section(
            VISION,
            "settings_section_vision",
            "settings_vision_desc",
            vec![contrast, space().height(4).into(), dyslexia],
        )
    }

    /// Accessibility → Motion: reduced motion, which silences every effect.
    pub fn motion_section<'a>(&self) -> Element<'a, M>
    where
        M: 'a,
    {
        let toggle = functional_toggle(
            &self.typo,
            i18n::t("settings_reduce_motion"),
            i18n::t("settings_reduce_motion_desc"),
            self.prefs.reduce_motion,
            (self.on_change)(Change::ReduceMotion(!self.prefs.reduce_motion)),
        );
        self.section(
            MOTION,
            "settings_section_motion",
            "settings_motion_desc",
            vec![toggle],
        )
    }

    /// Accessibility → Reading: text scaling, multiplying with Appearance's
    /// size.
    pub fn reading_section<'a>(&self) -> Element<'a, M>
    where
        F: 'a,
        M: 'a,
    {
        let on_change = self.on_change.clone();
        let size = setting_row(
            &self.typo,
            i18n::t("settings_text_size_a11y"),
            i18n::t("settings_text_size_a11y_desc"),
            dropdown(
                &self.typo,
                TextSize::ALL.map(|s| (s, i18n::t(s.label_key()).to_string())),
                &self.prefs.text_size_a11y,
                move |s| on_change(Change::TextSize(s)),
            ),
        );
        self.section(
            READING,
            "settings_section_reading",
            "settings_reading_desc",
            vec![size],
        )
    }

    fn section<'a>(
        &self,
        key: &'static str,
        title_key: &str,
        description_key: &str,
        controls: Vec<Element<'a, M>>,
    ) -> Element<'a, M>
    where
        M: 'a,
    {
        let body = Column::with_children(controls).width(Length::Fill);
        collapsible_section(
            &self.typo,
            i18n::t(title_key),
            self.expanded.contains(key),
            (self.on_section)(key),
            column![
                section_description(&self.typo, i18n::t(description_key)),
                body
            ]
            .into(),
        )
    }
}

/// Every label key this module and [`navigation`](super::navigation) draw.
/// A test holds them to both locales, so a typo shows up in `cargo test`
/// rather than as a raw key on someone's screen.
#[cfg(test)]
pub(crate) const KEYS: &[&str] = &[
    "settings_title",
    "settings_close",
    "settings_cat_general",
    "settings_general_desc",
    "settings_cat_appearance",
    "settings_appearance_desc",
    "settings_cat_accessibility",
    "settings_accessibility_desc",
    "settings_cat_about",
    "settings_section_theme",
    "settings_theme_desc",
    "settings_section_colors",
    "settings_colors_desc",
    "settings_auto_accent",
    "settings_auto_accent_desc",
    "settings_default_accent",
    "settings_section_typography",
    "settings_typography_desc",
    "settings_font_family",
    "settings_font_family_desc",
    "settings_font_size",
    "settings_font_size_desc",
    "settings_font_size_small",
    "settings_font_size_default",
    "settings_font_size_large",
    "settings_font_size_xlarge",
    "settings_section_effects",
    "settings_effects_desc",
    "settings_animations",
    "settings_animations_desc",
    "settings_section_preview",
    "settings_preview_desc",
    "settings_preview_card",
    "settings_preview_body",
    "settings_preview_button",
    "settings_section_vision",
    "settings_vision_desc",
    "settings_high_contrast",
    "settings_high_contrast_desc",
    "settings_dyslexia_font",
    "settings_dyslexia_font_desc",
    "settings_section_motion",
    "settings_motion_desc",
    "settings_reduce_motion",
    "settings_reduce_motion_desc",
    "settings_section_reading",
    "settings_reading_desc",
    "settings_text_size_a11y",
    "settings_text_size_a11y_desc",
    "theme_applied",
];

#[cfg(all(test, feature = "fonts"))]
mod tests {
    use super::*;
    use crate::i18n::{t_in, Locale};

    /// The claim on [`category_list`]: every standard category name fits on one
    /// line at every combination of the two text sizes, in either locale and
    /// either interface font, measured from the embedded files.
    #[test]
    fn the_category_names_fit_the_list_at_every_text_scale() {
        // nav_item's horizontal padding, then the list's right gutter.
        const CHROME: f32 = 14.0 * 2.0 + 16.0;

        let fonts = [
            (fonts::APP_FAMILY, fonts::BYTES[0]),
            (fonts::DYSLEXIA_FAMILY, fonts::BYTES[3]),
        ];
        for (font, bytes) in fonts {
            let face = ttf_parser::Face::parse(bytes, 0).unwrap();
            let em = |label: &str| -> f32 {
                label
                    .chars()
                    .map(|c| {
                        let glyph = face.glyph_index(c).expect("the font has every letter");
                        face.glyph_hor_advance(glyph).unwrap_or(0) as f32
                    })
                    .sum::<f32>()
                    / face.units_per_em() as f32
            };

            for font_size in FontSize::ALL {
                for text_size in TextSize::ALL {
                    let typo = Typography {
                        scale: font_size.factor() * text_size.factor(),
                        ..Typography::default()
                    };
                    let width = CATEGORY_LIST_WIDTH * typo.scale;
                    for key in STANDARD_CATEGORIES.iter().chain([&ABOUT_CATEGORY]) {
                        for locale in [Locale::En, Locale::Fr] {
                            let label = t_in(locale, key);
                            let needed = em(label) * typo.sz(13) + CHROME;
                            assert!(
                                needed <= width,
                                "{label:?} in {font} at {:.4}x needs {needed:.1}, the list is {width:.1}",
                                typo.scale
                            );
                        }
                    }
                }
            }
        }
    }
}
