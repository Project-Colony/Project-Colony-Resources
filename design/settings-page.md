# The settings page

Every Colony program's settings page has the same skeleton. A user who has
configured one should not have to relearn anything to configure the next.

Reference implementation: Colony's `src/ui/settings.rs`.

## Structure

The settings page **replaces the content area** — it is not a modal, not a
separate window, not a popover. The main sidebar stays where it is.

Inside the content area:

```
┌──────────────────────────────────────────────────┐
│  Settings                               [Close]  │
├──────────────┬───────────────────────────────────┤
│ General      │                                   │
│ Appearance   │   sections for the active         │
│ Accessibility│   category, stacked vertically    │
│ Storage      │                                   │
│ About        │                                   │
│ Shortcuts    │                                   │
└──────────────┴───────────────────────────────────┘
```

The header is the title at `sz(22)` bold `text_primary`, a spacer, then a close
button at `sz(13)` `text_muted`, padding `[6, 14]`, radius 6. The close button
sends the same toggle message as the app name in the sidebar.

Category buttons follow the same selection rules as the main sidebar: selected
gets an `accent` background with `text_primary`, hover gets `bg_card_hover`,
padding `[8, 14]`, radius 8, full width.

## The six categories

In this order. Do not reorder them, and do not add a seventh without a reason
that applies to the whole ecosystem.

| Category | Holds |
|---|---|
| **General** | startup, language, updates |
| **Appearance** | theme, colors, typography, effects, preview |
| **Accessibility** | vision, motion, navigation, reading |
| **Storage** | scan paths, install locations |
| **About** | version, licence, credits |
| **Shortcuts** | the keyboard reference |

A program without storage to configure omits the Storage category rather than
renaming it into something else. Omitting is fine; repurposing is not.

## Appearance, in detail

This is the category that shares the most machinery across programs, and the one
this repo feeds directly.

- **Theme** — the family/variant picker. Renders from `THEME_FAMILIES` in
  `generated/rust/palettes.rs`: one row per family, showing the family's Nerd
  Font glyph and its localized name, then a horizontal row of variant cards. Each
  card is drawn from that variant's `swatch` (a background and an accent), which
  is why the swatch is stored in `tokens/` next to the palette rather than
  recomputed — a picker card that does not resemble the theme it selects is a
  picker that lies.
- **Colors** — the accent override. Eight named accents from
  `tokens/accents.toml`, plus *auto*. "Auto" is the absence of an override: fall
  back to the active palette's own `accent_blue`. Never store "auto" as a colour.
- **Typography** — font family and size. See [typography.md](typography.md).
- **Effects** — visual extras. Must degrade to nothing when the user has asked
  for reduced motion.
- **Preview** — a live sample of the current settings, so the user can judge a
  theme without closing the page.

## Accessibility is not optional

The Accessibility category is part of the skeleton, not a nice-to-have. At
minimum a Colony program honours:

- **Vision** — the high-contrast toggle, and a dyslexia-friendly font.
- **Motion** — reduced motion. If it animates, this must silence it.
- **Reading** — text scaling, on top of the Typography size setting; the two
  multiply.

Colony's palettes are designed with this in mind: `with_high_contrast()` derives
a boosted palette from the active one, so a theme never has to ship a separate
high-contrast twin.

## Writing a setting

- Every visible string goes through i18n, in **both** locales. See
  [i18n.md](i18n.md).
- Every colour comes from the palette. A literal hex in a widget is a bug — it
  will be wrong on 56 of the 57 themes.
- Sections get a heading and a short description. The description is
  `text_muted`; it explains the consequence of the setting, not its name again.
- A setting that needs a restart says so, in the description, up front.
