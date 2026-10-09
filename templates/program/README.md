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
does instead. Concrete: a reader should be able to tell whether this solves
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

Full documentation is in [docs/](docs/). Start at
[docs/README.md](docs/README.md).

## Code signing policy

Every release asset is signed by the Project Colony organisation in CI, never on
a developer machine. Next to each asset on the release page:

| File | What it is |
|---|---|
| `<asset>.sig` | an ed25519 signature over the asset, made with the organisation's release key |
| `<asset>.meta` | three lines binding the asset to its file name, its sha256 and the release version |
| `<asset>.meta.sig` | an ed25519 signature over the `.meta` |

The private key is an organisation secret, used only by the shared
[sign-and-publish workflow](https://github.com/Project-Colony/Project-Colony-Resources/blob/main/.github/workflows/sign-and-publish.yml)
in a job that builds nothing; the jobs that compile {{NAME}} never see it.
Colony checks all three files before it installs or updates {{NAME}}, and
refuses a release older than the one installed. To check a download yourself
with OpenSSL 3 (the same commands work for every asset):

```bash
cat > colony-release.pub <<'EOF'
-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEARNjg3Nn8H6/aBg1unwGjkUTcrdTxERNefVaqU8cFu0s=
-----END PUBLIC KEY-----
EOF
a={{name}}-linux
openssl pkeyutl -verify -pubin -inkey colony-release.pub -rawin -in "$a" -sigfile "$a.sig"
openssl pkeyutl -verify -pubin -inkey colony-release.pub -rawin -in "$a.meta" -sigfile "$a.meta.sig"
cat "$a.meta"     # version=<tag>, asset=<file name>, sha256=<digest>
sha256sum "$a"    # the digest must equal the sha256 line
```

How releases are built, signed and published:
[design/releases.md](https://github.com/Project-Colony/Project-Colony-Resources/blob/main/design/releases.md#5-signing).

<!--
Add the following only once SignPath signs the Windows build, that is once
`signpath-project-slug` is set in .github/workflows/release.yml. Before that it
would promise a signature the .exe does not carry.

Windows releases are also Authenticode-signed. Free code signing provided by
[SignPath.io](https://about.signpath.io/), certificate by
[SignPath Foundation](https://signpath.org/).

- Committers and reviewers: <GitHub accounts allowed to change the source>
- Approvers: <GitHub accounts that approve each signing request in SignPath>
-->

## Privacy

{{NAME}} sends no telemetry, no analytics and no crash reports.

| Data | Stored or sent | Where, and why |
|---|---|---|
| Settings | stored | `Colony/{{NAME}}/` in this machine's config directory |
| Credentials | stored | _where they are kept, e.g. the operating system's keyring; delete the row if there are none_ |
| _one row for everything else the code keeps or sends_ | | |

{{NAME}} connects to no server except the ones in this table, and only for the
action named there.

<!--
Write what the code actually does, and re-read it whenever a change adds a
network call, a file or a credential: every host contacted (GitHub API, update
checks, Nexus, Discord...), what is sent to it and when, every file written, and
where secrets live. Link the privacy policy of each third-party service it
talks to. Nothing the code does may be missing, and nothing claimed may be
false.

A program that contacts nothing can say so in the sentence SignPath Foundation
accepts as a privacy policy: "This program will not transfer any information to
other networked systems unless specifically requested by the user or the person
installing or operating it."
-->

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

<!--
Placeholders: {{NAME}} display name, {{name}} lowercase binary name,
{{CATEGORY}} the colony.json category, {{RUST_VERSION}} the rust-version floor.

Write this in English. English is the base language of every Project Colony
repository: README, docs, code, comments, commits. French is a UI locale the
program ships, not a documentation language; see design/i18n.md.

The conventions behind this skeleton are in design/documentation.md.
-->
