<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/{{name}}-logo-512.png">
  <img src="assets/brand/png/{{name}}-logo-light-1024.png" alt="{{NAME}}" width="360">
</picture>

**One sentence saying what it is and who it is for.**

</div>

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](LICENSE)
[![Colony app](https://img.shields.io/badge/Colony-{{CATEGORY}}-purple)](https://github.com/Project-Colony/Colony)
[![Platforms](https://img.shields.io/badge/platforms-linux%20%7C%20windows%20%7C%20macOS-lightgrey)](#installation)

Two or three sentences: what exists today, why that is not enough, and what this
does instead. Concrete — a reader should be able to tell whether this solves
their problem without scrolling.

> **Status:** what actually works right now. What is proven in real use, and
> what is wired but untested. Be honest here; a reader who finds the gap
> themselves stops trusting the rest of the page.

## Why {{NAME}}

What the reader would otherwise use, and where it falls short. Three or four
bullets, each a concrete capability rather than an adjective. An ASCII diagram
here is often worth three paragraphs.

## What it does

The feature list, for someone already convinced they want it.

## Installation

### Via Colony (recommended)

Search for **{{NAME}}** in [Colony](https://github.com/Project-Colony/Colony) and
install it. Updates arrive through the launcher.

### Arch Linux (AUR)

<!-- Delete this section unless AUR packages exist. -->

```bash
paru -S {{name}}-bin
```

### Direct binary download

Grab the asset for your platform from the
[latest release](../../releases/latest):

| Platform | Asset |
|---|---|
| Linux | `{{name}}-linux` |
| Windows | `{{name}}-windows.exe` |
| macOS (Apple Silicon) | `{{name}}-macos` |
| macOS (Intel) | `{{name}}-macos-x86` |

```bash
chmod +x {{name}}-linux && ./{{name}}-linux
```

### Build from source

```bash
git clone https://github.com/Project-Colony/{{NAME}}
cd {{NAME}}
cargo build --release
```

Requires Rust {{RUST_VERSION}} or newer.

## Documentation

Full documentation is in [docs/](docs/) — start at
[docs/README.md](docs/README.md).

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

<!--
Placeholders: {{NAME}} display name, {{name}} lowercase binary name,
{{CATEGORY}} the colony.json category, {{RUST_VERSION}} the rust-version floor.

Write this in English. English is the base language of every Project Colony
repository — README, docs, code, comments, commits. French is a UI locale the
program ships, not a documentation language; see design/i18n.md.

The conventions behind this skeleton are in design/documentation.md.
-->
