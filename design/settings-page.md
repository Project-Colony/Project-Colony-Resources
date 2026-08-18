# The settings page

Every Colony program's settings page has the same skeleton. A user who has
configured one should not have to relearn anything to configure the next.

Reference implementation: Colony's `src/ui/settings.rs`. Cross-checked against
Digger (`src/ui.rs`) and Grape (`src/ui/app/preferences/`), which is where the
per-program variations below come from. How the user *gets* here is in
[navigation.md](navigation.md).

## Structure

The settings page **replaces the content area** — it is not a modal, not a
separate window, not a popover. The program's own chrome stays where it is.

Inside the content area, drawn with Colony's categories:

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
sends the same toggle message as the identity button that opened the page —
there are two ways out and they are the same message.

Category buttons follow the same selection rules as the main sidebar: selected
gets an `accent` background with `text_primary`, hover gets `bg_card_hover`,
padding `[8, 14]`, radius 8, full width.

## The categories

**The first three are fixed, and in this order:**

| Category | Holds |
|---|---|
| **General** | startup, updates, and anything that fits nowhere better |
| **Appearance** | theme, colors, typography, effects, preview |
| **Accessibility** | vision, motion, navigation, reading |

After those come the program's own, and **About last** where it exists. That
tail is genuinely per-program — what the three do today:

| | Colony | Digger | Grape |
|---|---|---|---|
| 1–3 | General, Appearance, Accessibility | General, Appearance, Accessibility | General, Appearance, Accessibility |
| then | Storage, Shortcuts | Language | Audio |
| last | About | About | — |

A program adds a category when it has a domain to configure that does not fit
the first three — Grape has audio devices and an equalizer, Digger gives
language its own tab rather than burying it in General as Colony does. It does
not rename one of the first three into something else. Omitting is fine;
repurposing is not.

Do not reorder the first three. They are what a user hunting for a setting scans
first, and they hold the same things in every program.

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
