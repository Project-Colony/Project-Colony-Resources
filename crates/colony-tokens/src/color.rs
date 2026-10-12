//! sRGB colour with `#rrggbb` (de)serialization and WCAG contrast maths.

use std::fmt;

use serde::de::{self, Deserialize, Deserializer};
use serde::Serialize;

/// An opaque sRGB colour. Stored as `0xRRGGBB` so equality is exact and cheap —
/// palettes are compared byte-for-byte in the regression test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color(pub u32);

impl Color {
    pub fn rgb(self) -> (u8, u8, u8) {
        (
            ((self.0 >> 16) & 0xFF) as u8,
            ((self.0 >> 8) & 0xFF) as u8,
            (self.0 & 0xFF) as u8,
        )
    }

    /// `#rrggbb`, always lowercase — the canonical form used everywhere we emit.
    pub fn to_css_hex(self) -> String {
        format!("#{:06x}", self.0)
    }

    /// `0xrrggbb`, for the Rust `hex(...)` literals.
    pub fn to_rust_hex(self) -> String {
        format!("0x{:06x}", self.0)
    }

    /// WCAG 2.1 relative luminance.
    pub fn luminance(self) -> f64 {
        fn channel(c: u8) -> f64 {
            let c = c as f64 / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        let (r, g, b) = self.rgb();
        0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
    }

    /// WCAG 2.1 contrast ratio, from 1.0 (identical) to 21.0 (black on white).
    pub fn contrast(self, other: Color) -> f64 {
        let (a, b) = (self.luminance(), other.luminance());
        let (hi, lo) = if a > b { (a, b) } else { (b, a) };
        (hi + 0.05) / (lo + 0.05)
    }

    /// Whether a surface reads as light. Drives the `mode` cross-check.
    pub fn is_light(self) -> bool {
        self.luminance() > 0.5
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_css_hex())
    }
}

impl std::str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s
            .strip_prefix('#')
            .ok_or_else(|| format!("colour {s:?} must start with '#' (expected \"#rrggbb\")"))?;
        if hex.len() != 6 {
            return Err(format!(
                "colour {s:?} must have exactly 6 hex digits (expected \"#rrggbb\")"
            ));
        }
        if let Some(bad) = hex.chars().find(|c| !c.is_ascii_hexdigit()) {
            return Err(format!("colour {s:?} contains a non-hex character {bad:?}"));
        }
        // Lowercase is the house style; uppercase parses fine but is normalized
        // away on the next `generate`, so flag it rather than silently rewriting.
        if hex.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(format!("colour {s:?} must be lowercase (write \"{}\")", {
                let mut s = String::from("#");
                s.push_str(&hex.to_ascii_lowercase());
                s
            }));
        }
        u32::from_str_radix(hex, 16)
            .map(Color)
            .map_err(|e| format!("colour {s:?}: {e}"))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(de::Error::custom)
    }
}

impl Serialize for Color {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_css_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_round_trips() {
        let c: Color = "#eff1f5".parse().unwrap();
        assert_eq!(c, Color(0xeff1f5));
        assert_eq!(c.to_css_hex(), "#eff1f5");
        assert_eq!(c.to_rust_hex(), "0xeff1f5");
    }

    #[test]
    fn rejects_malformed_colours() {
        for bad in ["eff1f5", "#eff1f", "#gggggg", "#EFF1F5"] {
            assert!(bad.parse::<Color>().is_err(), "{bad} should not parse");
        }
    }

    #[test]
    fn contrast_matches_wcag_reference_values() {
        let black = Color(0x000000);
        let white = Color(0xffffff);
        assert!((black.contrast(white) - 21.0).abs() < 1e-9);
        assert!((white.contrast(white) - 1.0).abs() < 1e-9);
        // Symmetric regardless of argument order.
        assert!((black.contrast(white) - white.contrast(black)).abs() < 1e-9);
    }

    #[test]
    fn light_and_dark_surfaces_are_classified() {
        assert!(Color(0xfbf1c7).is_light());
        assert!(!Color(0x282828).is_light());
    }
}
