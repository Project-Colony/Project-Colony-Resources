//! A Colony program's chrome and its whole Preferences page, built from
//! colony-ui and nothing else.
//!
//! ```text
//! cargo run -p colony-ui --example preferences
//! LANG=fr cargo run -p colony-ui --example preferences
//! ```
//!
//! What it shows, rule by rule (design/navigation.md, design/settings-page.md):
//!
//! - the program's name, top-left, is the way into Preferences, shows whether
//!   it is open, and closes what it opened;
//! - the page replaces the content area, the sidebar stays;
//! - General, Appearance, Accessibility first, About last;
//! - every section arrives collapsed;
//! - no Save button: a change applies the moment it is made.
//!
//! The demo's own strings ("Library", "Startup", ...) are literals to keep the
//! file short. A real program puts its own vocabulary in its own i18n table, in
//! both locales.

use std::collections::HashSet;

use colony_ui::preferences::{Change, StandardPreferences};
use colony_ui::theme::Palette;
use colony_ui::widgets::{self, PreferencesView, ABOUT_CATEGORY, STANDARD_CATEGORIES};
use colony_ui::{fonts, i18n, Typography};
use iced::widget::{column, container, row, space, stack, text};
use iced::{Element, Length};

fn main() -> iced::Result {
    fonts::BYTES
        .into_iter()
        .fold(
            iced::application(Demo::boot, Demo::update, Demo::view),
            |app, font| app.font(font),
        )
        .title("colony-ui preferences")
        .default_font(fonts::ui_font())
        .run()
}

const SECTIONS: [&str; 3] = ["Library", "Playlists", "Queue"];

#[derive(Default)]
struct Demo {
    /// A real program deserializes this, usually flattened into its own config
    /// struct, from `paths::config_dir("<Program>")?.join("preferences.json")`.
    prefs: StandardPreferences,
    restore_session: bool,
    preferences_open: bool,
    category: usize,
    expanded: HashSet<String>,
    section: usize,
    notice: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// Sent by the identity button AND by the page's Close button.
    TogglePreferences,
    Category(usize),
    Section(usize),
    ToggleSection(&'static str),
    Preference(Change),
    ToggleRestoreSession,
    DismissNotice,
}

impl Demo {
    fn boot() -> Self {
        let demo = Demo::default();
        demo.prefs.apply();
        let lang = std::env::var("LANG").unwrap_or_default();
        i18n::set_locale(i18n::Locale::from_tag(&lang));
        demo
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TogglePreferences => {
                self.preferences_open = !self.preferences_open;
                self.category = 0;
            }
            Message::Category(index) => self.category = index,
            Message::Section(index) => {
                self.section = index;
                self.preferences_open = false;
            }
            Message::ToggleSection(key) => {
                if !self.expanded.remove(key) {
                    self.expanded.insert(key.to_string());
                }
            }
            Message::Preference(change) => {
                self.prefs.update(change);
                // A real program saves its config here.
                if matches!(change, Change::Theme { .. }) {
                    self.notice = Some(i18n::t("theme_applied"));
                }
            }
            Message::ToggleRestoreSession => self.restore_session = !self.restore_session,
            Message::DismissNotice => self.notice = None,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Read once per frame: the user's sizes and, with the dyslexia font on,
        // OpenDyslexic in every weight.
        let typo = Typography::current();

        let mut sidebar = column![
            widgets::identity_button(
                &typo,
                "Demo",
                30,
                self.preferences_open,
                Message::TogglePreferences
            ),
            space().height(24),
            widgets::nav_label(&typo, "Sections"),
        ]
        .spacing(10);
        for (index, name) in SECTIONS.iter().enumerate() {
            let selected = !self.preferences_open && self.section == index;
            sidebar = sidebar.push(widgets::nav_item(
                &typo,
                name,
                selected,
                Message::Section(index),
            ));
        }
        // Like the category list, the sidebar grows with the text so the name
        // still fits at 1.68x.
        let sidebar = sidebar
            .push(space().height(Length::Fill))
            .push(widgets::nav_hint(&typo, "Click Demo to open Preferences"))
            .padding(16)
            .width(200.0 * typo.scale);

        let content = if self.preferences_open {
            self.preferences(&typo)
        } else {
            container(
                text(SECTIONS[self.section])
                    .size(typo.sz(18))
                    .font(typo.bold)
                    .color(Palette::TEXT_PRIMARY()),
            )
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        };

        // The theme_applied toast, over the bottom of the page.
        let main: Element<'_, Message> = match self.notice {
            Some(notice) => stack![
                content,
                container(widgets::toast(&typo, notice, Message::DismissNotice))
                    .padding(16)
                    .align_bottom(Length::Fill),
            ]
            .into(),
            None => content,
        };

        container(row![
            container(sidebar)
                .height(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(Palette::BG_SIDEBAR().into()),
                    ..Default::default()
                }),
            main,
        ])
        .style(|_theme| container::Style {
            background: Some(Palette::BG_PRIMARY().into()),
            ..Default::default()
        })
        .into()
    }

    fn preferences(&self, typo: &Typography) -> Element<'_, Message> {
        let kit = PreferencesView::new(
            *typo,
            &self.prefs,
            &self.expanded,
            Message::Preference,
            Message::ToggleSection,
        );

        let content: Element<'_, Message> = match self.category {
            0 => kit
                .general()
                .push(widgets::collapsible_section(
                    typo,
                    "Startup",
                    self.expanded.contains("startup"),
                    Message::ToggleSection("startup"),
                    column![
                        widgets::section_description(typo, "What happens when the program opens."),
                        widgets::functional_toggle(
                            typo,
                            "Restore last session",
                            "Reopens the section you were in.",
                            self.restore_session,
                            Message::ToggleRestoreSession,
                        ),
                    ]
                    .into(),
                ))
                .into(),
            1 => kit.appearance().into(),
            2 => kit.accessibility().into(),
            _ => widgets::category_heading(
                typo,
                i18n::t(ABOUT_CATEGORY),
                concat!("colony-ui ", env!("CARGO_PKG_VERSION")),
            )
            .into(),
        };

        let labels = STANDARD_CATEGORIES
            .iter()
            .map(|key| i18n::t(key))
            .chain([i18n::t(ABOUT_CATEGORY)]);

        widgets::preferences_page(
            typo,
            widgets::category_list(typo, labels, self.category, Message::Category),
            content,
            Message::TogglePreferences,
        )
    }
}
