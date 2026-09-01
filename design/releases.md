# Releasing a Colony program

How a Project Colony program is versioned, built, published, and picked up by the
launcher. Reference implementations: Colony's own release workflow, and Eidos.

## The chain

```
conventional commits  →  release-please opens a release PR
merge the PR          →  a tag is created
the tag               →  builds the platform binaries
                      →  uploads them as release assets
                      →  (optionally) signs them
colony.json           →  the launcher finds and installs them
```

Nothing is released by hand. The commit messages are the input; everything after
is mechanical.

## 1. Commit messages

Conventional commits, because release-please derives the version bump and the
changelog from them.

| Prefix | Effect | Changelog section |
|---|---|---|
| `feat:` | minor bump | Features |
| `fix:` | patch bump | Fixes |
| `perf:` | patch bump | Performance |
| `refactor:` | patch bump | Internals |
| `docs:` | patch bump | Documentation |
| `chore:`, `test:`, `ci:` | patch bump | hidden |
| `feat!:` or a `BREAKING CHANGE:` footer | major bump | Features |

Write the body for the person reading `git log` in a year, not for the diff —
what was broken, what is now true, and why the approach was chosen. The changelog
is generated from the subject line, so the subject is what the *user* reads.

### Squash-merge, always

**Merge a pull request with squash, not a merge commit.** release-please reads
the *merge commits* on the default branch and parses each one's own message as a
conventional commit. A squash produces exactly that: one commit, carrying the PR
title, which GitHub associates with the PR.

A merge commit produces `Merge pull request #76 from …`, which is not a
conventional commit. release-please sees nothing releasable, no release PR is
opened, and the change reaches users with no changelog entry — even though every
commit *inside* the branch was perfectly well formed. Those are invisible to it.

The same trap catches the obvious repair: pushing a conventional commit straight
to the branch does not help either, because a direct push is not a merge commit.
The fix has to arrive the way the tool looks for it — as a squashed pull request.

This is why the PR title matters more than the commit titles inside it: the
title *is* the changelog entry.

## 2. Versioning

release-please owns the version number. Two configurations are in use:

- **`release-type: rust`** — release-please understands Cargo and bumps
  `Cargo.toml` (and the lockfile) itself. Fewest moving parts; use this for a
  single-crate program.
- **`release-type: simple`** with `"extra-files": ["Cargo.toml"]` — release-please
  tracks the version in `.release-please-manifest.json` and rewrites the version
  wherever it is told to. Use this for a workspace, or when the version also has
  to appear somewhere that is not Cargo metadata.

Both keep `CHANGELOG.md` at the repo root. `templates/release-please-config.json`
is the shared starting point, with the changelog sections already set.

## 3. Asset naming — this is the contract

Colony auto-detects which platforms a program supports **from the release asset
names**. Follow the convention and the manifest stays two lines.

| Platform key | Asset name | Build target |
|---|---|---|
| `linux` | `<repo>-linux` | `x86_64-unknown-linux-gnu` |
| `windows` | `<repo>-windows.exe` | `x86_64-pc-windows-msvc` |
| `macos` | `<repo>-macos` | `aarch64-apple-darwin` (Apple Silicon) |
| `macos-x86` | `<repo>-macos-x86` | `x86_64-apple-darwin` (Intel) |

`<repo>` is the repository name, lowercased. Colony compares
case-insensitively, but write it lowercase.

`macos` means Apple Silicon and `macos-x86` means Intel — Colony chooses between
them with `cfg!(target_arch)` at runtime. A program shipping only `macos` is
simply unavailable to Intel Macs, which is a legitimate choice, not a bug.

If the assets cannot follow the convention — a versioned archive, a bundle, an
installer — declare them explicitly in `colony.json` instead. See below.

## 4. `colony.json`

Every repository that should appear in the launcher ships one at its root. The
authoritative shape is `generated/colony.schema.json`; point your editor at it:

```json
{
  "$schema": "https://raw.githubusercontent.com/Project-Colony/Project-Colony-Resources/main/generated/colony.schema.json",
  "name": "Eidos",
  "category": "system"
}
```

That is the whole file when the assets follow the naming convention. Working
examples for each shape live in [`manifests/examples/`](../manifests/examples/)
and are validated by `cargo test`.

