# The settings page

Every Colony program's settings page has the same skeleton. A user who has
configured one should not have to relearn anything to configure the next.

Reference implementation: the preferences kit in `colony-ui`
(`colony_ui::widgets`, `colony_ui::preferences`), ported from Colony's
`src/ui/settings.rs` and cross-checked against Digger (`src/ui.rs`) and Grape
(`src/ui/app/preferences/`), which is where the per-program variations below
come from. `crates/colony-ui/examples/preferences.rs` builds a complete page
with it. How the user *gets* here is in [navigation.md](navigation.md).

**A Rust/iced program builds this page from the kit rather than copying it.**
Every measurement on this page is what the kit draws, so a program that calls it
conforms by construction, and a fix to the kit reaches every program with a
version bump.

## Structure

The settings page **replaces the content area**. It is not a modal, not a
separate window, not a popover. The program's own chrome stays where it is.

Inside the content area, drawn with Colony's categories:

```
┌──────────────────────────────────────────────────┐
│  Preferences                            [Close]  │
├──────────────┬───────────────────────────────────┤
│ General      │  General                          │
│ Appearance   │  Preferences are saved            │
│ Accessibility│  automatically.                   │
│ Storage      │                                   │
│ About        │    Startup                     ›  │
│ Shortcuts    │    Language                    ›  │
│              │    Updates                     ›  │
└──────────────┴───────────────────────────────────┘
```

`widgets::preferences_page(typo, categories, content, on_close)` is that whole
frame: header, category list on the left, the selected category's content on
the right, scrolling.

The page is titled **Preferences**, not "Settings" (see
[navigation.md](navigation.md)). `Préférences` in the French locale; English is
the canonical string. Both ship in `colony_ui::i18n` as `settings_title`.

The header (`widgets::preferences_header`) is the title at `sz(22)` bold
`text_primary`, a spacer, then a Close button at `sz(13)` `text_muted`, padding
`[6, 14]`, radius 6. The Close button sends the same toggle message as the
identity button that opened the page: there are two ways out and they are the
same message.

Each category opens with its own heading and a one-line description underneath,
in `text_muted` (`widgets::category_heading`). General's description carries the
contract that matters most:

> Preferences are saved automatically.

**There is no Save button, and there must not be one.** A change applies when it
is made. Anything that cannot apply immediately says so in its own description.

## Sections are collapsible

Inside a category, each section is a collapsible row
(`widgets::collapsible_section`), and this is the pattern to copy exactly. It is
what makes a long preferences page navigable:

- The header is **flat**: no box, no background. Just the title and a chevron.
- Title at `sz(15)` bold, `text_primary`.
- The chevron sits at the far right: `\u{f054}` pointing right when collapsed,
  `\u{f078}` pointing down when expanded (`widgets::icons`).
- The whole header row is the toggle, not just the chevron.
- Expanded state is per-section and remembered while the page is open: the host
  keeps a `HashSet<String>` of open section keys and toggles a key when the
  section's message arrives.

A category page therefore reads as a short list of closed rows on arrival
(Startup, Language, Updates) rather than a wall of every control at once.

Category buttons (`widgets::category_list`, one `widgets::nav_item` each) follow
the same selection rules as the main sidebar: selected gets an `accent`
background with its label in `contrast_on(accent)`, hover gets `bg_card_hover`,
padding `[8, 14]`, radius 8, full width. The label is not `text_primary`: on an
accent fill that pairing is below 4.5:1 on almost every theme and reaches 1.01:1
on Ayu Dark, where the selected category disappeared.

The list is 180 wide at a text scale of 1.0 and grows with the scale. Colony's
fixed 160 holds "Accessibility" at the default size only: at large text, or in
OpenDyslexic, it no longer fits on one line. A test measures every standard
category name, in both locales and both fonts, at all twelve size combinations.

## The categories

**The first three are fixed, and in this order:**

| Category | Holds |
|---|---|
| **General** | startup, updates, and anything that fits nowhere better |
| **Appearance** | theme, colors, typography, effects, preview |
| **Accessibility** | vision, motion, navigation, reading |

`widgets::STANDARD_CATEGORIES` holds their label keys in that order, and
`widgets::ABOUT_CATEGORY` the key of About.

After those come the program's own, and **About last** where it exists. That
tail is genuinely per-program. What the three do today:

| | Colony | Digger | Grape |
|---|---|---|---|
| 1-3 | General, Appearance, Accessibility | General, Appearance, Accessibility | General, Appearance, Accessibility |
| then | Storage, Shortcuts | Language | Audio |
| last | About | About | none |

A program adds a category when it has a domain to configure that does not fit
the first three: Grape has audio devices and an equalizer, Digger gives language
its own tab rather than burying it in General as Colony does. It does not rename
one of the first three into something else. Omitting is fine; repurposing is
not.

Do not reorder the first three. They are what a user hunting for a setting scans
first, and they hold the same things in every program.

## The kit builds the first three

`widgets::PreferencesView` draws General's heading and the whole of Appearance
and Accessibility from the user's `preferences::StandardPreferences`:

