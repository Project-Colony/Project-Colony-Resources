# colony-ui

The shared user-interface layer for [Project Colony](https://github.com/Project-Colony)
programs: the theme palettes, the accent overrides, the display strings, the
filesystem layout, and the [iced](https://iced.rs) widgets they are all built
from.

```toml
[dependencies]
colony-ui = "0.1"
```

```rust
use colony_ui::{i18n, paths, theme, widgets, Typography};

// At startup, from the user's config — two strings, nothing else:
theme::set_active_theme("gruvbox", "dark");
i18n::set_locale(i18n::Locale::from_tag("fr"));

// Style anything from the active palette:
let background = theme::Palette::BG_PRIMARY();

// Find where this program keeps its files, on any platform:
let prefs = paths::config_dir("Digger")?.join("preferences.json");
# Ok::<(), std::io::Error>(())
```

## What it gives you

| | |
|---|---|
| `ThemePalette` + 57 palettes | 25 theme families, each a full 38-colour palette |
| `set_active_theme` / `Palette::*` | the active theme and its semantic accessors |
| `THEME_FAMILIES` | the ordered catalog a theme picker renders from |
| `ACCENT_OVERRIDES` | eight palette-independent accents the user can pick |
| `set_high_contrast` | derived from the active palette, so no theme ships a twin |
| `i18n::t` | theme and accent labels in English and French, embedded |
| `paths::*` | `Colony/<Program>/` config, data and cache on Linux, Windows, macOS |
| `widgets::*` | collapsible section, labelled toggle, theme picker, accent picker |

The palettes are **generated** from design tokens rather than hand-written, so
adding a theme family reaches every program that depends on this crate without
a line of code changing anywhere.

## Scope

This is a house design system, not a general-purpose theming library. The
palettes are Project Colony's, the directory layout is Project Colony's
convention, and the active theme is process-global state — the right shape for a
family of programs that share one look, and probably the wrong shape for
anything else.

Themes included: Catppuccin, Gruvbox, Everblush, Kanagawa, Nord, Dracula,
Solarized, Tokyo Night, Rosé Pine, One Dark, Monokai, Ayu, Everforest, Material,
Flexoki, Nightfox, Sonokai, Oxocarbon, Night Owl, Iceberg, Horizon, Melange,
Synthwave '84, Modus, and a fan-made Stellar Blade character set.

## Licence

GPL-3.0-or-later. Linking this crate makes your program GPL-3.0-or-later too.

Source, design tokens and conventions:
[Project-Colony-Resources](https://github.com/Project-Colony/Project-Colony-Resources).