**Categories** — `development`, `graphics`, `network`, `office`, `multimedia`,
`system`, `utility`, `security`, `game`, `other`. Matched case-insensitively;
`utilities` and `games` are accepted aliases. An unrecognized category is
*warned about and ignored*, which files the program nowhere — so a typo here
fails quietly. `cargo test` in this repo catches it for the examples; the schema
catches it in your editor.

**Icons** — a repo-relative square PNG via `"icon"`. When absent, Colony probes
`icon.png` at the repo root, then falls back to a hexagon tinted by
`app_tint(name)` — a deterministic hash of the program's *name* into the eight
shared accents, so a program without an icon still gets a stable identity colour.

**`releaseFiles`** — only when auto-detection cannot work. Per platform:

- `tag` — a git tag, or `"latest"` to always track the newest release.
- `file` — the exact asset name, **or** `filePattern` — a case-insensitive
  pattern for assets whose name carries the version. Exactly one of the two, and
  the pattern must match exactly one asset or the install fails.
- `binary` — the binary to extract from inside a `.zip` / `.tar.gz`. Omit when
  the downloaded file is itself the binary.
- `sha256` — 64 lowercase hex characters, optional.

If you provide `releaseFiles`, it must cover every platform you declare in
`platforms`. A partial map means a platform the launcher lists but cannot
install.

## 5. Signing

Three artefacts, three different jobs. They are often spoken of together, so it
is worth being precise about what each one is:

| | What it is | Where it lives | Proves |
|---|---|---|---|
| `"signed": true` | a boolean field | `colony.json`, in the repo | *policy*: this program promises every asset is signed, so a missing signature must abort the install rather than fall back |
| `<asset>.sig` | 64 raw bytes | a release asset | *provenance*: these bytes came from the organisation's key |
| `<asset>.meta` + `<asset>.meta.sig` | a three-line text file and its own signature | release assets | *identity*: these bytes are **this** asset at **this** version |

The sidecar exists because provenance alone is not enough. A signature over raw
bytes says the organisation produced them; it does not say which artefact or
which release they are. Without the sidecar, anyone able to control what the
release host serves could take a genuinely signed, older, known-vulnerable
build, publish it under a new tag, and the launcher would install it as an
update with every indicator green. The sidecar binds the bytes to a filename, a
digest and a version, so that replay is refused.

The version rule differs by consumer, and deliberately so:

- **The launcher** requires strictly newer than the running build. It is
  updating itself; reapplying its own version is never right.
- **A program** requires no *older* than what is installed. Equal is fine,
  because a program pinned to a fixed `tag` must stay reinstallable.

`templates/sign-release.sh` produces all three files. It needs nothing but
`openssl`:

```bash
COLONY_SIGNING_KEY=/path/to/colony-release.pem \
COLONY_RELEASE_VERSION=v1.2.3 \
  ./sign-release.sh <asset>...
```

In CI, write the private key from the `COLONY_SIGNING_KEY_PEM` organisation
secret to a temporary file and point `COLONY_SIGNING_KEY` at it. The private key
never lives in a repository. Upload every `.sig`, `.meta` and `.meta.sig`
alongside its binary — `templates/release.yml` does all of this.

### Adopting signatures in a program that already ships

Sidecars are **opportunistic and then pinned**. A program that publishes none
still installs normally; once the launcher has verified one for a program, a
later release that stops publishing them is refused. That is what lets the
organisation adopt them one repository at a time with no flag day, and what
stops a compromised repository from quietly opting back out.

So the migration is per-repo and safe in any order:

1. Copy `templates/sign-release.sh` to `scripts/sign-release.sh` in the program's
   repository, and `templates/release.yml` over
   `.github/workflows/release.yml`.
2. Set `"signed": true` in `colony.json` — **only after** the first release that
   actually carries signatures. Declaring it earlier fails closed and makes the
   current release uninstallable.
3. Release as usual.

**The trap, if you are editing an older hand-written signing job rather than
replacing it.** Those jobs download the previous release's assets and strip the
companions before signing:

```bash
rm -f dist/*.sig dist/*.sha256 dist/*.txt dist/*.yml dist/*.json dist/*.asc
```

`*.meta` is not in that list. The first release after adopting sidecars works;
the *second* downloads the previous `.meta`, treats it as an asset, and signs it
— producing `foo-linux.meta.sig.sig` and a `.meta` describing a `.meta`. Add
`dist/*.meta` to that `rm` line, or replace the job with the template, which
does not have the problem.

### Rotating the key