```rust
let kit = PreferencesView::new(
    Typography::current(),
    &self.prefs,            // StandardPreferences, part of the program's config
    &self.expanded,         // HashSet<String> of open sections
    Message::Preference,    // wraps a preferences::Change
    Message::ToggleSection, // wraps a section key
);
let content = match self.category {
    0 => kit.general().push(self.startup_section()).into(),
    1 => kit.appearance().into(),
    2 => kit.accessibility().into(),
    _ => self.about(),
};
```

On `Message::Preference(change)` the host calls `self.prefs.update(change)`,
which records the change and puts it into effect, then saves its config. That
is all "no Save button" takes.

`StandardPreferences` holds the theme, the accent, auto accent, both text sizes,
animations, high contrast, the dyslexia font and reduced motion. It serializes
under the field names Colony already writes, so a program embeds it in its own
config with `#[serde(flatten)]`. It never fails to load: an unknown theme, a
size from a newer build or a value of the wrong type falls back to its default,
and the rest of the file is kept. At startup, `prefs.apply()` sets every
colony-ui global from it.

A category the kit does not cover end to end composes the same parts. General
is `kit.general()` plus the program's sections. A program with a Navigation
section in Accessibility builds `kit.accessibility_heading()`, then
`vision_section()`, `motion_section()`, its own Navigation section, then
`reading_section()`. Every section of Appearance and Accessibility is available
on its own the same way.

## Appearance, in detail

This is the category that shares the most machinery across programs, and the one
this repo feeds directly.

- **Theme**: the family/variant picker (`widgets::theme_picker`). Renders from
  `colony_ui::THEME_FAMILIES`: one row per family, showing the family's Nerd
  Font glyph and its localized name, then a horizontal row of variant cards.

  A card is a wide rectangle filled with the variant's `swatch.bg`, crossed by a
  thin bar in its `swatch.accent`, with the variant's localized name at the
  bottom-left and a check mark on the selected one. That is why the swatch lives
  in `tokens/` next to the palette rather than being recomputed: a picker card
  that does not resemble the theme it selects is a picker that lies.

  Applying a theme raises a dismissible confirmation toast (`theme_applied`,
  "Theme applied."). The change itself is immediate; the toast exists because
  switching to a neighbouring variant can otherwise be hard to notice. The kit
  ships the string and the toast (`widgets::toast`); the host shows it when it
  receives `Change::Theme`, where its notifications go, and removes it on
  dismiss.
- **Colors**: the accent override (`widgets::accent_picker`). A row of eight
  swatches, one per accent in `tokens/accents.toml`, drawn as filled circles
  with a check on the selected one. Below them, a separate **auto accent from
  background** toggle (`settings_auto_accent`), which hands the accent back to
  the theme rather than pinning it. Off by default.

  Two different notions of "auto" that are easy to conflate: that toggle is a
  *behaviour*, whereas at the code level an unset override simply resolves to
  the active palette's own `accent_blue`. Never store "auto" as a colour value:
  `StandardPreferences` stores the picked accent's key, or none, and a separate
  `auto_accent` flag.
- **Typography**: font family and size. See [typography.md](typography.md). The
  three-font rule leaves one choice of family, the dyslexia-friendly font, and
  that switch lives in Accessibility; Appearance shows the family in use and
  sets the base size.
- **Effects**: visual extras. Must degrade to nothing when the user has asked
  for reduced motion: gate every animation on `colony_ui::effects_enabled()`,
  which is false whenever either toggle says so.
- **Preview**: a live sample of the current settings, so the user can judge a
  theme without closing the page. The kit's preview card is drawn in the live
  palette, accent, font and size.

## Accessibility is not optional

The Accessibility category is part of the skeleton, not a nice-to-have. At
minimum a Colony program honours:

- **Vision**: the high-contrast toggle, and a dyslexia-friendly font.
- **Motion**: reduced motion. If it animates, this must silence it.
- **Reading**: text scaling, on top of the Typography size setting; the two
  multiply.

Colony's palettes are designed with this in mind: `with_high_contrast()` derives
a boosted palette from the active one, so a theme never has to ship a separate
high-contrast twin. The dyslexia font is a whole-application swap through
`colony_ui::fonts`, and both text sizes reach every widget through
`Typography::current()`.

## Writing a setting

- Every visible string goes through i18n, in **both** locales. See
  [i18n.md](i18n.md). The kit's strings ship in `colony_ui::i18n`; a program's
  own stay in the program.
- Every colour comes from the palette. A literal hex in a widget is a bug: it
  will be wrong on every theme but one.
- Sections get a heading and a short description
  (`widgets::section_description`). The description is `text_muted`; it
  explains the consequence of the setting, not its name again.
- A setting is a row: title, a one-line description in `text_dimmer`, the
  control on the right. `widgets::functional_toggle` for on/off,
  `widgets::setting_row` with `widgets::dropdown` for a choice.
- A setting that needs a restart says so, in the description, up front.
