# Project Colony — Resources

The single place where Project Colony's shared design decisions are written down:
the theme palettes, the accent colours, the UI conventions every Colony program
is expected to follow.

Before this repo, each of those lived in whichever program happened to implement
it first, and everything else pointed at it. SphereCord, for instance, downloaded
Colony's `src/ui/theme.rs` over HTTP and parsed the Rust source with a regex to
recover the palettes — and its companion request for `src/i18n.rs` had already
gone stale, because Colony split that file into `src/i18n/{fr,en}.rs`. That is
the class of breakage this repo exists to end.

## How it works

```
tokens/          you edit this          ← the source of truth
   ↓             cargo run -p colony-tokens -- generate
generated/       never edit this        ← what every program consumes
```

A colour is written down exactly once, in `tokens/`. Every consumer reads a
generated artifact in the format it actually wants — Rust, JSON, CSS — instead of
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
| `tokens/families/*.toml` | 25 theme families, 57 variants, 38 colours each |
| `tokens/accents.toml` | the 8 accent overrides, order-sensitive |
| `generated/rust/palettes.rs` | palettes, picker catalog, resolver, accents |
| `generated/themes.json` | one bundle for every non-Rust consumer |
| `generated/css/colony-*.css` | one stylesheet per theme, plus a bundle |
| `generated/i18n/labels.{fr,en}.json` | display strings, both locales |
| `generated/palette.schema.json` | JSON Schema validating `tokens/families/*.toml` |
| `generated/colony.schema.json` | JSON Schema validating a program's `colony.json` |
| `design/*.md` | the conventions — navigation, settings, theming, type, i18n, releases |
| `manifests/examples/*.json` | working `colony.json` files for each shape |
| `templates/` | release workflow, release-please config, signing script |
| `tools/colony-tokens/` | the generator and its tests |

## Consuming this repo

### Rust / iced programs

The shared `colony-ui` crate is phase 2. Until it lands, a program can include
the generated palettes directly from a checkout:

```rust
// The module doing this must already define `ThemePalette`, `hex()` and `Color`.
include!("../../../Project-Colony-Resources/generated/rust/palettes.rs");
```

Once `colony-ui` exists, that becomes a git dependency and the `include!` goes
away:

```toml
colony-ui = { git = "https://github.com/Project-Colony/Project-Colony-Resources" }
```

The generated file defines:

- `ThemePalette::<CONST>` — one per variant, e.g. `ThemePalette::GRUVBOX_DARK`
- `THEME_FAMILIES` — the ordered catalog the Settings picker renders, replacing
  the hand-maintained `theme_families` vec in `settings.rs`
- `resolve(family, variant)` and `FALLBACK_PALETTE` — the config → palette lookup
- `ACCENT_OVERRIDES` and `accent_key_to_color(key)`

### Everything else

Read `generated/themes.json`. It carries the families, variants, all 38 fields
per palette, the light/dark mode, the picker swatches, the accent list, the
per-field CSS variable names, and the computed contrast ratios.

Web and Electron programs can skip the JSON and load the stylesheets directly.
Apply a theme by setting `data-colony-theme="<family>-<variant>"` on a root
element, or by adding the matching `.colony-theme-*` class:

```html
<link rel="stylesheet" href="generated/css/colony-themes.css">
<link rel="stylesheet" href="generated/css/colony-accents.css">
<body data-colony-theme="gruvbox-dark">
```

The variables are neutral (`--colony-bg-primary`, `--colony-text-primary`, …).
Mapping them onto a host application's own variable names — Discord's
`--background-base-*`, say — stays in that program's repo. The values are ours;
the mapping is theirs.

## Changing a colour

1. Edit the field in `tokens/families/<family>.toml`.
2. Run `cargo run -p colony-tokens -- generate`.
3. Run `cargo test`. The round-trip test will fail, listing exactly which values
   moved — confirm that diff is the one you meant.
4. Re-cut the snapshot only when it is: see
   `tools/colony-tokens/tests/fixtures/colony-theme-rs.snapshot`.
5. Commit `tokens/` and `generated/` together. They are never allowed to drift;
   `cargo run -p colony-tokens -- generate` followed by a dirty tree is a bug.

Adding a whole theme family — a character set, a brand — is a different job with
its own rules. Read [design/theming.md](design/theming.md) first.

## Guarantees

`cargo test` enforces all of the following:

- **Every one of Colony's 57 palettes round-trips unchanged.** The test parses a
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
  exception — Solarized Light, whose `#657b83` on `#fdf6e3` is upstream
  Solarized's own `base00` on `base3`. The test also fails if that exception
  list goes stale.
- **Both locales stay in step.** Every label carries an `fr` and an `en` string,
  and a key reused across families must carry the same string each time.
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

## Status

Phase 1 — foundations. Tokens, generators, conventions and the release contract
are in place; no consumer has been rewired yet. Colony, SphereCord and the rest
still ship their own copies until phase 2 lands `crates/colony-ui` and migrates
them one by one.

## Licence

GPL-3.0-or-later, matching the rest of the Project Colony organisation. See
[LICENSE](LICENSE).

A consequence worth stating plainly: a program that links `colony-ui` will have
to be GPL-3.0-or-later too. If that ever becomes the wrong trade for the shared
*values* — the palettes and conventions in `tokens/`, `generated/` and `design/`,
which are data rather than logic — relicensing those directories more permissively
while keeping `tools/` and `crates/` copyleft is the change to make, and it is
easier to make before there are outside contributors than after.
