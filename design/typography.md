# Typography and iconography

## Fonts

Three, and only three. Values from Colony's `src/state.rs`.

| Role | Family | Used for |
|---|---|---|
| Application | `JetBrainsMono Nerd Font` | all UI text |
| Accessibility | `OpenDyslexic` | replaces the application font when the user asks |
| Icons | `Font Awesome 6 Free` | glyphs outside the Nerd Font range |

The application font ships in Regular, Medium and Bold. Requesting a weight the
family does not have gets you a synthesized one, which looks wrong next to the
real weights — stick to those three.

The dyslexia font is a **whole-application swap**, not a per-widget option. A
program that hardcodes the application font anywhere will show a seam the moment
the user enables it. Route every font lookup through one accessor.

## Sizing

Never write a raw pixel size. Every size goes through the scaling helper:

```rust
text("Colony").size(self.sz(30))
```

`sz(base)` returns `round(base × font_scale())`, and `font_scale()` is the
product of two independent user preferences:

| Preference | Where | Values |
|---|---|---|
| Font size | Settings → Appearance → Typography | small 0.85, default 1.0, large 1.2 |
| Text size | Settings → Accessibility → Reading | small 0.85, default 1.0, large 1.2, xlarge 1.4 |

They **multiply**. A user on `large` typography and `xlarge` accessibility text
is at 1.68×, and the layout has to survive that. Test at the extremes: 0.7225×
and 1.68×.

### The base scale in practice

Sizes observed in Colony's chrome, as a starting point rather than a law:

| `sz()` base | Where |
|---|---|
| 30 | the app name in the sidebar |
| 22 | the settings page title |
| 14 | the gear glyph next to the app name |
| 13 | category buttons, list headers, secondary controls |
| 10 | the keyboard hint — the quietest text in the window |

Pair size with the palette's text ramp rather than inventing greys: a heading is
`text_primary` at a larger size, a hint is `text_dimmest` at a smaller one.

## Icons

Nerd Font glyphs are written as codepoints, not pasted characters — pasted glyphs
do not survive every editor, terminal and diff tool intact.

In `tokens/`, a family's icon is a bare lowercase hex codepoint:

```toml
icon = "f0f4"   # Nerd Font codepoint
```

The generator turns that into `"\u{f0f4}"` in the Rust output. An empty string
means the family has no glyph, which is allowed — Gruvbox has none.

Choosing a glyph: pick something that says what the family *is*, not what it
looks like. Catppuccin gets a coffee cup, Everblush a leaf, Kanagawa a torii,
Oxocarbon a molecule, Night Owl an owl, Stellar Blade a sword. The glyph is a
mnemonic in a long list, so it has to be legible at `sz(13)` and distinct from
its neighbours.

Verify a codepoint renders in JetBrainsMono Nerd Font before committing it. A
missing glyph shows as a tofu box, and it will be tofu on every user's machine,
not just yours.
