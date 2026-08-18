//! The token model: what a theme family, a variant and a palette *are*.
//!
//! This file is the schema. Adding a field to `palette!` below is the only way
//! to add a field to the Colony palette — every emitter derives its output from
//! `Palette::FIELDS`, so a new field reaches Rust, JSON, CSS and the docs at once.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::Deserialize;

use crate::color::Color;

/// One palette field: its name and the role group it belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    pub name: &'static str,
    pub group: &'static str,
}

impl Field {
    /// `bg_primary` -> `--colony-bg-primary`.
    pub fn css_var(&self) -> String {
        format!("--colony-{}", self.name.replace('_', "-"))
    }
}

/// Declares the palette struct, its field order, and each field's role group.
/// Field order here IS the order emitted everywhere downstream.
macro_rules! palette {
    ($( $group:literal => [ $($field:ident),* $(,)? ] ),* $(,)?) => {
        /// A full Colony palette. Every field is required — a partial palette is
        /// a bug, not a feature, so `deny_unknown_fields` catches typos too.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct Palette {
            $( $( pub $field: Color, )* )*
        }

        impl Palette {
            /// Canonical field order, with role groups.
            pub const FIELDS: &'static [Field] = &[
                $( $( Field { name: stringify!($field), group: $group }, )* )*
            ];

            /// Values in `FIELDS` order. The two are built from the same list,
            /// so they cannot drift apart.
            pub fn values(&self) -> Vec<Color> {
                vec![ $( $( self.$field, )* )* ]
            }

            /// `(field, value)` pairs in canonical order.
            pub fn entries(&self) -> Vec<(Field, Color)> {
                Self::FIELDS.iter().copied().zip(self.values()).collect()
            }
        }
    };
}

palette! {
    "backgrounds" => [
        bg_primary, bg_sidebar, bg_card, bg_card_hover, bg_card_pressed,
        bg_selected, bg_input, bg_progress,
    ],
    "text" => [
        text_primary, text_secondary, text_muted, text_dim, text_dimmer,
        text_dimmest, text_placeholder,
    ],
    "accent" => [accent_blue, accent_icon, accent_progress],
    "buttons" => [btn_default, btn_hover, btn_pressed],
    "success" => [
        success, success_bg, btn_success, btn_success_hover, btn_success_pressed,
    ],
    "warning" => [warning, warning_bg],
    "error" => [
        error, error_light, error_bg, btn_danger_bg, btn_danger_hover,
        btn_trash_hover, btn_trash_pressed,
    ],
    "modal" => [bg_modal_section, border_subtle, divider],
}

/// A localized display string. Every label must exist in **both** locales —
/// Colony has a test that fails when a key is present in one locale only.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Label {
    pub fr: String,
    pub en: String,
}

/// The two colours drawn on a variant's card in the Settings theme picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Swatch {
    pub bg: Color,
    pub accent: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Light,
    Dark,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Light => "light",
            Mode::Dark => "dark",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub key: String,
    /// Rust const name in Colony's `theme.rs`. Stable identifier — renaming one
    /// is a breaking change for every consumer that pins it.
    #[serde(rename = "const")]
    pub const_name: String,
    pub label_key: String,
    pub mode: Mode,
    pub label: Label,
    pub swatch: Swatch,
    /// Why this palette looks the way it does — the material, the ink, which
    /// colour was demoted and why. Design intent that used to live in a code
    /// comment or in someone's head. Optional, but write it for anything whose
    /// colours are not self-explanatory.
    #[serde(default)]
    pub notes: Option<String>,
    pub palette: Palette,
}

impl Variant {
    /// `gruvbox-dark` — the slug used for CSS filenames and selectors.
    pub fn slug(&self, family: &Family) -> String {
        format!(
            "{}-{}",
            family.key.replace('_', "-"),
            self.key.replace('_', "-")
        )
    }

