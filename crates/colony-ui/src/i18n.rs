//! Display strings for the shared design objects — theme families, theme
//! variants and accents — in both shipped locales.
//!
//! English is the base language of every Project Colony project; French is the
//! other UI locale. The strings are generated from `tokens/` and embedded here,
//! so a program gets them by depending on this crate rather than by copying
//! them into its own locale files and letting them drift.
//!
//! Only the strings that name shared objects live here. A program's own
//! vocabulary — its section titles, its button labels — stays in the program.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

const EN: &str = include_str!("../../../generated/i18n/labels.en.json");
const FR: &str = include_str!("../../../generated/i18n/labels.fr.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    #[default]
    En,
    Fr,
}

impl Locale {
    /// Parse a language tag: `"fr"`, `"fr-FR"`, `"fr_FR"` → French, anything
    /// else → English. Unknown tags fall back to the base language rather than
    /// failing, because a bad `LANG` should not stop a program from starting.
    pub fn from_tag(tag: &str) -> Self {
        if tag
            .split(['-', '_', '.'])
            .next()
            .is_some_and(|l| l.eq_ignore_ascii_case("fr"))
        {
            Locale::Fr
        } else {
            Locale::En
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Fr => "fr",
        }
    }

    fn source(self) -> &'static str {
        match self {
            Locale::En => EN,
            Locale::Fr => FR,
        }
    }
}

static ACTIVE: RwLock<Locale> = RwLock::new(Locale::En);

pub fn set_locale(locale: Locale) {
    *ACTIVE.write().unwrap() = locale;
}

pub fn locale() -> Locale {
    *ACTIVE.read().unwrap()
}

fn table(locale: Locale) -> &'static HashMap<String, String> {
    static EN_TABLE: OnceLock<HashMap<String, String>> = OnceLock::new();
    static FR_TABLE: OnceLock<HashMap<String, String>> = OnceLock::new();

    let cell = match locale {
        Locale::En => &EN_TABLE,
        Locale::Fr => &FR_TABLE,
    };
    cell.get_or_init(|| {
        // The JSON is generated and checked into the repository, and a test in
        // this crate parses both tables, so a panic here would mean the crate
        // was built from a tampered checkout.
        serde_json::from_str(locale.source()).expect("embedded label table is valid JSON")
    })
}

/// Look up a label in the active locale.
///
/// Falls back to English, then to the key itself. A missing key renders as the
/// key — visible in the UI, which is what you want: silently rendering nothing
/// hides the bug until a user reports a blank row.
pub fn t(key: &str) -> &'static str {
    lookup(locale(), key)
}

/// Look up a label in a specific locale, ignoring the active one.
pub fn t_in(locale: Locale, key: &str) -> &'static str {
    lookup(locale, key)
}

fn lookup(locale: Locale, key: &str) -> &'static str {
    table(locale)
        .get(key)
        .or_else(|| table(Locale::En).get(key))
        .map(String::as_str)
        .unwrap_or_else(|| leak_key(key))
}

/// Render an unknown key as itself. Keys are `&'static str` in practice, but
/// `t` accepts any `&str`, so an unknown one is interned to keep the signature
/// simple. Unknown keys are a bug and there is a finite number of them.
fn leak_key(key: &str) -> &'static str {
    static UNKNOWN: OnceLock<RwLock<Vec<&'static str>>> = OnceLock::new();
    let store = UNKNOWN.get_or_init(|| RwLock::new(Vec::new()));

    if let Some(found) = store.read().unwrap().iter().find(|k| **k == key) {
        return found;
    }
    let mut store = store.write().unwrap();
    if let Some(found) = store.iter().find(|k| **k == key) {
        return found;
    }
    let leaked: &'static str = Box::leak(key.to_string().into_boxed_str());
    store.push(leaked);
    leaked
}

/// Every key this crate ships, in both locales. Useful to a program merging
/// these into its own table.
pub fn all(locale: Locale) -> impl Iterator<Item = (&'static str, &'static str)> {
    table(locale).iter().map(|(k, v)| (k.as_str(), v.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_tables_parse_and_agree_on_keys() {
        let en: Vec<&str> = {
            let mut k: Vec<&str> = table(Locale::En).keys().map(String::as_str).collect();
            k.sort_unstable();
            k
        };
        let fr: Vec<&str> = {
            let mut k: Vec<&str> = table(Locale::Fr).keys().map(String::as_str).collect();
            k.sort_unstable();
            k
        };
        assert_eq!(en, fr, "the two locales ship different key sets");
        assert!(!en.is_empty());
    }

    #[test]
    fn translates_a_known_key() {
        assert_eq!(t_in(Locale::En, "settings_theme_dark_mode"), "Dark mode");
        assert_eq!(t_in(Locale::Fr, "settings_theme_dark_mode"), "Mode sombre");
    }

    #[test]
    fn proper_nouns_are_identical_in_both_locales() {
        for key in [
            "settings_theme_catppuccin",
            "settings_theme_gruvbox",
            "settings_theme_stellar_blade_eve",
        ] {
            assert_eq!(
                t_in(Locale::En, key),
                t_in(Locale::Fr, key),
                "{key} is a proper noun and must not be translated"
            );
        }
    }

    #[test]
    fn an_unknown_key_renders_as_itself() {
        assert_eq!(t_in(Locale::Fr, "no_such_key"), "no_such_key");
        // And interning is stable across calls.
        assert_eq!(
            t_in(Locale::Fr, "no_such_key").as_ptr(),
            t_in(Locale::En, "no_such_key").as_ptr()
        );
    }

    #[test]
    fn parses_language_tags() {
        assert_eq!(Locale::from_tag("fr"), Locale::Fr);
        assert_eq!(Locale::from_tag("fr_FR.UTF-8"), Locale::Fr);
        assert_eq!(Locale::from_tag("FR-be"), Locale::Fr);
        assert_eq!(Locale::from_tag("en_GB"), Locale::En);
        assert_eq!(Locale::from_tag("nonsense"), Locale::En);
    }
}
