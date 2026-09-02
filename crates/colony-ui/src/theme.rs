//! The Colony palette: its shape, the active-theme state, and the accessors
//! every program styles its widgets with.
//!
//! The 57 palettes, the `(family, variant)` resolver, the picker catalog and the
//! accent overrides are **not** written here — they are generated from
//! `tokens/` and pulled in by the `include!` below. Adding a theme family
//! therefore touches no Rust at all.

use std::sync::RwLock;

use iced::Color;

/// Runtime theme palette — all semantic UI colors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemePalette {
    // --- Backgrounds ---
    pub bg_primary: Color,
    pub bg_sidebar: Color,
    pub bg_card: Color,
    pub bg_card_hover: Color,
    pub bg_card_pressed: Color,
    pub bg_selected: Color,
    pub bg_input: Color,
    pub bg_progress: Color,

    // --- Text ---
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_dim: Color,
    pub text_dimmer: Color,
    pub text_dimmest: Color,
    pub text_placeholder: Color,

    // --- Accent ---
    pub accent_blue: Color,
    pub accent_icon: Color,
    pub accent_progress: Color,

    // --- Buttons ---
    pub btn_default: Color,
    pub btn_hover: Color,
    pub btn_pressed: Color,

    // --- Success ---
    pub success: Color,
    pub success_bg: Color,
    pub btn_success: Color,
    pub btn_success_hover: Color,
    pub btn_success_pressed: Color,

    // --- Warning ---
    pub warning: Color,
    pub warning_bg: Color,

    // --- Error ---
    pub error: Color,
    pub error_light: Color,
    pub error_bg: Color,
    pub btn_danger_bg: Color,
    pub btn_danger_hover: Color,
    pub btn_trash_hover: Color,
    pub btn_trash_pressed: Color,

    // --- Modal ---
    pub bg_modal_section: Color,
    pub border_subtle: Color,
    pub divider: Color,
}

/// Compile-time `0xRRGGBB` → [`Color`].
pub const fn hex(h: u32) -> Color {
    let r = ((h >> 16) & 0xFF) as f32 / 255.0;
    let g = ((h >> 8) & 0xFF) as f32 / 255.0;
    let b = (h & 0xFF) as f32 / 255.0;
    Color { r, g, b, a: 1.0 }
}

// Every palette const, `THEME_FAMILIES`, `resolve()`, `FALLBACK_PALETTE`,
// `ACCENT_OVERRIDES` and `accent_key_to_color()`. Generated from tokens/ —
// run `cargo run -p colony-tokens -- generate` after editing a colour.
include!("generated/palettes.rs");

impl ThemeVariantMeta {
    /// The picker card's background, as a colour rather than a raw `0xRRGGBB`.
    pub fn swatch_bg_color(&self) -> Color {
        hex(self.swatch_bg)
    }

    /// The picker card's accent bar.
    pub fn swatch_accent_color(&self) -> Color {
        hex(self.swatch_accent)
    }

    /// Whether this variant is a light theme, from its declared mode.
    pub fn is_light(&self) -> bool {
        self.mode == "light"
    }
}

impl ThemeFamilyMeta {
    /// Look up one of this family's variants by key.
    pub fn variant(&self, key: &str) -> Option<&'static ThemeVariantMeta> {
        self.variants.iter().find(|v| v.key == key)
    }
}

/// Look up a family in the catalog.
pub fn family(key: &str) -> Option<&'static ThemeFamilyMeta> {
    THEME_FAMILIES.iter().find(|f| f.key == key)
}

// ── Global active state ──

static ACTIVE_PALETTE: RwLock<ThemePalette> = RwLock::new(FALLBACK_PALETTE);

/// User-chosen accent override. `None` means "auto" — use the theme's own.
static ACTIVE_ACCENT: RwLock<Option<Color>> = RwLock::new(None);

static HIGH_CONTRAST: RwLock<bool> = RwLock::new(false);

/// Set the active palette from the `(family, variant)` keys stored in config.
/// An unknown pair resolves to [`FALLBACK_PALETTE`] rather than failing, so an
/// old config or a removed family degrades instead of breaking startup.
pub fn set_active_theme(family: &str, variant: &str) {
    *ACTIVE_PALETTE.write().unwrap() = resolve(family, variant);
}

