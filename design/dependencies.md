# Dependencies

## The rule

**Every dependency is on its latest release. Always.**

This is not a preference or a nice-to-have. It applies:

- when a repository is created: start on the current version of everything, never
  on whatever a tutorial or an older sibling repo happened to use;
- when a feature is added: bring the tree current first, then write the feature;
- when a bug is fixed: same;
- on the weekly automated pass, whether or not anything else is happening.

There is no "we'll upgrade later". Later is how a two-line bump becomes a
week-long migration.

## Why it is absolute

An upgrade is cheap in proportion to how recently you last did one. A crate
skipped for six months is a changelog you have to read, an API you have to
relearn, and a set of behaviour changes you have to distinguish from your own
bugs, all at once, usually while trying to ship something else. Doing it weekly
means each step is small enough to be boring.

The second reason is the supply chain. A dependency you have not updated is a
dependency whose published advisories you have not read.

## How versions are written

Pin the **full current version** in the manifest:

```toml
anyhow = "1.0.104"
serde = { version = "1.0.229", features = ["derive"] }
toml = "1.1.4"
```

Not `"1.0"`. Cargo would happily resolve `"1.0"` to the newest 1.x, so a
floating minor *looks* current forever while the lockfile quietly ages, and the
manifest, which is what a human reads, stops telling the truth. Writing the full
version means staleness shows up in a diff, and `cargo update` produces a change
somebody has to look at.

A workspace declares them once in `[workspace.dependencies]` and members inherit
with `dep.workspace = true`. One place to bump.

## Lockfiles

Commit `Cargo.lock`. Every repository here either ships a binary or is consumed
as a git dependency, and in both cases a reproducible build matters more than
the flexibility a floating lock would buy.

## Automation

`templates/dependabot.yml`: weekly, grouped so a routine week is one PR rather
than fifteen. It covers both Cargo and GitHub Actions: a stale action is the
same problem as a stale crate, with more privileges.

Dependabot opening a PR is not the same as the rule being followed. The PR still
has to be merged, and the week it is ignored is the week the rule stopped
applying.

A repository without Cargo drops the `cargo` entry and adds the one for its own
ecosystem instead (`npm`, and so on), with the same schedule.

One exception, in Project-Colony/nix only. Its `.github/dependabot.yml` keeps
the `github-actions` entry and has no `nix` entry, although Dependabot supports
that ecosystem (version updates of `flake.lock` inputs, public repositories).
Its update workflow already keeps `flake.lock` current: any run that bumps no
app and finds the pinned nixpkgs more than 7 days old runs `nix flake update`,
builds every package on both architectures, and commits only once they all
build. Nothing waits for a merge there, so the rule is followed without a pull
request. A `nix` entry would add a second writer of `flake.lock`, and every one
of those commits would leave its open pull request stale. A check of that
repository expects the `github-actions` entry alone.

## Toolchain

Declare `rust-version` in `Cargo.toml` and mean it. It is a promise about who can
build the program, so it belongs in CI: an untested floor is a guess.

Move it forward deliberately, when a language feature is worth it, not by
accident because a dependency raised its own floor.

Make the lockfile keep that promise too: resolve with `resolver = "3"`. It
picks, for each crate, the newest release whose own `rust-version` is not above
yours, so `cargo update` never pulls in a transitive crate that needs a newer
toolchain than the one you declare; raise `rust-version` and the next update
takes those releases. Edition 2024 turns it on by default. On edition 2021,
set it in `[workspace]`, or in `[package]` for a single crate:

```toml
[workspace]
resolver = "3"
```

"Latest release" in the rule above means the latest release that builds on
your declared `rust-version`.

In CI, install Rust with `rustup`, which every GitHub-hosted runner image
already carries:

```yaml
- name: Install Rust
  shell: bash
  run: |
    rustup toolchain install stable --profile minimal --no-self-update
    rustup default stable
```

A third-party toolchain action is one more pin to keep current, and one that
publishes no versions to bump has nothing Dependabot can move. `shell: bash`
keeps the same step working on the Windows and macOS legs.

## GitHub Actions

Colony pins actions by commit SHA with the version in a trailing comment, and
lets Dependabot bump both together. That is the right posture for any workflow
holding `contents: write`. A release workflow can sign and publish, so a
compromised tag in a third-party action is a compromised release.

The workflow template in this repository ships pinned the same way, including
its call to the shared signing workflow. Copy it with the pins intact and add
the `github-actions` entry from `templates/dependabot.yml`, so the action SHAs
keep moving after you adopt them. Dependabot does not read `templates/`, so
the pins there are bumped by hand, to the current release of each action.

This repository's CI keeps those hand bumps honest: its `action pins` job
fails when any `uses:` in `.github/workflows/` or `templates/` is not a full
commit SHA with its version in a trailing comment, or when one action is pinned
at two different commits. A Dependabot PR that bumps an action the template
also uses goes red until the template is bumped in the same PR.

## Where the ecosystem stands

`colony-ui` is the shared floor. It declares iced once, in
`[workspace.dependencies]`, so every program that depends on it builds against
the same iced instead of each repository drifting on its own. Its `rust-version`
is the oldest toolchain that iced accepts, and CI builds on exactly that
toolchain, so a program that depends on `colony-ui` cannot usefully declare a
lower one.

## Doing an upgrade

```bash
cargo update
cargo test
```

If the tests pass, commit as `chore(deps): update dependencies`, with the
notable jumps in the body. If they fail, fix them *now*: a failing upgrade you
postpone is the exact thing this page exists to prevent, and it will not be
smaller next week.

For a major bump, read the changelog before the compiler tells you. `cargo test`
proves the code still compiles and the tests still pass; it does not prove a
behaviour change did not slip past.
