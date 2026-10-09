# Navigation

How a Colony program is laid out and how the user reaches its settings.

Read from the three programs that implement it today: Colony's
`src/ui/sidebar.rs`, Digger's `src/ui.rs`, Grape's `src/ui/app/view.rs`. They
agree on the rule and differ on the chrome, so this page separates the two. The
parts every program shares are in `colony_ui::widgets`; a Rust/iced program
builds its chrome from them rather than copying Colony's.

## The rule: the program's identity is the way in

**The program's name in the top-left corner is what the user clicks to reach its
settings.** Not a gear floating in a toolbar, not an entry buried in a list of
sections, not a menu bar.

That corner is the largest, most stable target in the window, and it is already
where a user looks to answer "what am I in?". Making it also answer "how do I
configure this?" costs no screen space and no extra chrome. All three programs
do it, and a new Colony program that puts settings somewhere else is the one
behaving oddly.

Three things follow, and they are the actual requirements:

1. **The identity element is a button.** Whether that is a text label, a label
   with a glyph, or a logo image plus the name.
2. **It carries a visible open/closed state.** The user must be able to tell,
   without clicking, whether they are already in settings.
3. **The same control closes what it opened.** Toggling is symmetric.

`widgets::identity_button(typo, name, size, open, on_toggle)` is all three: the
name in bold at `sz(size)` with the gear glyph `\u{f013}` at `sz(14)`, the gear
brightening and the background turning `bg_selected` while Preferences is open.
Pass it the same message as the Preferences page's Close button.

## Two shapes, both correct

**Direct**: the click goes straight to the settings page. Colony and Digger.
Right when settings is the only thing behind the name. This is what
`identity_button` draws.

**Menu**: the click opens a small menu, and settings is one entry in it. Grape,
whose logo menu also holds Library, Playlist and Queue. Right when the program
has several top-level destinations that are not tabs.

Do not add a menu with a single entry in it. If settings is the only thing
behind the name, go direct.

## What the three actually do

| | Colony | Digger | Grape |
|---|---|---|---|
| Chrome | vertical sidebar | horizontal tab bar | top bar |
| The button | `Colony` at `sz(30)` bold + gear glyph `\u{f013}` at `sz(14)`, one button | `{ICON} Digger`, `button::text`, size 15, accent-coloured | logo image 28×28 + `Grape` at `size(20)` semibold |
| Opens | the settings page | the settings page | a menu: Library / Playlist / Queue / Preferences / filters |
| Open indicator | gear goes `text_dimmer` → `text_primary`; background `bg_selected` | label gains a trailing close glyph | the menu is visible |
| Padding / radius | `[4, 8]`, radius 8 | `[2, 4]` | `[XS, MD]` |

The indicator mechanism is deliberately not standardised: a sidebar can afford a
background change, a text-styled button in a tab bar cannot. Pick whatever reads
clearly in your chrome. What is not optional is that *something* reads.
`identity_button` uses Colony's, which suits a sidebar; its `size` argument
covers a tab bar's smaller name.

## The rest of the sidebar

For a program with a Colony-style sidebar, below the identity button, in order:

1. A section-list label (`widgets::nav_label`): `sz(13)`, `text_muted`. Names
   the list; not clickable.
2. The section buttons (`widgets::nav_item`), each dispatching a select-section
   message with its index.
3. A keyboard-shortcut hint (`widgets::nav_hint`): `sz(10)`, `text_dimmest`.
   Deliberately the quietest text in the window: discoverable, never competing.

The dimming ramp is the point. Colony's palettes carry a six-step text ramp
(`text_primary` → `text_dimmest`) precisely so this hierarchy is expressible
without inventing one-off greys.

## Selection state

A selected item is marked with **background**, not with a coloured label:

- selected → background `accent`, text `contrast_on(accent)`
- hovered → background `bg_card_hover`
- neither → transparent background, text `text_muted`

`widgets::selection_style(selected)` is that style, for a button whose content
is more than a label; `widgets::nav_item` applies it to one.

The selected label is `contrast_on(accent)`, the legible end of near-black and
near-white for whatever accent is active, and not `text_primary`: on an accent
fill `text_primary` is below 4.5:1 on almost every theme, and on Ayu Dark it is
1.01:1.

Keep hover and selected visually distinct; a hover state that looks like
selection makes a list feel broken.

## Animation

Colony's sidebar slide is 200 ms (`App::SIDEBAR_ANIM_MS`). Motion is a user
preference (Preferences → Accessibility → Motion), and a program that animates
must honour it. Gate every animation on `colony_ui::effects_enabled()`: it is
false when the user turned Appearance → Effects off or asked for reduced motion.

## The word the user sees is "Preferences"

Colony's `settings_title` is `Preferences` in English (the canonical string)
and `Préférences` in the French locale. Grape agrees; its menu entry is
Preferences. That is settled: **the user-facing word is Preferences**, not
Settings. `colony_ui::i18n` ships the page's strings, `settings_title` among
them, so a program using the kit cannot get this wrong.

Only the *code* is inconsistent. Colony calls the message `ToggleSettings` and
prefixes every key `settings_*`; Grape calls it `OpenPreferences`. That costs
nothing to a user and is not worth a rename on its own, but a new program should
name its internals after what the screen says (`TogglePreferences`), and its own
keys `preferences_*`. The shared keys keep the `settings_*` prefix the theme and
accent labels already use; see [i18n.md](i18n.md).

Colony's own tutorial states the rule better than this page can:

> The gear icon next to the title opens the preferences.
