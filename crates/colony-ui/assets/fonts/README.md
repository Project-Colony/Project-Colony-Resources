# Fonts

The three font families every Colony program uses, and only those three
(design/typography.md). They are embedded by `colony_ui::fonts` when the
`fonts` feature is on, which it is by default.

| Directory | Family | Files | Version | Licence |
|---|---|---|---|---|
| `JetBrainsMonoNerdFont/` | `JetBrainsMono Nerd Font` | Regular, Medium, Bold | JetBrains Mono 2.304, Nerd Fonts 3.4.0 | SIL OFL 1.1, `JetBrainsMonoNerdFont/OFL.txt` |
| `OpenDyslexic/` | `OpenDyslexic` | Regular | 0.990 | SIL OFL 1.1, `OpenDyslexic/OFL.txt` |
| `FontAwesome/` | `Font Awesome 6 Free` | Solid, Regular | 6.7.2 | SIL OFL 1.1, `FontAwesome/OFL.txt` |

Sources:

- JetBrains Mono: <https://github.com/JetBrains/JetBrainsMono>, patched with the
  Nerd Fonts glyphs: <https://github.com/ryanoasis/nerd-fonts>
- OpenDyslexic: <https://github.com/antijingoist/opendyslexic>
- Font Awesome Free: <https://github.com/FortAwesome/Font-Awesome>

The SIL Open Font License lets these files be bundled and redistributed with
software, including GPL software like this crate, as long as each font keeps
its licence text and is not sold on its own. The reserved font names
(OpenDyslexic, Font Awesome) mean a modified copy must be renamed; these are
the unmodified upstream files.

The files are byte-identical to the ones the Colony launcher has shipped since
it adopted these fonts.
