//! Zero-regression guarantee.
//!
//! `tokens/` was imported from Project Colony's launcher, so the palettes it
//! produces must still be the palettes Colony shipped. These tests parse the
//! verbatim snapshot in `fixtures/` and compare it against what the generator
//! produces today — const by const, field by field.
//!
//! When a colour is changed on purpose, this test fails on exactly the entries
//! that moved. Re-cut the snapshot only when that diff is the intended one.

use std::collections::{BTreeMap, BTreeSet};

use colony_tokens::model::Tokens;

const SNAPSHOT: &str = include_str!("fixtures/colony-theme-rs.snapshot");

/// `NAME -> [(field, 0xRRGGBB)]`, in declaration order.
type Consts = BTreeMap<String, Vec<(String, u32)>>;

/// Parse every `pub const NAME: Self = Self { field: hex(0x……), … };` block.
fn parse_palette_consts(src: &str) -> Consts {
    const ANCHOR: &str = ": Self = Self {";
    let mut out = Consts::new();
    let mut pos = 0;

    while let Some(rel) = src[pos..].find(ANCHOR) {
        let at = pos + rel;

        // The constant's name is the identifier immediately before the anchor.
        let name: String = {
            let mut chars: Vec<char> = src[..at]
                .chars()
                .rev()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            chars.reverse();
            chars.into_iter().collect()
        };
        assert!(!name.is_empty(), "found a palette literal with no name");

        let body_start = at + ANCHOR.len();
        let end = src[body_start..]
            .find("\n    };")
            .unwrap_or_else(|| panic!("{name}: unterminated palette literal"));
        let body = &src[body_start..body_start + end];

        let mut fields = Vec::new();
        for line in body.lines() {
            // Colony annotates some colours inline ("// warm parchment"); the
            // note is prose, the value is what we compare.
            let line = line.split("//").next().unwrap_or(line).trim();
            if line.is_empty() {
                continue;
            }
            let (field, value) = line
                .split_once(':')
                .unwrap_or_else(|| panic!("{name}: cannot parse line {line:?}"));
            let hex = value
                .trim()
                .trim_end_matches(',')
                .trim()
                .strip_prefix("hex(0x")
                .and_then(|v| v.strip_suffix(')'))
                .unwrap_or_else(|| panic!("{name}: unexpected value in {line:?}"));
            let value = u32::from_str_radix(hex, 16)
                .unwrap_or_else(|e| panic!("{name}: bad hex {hex:?}: {e}"));
            fields.push((field.trim().to_string(), value));
        }

        assert_eq!(fields.len(), 38, "{name} should have 38 fields");
        assert!(
            out.insert(name.clone(), fields).is_none(),
            "{name} is declared twice"
        );
        pos = body_start + end;
    }

    out
}

/// Parse `("family", "variant") => ThemePalette::CONST,` arms.
fn parse_resolver(src: &str) -> BTreeMap<(String, String), String> {
    let mut out = BTreeMap::new();
    for line in src.lines() {
        let line = line.trim();
        if !line.starts_with('(') {
            continue;
        }
        let Some((pair, target)) = line.split_once("=> ThemePalette::") else {
            continue;
        };
        let keys: Vec<String> = pair
            .split('"')
            .skip(1)
            .step_by(2)
            .map(str::to_string)
            .collect();
        assert_eq!(keys.len(), 2, "cannot parse resolver arm {line:?}");
        let target = target.trim().trim_end_matches(',').to_string();
        out.insert((keys[0].clone(), keys[1].clone()), target);
    }
    out
}

fn parse_fallback(src: &str) -> String {
    src.lines()
        .find_map(|l| l.trim().strip_prefix("_ => ThemePalette::"))
        .or_else(|| {
            src.lines().find_map(|l| {
                l.trim()
                    .strip_prefix("pub const FALLBACK_PALETTE: ThemePalette = ThemePalette::")
            })
        })
        .map(|s| s.trim().trim_end_matches([',', ';']).to_string())
        .expect("no fallback palette found")
}

/// The Rust artifact as the generator would write it right now.
fn generated_rust() -> String {
    let root = colony_tokens::repo_root().expect("repository root");
    let tokens = Tokens::load(&root.join("tokens")).expect("tokens load");
    let artifacts = colony_tokens::plan(&tokens).expect("plan");
    artifacts
        .into_iter()
        .find(|a| a.path.ends_with("palettes.rs"))
        .expect("palettes.rs is part of the plan")
        .contents
}

#[test]
fn every_colony_palette_survives_the_round_trip() {
    let before = parse_palette_consts(SNAPSHOT);
    let after = parse_palette_consts(&generated_rust());

    assert_eq!(before.len(), 57, "the snapshot should hold 57 palettes");

    let names_before: BTreeSet<&String> = before.keys().collect();
    let names_after: BTreeSet<&String> = after.keys().collect();
    assert_eq!(
        names_before, names_after,
        "the set of palette constants changed"
    );

    let mut drifted = Vec::new();
    for (name, expected) in &before {
        let actual = &after[name];
        if expected != actual {
            for (i, (field, value)) in expected.iter().enumerate() {
                match actual.get(i) {
                    Some((f, v)) if f == field && v == value => {}
                    Some((f, v)) => {
                        drifted.push(format!("{name}.{field}=#{value:06x} -> {f}=#{v:06x}"))
                    }
                    None => drifted.push(format!("{name}.{field} is missing")),
                }
            }
        }
    }
    assert!(
        drifted.is_empty(),
        "palette values drifted:\n  {}",
        drifted.join("\n  ")
    );
}

