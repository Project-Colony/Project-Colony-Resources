---
name: colony-theme-set
description: Port a character/branded theme set (e.g. Stellar Blade) into any Project Colony program — palette design rules, per-target wiring recipes (Rust/iced launcher, SphereCord/Discord CSS), where the canonical palettes live, and the verification checklist.
---

# Colony Theme Set — port a themed palette family into a Colony program

You are adding a themed palette family (characters, a brand, a game…) to a
Project Colony program.

## 0. The palettes live in Project-Colony-Resources. Do not re-derive them.

`tokens/families/*.toml` in this repository is the single source of truth for
every Colony palette — 25 families, 57 variants, 38 fields each, including the
Stellar Blade set.

**Never paste hex values into this skill file, into a program, or into a chat
answer as if they were authoritative.** Read them from `tokens/`, or from
`generated/themes.json` if you want them pre-parsed. A hex table written down
twice is a hex table that will disagree with itself within a month — that is the
specific failure this repository was created to end.

To change a colour: edit `tokens/families/<family>.toml`, run
`cargo run -p colony-tokens -- generate`, run `cargo test`, commit both
directories together.

## 1. The design language: DUOTONE

One surface material, one ink. Surfaces form a depth ramp from a single material
of the subject; the subject's signature colour becomes **all** the text plus the
accent. Light subjects get light themes. A second saturated colour is demoted to
a status role rather than becoming a second surface.

The full rule set, the derivation recipe for the non-identity fields, and the
legibility thresholds are in [`design/theming.md`](../../../design/theming.md).
Read it before designing a palette; it is longer than this section and it is the
authority.

## 2. Adding a family to the source of truth

Follow "Adding a theme family" in `design/theming.md`. In short: one TOML file
named after the family key, an unused `order`, all 38 fields lowercase
`#rrggbb`, a `mode` that matches `bg_primary`'s luminance, a picker `swatch`,
`notes` explaining the material and the ink, and both locales for every label.

`cargo test` then checks legibility, mode consistency, locale parity and that
nothing already-shipped drifted.

## 3. Target: Colony launcher (Rust/iced)

Once the family is in `tokens/`, the generated
`crates/colony-ui/src/generated/palettes.rs` already contains everything the launcher needs:

- `ThemePalette::<CONST>` for each variant
- `THEME_FAMILIES`, the ordered picker catalog with glyphs and swatches
- `resolve(family, variant)` and `FALLBACK_PALETTE`
- `ACCENT_OVERRIDES` and `accent_key_to_color(key)`

A launcher consuming that file needs **no** per-family edit: no new match arm, no
new entry in a `theme_families` vec, no i18n edit. That is the whole point.
Labels come from `crates/colony-ui/src/generated/labels.{fr,en}.json`, embedded in the crate.

Until a program is migrated it still carries its own hand-maintained copies, and
adding a family there means the four manual edits it always did (palette consts,
resolver arm, picker vec, both locale files). Prefer migrating the program.

## 4. Target: SphereCord (Discord CSS)

SphereCord historically downloaded Colony's `src/ui/theme.rs` over HTTP and
regex-parsed the Rust source to recover palettes — and its companion fetch of
`src/i18n.rs` had already gone stale after Colony split that file into
`src/i18n/{fr,en}.rs`.

Replace that with `generated/themes.json`, or load
`generated/css/colony-themes.css` directly. The generated CSS exposes neutral
`--colony-*` custom properties under `[data-colony-theme="<family>-<variant>"]`;
mapping those onto Discord's `--background-base-*` / `--text-*` / `--brand-*`
variables stays in SphereCord's own repo.

Watch the dedupe rule: SphereCord ships some families natively. Any family it
generates itself must be skipped when it walks the shared catalog, or the set
ships twice.

## 5. Porting to any other Colony program

1. Find its palette shape — a struct, a token map, CSS variables — and identify
   the four role groups: surface ramp, text/ink ramp, accent(s), status colours.
2. Map the shared fields onto those roles. `generated/themes.json` carries the
   group for every field, so the mapping is mechanical.
3. Wire family and variants wherever the program enumerates themes: picker UI,
   resolver, i18n in **all** locales.
4. Verify: compiler and tests green; look at one dark and one light theme;
   `text_primary` on `bg_primary` ≥ 4.5:1.

## 6. Iteration protocol

The user validates by **seeing**, not by reading hex lists. Render swatches or a
UI mockup first, ship, then iterate live. "I can't see colour X" means the colour
is buried in a rarely-visible variable — move it to the text or the accent rather
than merely saturating it in place. Bold beats subtle; near-black tinted
backgrounds read as plain black and get rejected.
