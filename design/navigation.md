# Navigation

How a Colony program is laid out and how the user moves through it. Colony (the
launcher) is the reference implementation; the specifics below were read from
`src/ui/sidebar.rs` and `src/ui/settings.rs`.

## The shape

A Colony window is two columns:

```
┌───────────────┬──────────────────────────────────┐
│  Colony  ⚙    │                                  │
│               │                                  │
│  CATEGORIES   │           content area           │
│  ▸ Games      │                                  │
│  ▸ System     │   (replaced wholesale by the     │
│  ▸ Utilities  │    settings page when open)      │
│               │                                  │
│  hint text    │                                  │
└───────────────┴──────────────────────────────────┘
```

The sidebar is persistent. The content area is what changes.

## Settings live behind the app name

**The program's name in the top-left corner is the settings button.** There is no
separate gear button in a toolbar, no menu bar, no settings entry in the category
list. The user clicks the word `Colony` and lands in settings.

Concretely, in the sidebar header:

- The app name, bold, at `sz(30)`.
- A gear glyph immediately after it, at `sz(14)`. This is the affordance — it is
  what tells the user the name is clickable. It is `text_primary` while settings
  are open and `text_dimmer` otherwise, so it doubles as the open/closed
  indicator.
- Both sit inside **one** button, padding `[4, 8]`, corner radius 8, which
  toggles settings.
- That button's background is `bg_card_hover` on hover, `bg_selected` while
  settings are open, and transparent otherwise.

Toggling is symmetric: the same control opens and closes. The settings page also
carries its own close button (see [settings-page.md](settings-page.md)), so the
user is never trapped, but the name in the corner always works.

### Why it is done this way

The name is the largest, most stable target in the window, and it is where a user
looks to answer "what am I in?". Making it also answer "how do I configure this?"
costs no screen space and no extra chrome. Colony has shipped it long enough that
it is now what users of the ecosystem expect — a new Colony program that puts
settings somewhere else is the one behaving oddly.

Port this convention as-is. If a program genuinely cannot (no sidebar, no
persistent name), keep the rule that settings are reached from the program's
identity, not from a floating gear icon.

## Below the header

In order:

1. A `categories` label, `sz(13)`, `text_muted`. It names the list, it is not
   clickable.
2. The category buttons, one per section, each dispatching a "select section"
   message carrying its index.
3. A keyboard-shortcut hint, `sz(10)`, `text_dimmest`. Deliberately the quietest
   text in the window — discoverable, never competing.

The dimming ramp here is the point: `text_muted` for the list header,
`text_dimmest` for the hint. Colony's palettes carry a six-step text ramp
(`text_primary` → `text_dimmest`) precisely so this kind of hierarchy is
expressible without inventing one-off greys.

## Selection state

A selected item is marked with **background**, not with a coloured label:

- selected → background `accent`, text `text_primary`
- hovered → background `bg_card_hover`
- neither → transparent background, text `text_muted`

Corner radius 8, padding `[8, 14]`, full width. Keep hover and selected visually
distinct; a hover state that looks like selection makes the list feel broken.

## Animation

The sidebar slide is 200 ms (`App::SIDEBAR_ANIM_MS`). Motion is a preference —
Settings → Accessibility → Motion — and a program that animates must honour it.