/// Read the current palette, with high contrast applied when it is on.
pub fn active_palette() -> ThemePalette {
    let base = *ACTIVE_PALETTE.read().unwrap();
    if is_high_contrast() {
        base.with_high_contrast()
    } else {
        base
    }
}

/// Set the user accent override. `None` restores "auto".
pub fn set_active_accent(color: Option<Color>) {
    *ACTIVE_ACCENT.write().unwrap() = color;
}

/// The accent override currently set, if any.
pub fn active_accent() -> Option<Color> {
    *ACTIVE_ACCENT.read().unwrap()
}

/// The effective accent: the user's override, or the theme's own.
pub fn effective_accent() -> Color {
    ACTIVE_ACCENT
        .read()
        .unwrap()
        .unwrap_or_else(|| active_palette().accent_blue)
}

pub fn set_high_contrast(enabled: bool) {
    *HIGH_CONTRAST.write().unwrap() = enabled;
}

pub fn is_high_contrast() -> bool {
    *HIGH_CONTRAST.read().unwrap()
}

impl ThemePalette {
    /// Boost contrast for accessibility: brighten text, darken backgrounds,
    /// sharpen borders. Derived from the active palette so a theme never has to
    /// ship a separate high-contrast twin.
    pub fn with_high_contrast(mut self) -> Self {
        fn boost(c: Color, amount: f32) -> Color {
            Color {
                r: (c.r + amount).min(1.0),
                g: (c.g + amount).min(1.0),
                b: (c.b + amount).min(1.0),
                a: c.a,
            }
        }
        fn darken(c: Color, amount: f32) -> Color {
            Color {
                r: (c.r - amount).max(0.0),
                g: (c.g - amount).max(0.0),
                b: (c.b - amount).max(0.0),
                a: c.a,
            }
        }

        if self.bg_primary.is_light() {
            self.text_primary = darken(self.text_primary, 0.15);
            self.text_secondary = darken(self.text_secondary, 0.12);
            self.text_muted = darken(self.text_muted, 0.10);
            self.text_dim = darken(self.text_dim, 0.10);
            self.text_dimmer = darken(self.text_dimmer, 0.08);
            self.border_subtle = darken(self.border_subtle, 0.15);
            self.divider = darken(self.divider, 0.15);
        } else {
            self.text_primary = boost(self.text_primary, 0.12);
            self.text_secondary = boost(self.text_secondary, 0.10);
            self.text_muted = boost(self.text_muted, 0.10);
            self.text_dim = boost(self.text_dim, 0.08);
            self.text_dimmer = boost(self.text_dimmer, 0.08);
            self.border_subtle = boost(self.border_subtle, 0.12);
            self.divider = boost(self.divider, 0.12);
        }
        self
    }
}

/// Perceptual helpers on iced's [`Color`], shared so every program answers
/// "is this surface light?" the same way.
// `Color` is `Copy`, so taking `self` by value is both correct and cheaper than
// a reference — the same choice `f32::is_nan(self)` makes in std.
#[allow(clippy::wrong_self_convention)]
pub trait ColorExt {
    /// YIQ luminance — the perceptual weighting used throughout Colony.
    fn luma(self) -> f32;
    /// Whether the colour reads as a light surface.
    fn is_light(self) -> bool;
}

impl ColorExt for Color {
    fn luma(self) -> f32 {
        0.299 * self.r + 0.587 * self.g + 0.114 * self.b
    }

    fn is_light(self) -> bool {
        self.luma() > 0.5
    }
}

/// Pick a legible foreground — near-black or near-white — for a glyph drawn on
/// top of `bg`.
///
/// Chosen by the WCAG contrast each end actually achieves, not by a luminance
/// threshold. A threshold has to guess where the crossover is, and it guessed
/// wrong for the green accent: at YIQ 0.578 it took the near-white branch for a
/// 2.34:1 check mark where near-black gives 5.9:1. Comparing the two ends is
/// both simpler and exact.
pub fn contrast_on(bg: Color) -> Color {
    const INK: Color = Color {
        r: 0.08,
        g: 0.08,
        b: 0.10,
        a: 1.0,
    };
    const PAPER: Color = Color {
        r: 0.97,
        g: 0.98,
        b: 1.0,
        a: 1.0,
    };
    if contrast_ratio(INK, bg) >= contrast_ratio(PAPER, bg) {
        INK
    } else {
        PAPER
    }
}

