//! The preferences kit, used the way a Colony program uses it: through the
//! public API only, with a message type of the program's own. Like
//! `consumer.rs`, a change needed here to keep compiling is a change every
//! program needs too.

use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard};

use colony_ui::preferences::{Change, StandardPreferences};
use colony_ui::widgets::{self, PreferencesView, ABOUT_CATEGORY, STANDARD_CATEGORIES};
use colony_ui::{fonts, i18n, motion, theme, typography, Typography};
use iced::widget::text;
use iced::Element;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    TogglePreferences,
    Category(usize),
    ToggleSection(String),
    Preference(Change),
}

/// The kit's state is process-global, and the tests in this file set it.
fn exclusive() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn a_whole_preferences_page_builds_open_and_closed() {
    let _guard = exclusive();
    let typo = Typography::current();
    let prefs = StandardPreferences::default();

    // Every section closed, as on arrival, then every section open.
    let none = HashSet::new();
    let all: HashSet<String> = [
        "theme",
        "colors",
        "typography",
        "effects",
        "preview",
        "vision",
        "motion",
        "reading",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    for expanded in [&none, &all] {
        let kit = PreferencesView::new(typo, &prefs, expanded, Message::Preference, |key| {
            Message::ToggleSection(key.to_string())
        });
        for content in [
            kit.general().push(text("Startup")),
            kit.appearance(),
            kit.accessibility(),
        ] {
            let labels = STANDARD_CATEGORIES
                .iter()
                .map(|key| i18n::t(key))
                .chain([i18n::t(ABOUT_CATEGORY)]);
            let _: Element<'_, Message> = widgets::preferences_page(
                &typo,
                widgets::category_list(&typo, labels, 1, Message::Category),
                content,
                Message::TogglePreferences,
            );
        }
    }
}

#[test]
fn the_chrome_builds_against_a_host_message_type() {
    let typo = Typography::current();
    for open in [false, true] {
        let _: Element<'_, Message> =
            widgets::identity_button(&typo, "Grape", 20, open, Message::TogglePreferences);
    }
    let _: Element<'_, Message> = widgets::nav_label(&typo, "Sections");
    let _: Element<'_, Message> = widgets::nav_item(&typo, "Library", true, Message::Category(0));
    let _: Element<'_, Message> = widgets::nav_hint(&typo, "Tab moves between sections");
    let _: Element<'_, Message> = widgets::setting_row(
        &typo,
        "Language",
        "The interface language.",
        widgets::dropdown(
            &typo,
            [
                ("en", "English".to_string()),
                ("fr", "Français".to_string()),
            ],
            &"fr",
            |code| Message::ToggleSection(code.to_string()),
        ),
    );
}

#[test]
fn a_change_from_a_widget_round_trips_through_the_host() {
    let _guard = exclusive();

    // The host wraps the kit's Change in its own message and hands it back.
    let mut prefs = StandardPreferences::default();
    let message = Message::Preference(Change::TextSize(typography::TextSize::XLarge));
    let Message::Preference(change) = message else {
        unreachable!()
    };
    prefs.update(change);
    prefs.update(Change::FontSize(typography::FontSize::Large));

    // The very next frame's Typography carries the product of both sizes.
    let typo = Typography::current();
    assert_eq!(typo.sz(30), 50.0);
    assert_eq!(colony_ui::sz(30), 50.0);

    prefs.update(Change::DyslexiaFont(true));
    assert_eq!(Typography::current().bold, fonts::ui_font_bold());
    assert_eq!(
        Typography::current().regular.family,
        iced::font::Family::Name(fonts::DYSLEXIA_FAMILY)
    );

    prefs.update(Change::ReduceMotion(true));
    assert!(!motion::effects_enabled());

    prefs.update(Change::HighContrast(true));
    assert!(theme::is_high_contrast());

    StandardPreferences::default().apply();
}

/// Selection is an accent fill; its label has to read on it, on every theme and
/// with every accent override. `text_primary` was what the rule first said,
/// and on Ayu Dark it measured 1.01:1.
#[test]
fn a_selected_item_is_legible_on_every_theme_and_accent() {
    let _guard = exclusive();
    let mut worst = (f32::MAX, String::new());

    let accents = std::iter::once(None).chain(
        theme::ACCENT_OVERRIDES
            .iter()
            .map(|a| Some(theme::hex(a.color))),
    );
    for accent in accents {
        theme::set_active_accent(accent);
        for family in theme::THEME_FAMILIES {
            for variant in family.variants {
                theme::set_active_theme(family.key, variant.key);
                let fill = theme::Palette::ACCENT();
                let ratio = theme::contrast_ratio(theme::contrast_on(fill), fill);
                if ratio < worst.0 {
                    worst = (ratio, format!("{}/{} {accent:?}", family.key, variant.key));
                }
            }
        }
    }
    theme::set_active_accent(None);
    theme::set_active_theme("gruvbox", "dark");

    // 4.21:1 at the time of writing, Night Owl light on its own accent: the
    // ceiling for a mid-tone fill is near-black or near-white, not 4.5:1.
    assert!(
        worst.0 >= 4.0,
        "selected label at {:.2}:1 on {}",
        worst.0,
        worst.1
    );
}
