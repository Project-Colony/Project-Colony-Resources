# Changelog

Notable changes to the `colony-ui` crate. Versions follow semver; while the
crate is 0.x, anything that breaks a consumer bumps the minor version.

## 0.1.6

### Features

* **Preferences kit.** `widgets::PreferencesView` builds General's heading and
  the whole of Appearance (theme, colors and auto accent, typography, effects,
  live preview) and Accessibility (high contrast, dyslexia-friendly font,
  reduced motion, text size) from the user's preferences. Every section is
  collapsible and available on its own.
* **Preferences page.** `preferences_page`, `preferences_header`,
  `category_list`, `category_heading`, `section_description`, `setting_row`
  and `dropdown`, plus `STANDARD_CATEGORIES` and `ABOUT_CATEGORY`.
* **Navigation.** `identity_button` (the program's name as the way into
  Preferences, with its open state), `nav_item`, `selection_style`,
  `nav_label` and `nav_hint`. Selection is an accent background with its label
  in `contrast_on(accent)`.
* **`preferences::StandardPreferences`**: theme, accent, auto accent, both text
  sizes, animations, high contrast, dyslexia font and reduced motion, with
  `apply()` setting every global and `update(Change)` taking a change from the
  widgets. It reads the field names Colony already writes, embeds with
  `#[serde(flatten)]`, and never fails on a value: anything it does not
  understand falls back to its default.
* **Typography.** `FontSize` and `TextSize` with their globals, `font_scale()`,
  `sz()` and `Typography::current()`.
* **Fonts.** JetBrainsMono Nerd Font (Regular, Medium, Bold), OpenDyslexic and
  Font Awesome 6 Free embedded behind the default `fonts` feature, with their
  SIL OFL 1.1 texts; `ui_font()`, `ui_font_medium()` and `ui_font_bold()`
  follow the dyslexia toggle.
* **Motion.** `set_reduced_motion`, `set_effects` and `effects_enabled()`, which
  is false whenever reduced motion is on.
* **Labels.** The Preferences page's strings in English and French, generated
  from `tokens/labels.toml` into the existing `i18n` tables.
* `is_high_contrast` is re-exported at the crate root next to
  `set_high_contrast`.
* The GPL-3.0 licence text ships inside the crate.
* `examples/preferences.rs`: a complete program built from the kit.

## 0.1.5

* Picker check marks are drawn in `contrast_on` of the swatch they sit on, so
  they stay legible on light accents and on other variants' cards.

## 0.1.4

* Six palettes (all four Catppuccin variants, Kanagawa journal, Parchment
  light) no longer give a progress track the colour of the card it sits on.

## 0.1.3

* Kanagawa Dragon, from upstream, as its own variant.

## 0.1.2

* The Parchment theme family; the catalog can grow without a consumer
  changing.

## 0.1.1

* Publishable to crates.io: everything the crate embeds lives inside it.

## 0.1.0

* First release: palettes, theme resolver, accents, labels, filesystem paths
  and the shared widgets, generated from `tokens/`.
