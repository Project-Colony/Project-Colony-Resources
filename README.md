<div align="center">

# Project Colony Resources

**Shared design tokens, rules, templates and the colony-ui crate every Project Colony program builds on.**

</div>

The single place where Project Colony's shared design decisions are written down:
the theme palettes, the accent colours, the UI conventions every Colony program
is expected to follow.

Before this repo, each of those lived in whichever program happened to implement
it first, and everything else pointed at it. SphereCord, for instance, downloaded
Colony's `src/ui/theme.rs` over HTTP and parsed the Rust source with a regex to
recover the palettes, and its companion request for `src/i18n.rs` had already
gone stale, because Colony split that file into `src/i18n/{fr,en}.rs`. That is
the class of breakage this repo exists to end.

> **Status:** `colony-ui` is published on crates.io, and Colony uses its
> palettes and its `paths` module: Colony's hand-maintained palette constants
> are gone. The preferences kit new in 0.1.6 (`StandardPreferences`, the fonts,
> the navigation and Preferences page widgets) is adopted by no program yet;
> Colony still draws its own page, so the kit is proven by unit tests and a
> compiled example, not by a program in daily use. CI runs the `paths` tests on
> Linux, Windows and macOS. `colony-ui` releases now go through release-please
> and crates.io Trusted Publishing; that path has not published yet, because
> 0.1.7 waits on the Trusted Publisher registration on crates.io. The SignPath
> path of the shared signing workflow is wired but unused: the organisation
> secret and variable it needs do not exist yet.

## How it works

```
tokens/          you edit this          ← the source of truth
   ↓             cargo run -p colony-tokens -- generate
generated/       never edit this        ← what every program consumes
```

A colour is written down exactly once, in `tokens/`. Every consumer reads a
generated artifact in the format it actually wants (Rust, JSON, CSS) instead of
re-deriving it from another program's source file.

```bash
cargo run -p colony-tokens -- generate
```

```bash
cargo test
```

