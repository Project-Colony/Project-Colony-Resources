# Dependencies

## The rule

**Every dependency is on its latest release. Always.**

This is not a preference or a nice-to-have. It applies:

- when a repository is created — start on the current version of everything, never
  on whatever a tutorial or an older sibling repo happened to use;
- when a feature is added — bring the tree current first, then write the feature;
- when a bug is fixed — same;
- on the weekly automated pass, whether or not anything else is happening.

There is no "we'll upgrade later". Later is how a two-line bump becomes a
week-long migration.

## Why it is absolute

An upgrade is cheap in proportion to how recently you last did one. A crate
skipped for six months is a changelog you have to read, an API you have to
relearn, and a set of behaviour changes you have to distinguish from your own
bugs — all at once, usually while trying to ship something else. Doing it weekly
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
floating minor *looks* current forever while the lockfile quietly ages — and the
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
than fifteen. It covers both Cargo and GitHub Actions — a stale action is the
same problem as a stale crate, with more privileges.

Dependabot opening a PR is not the same as the rule being followed. The PR still
has to be merged, and the week it is ignored is the week the rule stopped
applying.

## Toolchain

Declare `rust-version` in `Cargo.toml` and mean it. It is a promise about who can
build the program, so it belongs in CI — an untested floor is a guess.

Move it forward deliberately, when a language feature is worth it, not by
accident because a dependency raised its own floor.

## GitHub Actions

Colony pins actions by commit SHA with the version in a trailing comment, and
lets Dependabot bump both together. That is the right posture for any workflow
holding `contents: write` — a release workflow can sign and publish, so a
compromised tag in a third-party action is a compromised release.

The templates in this repository ship with version tags (`@v4`) because a SHA
has to be resolved against the real repository at the time you adopt them. Pin
them when you copy them, in anything that can write.

## Where the ecosystem stands

Measured across the Rust programs, at the time this was written:

| | |
|---|---|
| iced | 0.14 everywhere except **D1Gg2r, still on 0.13** |
| `rust-version` | declared in 2 repositories of 5, at **1.78** and **1.80** |

Both are exactly the drift this rule exists to prevent, and neither was noticed
until someone went looking. Nothing pins a shared floor today; `colony-ui` will
carry one in `[workspace.dependencies]` when it lands, and the programs that
depend on it inherit it.

## Doing an upgrade

```bash
cargo update
cargo test
```

If the tests pass, commit — `chore(deps): update dependencies`, with the
notable jumps in the body. If they fail, fix them *now*: a failing upgrade you
postpone is the exact thing this page exists to prevent, and it will not be
smaller next week.

For a major bump, read the changelog before the compiler tells you. `cargo test`
proves the code still compiles and the tests still pass; it does not prove a
behaviour change did not slip past.