#[test]
fn the_resolver_still_maps_to_the_same_constants() {
    let before = parse_resolver(SNAPSHOT);
    let after = parse_resolver(&generated_rust());
    assert_eq!(
        before.len(),
        57,
        "the snapshot should hold 57 resolver arms"
    );
    assert_eq!(
        before, after,
        "a (family, variant) pair now resolves to a different palette"
    );
}

#[test]
fn the_fallback_palette_is_unchanged() {
    // An existing config naming a removed theme must land where it always did.
    assert_eq!(parse_fallback(SNAPSHOT), parse_fallback(&generated_rust()));
}

#[test]
fn every_variant_carries_its_swatch_and_labels() {
    let root = colony_tokens::repo_root().expect("repository root");
    let tokens = Tokens::load(&root.join("tokens")).expect("tokens load");

    assert_eq!(tokens.families.len(), 25);
    assert_eq!(tokens.variant_count(), 57);

    for (family, variant) in tokens.variants() {
        let id = format!("{}/{}", family.key, variant.key);
        assert!(!variant.label.fr.is_empty(), "{id}: empty fr label");
        assert!(!variant.label.en.is_empty(), "{id}: empty en label");
        // The swatch is what the user actually clicks on in the picker; if it
        // does not resemble the palette it selects, the picker lies.
        assert_eq!(
            variant.swatch.bg.is_light(),
            variant.palette.bg_primary.is_light(),
            "{id}: swatch background and bg_primary disagree on light/dark"
        );
    }
}

/// Colony's `accent_key_to_color`, verbatim from `src/ui/theme.rs` at import
/// time. Both the values and the ORDER matter: `app_tint()` buckets a hash of
/// each app's name into this list, so a reorder silently re-colours every
/// installed app's icon.
const COLONY_ACCENTS: &[(&str, u32)] = &[
    ("red", 0xE05555),
    ("orange", 0xE0855A),
    ("yellow", 0xC8A832),
    ("green", 0x55B87A),
    ("blue", 0x6B8BD6),
    ("indigo", 0x7B6BD6),
    ("violet", 0xB06BD6),
    ("amber", 0xD4A030),
];

#[test]
fn accent_overrides_match_colony_in_value_and_order() {
    let root = colony_tokens::repo_root().expect("repository root");
    let tokens = Tokens::load(&root.join("tokens")).expect("tokens load");

    let actual: Vec<(String, u32)> = tokens
        .accents
        .iter()
        .map(|a| (a.key.clone(), a.color.0))
        .collect();
    let expected: Vec<(String, u32)> = COLONY_ACCENTS
        .iter()
        .map(|(k, v)| ((*k).to_string(), *v))
        .collect();

    assert_eq!(
        actual, expected,
        "accent overrides drifted from Colony — per-app identity tints would change"
    );
}

/// Solarized Light is the one palette below AA, and deliberately so: `#657b83`
/// on `#fdf6e3` is upstream Solarized's own `base00` on `base3`. Changing it
/// would make Colony's Solarized not Solarized. Every other theme must pass.
const CONTRAST_EXCEPTIONS: &[(&str, &str)] = &[("solarized", "light")];

#[test]
fn text_is_legible_on_every_theme() {
    let root = colony_tokens::repo_root().expect("repository root");
    let tokens = Tokens::load(&root.join("tokens")).expect("tokens load");

    let mut failures = Vec::new();
    let mut exceptions_hit = BTreeSet::new();

    for (family, variant) in tokens.variants() {
        let p = &variant.palette;
        let id = format!("{}/{}", family.key, variant.key);
        let excepted = CONTRAST_EXCEPTIONS
            .iter()
            .any(|(f, v)| *f == family.key && *v == variant.key);

        let primary = p.text_primary.contrast(p.bg_primary);
        if primary < 4.5 {
            if excepted {
                exceptions_hit.insert(id.clone());
            } else {
                failures.push(format!("{id}: text_primary {primary:.2}:1 < 4.5:1"));
            }
        }

        let muted = p.text_muted.contrast(p.bg_primary);
        if muted < 3.0 {
            failures.push(format!("{id}: text_muted {muted:.2}:1 < 3.0:1"));
        }
    }

    assert!(
        failures.is_empty(),
        "illegible text:\n  {}",
        failures.join("\n  ")
    );

    // Keep the exception list honest: an entry that no longer fails should be
    // deleted rather than left behind as folklore.
    let declared: BTreeSet<String> = CONTRAST_EXCEPTIONS
        .iter()
        .map(|(f, v)| format!("{f}/{v}"))
        .collect();
    assert_eq!(
        declared, exceptions_hit,
        "CONTRAST_EXCEPTIONS is stale — an entry listed there now passes"
    );
}

#[test]
fn generated_is_up_to_date_with_tokens() {
    // The same check CI runs, so a forgotten `generate` fails locally first.
    let root = colony_tokens::repo_root().expect("repository root");
    let problems = colony_tokens::check(&root).expect("check");
    assert!(
        problems.is_empty(),
        "generated/ is out of date — run `cargo run -p colony-tokens -- generate`:\n  {}",
        problems.join("\n  ")
    );
}