`cargo test` proves the generated palettes still match, colour for colour, what
Colony shipped before the import; see [Guarantees](#guarantees).

## What's in here

| Path | What it is |
|---|---|
| `tokens/families/*.toml` | one file per theme family, every variant a full palette |
| `tokens/accents.toml` | the 8 accent overrides, order-sensitive |
| `tokens/labels.toml` | the shared Preferences strings, English and French |
| `crates/colony-ui/src/generated/` | palettes and labels, embedded in the crate so they ship with it |
| `generated/themes.json` | one bundle for every non-Rust consumer |
| `generated/css/colony-*.css` | one stylesheet per theme, plus a bundle |
| `generated/palette.schema.json` | JSON Schema validating `tokens/families/*.toml` |
| `generated/colony.schema.json` | JSON Schema validating a program's `colony.json` |
| `design/*.md` | the conventions: layout, filesystem, navigation, settings, theming, type, i18n, releases, dependencies, docs |
| `manifests/examples/*.json` | working `colony.json` files for each shape |
| `templates/` | release workflow (signing through the shared workflow), release-please and Dependabot configs, README skeletons |
| `crates/colony-ui/` | the crate programs depend on: theme, fonts, labels, preferences, widgets |
| `crates/colony-tokens/` | the generator and its tests; never published |

## Design rules

`design/` is this repository's product rather than its documentation, so it
sits at the top level instead of under `docs/`. One page per question:

| Page | What it answers |
|---|---|
| [repository-layout.md](design/repository-layout.md) | how a repository and its crates are arranged and named |
| [filesystem.md](design/filesystem.md) | where a program keeps its config, data and cache, on all three platforms |
| [navigation.md](design/navigation.md) | how a program is laid out, and how the user reaches its settings |
| [settings-page.md](design/settings-page.md) | what every Preferences page contains, and in what order |
| [theming.md](design/theming.md) | how a palette is built, and how to add a theme family |
| [typography.md](design/typography.md) | which fonts and text sizes a program uses, and where its icons come from |
| [i18n.md](design/i18n.md) | which languages a program ships, and how its strings are named and written |
| [releases.md](design/releases.md) | how a program is versioned, built, signed and picked up by the launcher |
| [dependencies.md](design/dependencies.md) | how crates, the toolchain and action pins stay on their latest release |
| [documentation.md](design/documentation.md) | how the README and `docs/` are laid out |

## Consuming this repo

### Rust / iced programs

One dependency:

<!-- x-release-please-start-version -->
```toml
colony-ui = "0.1.6"
```
<!-- x-release-please-end -->

```rust
use colony_ui::preferences::StandardPreferences;
use colony_ui::widgets::PreferencesView;
use colony_ui::{i18n, theme, Typography};

// At startup: the user's standard preferences, part of the program's own
// config file, put into effect in one call. An unknown theme or a bad value
// falls back to its default instead of failing the load.
config.standard.apply(); // a StandardPreferences
i18n::set_locale(i18n::Locale::from_tag(&user_language));

// Style anything from the active palette, sized and set in the user's font:
let typo = Typography::current();
let bg = theme::Palette::BG_PRIMARY();

// And build the Preferences page out of the shared kit:
let kit = PreferencesView::new(typo, &config.standard, &expanded, Message::Preference, Message::ToggleSection);
let appearance = kit.appearance();
```

`crates/colony-ui/examples/preferences.rs` is a complete program built this
way: the identity button, a sidebar, and the whole Preferences page.

`colony-ui` gives you:

| | |
|---|---|
| `ThemePalette` + one const per variant | the palette shape and every theme, e.g. `ThemePalette::GRUVBOX_DARK` |
| `set_active_theme` / `active_palette` / `Palette::*` | the active theme, and the screaming-case accessors Colony's widgets already use |
| `THEME_FAMILIES` | the ordered picker catalog: glyph, labels, swatches, modes |
| `resolve` / `FALLBACK_PALETTE` | config → palette, degrading instead of failing on an unknown theme |
| `ACCENT_OVERRIDES` / `accent_key_to_color` / `set_active_accent` | the eight accents and the user override |
| `set_high_contrast` / `with_high_contrast` | derived, so no theme ships a high-contrast twin |
| `app_tint` / `contrast_on` / `ColorExt` | identity tints and the shared "is this light?" answer |
| `preferences::StandardPreferences` | theme, accent, text sizes, high contrast, dyslexia font, motion: load, `apply`, `update` |
| `typography::*` / `sz` / `Typography` | the two text-size preferences, their product, and the scaling helper |
| `fonts::*` | the three fonts, embedded, and the accessor that honours the dyslexia toggle |
| `motion::*` | reduced motion, and `effects_enabled` for every animation to check |
| `i18n::t` | every shared label (themes, accents, the Preferences page), both locales, embedded |
| `paths::*` | `Colony/<Program>/` config, data and cache dirs on all three platforms |
| `widgets::*` | identity button, sidebar items, the Preferences page, its categories and sections, theme and accent pickers, toast |

**A new theme family reaches every program with zero code changes**: no match
arm, no picker entry, no locale edit. Add the TOML, regenerate, bump the
dependency.

A program that still ships its own copy of the palettes migrates by depending
on `colony-ui` and deleting that copy.

The shared widgets take a [`Typography`] rather than reading the host's state;
`Typography::current()` carries the product of the user's two text-size
preferences and the font the dyslexia toggle selects.

### Everything else

Read `generated/themes.json`. It carries the families, variants, every field
of every palette, the light/dark mode, the picker swatches, the accent list, the
per-field CSS variable names, the computed contrast ratios, and the shared
labels: every theme, accent and Preferences string `colony_ui::i18n` has, in
English and French, under the same keys.

```js
const bundle = await (await fetch("generated/themes.json")).json();
const lang = navigator.language.startsWith("fr") ? "fr" : "en";
const t = (key) => bundle.labels[lang][key] ?? bundle.labels.en[key] ?? key;

t("settings_title");         // "Preferences" or "Préférences"
t("settings_theme_gruvbox"); // a theme family's name
```

Take the strings from there rather than copying them into the program's own
locale files; see [design/i18n.md](design/i18n.md#labels-owned-by-this-repo).

Web and Electron programs can skip the JSON and load the stylesheets directly.
Apply a theme by setting `data-colony-theme="<family>-<variant>"` on a root
element, or by adding the matching `.colony-theme-*` class:

```html
<link rel="stylesheet" href="generated/css/colony-themes.css">
<link rel="stylesheet" href="generated/css/colony-accents.css">
<body data-colony-theme="gruvbox-dark">
```

`colony-accents.css` also carries `--colony-ink` and `--colony-paper`, the
near-black and near-white label colours `colony_ui::theme::contrast_on()` picks
between. For text on an accent or a swatch, use whichever of the two contrasts
more with it, and read them from there (or from `contrastEnds` in
`themes.json`) instead of copying the values.

The variables are neutral (`--colony-bg-primary`, `--colony-text-primary`, …).
Mapping them onto a host application's own variable names (Discord's
`--background-base-*`, say) stays in that program's repo. The values are ours;
the mapping is theirs.

## Changing a colour

1. Edit the field in `tokens/families/<family>.toml`.
2. Run `cargo run -p colony-tokens -- generate`.
3. Run `cargo test`. The round-trip test will fail, listing exactly which values
   moved; confirm that diff is the one you meant.
4. Re-cut the snapshot only when it is: see
   `crates/colony-tokens/tests/fixtures/colony-theme-rs.snapshot`.
5. Commit `tokens/` and `generated/` together. They are never allowed to drift;
   `cargo run -p colony-tokens -- generate` followed by a dirty tree is a bug.

Adding a whole theme family (a character set, a brand) is a different job with
its own rules. Read [design/theming.md](design/theming.md) first.

## Guarantees

`cargo test` enforces all of the following:

- **Every palette Colony shipped before the import round-trips unchanged.** The test parses a
  verbatim snapshot of Colony's `theme.rs` and compares it, const by const and
  field by field, against what the generator produces today.
- **The resolver is stable.** Every `(family, variant)` pair still maps to the
  same constant, and the fallback for an unknown theme is still `GRUVBOX_DARK`,
  so an existing user config resolves exactly as it did before.
- **Accent overrides keep their values *and their order*.** Colony derives each
  installed app's identity tint by hashing the app's name into that list;
  reordering it would silently re-colour every icon on every machine.
- **Text stays legible.** `text_primary` clears 4.5:1 against `bg_primary` and
  `text_muted` clears 3:1, on every theme. There is exactly one documented
  exception: Solarized Light, whose `#657b83` on `#fdf6e3` is upstream
  Solarized's own `base00` on `base3`. The test also fails if that exception
  list goes stale.
- **Both locales stay in step.** Every label carries an `fr` and an `en` string,
  a key reused across families must carry the same string each time, and
  `generated/themes.json` carries exactly the tables `colony-ui` embeds.
- **`generated/` matches `tokens/`.** The same check CI runs, so a forgotten
  `generate` fails locally first.

## Shipping a program on Colony

Publishing a program the launcher can list and install is a separate contract
from the design tokens, and it is written down in
[design/releases.md](design/releases.md): conventional commits feed release-please,
release-please tags, the tag builds four platform assets whose **names** are what
Colony auto-detects, and `colony.json` describes the rest.

The short version, for a program whose assets follow the naming convention:

```json
{
  "$schema": "https://raw.githubusercontent.com/Project-Colony/Project-Colony-Resources/main/generated/colony.schema.json",
  "name": "Eidos",
  "category": "system"
}
```

Copy the workflow and configs from [`templates/`](templates/), a manifest from
[`manifests/examples/`](manifests/examples/), and follow the checklist at the end
of `design/releases.md`.

Four rules apply across the organisation:

- **[design/repository-layout.md](design/repository-layout.md): crates and
  directories are arranged the same way everywhere.** `crates/<prefix>-<role>/`
  with role names that already mean something across the ecosystem, one version
  for the whole workspace, a mandatory one-line `description` per crate, and a
  `src/` where a directory earns its existence by holding more than one file.
- **[design/dependencies.md](design/dependencies.md): everything is on its
  latest release, always.** At creation, at every change, and on the weekly
  automated pass. Versions are pinned in full in the manifest so staleness shows
  up in a diff instead of hiding in the lockfile.
- **[design/filesystem.md](design/filesystem.md): every program writes to
  `Colony/<Program>/`.** Config, data and cache, on all three platforms, with
  Windows on `AppData\Local` rather than Roaming. `colony_ui::paths` computes
  it so nobody rebuilds the path by hand.
- **[design/documentation.md](design/documentation.md): the README and `docs/`
  are laid out the same way everywhere.** `docs/` is sorted by *who is reading*
  (`guide/`, `internals/`, `project/`) behind a `docs/README.md` index, and
  the README answers what/why/how-to-get-it in that order.

These four are what make an unfamiliar repository cheap to work in: most of the
cost of changing code you did not write is finding what to change, and a layout
you can predict removes that cost outright.

`templates/program/` holds a README and a `docs/README.md` skeleton to start
from.

## Releasing colony-ui

Nothing about a `colony-ui` release is done by hand. Conventional commits that
touch `crates/colony-ui/` feed a release PR,
`chore(main): release colony-ui X.Y.Z`, which release-please keeps open and up
to date. Merging it bumps the crate's version in `Cargo.toml` and `Cargo.lock`,
writes [`crates/colony-ui/CHANGELOG.md`](crates/colony-ui/CHANGELOG.md),
updates the version in both READMEs, tags `colony-ui-vX.Y.Z` and publishes the
crate to crates.io from that tag, through Trusted Publishing
([`.github/workflows/release.yml`](.github/workflows/release.yml)). Do not bump
the version, edit the changelog or push a tag yourself.

If publishing fails after the tag exists, fix the cause, then publish that tag:

```bash
gh workflow run release.yml --ref colony-ui-vX.Y.Z -f tag=colony-ui-vX.Y.Z
```

## Code signing policy

This repository ships no binaries, so there is nothing here to sign. `colony-ui`
is published to crates.io only from CI, by
[`.github/workflows/release.yml`](.github/workflows/release.yml) on a
release-please tag, with a short-lived token that crates.io mints for that
workflow through Trusted Publishing; no registry token is stored anywhere.

The programs built on these resources are signed by the shared
[sign-and-publish workflow](.github/workflows/sign-and-publish.yml) with the
organisation's ed25519 release key, in a job that builds nothing. How that
works: [design/releases.md](design/releases.md#5-signing). How a program
documents it and how to verify a download:
[templates/program/README.md](templates/program/README.md#code-signing-policy).

## Privacy

`colony-ui` and `colony-tokens` collect nothing, store nothing and send
nothing, and neither makes a network call. The only thing `colony-ui` does to
the disk is create the directories `paths::config_dir`, `data_dir`,
`cache_dir`, `apps_dir` and `app_dir` return, under
`<platform root>/Colony/`; what goes inside them is up to the program.
`colony-tokens` reads `tokens/`, and writes `generated/` and
`crates/colony-ui/src/generated/` in this repository's checkout, deleting the
stale files it finds there.

## License

GPL-3.0-or-later, matching the rest of the Project Colony organisation. See
[LICENSE](LICENSE).

The fonts embedded in `colony-ui` are not ours and keep their own licence, the
SIL Open Font License 1.1, whose texts ship next to them in
[`crates/colony-ui/assets/fonts/`](crates/colony-ui/assets/fonts/).

A consequence worth stating plainly: a program that links `colony-ui` will have
to be GPL-3.0-or-later too. If that ever becomes the wrong trade for the shared
*values* (the palettes and conventions in `tokens/`, `generated/` and `design/`,
which are data rather than logic), relicensing those directories more permissively
while keeping `crates/` copyleft is the change to make, and it is
easier to make before there are outside contributors than after.
