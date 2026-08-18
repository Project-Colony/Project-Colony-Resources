# Theming

How a Colony palette is built, and how to add a new theme family.

## The 38 fields

A palette is 38 colours in eight role groups. The authoritative list — names,
order, groups — is the `palette!` macro in
`tools/colony-tokens/src/model.rs`; `generated/palette.schema.json` is its
machine-readable form.

| Group | Fields | What they are |
|---|---|---|
| backgrounds | `bg_primary`, `bg_sidebar`, `bg_card`, `bg_card_hover`, `bg_card_pressed`, `bg_selected`, `bg_input`, `bg_progress` | the surface depth ramp |
| text | `text_primary` … `text_dimmest`, `text_placeholder` | the ink ramp, six steps |
| accent | `accent_blue`, `accent_icon`, `accent_progress` | links, selection, emphasis |
| buttons | `btn_default`, `btn_hover`, `btn_pressed` | the interactive card ramp |
| success | `success`, `success_bg`, `btn_success{,_hover,_pressed}` | |
| warning | `warning`, `warning_bg` | |
| error | `error`, `error_light`, `error_bg`, `btn_danger_bg`, `btn_danger_hover`, `btn_trash{_hover,_pressed}` | |
| modal | `bg_modal_section`, `border_subtle`, `divider` | |

`accent_blue` is named for history, not for hue — it is *the* accent, whatever
colour it happens to be. Rose Pine's is pink.

## Duotone: one material, one ink

This is the rule that makes a themed palette read as its subject instead of as
"a dark UI with tinted links". It was validated on the Stellar Blade set and
should not be regressed.

**One surface material, one ink.**

- **Surfaces** — every background comes from a single material of the subject:
  carbon black, white ceramic, deep violet. They form a coherent depth ramp:
  sidebar (deepest) → primary → card → card_hover → card_pressed, each step
  slightly lighter in a dark theme, slightly darker in a light one.
- **The ink** — the subject's signature colour, applied to **all** text as a
  ramp (primary → secondary → muted → dim) **and** to the accent. The ink is
  what the user actually reads. That is where identity lives; a signature colour
  hidden in a border is a signature colour nobody will ever see.

Corollaries, each learned the hard way:

- **If the subject is white or very light, build a light theme.** A dark theme
  can never say "white".
- **With two or three colours**, surfaces take the darkest/most material one, the
  most luminous becomes the accent, and the third is demoted to a small role —
  a status colour. Two big saturated materials side by side clash. A khaki
  sidebar against lavender chat was tried and rejected; the khaki became
  `success` instead.
- **Status colours stay recognizable.** Success reads green-ish, warning
  amber-ish, error red-ish — tinted toward the theme's world, but never so far
  that an error stops looking like an error. A signature colour *may* take one of
  these roles, which is exactly how Kaya's khaki and Tachy's teal are used.
- **Near-black tinted backgrounds read as plain black** and will be rejected.
  Bold beats subtle.

## Legibility is not negotiable

Enforced by `cargo test`:

- `text_primary` against `bg_primary` ≥ **4.5:1**
- `text_muted` against `bg_primary` ≥ **3:1**

Darken or lighten the ink until it passes. Identity never excuses unreadable
text. There is one recorded exception, Solarized Light, because its `#657b83` on
`#fdf6e3` is upstream Solarized's own pairing and changing it would make Colony's
Solarized not Solarized. The test fails if that exception list grows stale, so an
exception cannot be quietly added and forgotten.

## Adding a theme family

1. **Create `tokens/families/<key>.toml`.** The file name must equal the `key`.
   Give it an unused `order` — that is its position in the picker.
2. **Write the palette.** All 38 fields, lowercase `#rrggbb`. Derive the fields
   that are not part of the core identity:
   - `text_dimmer` / `text_dimmest` continue the text ramp; `text_placeholder`
     equals `text_dimmer`.
   - `bg_progress` sits slightly darker than `bg_primary` in a dark theme; a
     light grey track in a light one.
   - `btn_default` / `btn_hover` / `btn_pressed` mirror the card ramp.
   - `accent_icon` and `accent_progress` follow `accent_blue`, unless a secondary
     signature hue earns `accent_icon`.
   - `success_bg` / `warning_bg` / `error_bg` are very dark tinted washes in a
     dark theme, pale pastel washes in a light one. `btn_danger_bg` equals
     `error_bg`, `btn_danger_hover` a step lighter; `error_light` is `error`
     lightened; `btn_trash_hover` / `_pressed` are a strong red, darker when
     pressed.
   - `divider` equals `border_subtle`.
3. **Set `mode`.** It must agree with `bg_primary`'s luminance — the generator
   refuses the file otherwise.
4. **Pick the swatch.** `bg` and `accent` are what the picker card shows. Choose
   the two colours that make the theme recognizable at 40 pixels.
5. **Write `notes`.** One or two sentences: what the material is, what the ink
   is, which colour was demoted and why. This is the field that stops the next
   person re-deriving your reasoning from the hexes.
6. **Both locales.** `label.fr` and `label.en`, for the family and every variant.
7. `cargo run -p colony-tokens -- generate && cargo test`.

## Character and brand sets

Palettes are not copyrightable, and there are no official colour codes for game
characters. Derive them from screenshots: read the hair, the suit, the glow, the
material. Mark the family fan-made and unofficial in its `notes`, and ship no
game art — the colours, not the assets.

`tokens/families/stellar_blade.toml` is the worked example: EVE (green ink on
white-grey ceramic, light), Tachy (amber ink and orange accent on carbon-navy),
Lily (strict gold-on-black), Enya (white ceramic, ice-blue sheen), Kaya
(lavender ink on deep violet, khaki demoted to success).

## Iterating with a human

They validate by **seeing**, not by reading hex lists. Render swatches or a UI
mockup first, ship it, then iterate live. Expect direction like "I can't see
colour X" — that means the colour is buried in a rarely-visible variable. Move it
to the text or the accent; do not merely saturate it where it is.
