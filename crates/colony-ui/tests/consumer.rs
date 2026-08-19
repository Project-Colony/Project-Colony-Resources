//! Exercises the crate the way a Colony program does, through its public API
//! only. If this file needs a change to keep compiling, every consumer needs
//! that change too — which makes it the crate's compatibility contract.

use colony_ui::i18n::{self, Locale};
use colony_ui::widgets::{accent_picker, collapsible_section, functional_toggle, theme_picker};
use colony_ui::{theme, Typography};
use iced::widget::text;
use iced::Element;

/// A host program's message type. Deliberately not `Copy`, and carrying owned
/// data, because that is the realistic case.
#[derive(Debug, Clone, PartialEq)]
enum Message {
    ToggleSection(String),
    SelectTheme { family: String, variant: String },
    SelectAccent(String),
    ToggleAutoAccent,
}

#[test]
fn a_program_can_restore_its_theme_from_config_strings() {
    // What a launcher does at startup: two strings out of a config file.
    theme::set_active_theme("catppuccin", "mocha");
    assert_eq!(
        theme::Palette::BG_PRIMARY(),
        theme::ThemePalette::CATPPUCCIN_MOCHA.bg_primary
    );

    // Including a config naming something this build has never heard of.
    theme::set_active_theme("a_theme_from_the_future", "shiny");
    assert_eq!(
        theme::Palette::BG_PRIMARY(),
        theme::FALLBACK_PALETTE.bg_primary
    );

    theme::set_active_theme("gruvbox", "dark");
}

#[test]
fn the_catalog_is_enough_to_build_a_picker_without_touching_the_crate() {
    // The point of the whole exercise: a host enumerates families and variants
    // without knowing any of their names at compile time.
    let mut families = 0;
    let mut variants = 0;

    for family in theme::THEME_FAMILIES {
        families += 1;
        assert!(!i18n::t(family.label_key).is_empty());
        for variant in family.variants {
            variants += 1;
            assert!(!i18n::t(variant.label_key).is_empty());
            assert!(variant.mode == "light" || variant.mode == "dark");
            // Resolving by key gives the same palette the catalog carries.
            assert_eq!(theme::resolve(family.key, variant.key), variant.palette);
        }
    }

    assert_eq!(families, 25);
    assert_eq!(variants, 57);
}

#[test]
fn labels_follow_the_active_locale() {
    i18n::set_locale(Locale::Fr);
    assert_eq!(i18n::t("settings_theme_dark_mode"), "Mode sombre");
    i18n::set_locale(Locale::En);
    assert_eq!(i18n::t("settings_theme_dark_mode"), "Dark mode");
}

#[test]
fn every_shared_widget_builds_against_a_host_message_type() {
    let typo = Typography::default();

    let _: Element<'_, Message> = collapsible_section(
        &typo,
        "Startup",
        true,
        Message::ToggleSection("startup".into()),
        text("body").into(),
    );

    let _: Element<'_, Message> = collapsible_section(
        &typo,
        "Startup",
        false,
        Message::ToggleSection("startup".into()),
        text("body").into(),
    );

    let _: Element<'_, Message> = functional_toggle(
        &typo,
        "Auto accent from background",
        "Adapts the accent to the surfaces.",
        false,
        Message::ToggleAutoAccent,
    );

    let _: Element<'_, Message> = theme_picker(&typo, "gruvbox", "dark", |family, variant| {
        Message::SelectTheme {
            family: family.to_string(),
            variant: variant.to_string(),
        }
    });

    let _: Element<'_, Message> = accent_picker(&typo, Some("violet"), |key| {
        Message::SelectAccent(key.into())
    });

    // And with no accent chosen, i.e. auto.
    let _: Element<'_, Message> =
        accent_picker(&typo, None, |key| Message::SelectAccent(key.into()));
}

#[test]
fn typography_scaling_multiplies_the_way_the_host_expects() {
    // Appearance -> Typography "large" (1.2) times Accessibility -> Reading
    // "xlarge" (1.4). A layout has to survive the product, not each factor.
    let typo = Typography {
        scale: 1.2 * 1.4,
        ..Typography::default()
    };
    assert_eq!(typo.sz(30), 50.0);
    assert_eq!(typo.sz(13), 22.0);

    let smallest = Typography {
        scale: 0.85 * 0.85,
        ..Typography::default()
    };
    assert_eq!(smallest.sz(30), 22.0);
}

#[test]
fn a_program_gets_a_stable_identity_tint_without_shipping_an_icon() {
    let colony = theme::app_tint("Colony");
    assert_eq!(colony, theme::app_tint("Colony"));
    assert_ne!(colony, theme::app_tint("Eidos"));
}