`src/signing.rs` in Colony embeds a **list** of accepted keys, and a signature is
accepted if any listed key validates it. That list is what makes rotation
possible at all: with a single key, the one `.sig` a release carries is either
old-key (refused by every updated client) or new-key (refused by every client in
the field), and verification is fail-closed, so the refusal is permanent either
way.

Rotate over three releases of the launcher:

| Release | embedded keys | signed with | who can still update |
|---|---|---|---|
| N | `[new, old]` | **old** | everyone; afterwards they trust both |
| N+1 | `[new, old]` | **new** | everyone on N or later |
| N+2 | `[new]` | **new** | everyone on N or later; `old` is revoked |

N **must** be signed with the outgoing key — its whole job is to widen the
trusted set on machines that only trust `old`. Do not skip to N+2: anyone still
on N-1 when `old` is dropped can no longer self-update and must reinstall by
hand.

### Validating before you ship

```bash
colony validate-manifest colony.json
```

Pass the asset names the release publishes to also check that every platform
actually **resolves** — which is the failure that matters, because a manifest can
be structurally perfect and still leave the program listed with no Download
button:

```bash
gh release view v1.2.3 --json assets --jq '.assets[].name' > names.txt
colony validate-manifest colony.json $(tr '\n' ' ' < names.txt)
```

It exits non-zero on any problem. `templates/release.yml` runs it on every
release, before signing.

## 6. Release profile

Colony's `[profile.release]`, worth copying for anything shipping a binary:

```toml
[profile.release]
lto = "thin"
codegen-units = 1
strip = "symbols"
```

Thin LTO and a single codegen unit for speed and size; stripping symbols because
users download this over the network. Set `rust-version` in `Cargo.toml` to the
oldest toolchain you actually support, and mean it — it is a promise, and CI
should be the thing that keeps you honest about it.

## 7. Checklist for a new program

1. `colony.json` at the repo root — name and category, plus an icon if you have
   one. Validate it: `colony validate-manifest colony.json`.
2. `release-please-config.json` and `.release-please-manifest.json` from
   `templates/`.
3. `.github/workflows/release.yml` from `templates/release.yml`, with
   `{{APP_NAME}}` replaced by the binary name.
4. If you are signing: `templates/sign-release.sh` to `scripts/sign-release.sh`,
   and set `"signed": true` in `colony.json` **only after** the first release
   that carries signatures.
5. `CHANGELOG.md` — release-please creates it on the first release; you do not
   write it.
6. GPL-3.0-or-later `LICENSE`, matching the rest of the organisation.
7. Conventional commits from the first commit onward.

## 8. Checklist for a program that already ships

For the repositories that predate the current template. Safe in any order, one
repository at a time — nothing here requires coordinating a flag day.

1. Replace `.github/workflows/release.yml` with `templates/release.yml`, rather
   than patching the existing job. Hand-written signing jobs in the
   organisation predate the `.meta` sidecar, sign only on Linux and macOS, and
   strip companions with an `rm` line that does not know about `.meta` (see
   §5). Replacing avoids all three.
2. Copy `templates/sign-release.sh` to `scripts/sign-release.sh`.
3. Confirm the repository can read the `COLONY_SIGNING_KEY_PEM` organisation
   secret — it is restricted to Project-Colony repositories, so a new or renamed
   repository has to be added to that list.
4. Release. The first signed release publishes `.sig`, `.meta` and `.meta.sig`
   for every asset.
5. Only now, if it is not already set, add `"signed": true` to `colony.json`.

The launcher pins the sidecar the first time it verifies one for a program, so
step 4 is what actually switches the protection on for that program's users, and
after it no release of that program may stop publishing sidecars.

### Where each piece lives

| Piece | Canonical location |
|---|---|
| Release workflow | `templates/release.yml` (this repo) |
| Signing script | `templates/sign-release.sh` (this repo) |
| Manifest schema | `generated/colony.schema.json` (this repo) |
| Manifest examples | `manifests/examples/` (this repo) |
| Manifest validator | `colony validate-manifest`, shipped in the launcher |
| Embedded trust keys | `src/signing.rs` in Project-Colony/Colony |
| Private signing key | off-machine; `COLONY_SIGNING_KEY_PEM` in CI, never in a repository |

There is deliberately **one** copy of the workflow and **one** of the script.
They used to be duplicated in the Colony repository, the two drifted, and the
result was a template naming a secret that does not exist and skipping Windows
signing entirely — neither visible from either side.