    /// Human-readable English name, e.g. `Gruvbox · Dark mode`.
    pub fn display_name(&self, family: &Family) -> String {
        format!("{} · {}", family.label.en, self.label.en)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Family {
    pub key: String,
    /// Position in the Settings picker. Unique across families.
    pub order: u32,
    pub label_key: String,
    /// Nerd Font codepoint as bare lowercase hex (`f0f4`), or empty for none.
    pub icon: String,
    pub label: Label,
    /// Provenance and licensing context for the family as a whole — where the
    /// colours came from, and anything a port needs to know before shipping it.
    #[serde(default)]
    pub notes: Option<String>,
    pub variants: Vec<Variant>,
}

impl Family {
    /// The icon as a `char`, or `None` when the family has no glyph.
    pub fn icon_char(&self) -> Result<Option<char>> {
        if self.icon.is_empty() {
            return Ok(None);
        }
        let cp = u32::from_str_radix(&self.icon, 16)
            .with_context(|| format!("family {:?}: icon {:?} is not hex", self.key, self.icon))?;
        char::from_u32(cp)
            .map(Some)
            .with_context(|| format!("family {:?}: U+{cp:X} is not a character", self.key))
    }

    /// `\u{f0f4}` for Rust source, or an empty string.
    pub fn icon_rust_escape(&self) -> String {
        if self.icon.is_empty() {
            String::new()
        } else {
            format!("\\u{{{}}}", self.icon)
        }
    }
}

/// A palette-independent accent the user can pick to override the active
/// theme's own accent.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Accent {
    pub key: String,
    pub label_key: String,
    pub label: Label,
    pub color: Color,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct AccentsFile {
    accents: Vec<Accent>,
}

/// Every family, in picker order, plus the shared accent overrides.
#[derive(Debug, Clone)]
pub struct Tokens {
    pub families: Vec<Family>,
    /// In file order — see the warning in `tokens/accents.toml`, this order is
    /// what Colony's per-app identity tint hashes into.
    pub accents: Vec<Accent>,
}

impl Tokens {
    /// Load and validate `tokens/families/*.toml`.
    pub fn load(tokens_dir: &Path) -> Result<Self> {
        let dir = tokens_dir.join("families");
        let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
            .with_context(|| format!("reading {}", dir.display()))?
            .map(|e| e.map(|e| e.path()))
            .collect::<std::result::Result<_, _>>()?;
        paths.retain(|p| p.extension().is_some_and(|e| e == "toml"));
        paths.sort();

        if paths.is_empty() {
            bail!("no theme families found in {}", dir.display());
        }

        let mut families = Vec::with_capacity(paths.len());
        for path in &paths {
            let src = std::fs::read_to_string(path)
                .with_context(|| format!("reading {}", path.display()))?;
            let family: Family =
                toml::from_str(&src).with_context(|| format!("parsing {}", path.display()))?;

            let stem = path.file_stem().unwrap().to_string_lossy();
            if family.key != stem {
                bail!(
                    "{}: key {:?} must match the file name {:?}",
                    path.display(),
                    family.key,
                    stem
                );
            }
            families.push(family);
        }

        families.sort_by_key(|f| f.order);

        let accents_path = tokens_dir.join("accents.toml");
        let accents_src = std::fs::read_to_string(&accents_path)
            .with_context(|| format!("reading {}", accents_path.display()))?;
        let accents: AccentsFile = toml::from_str(&accents_src)
            .with_context(|| format!("parsing {}", accents_path.display()))?;

        let tokens = Tokens {
            families,
            accents: accents.accents,
        };
        tokens.validate()?;
        Ok(tokens)
    }

    pub fn variants(&self) -> impl Iterator<Item = (&Family, &Variant)> {
        self.families
            .iter()
            .flat_map(|f| f.variants.iter().map(move |v| (f, v)))
    }

    pub fn variant_count(&self) -> usize {
        self.families.iter().map(|f| f.variants.len()).sum()
    }

    /// Every invariant a consumer is allowed to rely on.
    fn validate(&self) -> Result<()> {
        let mut orders: BTreeMap<u32, &str> = BTreeMap::new();
        let mut family_keys: BTreeMap<&str, ()> = BTreeMap::new();
        let mut consts: BTreeMap<&str, String> = BTreeMap::new();

        for family in &self.families {
            if family_keys.insert(&family.key, ()).is_some() {
                bail!("duplicate family key {:?}", family.key);
            }
            if let Some(other) = orders.insert(family.order, &family.key) {
                bail!(
                    "families {:?} and {:?} share order {}",
                    other,
                    family.key,
                    family.order
                );
            }
            family.icon_char()?;

            if family.variants.is_empty() {
                bail!("family {:?} has no variants", family.key);
            }

            let mut variant_keys: BTreeMap<&str, ()> = BTreeMap::new();
            for variant in &family.variants {
                let id = format!("{}/{}", family.key, variant.key);
                if variant_keys.insert(&variant.key, ()).is_some() {
                    bail!(
                        "family {:?} has duplicate variant {:?}",
                        family.key,
                        variant.key
                    );
                }
                if let Some(other) = consts.insert(&variant.const_name, id.clone()) {
                    bail!(
                        "const {:?} is claimed by both {} and {}",
                        variant.const_name,
                        other,
                        id
                    );
                }
                if !variant
                    .const_name
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                {
                    bail!(
                        "{id}: const {:?} must be SCREAMING_SNAKE_CASE",
                        variant.const_name
                    );
                }

                // The declared mode must agree with what the surface actually is,
                // otherwise light/dark-aware consumers pick the wrong companion.
                let actual = if variant.palette.bg_primary.is_light() {
                    Mode::Light
                } else {
                    Mode::Dark
                };
                if actual.as_str() != variant.mode.as_str() {
                    bail!(
                        "{id}: declared mode {:?} but bg_primary {} reads as {:?}",
                        variant.mode.as_str(),
                        variant.palette.bg_primary,
                        actual.as_str()
                    );
                }
            }
        }

        if self.accents.is_empty() {
            bail!("tokens/accents.toml declares no accents");
        }
        let mut accent_keys: BTreeMap<&str, ()> = BTreeMap::new();
        for accent in &self.accents {
            if accent_keys.insert(&accent.key, ()).is_some() {
                bail!("duplicate accent key {:?}", accent.key);
            }
            if accent.label.fr.is_empty() || accent.label.en.is_empty() {
                bail!("accent {:?} is missing a label", accent.key);
            }
        }

        Ok(())
    }
}
