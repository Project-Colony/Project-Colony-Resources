# README and docs

Every Project Colony repository is laid out the same way, so that landing in an
unfamiliar one costs nothing. This is the pattern the existing repos converged
on; where they disagree, the choice below is the one to follow going forward.

## Repository root

| File | Always? | What it is |
|---|---|---|
| `README.md` | yes | the front door — see below |
| `LICENSE` | yes | GPL-3.0-or-later, same text as the rest of the org |
| `colony.json` | for programs | the launcher manifest, see [releases.md](releases.md) |
| `CHANGELOG.md` | for programs | written by release-please, never by hand |
| `CONTRIBUTING.md` | when contributions are wanted | how to build, test, and what a good PR looks like |
| `SECURITY.md` | when there is an attack surface | how to report a vulnerability, privately |
| `docs/` | beyond a few screens | see below |

Nothing else at the root that a reader has to skip past. The *directories* that
belong there, and how crates are arranged inside them, are in
[repository-layout.md](repository-layout.md).

## The README

The front door is for someone who has never heard of the project. It answers
"what is this, why would I want it, how do I get it" — in that order, before any
detail.

```markdown
<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/<name>-logo-512.png">
  <img src="assets/brand/png/<name>-logo-light-1024.png" alt="<Name>" width="360">
</picture>

**One sentence that says what it is and who it is for.**

</div>

[![License: GPL-3.0-or-later](...)](LICENSE)
[![Colony app](...)](https://github.com/Project-Colony/Colony)
[![Platforms](...)](#installation)

Two or three sentences: what exists today, why that is not enough, and what
this does instead. Concrete, not adjectives.

> **Status:** what actually works right now, honestly.

## Why <Name>
## What it does
## Installation
### Via Colony (recommended)
### Arch Linux (AUR)
### Direct binary download
### Build from source
## Documentation
## License
```

Notes on each part:

- **The header block** is centred, with a logo that has a light and a dark
  variant via `prefers-color-scheme`. A project without a logo skips the
  `<picture>` and keeps the bold tagline.
- **Badges** — licence, the Colony category, supported platforms. Add AUR badges
  where packages exist. Badges are navigation, not decoration; if a badge links
  nowhere useful, drop it.
- **The Status blockquote** is not optional and not marketing. Say what is
  proven, and what is wired but untested. A reader who discovers the gap
  themselves stops trusting the rest of the page.
- **`## Why <Name>`** is the section that earns the reader's attention. Compare
  against what they would otherwise use. An ASCII diagram here is often worth
  three paragraphs.
- **Installation** leads with Colony, because that is the point of the
  ecosystem, then the platform-native path, then source.
- **`## Documentation`** links into `docs/`. It does not duplicate it.

Write the README in the language of its audience. An end-user program written
for a French-speaking audience has a French README; a library other developers
consume has an English one. Do not machine-translate one into the other — pick
one and write it properly.

## `docs/`

The rule, and the only one that matters:

> **Sorted by who is reading, not by subject.**

Three audiences, three directories:

```
docs/
├── README.md      the index — every page, one line each, grouped by audience
├── guide/         someone USING the program
├── internals/     someone READING or CHANGING the code
└── project/       someone asking WHY it exists and where it stands
```

| Directory | Typical pages |
|---|---|
| `guide/` | `install.md`, `usage.md`, `troubleshooting.md`, `configuration.md` |
| `internals/` | `architecture.md`, `contributing.md`, `performance.md`, `packaging.md` |
| `project/` | `landscape.md` (the problem and the alternatives), `status.md` (the done/remaining ledger), any study that drove a decision |

`docs/README.md` is the index and the first thing anyone opens. One table per
audience, one line per page, and that line says what the page answers — not what
it is called:

```markdown
# <Name> documentation

Sorted by who is reading, not by subject.

## Using <Name>

| | |
|---|---|
| [guide/install.md](guide/install.md) | getting it on your machine |
| [guide/usage.md](guide/usage.md) | the CLI and the GUI, end to end |

## Reading the code

| | |
|---|---|
| [internals/architecture.md](internals/architecture.md) | why the design is what it is |
```

**File names are lowercase kebab-case**: `adding-games.md`, not `ADDING_GAMES.md`
and not `AddingGames.md`. Colony-Firewall-Control's `ARCHITECTURE.md` /
`HARDENING.md` / `ROADMAP.md` predate this and should be renamed when that repo
is next touched.

**No dated or numbered files.** `2026-08-03-etude-de-marche.md` and `j2-*.md`
name *when they were written*, which stops being useful the moment they are
written. A study goes in `project/` under what it is about; a work journal is
what the git log is for.

**A repository small enough not to need three directories does not need them.**
Three files at `docs/` root is fine. What is not fine is growing past that
without an index — the moment someone has to `ls` to find a page, `docs/README.md`
is overdue.

## Keeping it honest

Documentation that describes something that no longer exists is worse than no
documentation, because it is trusted. When a change makes a page wrong, the page
is part of the change, not a follow-up.

`project/status.md` in particular ages badly by nature. Either keep it current or
delete it; a status page describing last quarter is actively misleading.

## This repository

Project-Colony-Resources is an exception, deliberately: its `design/` directory
*is* its product rather than its documentation, so it sits at the top level
rather than under `docs/`. A repository whose whole content is documentation says
so in its README and skips the nesting.
