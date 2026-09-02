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

    // Lower bounds, not exact counts: the point of the catalog is that families
    // get added without a consumer changing, so a test that pins the number
    // would have to be edited every time one is — and would be the only thing
    // standing in the way.
    assert!(families >= 25, "families were removed from the catalog");
    assert!(variants >= 57, "variants were removed from the catalog");
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

/// A check mark drawn on a swatch has to be visible on it. Both pickers used a
/// colour that does not depend on what it sits on — white for the accent dot,
/// the active theme's accent for the variant card — and four of the eight
/// accents are light enough that the white mark fell below 3:1, yellow reaching
/// 2.31:1.
#[test]
fn a_check_mark_is_legible_on_every_swatch_it_can_be_drawn_on() {
    const FLOOR: f32 = 3.0;
    let mut failures = Vec::new();

    for accent in theme::ACCENT_OVERRIDES {
        let dot = theme::hex(accent.color);
        let ratio = theme::contrast_ratio(theme::contrast_on(dot), dot);
        if ratio < FLOOR {
            failures.push(format!("accent {}: {ratio:.2}:1", accent.key));
        }
    }

    for family in theme::THEME_FAMILIES {
        for variant in family.variants {
            let bg = theme::hex(variant.swatch_bg);
            let ratio = theme::contrast_ratio(theme::contrast_on(bg), bg);
            if ratio < FLOOR {
                failures.push(format!(
                    "{}/{} swatch: {ratio:.2}:1",
                    family.key, variant.key
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "check marks below {FLOOR}:1:\n  {}",
        failures.join("\n  ")
    );
}

/// `contrast_on` picks by the ratio each end achieves, not by a luminance
/// threshold. The green accent is the case that proves the difference: its YIQ
/// luma is 0.578, so the old threshold took the near-white branch and produced
/// a 2.34:1 mark where near-black gives well over twice that.
#[test]
fn contrast_on_picks_the_end_that_actually_reads() {
    for accent in theme::ACCENT_OVERRIDES {
        let bg = theme::hex(accent.color);
        let chosen = theme::contrast_on(bg);
        let ink = theme::hex(0x141419);
        let paper = theme::hex(0xf7f9ff);
        let best = theme::contrast_ratio(ink, bg).max(theme::contrast_ratio(paper, bg));
        assert!(
            (theme::contrast_ratio(chosen, bg) - best).abs() < 0.5,
            "{}: chose {:.2}:1 when {:.2}:1 was available",
            accent.key,
            theme::contrast_ratio(chosen, bg),
            best
        );
    }
}