/// WCAG relative luminance.
fn relative_luminance(c: Color) -> f32 {
    fn channel(v: f32) -> f32 {
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * channel(c.r) + 0.7152 * channel(c.g) + 0.0722 * channel(c.b)
}

/// WCAG contrast ratio between two colours, 1.0 to 21.0.
///
/// Distinct from [`ColorExt::luma`], which is the cheaper YIQ approximation and
/// the right tool for "is this surface light?". Deciding whether a glyph is
/// legible needs the real curve.
pub fn contrast_ratio(a: Color, b: Color) -> f32 {
    let (la, lb) = (relative_luminance(a), relative_luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

/// Deterministic identity tint for a program, derived from its NAME only —
/// stable across install, uninstall and machines. Buckets the name's hash into
/// the shared accents so every program gets a distinct, palette-harmonious
/// colour without shipping an icon.
///
/// The bucket is an index into [`ACCENT_OVERRIDES`], which is why that list is
/// append-only: reordering it re-colours every installed program.
pub fn app_tint(name: &str) -> Color {
    // Classic string hash: h = c + h*31, wrapping.
    let mut h: i64 = 0;
    for b in name.bytes() {
        h = (b as i64).wrapping_add(h.wrapping_shl(5)).wrapping_sub(h);
    }
    let idx = (h.unsigned_abs() % ACCENT_OVERRIDES.len() as u64) as usize;
    hex(ACCENT_OVERRIDES[idx].color)
}

// ── Public façade ──

/// Screaming-case accessors for the active palette, so a widget reads
/// `Palette::TEXT_PRIMARY()` rather than `active_palette().text_primary`.
///
/// This is the API Colony's widgets already use; it exists so migrating a
/// program to this crate is an import change rather than a rewrite.
pub struct Palette;

#[allow(non_snake_case)]
impl Palette {
    // Backgrounds
    pub fn BG_PRIMARY() -> Color {
        active_palette().bg_primary
    }
    pub fn BG_SIDEBAR() -> Color {
        active_palette().bg_sidebar
    }
    pub fn BG_CARD() -> Color {
        active_palette().bg_card
    }
    pub fn BG_CARD_HOVER() -> Color {
        active_palette().bg_card_hover
    }
    pub fn BG_CARD_PRESSED() -> Color {
        active_palette().bg_card_pressed
    }
    pub fn BG_SELECTED() -> Color {
        active_palette().bg_selected
    }
    pub fn BG_INPUT() -> Color {
        active_palette().bg_input
    }
    pub fn BG_PROGRESS() -> Color {
        active_palette().bg_progress
    }

    // Text
    pub fn TEXT_PRIMARY() -> Color {
        active_palette().text_primary
    }
    pub fn TEXT_SECONDARY() -> Color {
        active_palette().text_secondary
    }
    pub fn TEXT_MUTED() -> Color {
        active_palette().text_muted
    }
    pub fn TEXT_DIM() -> Color {
        active_palette().text_dim
    }
    pub fn TEXT_DIMMER() -> Color {
        active_palette().text_dimmer
    }
    pub fn TEXT_DIMMEST() -> Color {
        active_palette().text_dimmest
    }
    pub fn TEXT_PLACEHOLDER() -> Color {
        active_palette().text_placeholder
    }

    // Accent — ACCENT() is the user's override or the theme default.
    pub fn ACCENT() -> Color {
        effective_accent()
    }
    pub fn ACCENT_ICON() -> Color {
        active_palette().accent_icon
    }
    pub fn ACCENT_PROGRESS() -> Color {
        active_palette().accent_progress
    }

    // Buttons
    pub fn BTN_DEFAULT() -> Color {
        active_palette().btn_default
    }
    pub fn BTN_HOVER() -> Color {
        active_palette().btn_hover
    }
    pub fn BTN_PRESSED() -> Color {
        active_palette().btn_pressed
    }

    // Success
    pub fn SUCCESS() -> Color {
        active_palette().success
    }
    pub fn SUCCESS_BG() -> Color {
        active_palette().success_bg
    }
    pub fn BTN_SUCCESS() -> Color {
        active_palette().btn_success
    }
    pub fn BTN_SUCCESS_HOVER() -> Color {
        active_palette().btn_success_hover
    }
    pub fn BTN_SUCCESS_PRESSED() -> Color {
        active_palette().btn_success_pressed
    }

    // Warning
    pub fn WARNING() -> Color {
        active_palette().warning
    }
    pub fn WARNING_BG() -> Color {
        active_palette().warning_bg
    }

    // Error
    pub fn ERROR() -> Color {
        active_palette().error
    }
    pub fn ERROR_LIGHT() -> Color {
        active_palette().error_light
    }
    pub fn ERROR_BG() -> Color {
        active_palette().error_bg
    }
    pub fn BTN_DANGER_BG() -> Color {
        active_palette().btn_danger_bg
    }
    pub fn BTN_DANGER_HOVER() -> Color {
        active_palette().btn_danger_hover
    }
    pub fn BTN_TRASH_HOVER() -> Color {
        active_palette().btn_trash_hover
    }
    pub fn BTN_TRASH_PRESSED() -> Color {
        active_palette().btn_trash_pressed
    }

    // Modal
    pub fn BG_MODAL_SECTION() -> Color {
        active_palette().bg_modal_section
    }
    pub fn BORDER_SUBTLE() -> Color {
        active_palette().border_subtle
    }
    pub fn DIVIDER() -> Color {
        active_palette().divider
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    /// The active theme is process-global, so tests that set it must not run
    /// concurrently with each other.
    static GLOBALS: Mutex<()> = Mutex::new(());

    fn exclusive() -> MutexGuard<'static, ()> {
        GLOBALS.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn resolves_every_catalogued_theme() {
        for family in THEME_FAMILIES {
            for variant in family.variants {
                assert_eq!(
                    resolve(family.key, variant.key),
                    variant.palette,
                    "{}/{} resolves to a different palette than its catalog entry",
                    family.key,
                    variant.key
                );
            }
        }
    }

    #[test]
    fn an_unknown_theme_falls_back_instead_of_failing() {
        // An old config, a hand-edited file or a removed family must degrade,
        // not stop the program from starting.
        assert_eq!(resolve("no_such_family", "dark"), FALLBACK_PALETTE);
        assert_eq!(resolve("gruvbox", "no_such_variant"), FALLBACK_PALETTE);
    }

    #[test]
    fn the_catalog_covers_every_family_exactly_once() {
        let mut keys: Vec<&str> = THEME_FAMILIES.iter().map(|f| f.key).collect();
        let count = keys.len();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), count, "a family key is listed twice");
        assert!(
            count >= 25,
            "expected the imported catalog, got {count} families"
        );
    }

    #[test]
    fn declared_mode_matches_the_actual_surface() {
        for family in THEME_FAMILIES {
            for variant in family.variants {
                let actual = if variant.palette.bg_primary.is_light() {
                    "light"
                } else {
                    "dark"
                };
                assert_eq!(
                    variant.mode, actual,
                    "{}/{} claims {} but its bg_primary reads {}",
                    family.key, variant.key, variant.mode, actual
                );
            }
        }
    }

    #[test]
    fn setting_a_theme_changes_what_the_facade_reads() {
        let _guard = exclusive();

        set_active_theme("gruvbox", "dark");
        assert_eq!(Palette::BG_PRIMARY(), ThemePalette::GRUVBOX_DARK.bg_primary);

        set_active_theme("catppuccin", "latte");
        assert_eq!(
            Palette::BG_PRIMARY(),
            ThemePalette::CATPPUCCIN_LATTE.bg_primary
        );

        set_active_theme("gruvbox", "dark");
    }

    #[test]
    fn the_accent_override_wins_over_the_theme_and_none_restores_auto() {
        let _guard = exclusive();

        set_active_theme("gruvbox", "dark");
        set_active_accent(None);
        assert_eq!(
            Palette::ACCENT(),
            ThemePalette::GRUVBOX_DARK.accent_blue,
            "auto should fall back to the theme's own accent"
        );

        let violet = accent_key_to_color("violet").expect("violet is a shipped accent");
        set_active_accent(Some(violet));
        assert_eq!(Palette::ACCENT(), violet);

        set_active_accent(None);
        assert_eq!(Palette::ACCENT(), ThemePalette::GRUVBOX_DARK.accent_blue);
    }

    #[test]
    fn high_contrast_pushes_text_away_from_the_background() {
        let _guard = exclusive();

        for (family, variant) in THEME_FAMILIES
            .iter()
            .flat_map(|f| f.variants.iter().map(move |v| (f, v)))
        {
            let base = variant.palette;
            let boosted = base.with_high_contrast();
            let id = format!("{}/{}", family.key, variant.key);

            if base.bg_primary.is_light() {
                assert!(
                    boosted.text_primary.luma() <= base.text_primary.luma(),
                    "{id}: light theme should darken text"
                );
            } else {
                assert!(
                    boosted.text_primary.luma() >= base.text_primary.luma(),
                    "{id}: dark theme should brighten text"
                );
            }
            // The surface itself must not move — only the ink and the borders.
            assert_eq!(boosted.bg_primary, base.bg_primary, "{id}");
        }
    }

    #[test]
    fn high_contrast_applies_through_the_accessor() {
        let _guard = exclusive();

        set_active_theme("gruvbox", "dark");
        set_high_contrast(false);
        let plain = active_palette();
        set_high_contrast(true);
        let boosted = active_palette();
        set_high_contrast(false);

        assert_ne!(plain.text_primary, boosted.text_primary);
        assert_eq!(active_palette().text_primary, plain.text_primary);
    }

    /// Expected buckets computed independently from Colony's hash
    /// (`h = byte + h*31`, wrapping, then `|h| % 8`) rather than by running
    /// this implementation — otherwise the test proves nothing.
    const KNOWN_TINTS: &[(&str, &str)] = &[
        ("Colony", "yellow"),
        ("Eidos", "blue"),
        ("Grape", "green"),
        ("Digger", "yellow"),
        ("SphereCord", "green"),
        ("", "red"),
    ];

    #[test]
    fn app_tint_buckets_names_the_way_colony_does() {
        for (name, expected_key) in KNOWN_TINTS {
            let expected = accent_key_to_color(expected_key).expect("shipped accent");
            assert_eq!(
                app_tint(name),
                expected,
                "{name:?} should tint {expected_key}"
            );
        }
    }

    #[test]
    fn app_tint_is_stable_and_always_a_shipped_accent() {
        for name in ["Colony", "Eidos", "a", "a much longer program name"] {
            let first = app_tint(name);
            assert_eq!(first, app_tint(name), "{name}: not deterministic");
            assert!(
                ACCENT_OVERRIDES.iter().any(|a| hex(a.color) == first),
                "{name}: tinted with a colour that is not a shipped accent"
            );
        }
    }

    #[test]
    fn contrast_on_picks_a_legible_foreground() {
        assert!(contrast_on(Color::WHITE).luma() < 0.2);
        assert!(contrast_on(Color::BLACK).luma() > 0.8);

        for family in THEME_FAMILIES {
            for variant in family.variants {
                let fg = contrast_on(variant.swatch_bg_color());
                assert!(
                    (fg.luma() - variant.swatch_bg_color().luma()).abs() > 0.3,
                    "{}/{}: glyph would be unreadable on its own swatch",
                    family.key,
                    variant.key
                );
            }
        }
    }

    #[test]
    fn hex_round_trips() {
        let c = hex(0x1e66f5);
        assert_eq!((c.r * 255.0).round() as u32, 0x1e);
        assert_eq!((c.g * 255.0).round() as u32, 0x66);
        assert_eq!((c.b * 255.0).round() as u32, 0xf5);
        assert_eq!(c.a, 1.0);
    }
}
