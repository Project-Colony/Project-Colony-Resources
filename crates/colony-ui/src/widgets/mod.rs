//! The widgets every Colony program's chrome and Preferences page are built
//! from.
//!
//! Ported from Colony's `src/ui/settings.rs` and `src/ui/sidebar.rs`, which is
//! where they were first written and where they were being copied from by hand.
//! They are generic over the host's message type: you pass the message to emit,
//! the crate draws the control.
//!
//! | | |
//! |---|---|
//! | [`identity_button`] | the program's name, top-left: the way into Preferences |
//! | [`nav_item`], [`selection_style`], [`nav_label`], [`nav_hint`] | a sidebar or category list, selection by background |
//! | [`preferences_page`], [`preferences_header`], [`category_list`] | the page that replaces the content area |
//! | [`category_heading`], [`section_description`], [`setting_row`], [`dropdown`] | the parts of a category |
//! | [`collapsible_section`], [`functional_toggle`] | a section and an on/off setting |
//! | [`theme_picker`], [`accent_picker`] | Appearance's two pickers |
//! | [`PreferencesView`] | General, Appearance and Accessibility, ready-made |
//!
//! Sizes go through [`Typography`](crate::Typography) rather than being
//! hardcoded, so the host's two font-scale preferences reach them; pass
//! [`Typography::current`](crate::Typography::current) and the dyslexia font
//! reaches them too.
//!
//! `examples/preferences.rs` builds a complete page from these.

mod accent_picker;
mod collapsible;
mod navigation;
mod preferences;
mod theme_picker;
mod toggle;

pub use accent_picker::accent_picker;
pub use collapsible::collapsible_section;
pub use navigation::{identity_button, nav_hint, nav_item, nav_label, selection_style};
pub use preferences::{
    category_heading, category_list, dropdown, preferences_header, preferences_page,
    section_description, setting_row, PreferencesView, ABOUT_CATEGORY, STANDARD_CATEGORIES,
};
pub use theme_picker::theme_picker;
pub use toggle::functional_toggle;

/// Glyphs the shared widgets draw, in the Nerd Font range that
/// `JetBrainsMono Nerd Font` covers. Exposed so a program's own widgets can use
/// the same ones rather than picking a near-miss.
pub mod icons {
    /// Section expanded.
    pub const CHEVRON_DOWN: &str = "\u{f078}";
    /// Section collapsed.
    pub const CHEVRON_RIGHT: &str = "\u{f054}";
    /// Selected.
    pub const CHECK: &str = "\u{f00c}";
    /// Opens the preferences, next to the program name.
    pub const GEAR: &str = "\u{f013}";
}

#[cfg(test)]
mod tests {
    use crate::i18n::{t_in, Locale};

    #[test]
    fn every_label_the_widgets_draw_exists_in_both_locales() {
        for key in super::preferences::KEYS {
            for locale in [Locale::En, Locale::Fr] {
                assert_ne!(
                    t_in(locale, key),
                    *key,
                    "{key} is missing from the {} table",
                    locale.as_str()
                );
            }
        }
    }

    #[test]
    fn the_user_facing_word_is_preferences() {
        assert_eq!(t_in(Locale::En, "settings_title"), "Preferences");
        assert_eq!(t_in(Locale::Fr, "settings_title"), "Préférences");
        assert_eq!(
            t_in(Locale::En, "settings_general_desc"),
            "Preferences are saved automatically."
        );
    }

    #[test]
    fn the_first_three_categories_are_fixed() {
        let names: Vec<&str> = super::STANDARD_CATEGORIES
            .iter()
            .map(|key| t_in(Locale::En, key))
            .collect();
        assert_eq!(names, ["General", "Appearance", "Accessibility"]);
    }
}
