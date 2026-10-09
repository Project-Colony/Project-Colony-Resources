# colony-ui

The shared user-interface layer for [Project Colony](https://github.com/Project-Colony)
programs: the theme palettes, the accent overrides, the fonts, the display
strings, the user's standard preferences, the filesystem layout, and the
[iced](https://iced.rs) widgets a Colony program's chrome and Preferences page
are built from.

```toml
[dependencies]
colony-ui = "0.1"
```

```rust
use colony_ui::preferences::StandardPreferences;
use colony_ui::{i18n, paths, theme, Typography};

// At startup, from the user's config. StandardPreferences is part of the
// program's own config struct (#[serde(flatten)]), and never fails to load:
// an unknown theme or a bad value falls back to its default.
let prefs = StandardPreferences::default();
prefs.apply();
i18n::set_locale(i18n::Locale::from_tag("fr"));

// Style anything from the active palette, sized and set in the user's font:
let typo = Typography::current();
let title_size = typo.sz(22);
let background = theme::Palette::BG_PRIMARY();

// Find where this program keeps its files, on any platform:
let config = paths::config_dir("Digger")?.join("preferences.json");
# Ok::<(), std::io::Error>(())
```

## What it gives you

| | |
|---|---|
| `ThemePalette` + one const per variant | every theme family, each variant a full palette |
| `set_active_theme` / `Palette::*` | the active theme and its semantic accessors |
| `THEME_FAMILIES` | the ordered catalog a theme picker renders from |
| `ACCENT_OVERRIDES` | eight palette-independent accents the user can pick |
| `set_high_contrast` | derived from the active palette, so no theme ships a twin |
| `preferences::StandardPreferences` | what Appearance and Accessibility configure: load leniently, `apply`, `update` |
| `typography::*`, `sz`, `Typography` | the two text-size preferences, which multiply, and the scaling helper |
| `fonts::*` | JetBrainsMono Nerd Font, OpenDyslexic, Font Awesome, embedded, and the accessor that honours the dyslexia toggle |
| `motion::*` | reduced motion, and `effects_enabled` for every animation to check |
| `i18n::t` | theme, accent and Preferences labels in English and French, embedded |
| `paths::*` | `Colony/<Program>/` config, data and cache on Linux, Windows, macOS |
| `widgets::*` | identity button, sidebar items, the Preferences page and its categories, collapsible section, toggle, drop-down, theme and accent pickers |

The palettes are **generated** from design tokens rather than hand-written, so
adding a theme family reaches every program that depends on this crate without
a line of code changing anywhere.

## A Preferences page

The kit draws the page the design rules describe (navigation, settings page,
typography, i18n, accessibility), so a program that uses it conforms by
construction:

```rust
use colony_ui::widgets::{self, PreferencesView, STANDARD_CATEGORIES};

let typo = Typography::current();
let kit = PreferencesView::new(typo, &self.prefs, &self.expanded,
                               Message::Preference, Message::ToggleSection);
let content = match self.category {
    0 => kit.general().push(self.startup_section()).into(),
    1 => kit.appearance().into(),
    2 => kit.accessibility().into(),
    _ => self.about(),
};
let labels = STANDARD_CATEGORIES.iter().map(|key| i18n::t(key));
widgets::preferences_page(&typo,
    widgets::category_list(&typo, labels, self.category, Message::Category),
    content,
    Message::TogglePreferences)
```

and handles a change in one line, with no Save button anywhere:

```rust
Message::Preference(change) => { self.prefs.update(change); self.save(); }
```

[`examples/preferences.rs`](https://github.com/Project-Colony/Project-Colony-Resources/blob/main/crates/colony-ui/examples/preferences.rs) is a complete program:
`cargo run -p colony-ui --example preferences`.

## Fonts

The `fonts` feature, on by default, embeds the three Colony font families
(about 8 MB). Register them with iced once:

```rust
colony_ui::fonts::BYTES
    .into_iter()
    .fold(iced::application(boot, update, view), |app, font| app.font(font))
    .default_font(colony_ui::ui_font())
    .run()
```

Turn the feature off to ship the files yourself; the family names and the
accessors stay.

## Scope

This is a house design system, not a general-purpose theming library. The
palettes are Project Colony's, the directory layout is Project Colony's
convention, and the active theme, text sizes and accessibility switches are
process-global state: the right shape for a family of programs that share one
look, and probably the wrong shape for anything else.

Themes included: Catppuccin, Gruvbox, Everblush, Kanagawa, Nord, Dracula,
Solarized, Tokyo Night, Rosé Pine, One Dark, Monokai, Ayu, Everforest, Material,
Flexoki, Nightfox, Sonokai, Oxocarbon, Night Owl, Iceberg, Horizon, Melange,
Synthwave '84, Modus, Parchment, and a fan-made Stellar Blade character set.

## Licence

GPL-3.0-or-later, see [LICENSE](https://github.com/Project-Colony/Project-Colony-Resources/blob/main/crates/colony-ui/LICENSE). Linking this crate makes your program
GPL-3.0-or-later too.

The embedded fonts keep their own licence, the SIL Open Font License 1.1; the
texts are in [`fonts/`](https://github.com/Project-Colony/Project-Colony-Resources/tree/main/crates/colony-ui/fonts).

Source, design tokens and conventions:
[Project-Colony-Resources](https://github.com/Project-Colony/Project-Colony-Resources).
