//! `generated/i18n/theme_labels.{fr,en}.json` — the theme display strings.
//!
//! Label keys are deliberately shared across families (`settings_theme_light` is
//! used by a dozen of them), so the same key must always carry the same string.
//! Conflicts are a hard error rather than a last-writer-wins surprise.

use anyhow::{bail, Result};
use serde_json::{Map, Value};

use crate::model::Tokens;

pub struct Locales {
    pub fr: Value,
    pub en: Value,
}

pub fn render(tokens: &Tokens) -> Result<Locales> {
    let mut fr = Map::new();
    let mut en = Map::new();

    for family in &tokens.families {
        insert(&mut fr, &family.label_key, &family.label.fr, "fr")?;
        insert(&mut en, &family.label_key, &family.label.en, "en")?;

        for variant in &family.variants {
            insert(&mut fr, &variant.label_key, &variant.label.fr, "fr")?;
            insert(&mut en, &variant.label_key, &variant.label.en, "en")?;
        }
    }

    for accent in &tokens.accents {
        insert(&mut fr, &accent.label_key, &accent.label.fr, "fr")?;
        insert(&mut en, &accent.label_key, &accent.label.en, "en")?;
    }

    // Colony's `fr_and_en_have_identical_key_sets` test exists for a reason;
    // catch a divergence here rather than in the launcher's test suite.
    let fr_keys: Vec<&String> = fr.keys().collect();
    let en_keys: Vec<&String> = en.keys().collect();
    if fr_keys != en_keys {
        bail!("fr and en label key sets differ — every label needs both locales");
    }

    Ok(Locales {
        fr: Value::Object(fr),
        en: Value::Object(en),
    })
}

fn insert(map: &mut Map<String, Value>, key: &str, value: &str, locale: &str) -> Result<()> {
    if value.is_empty() {
        bail!("label key {key:?} has an empty {locale} string");
    }
    if let Some(existing) = map.get(key) {
        if existing != value {
            bail!(
                "label key {key:?} is used with two different {locale} strings: \
                 {existing} and {value:?}"
            );
        }
        return Ok(());
    }
    map.insert(key.to_string(), Value::String(value.to_string()));
    Ok(())
}
