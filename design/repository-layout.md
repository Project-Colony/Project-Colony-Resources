# Repository and crate layout

Every Project Colony repository is arranged the same way, so that finding a thing
does not require reading the whole tree first. This matters more than it sounds:
most of the cost of changing unfamiliar code is locating what to change, and a
predictable layout removes that cost entirely — for a person returning after six
months, for a new contributor, and for an assistant asked to make a change in a
repository it has never seen.

This is the pattern the existing repos converged on.

## One crate, or a workspace?

Both are correct. The question is whether there is a real boundary.

**One crate** when there is one program and one process. Colony is this: a single
binary, subsystems as directories under `src/`.

**A workspace** when a boundary genuinely exists — a separate process, a different
build target, or a library that something else really does consume. Eidos has 17
crates because it really does ship a CLI, a GUI, a FUSE daemon and a launch
shim, all sharing a core.

Do not split by layer for its own sake. Three crates named `-model`, `-service`
and `-view` for one binary buy nothing and cost a dependency graph. Split when
two pieces have different lifetimes, different consumers, or different targets.

## Workspace shape

```
<repo>/
├── Cargo.toml            [workspace] + [workspace.package] + [workspace.dependencies]
├── Cargo.lock            committed
├── crates/
│   ├── <prefix>/         the main binary — bare prefix, no suffix
│   ├── <prefix>-core/    domain logic, the fewest dependencies of anything here
│   ├── <prefix>-proto/   wire types shared across a process boundary
│   ├── <prefix>-gui/     the iced frontend
│   └── <prefix>-cli/     the terminal frontend
├── assets/               fonts, brand, icons
├── docs/                 see documentation.md
├── packaging/            distribution: AUR, .desktop, systemd units
├── scripts/              sign-release.sh and friends
└── colony.json
```

`<prefix>` is the program's short lowercase name — `eidos`, `cfc`, `vn`, `xion`.
Every crate in the workspace carries it, so a crate name is unambiguous in a
`cargo` error message, in a dependency tree, and in a search across all repos.

Role suffixes that already mean something across the ecosystem — reuse them
rather than inventing a synonym:

| Suffix | What lives there |
|---|---|
| *(none)* | the main binary |
| `-core` | the domain logic; the crate with the fewest dependencies |
| `-proto` | types crossing a process or network boundary |
| `-gui` / `-ui` | the iced frontend |
| `-cli` | the terminal frontend |
| `-daemon` | the long-running or privileged service |
| `-client` | the thing that talks to the daemon |
| `-log` | logging setup, shared so every binary logs identically |

Anything past that is domain vocabulary and should read as such: `eidos-fomod`,
`eidos-conflicts`, `cfc-proto`. A crate whose name does not tell you what it is
about is a crate that will accumulate whatever has nowhere else to go.

## `[workspace.package]`

One version for the whole workspace, inherited everywhere:

```toml
[workspace.package]
version = "1.4.1"      # x-release-please-version
license = "GPL-3.0-or-later"
repository = "https://github.com/Project-Colony/<Repo>"
edition = "2021"
rust-version = "1.80"
```

```toml
[package]
name = "eidos-core"
description = "Layer-resolution engine for the Eidos mod VFS"
version.workspace = true
license.workspace = true
repository.workspace = true
edition.workspace = true
```

The `description` is per-crate and mandatory. One line saying what this crate is
for. It is what `cargo tree` shows and what tells the next person whether they
are in the right file, and it is the cheapest documentation in the repository.

Per-crate `README.md` files are *not* the convention — none of the repos have
them, and they rot. The crate description plus `docs/internals/architecture.md`
carry that weight.

A single version across the workspace is deliberate. Eidos states the reason in a
comment worth copying: the version reaches users in the About dialog, in the
`Application-Version` header of every Nexus API call, and in the log header — so
a crate left behind at `0.0.0` would be telling a third party something untrue.

## Platform-specific crates

When part of the workspace only builds for one platform, keep it out of the
default set so a bare `cargo test` still does the right thing everywhere:

```toml
default-members = ["crates/xion-proto", "crates/xion-console"]
```

and build the excluded crates explicitly, with `--target`, from the justfile or
Makefile. Without this, `cargo test` at the root fails on the developer's own
machine for reasons that have nothing to do with their change — and a test
command people learn to avoid is a test suite that stops running.

## Inside `src/`

A subsystem gets a directory with a `mod.rs`; everything else is a flat file.
Colony:

```
src/
├── main.rs        entry point, nothing else
├── app.rs         the update loop
├── state.rs       the application state
├── message.rs     every message the app can receive
├── github/        mod.rs, catalog.rs, releases.rs, http.rs, types.rs
├── ui/            mod.rs, sidebar.rs, settings.rs, app_grid.rs, theme.rs
├── i18n/          mod.rs, fr.rs, en.rs
└── update/        mod.rs
```

The rule is that a directory earns its existence by having more than one file in
it. `github/` is a directory because there are five files about talking to
GitHub; `state.rs` is one file because there is one thing to say about state.

Put `types.rs` next to the code that owns the types, not in a repo-wide `types`
crate. `github/types.rs` holds the wire shapes `github/` deserializes, and its
doc comment says "no behaviour lives here" — which is what keeps it honest.

## File size

There is no hard limit, but a file you navigate by scrolling is a module asking
to be split. For calibration, the largest files in Colony today:

| | |
|---|---|
| `src/ui/theme.rs` | 100 KB |
| `src/ui/settings.rs` | 64 KB |
| `src/download.rs` | 52 KB |

`theme.rs` is the cautionary one, and the reason this repository exists: 100 KB
of hand-maintained colour constants that another repository ended up downloading
and regex-parsing because there was no better way to reach them. It is now
generated from `tokens/`. `settings.rs` at 64 KB is the next candidate — its
sections are independent and would read better as `ui/settings/{general,
appearance,accessibility}.rs`.

Data does not belong in source files. A table that a human maintains and a
machine reads is a data file plus a generator, not a `const` array.

## Repository root

Nothing at the root that a reader has to skip past. Directories only:

| Directory | When |
|---|---|
| `crates/` or `src/` | always — one or the other, never both |
| `assets/` | fonts, brand images, icons |
| `docs/` | beyond a few screens of prose — see [documentation.md](documentation.md) |
| `packaging/` | AUR PKGBUILDs, `.desktop` files, systemd units |
| `scripts/` | release signing, generators, dev helpers |
| `config/` | data files the program embeds or reads at runtime |
| `reference/` | third-party material kept for study; never compiled |

Root *files* are listed in [documentation.md](documentation.md). Where the
program writes at **runtime** — config, data and cache on each platform — is
[filesystem.md](filesystem.md); none of it belongs in the repository.

## When you add a crate

1. `crates/<prefix>-<role>/`, with a role that means what it already means above.
2. Add it to `members`, and to `default-members` too if it is platform-specific.
3. Inherit `version`, `license`, `repository`, `edition` from the workspace.
4. Write the `description`. One line, what it is for.
5. Dependencies at their latest release, declared in `[workspace.dependencies]`
   and inherited — see [dependencies.md](dependencies.md).
6. If it changes how the program is built or laid out, say so in
   `docs/internals/architecture.md` in the same commit.
