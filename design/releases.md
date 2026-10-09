# Releasing a Colony program

How a Project Colony program is versioned, built, published, and picked up by the
launcher. Reference implementation:
[`templates/sign-and-publish-caller.yml`](../templates/sign-and-publish-caller.yml),
which builds on each platform and hands signing and publishing to the
organisation's shared workflow. Colony's own release workflow calls the same
shared workflow.

## The chain

```
conventional commits  →  release-please opens a release PR
merge the PR          →  a tag is created, its release held as a draft
the tag               →  builds the platform binaries (no key on these runners)
                      →  sign-and-publish signs them, uploads them,
                         verifies them and publishes the release
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

Write the body for the person reading `git log` in a year, not for the diff:
what was broken, what is now true, and why the approach was chosen. The
changelog is generated from the subject line, so the subject is what the *user*
reads.

### From adoption onward

Conventional commits, and the squash-only merges below, are required on the
default branch from the moment a repository adopts these rules. A repository
created in the organisation adopts them with its first commit. For the
repositories that were already in the organisation, adoption is 2026-10-09.

History from before adoption is kept exactly as it is, and is never rewritten
to satisfy either rule. Rewriting a published default branch breaks every
clone and every open branch, loses the commit release-please last released
from, and leaves published tags pointing at commits the branch no longer
contains. A check of these rules therefore reads the history from adoption on
(`git log --since='2026-10-09 00:00 +0000'` for those repositories), not the
whole log. Give the time and the zone: with a bare date, git keeps the current
time of day, so the check silently skips the commits of the adoption day itself.

### When a release goes out empty

`gh` in a job with no `actions/checkout` has no git remote to infer the
repository from, and dies with `fatal: not a git repository`. Every `gh` call in
the release workflow template and in the shared signing workflow therefore
passes `-R "$GITHUB_REPOSITORY"`. Without it the draft-hold step fails, the
builds and the publishing are skipped as a consequence, and the tag is
published carrying no assets at all, which Colony's installer then finds nothing
to fetch from. Both Colony (v0.10.0) and Grape (v0.3.0) shipped an empty release
this way.

The second half of the same lesson: release-please emits `release_created` only
once per release, so re-running the workflow after such a failure does nothing.
It reports no work and every downstream job skips, while the run goes green.
The template's `workflow_dispatch` input exists for exactly that repair, and the
shared workflow refuses a tag that is already published, because overwriting
live assets would leave their signatures describing bytes that no longer exist.

### Squash-merge, always

**Merge a pull request with squash, not a merge commit, and never push feature
work straight to the default branch.** release-please reads the *merge commits*
on the default branch and parses each one's own message as a conventional
commit. A squash produces exactly that: one commit, carrying the PR title,
which GitHub associates with the PR.

A merge commit produces `Merge pull request #76 from …`, which is not a
conventional commit. release-please sees nothing releasable, no release PR is
opened, and the change reaches users with no changelog entry, even though every
commit *inside* the branch was perfectly well formed. Those are invisible to it.

The same trap catches the obvious repair: pushing a conventional commit straight
to the branch does not help either, because a direct push is not a merge commit.
The fix has to arrive the way the tool looks for it: as a squashed pull request.

Every repository therefore allows squash merges only, with the squash commit
title set to `PR_TITLE` and the message to `COMMIT_MESSAGES`, and deletes the
branch on merge. **The PR title is the changelog entry**: write it as a
conventional commit, for the user.

The title setting is not cosmetic. GitHub's default, `COMMIT_OR_PR_TITLE`, uses
the pull request title only when the branch has more than one commit; a
single-commit branch is squashed under that commit's own subject, and the PR
title is ignored, including one edited just before merging. A one-commit
branch whose commit says `chore:` then releases nothing, however the pull
request is titled. Check the setting on any repository that is new to the
organisation.

Recovering afterwards is awkward: the change is already on `main` under a
subject release-please will not act on, and pushing a corrected commit directly
does not help either (see the trap above). `Release-As: X.Y.Z` in the footer of
a later commit is the designed escape hatch.

## 2. Versioning

release-please owns the version number. Two configurations are in use:

- **`release-type: rust`**: release-please understands Cargo and bumps
  `Cargo.toml` (and the lockfile) itself. Fewest moving parts; use this for a
  single-crate program.
- **`release-type: simple`** with `"extra-files": ["Cargo.toml"]`: release-please
  tracks the version in `.release-please-manifest.json` and rewrites the version
  wherever it is told to. Use this for a workspace, or when the version also has
  to appear somewhere that is not Cargo metadata.

Both keep `CHANGELOG.md` at the repo root. `templates/release-please-config.json`
is the shared starting point, with the changelog sections already set.

Either way, the release type goes in `release-please-config.json` and the
workflow passes the action `config-file` and `manifest-file` only. Passing the
action's `release-type` input as well makes it ignore both files, and with them
the changelog sections, `extra-files` and the manifest version, with no
warning.

Do not set `separate-pull-requests` to `false` for a single-package repository. Its default is `true` when the config has one package, which names the release branch `release-please--branches--main--components--<name>`. Forced to `false`, the branch loses its component, and release-please 17.x then refuses to release the merged release PR (`PR component: undefined does not match configured component`) whenever the package has a name, as Rust and Node packages do. SAM-Colony-Edition 0.7.0 was stuck this way.

### Libraries published to crates.io

A library, such as `colony-ui` in this repository, goes through the same
release-please chain, with two differences. It is a release-please component
in a workspace (`include-component-in-tag`, tags like `colony-ui-v0.1.7`, the
`cargo-workspace` plugin so the root `Cargo.lock` moves in the release PR),
and its tag publishes to crates.io instead of building assets. Publishing uses
crates.io Trusted Publishing: the publish job exchanges its GitHub OIDC
identity for a token that lives only for that job, and the crate's trusted
publisher names the repository, `release.yml` and the `crates-io`
environment. No registry token is stored as a secret, and no signing key is
involved: the ed25519 signatures of section 5 are for what the launcher
installs, and crates.io checksums what cargo downloads. See
[`.github/workflows/release.yml`](../.github/workflows/release.yml) and
`release-please-config.json` in this repository.

## 3. Asset naming: this is the contract

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

`macos` means Apple Silicon and `macos-x86` means Intel; Colony chooses between
them with `cfg!(target_arch)` at runtime. A program shipping only `macos` is
simply unavailable to Intel Macs, which is a legitimate choice, not a bug.

If the assets cannot follow the convention (a versioned archive, a bundle, an
installer), declare them explicitly in `colony.json` instead. See below.

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

**Categories**: `development`, `graphics`, `network`, `office`, `multimedia`,
`system`, `utility`, `security`, `game`, `other`. Matched case-insensitively;
`utilities` and `games` are accepted aliases. An unrecognized category is
*warned about and ignored*, which files the program nowhere, so a typo here
fails quietly. `cargo test` in this repo catches it for the examples; the schema
catches it in your editor.

**Icons**: a repo-relative square PNG via `"icon"`. When absent, Colony probes
`icon.png` at the repo root, then falls back to a hexagon tinted by
`app_tint(name)`, a deterministic hash of the program's *name* into the eight
shared accents, so a program without an icon still gets a stable identity colour.

**`releaseFiles`**: only when auto-detection cannot work. Per platform:

- `tag`: a git tag, or `"latest"` to always track the newest release.
- `file`: the exact asset name, **or** `filePattern`: a case-insensitive
  pattern for assets whose name carries the version. Exactly one of the two, and
  the pattern must match exactly one asset or the install fails.
- `binary`: the binary to extract from inside a `.zip` / `.tar.gz`. Omit when
  the downloaded file is itself the binary.
- `sha256`: 64 lowercase hex characters, optional.

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

**Who signs, and where.** The [shared signing workflow](#shared-signing-workflow)
writes all three files, in CI, for every program. Its signing job checks out
nothing and compiles nothing: it downloads the finished binaries, writes the
key from the `COLONY_SIGNING_KEY_PEM` organisation secret to a temporary file,
signs, and removes the file. The private key never lives in a repository or on
a developer machine.

**The key never reaches a job that compiles code.** `cargo build` runs the
`build.rs` and procedural macros of every dependency, and a process started
there can keep running until the job ends, reading whatever a later step of the
same job writes, a key file included. `COLONY_SIGNING_KEY_PEM` is also the key
Colony trusts for its own self-updates, so one compromised crate in one
program's build would be enough to forge launcher updates for everyone. That is
why no template signs inside a build leg any more, and why a program's release
workflow passes the secret to the shared workflow and to nothing else.

**Checking a release by hand** takes OpenSSL 3 and the public key, which is in
`src/signing.rs` of Colony and, as PEM, in `templates/program/README.md`:

```bash
openssl pkeyutl -verify -pubin -inkey colony-release.pub -rawin -in <asset> -sigfile <asset>.sig
openssl pkeyutl -verify -pubin -inkey colony-release.pub -rawin -in <asset>.meta -sigfile <asset>.meta.sig
```

Then `<asset>.meta` must read exactly `version=<tag>`, `asset=<file name>` and
`sha256=<sha256 of the asset>`. A program's README says this under
`## Code signing policy`; `templates/program/README.md` has that section ready,
public key included, and the shared workflow links every release's notes to it.

### Adopting signatures in a program that already ships

Sidecars are **opportunistic and then pinned**. A program that publishes none
still installs normally; once the launcher has verified one for a program, a
later release that stops publishing them is refused. That is what lets the
organisation adopt them one repository at a time with no flag day, and what
stops a compromised repository from quietly opting back out.

So the migration is per-repo and safe in any order:

1. Copy `templates/sign-and-publish-caller.yml` over
   `.github/workflows/release.yml`.
2. Release as usual. Every asset now ships its `.sig`, `.meta` and `.meta.sig`.
3. Only then set `"signed": true` in `colony.json`. Declaring it before a
   release actually carries signatures fails closed and makes the current
   release uninstallable.

**Replace an older signing job; do not patch it.** Release workflows written
before the shared workflow sign inside the build legs, where the key shares a
job with the compiler (see above). Some of them sign only on Linux and macOS,
and some strip the previous release's companions with an `rm` line that does
not know about `.meta`, so the second release after adopting sidecars signs the
previous `.meta` as if it were an asset and publishes
`foo-linux.meta.sig.sig`. Patching keeps all of that. Replace the whole
workflow with the template; a `scripts/sign-release.sh` the repository may have
is then unused, and goes.

### Shared signing workflow

[`.github/workflows/sign-and-publish.yml`](../.github/workflows/sign-and-publish.yml)
is a reusable workflow that does everything after the build: Authenticode
through SignPath for the Windows files (once a repository turns it on), the
ed25519 `.sig`, `.meta` and `.meta.sig` for every asset, upload, verification,
and publishing. A program's release workflow calls it; the template is
[`templates/sign-and-publish-caller.yml`](../templates/sign-and-publish-caller.yml).
It works the same with SignPath off, so a program can adopt it today and turn
SignPath on later without touching anything else.

**When to call it.** As the last job of the release workflow, in the **same
run** as the builds, with `needs:` on every build job. The build legs upload
each binary with `actions/upload-artifact`, at the artifact root under its
release asset name, and upload nothing to the release themselves. The release
must already exist as a **draft** for the tag (the draft hold after
release-please); the workflow refuses a release that is already published, and
checks again just before uploading, since the SignPath wait can take hours.

```yaml
sign-and-publish:
  needs: [release-please, build]
  if: ${{ !cancelled() && needs.build.result == 'success' }}
  permissions:
    actions: read     # SignPath downloads the unsigned artifact with the job token
    contents: write   # drafts, uploads, publishing
  uses: Project-Colony/Project-Colony-Resources/.github/workflows/sign-and-publish.yml@619460ff4dc0049f129955f0988d368427b9eedb # organisation pin
  with:
    tag: ${{ needs.release-please.outputs.tag_name || inputs.tag }}
    assets: "grape-linux grape-windows.exe grape-macos grape-macos-x86"
    artifact-pattern: build-*
    signpath-project-slug: grape   # leave out while SignPath is off
  secrets:
    COLONY_SIGNING_KEY_PEM: ${{ secrets.COLONY_SIGNING_KEY_PEM }}
    SIGNPATH_API_TOKEN: ${{ secrets.SIGNPATH_API_TOKEN }}
```

**The pin.** Every caller pins the shared workflow at the same commit,
`619460ff4dc0049f129955f0988d368427b9eedb`, which is the one the template names.
The pin moves only when `sign-and-publish.yml` itself changes, and then the
template and every caller move to the new commit together. A commit to this
repository that does not touch the workflow is no reason to move it. That is
why `templates/dependabot.yml` tells Dependabot to ignore this repository: it
would otherwise propose the newest commit of `main` to each caller on its own.

| Input | Meaning |
|---|---|
| `tag` | The release tag. Required. |
| `assets` | Space-separated asset names. Required. Plain file names only. |
| `artifact-pattern` | Glob matching the build artifacts. Required. Use a prefix such as `build-*`, so it never matches the workflow's own `signpath-*` artifacts. |
| `windows-assets` | The assets to Authenticode-sign. Empty means every asset ending in `.exe`. |
| `signpath-project-slug` | The repository's SignPath project. Empty means SignPath is off. |
| `signpath-signing-policy-slug` | Default `release-signing`. |
| `signpath-artifact-configuration-slug` | Empty uses the project's default artifact configuration. |
| `code-signing-policy` | Default `true`: appends a "Code signing policy" link to the calling repository's `README#code-signing-policy`. Set `false` in a repository whose README has no such section. |

The organisation variable `SIGNPATH_ORGANIZATION_ID` is read directly; the two
secrets are passed by name, never with `secrets: inherit`.

**The order, and why it cannot change.** Authenticode embeds its signature in
the PE itself: a certificate table is appended and the header's security
directory and checksum are rewritten. The `.exe` that comes back from SignPath
is different bytes from the one the compiler wrote. The ed25519 `.sig` and the
sha256 in `.meta` must describe the bytes users download, so the only correct
chain is:

```
build -> Authenticode (SignPath) -> ed25519 .sig/.meta/.meta.sig -> upload -> verify -> publish
```

If the ed25519 step runs first, or an unsigned `.exe` reaches the release and is
later replaced by the signed one, the published signatures describe bytes that
no longer exist and Colony refuses the file. That is why the builds upload only
artifacts, why nothing is uploaded to the release until every byte is final,
and why the release stays a draft for the whole round trip.

Nothing from the calling repository runs in the job that holds the key: it
checks nothing out. The `.meta` format is written in that one workflow and
verified by Colony (`src/signing.rs`). Change it in both, or not at all.

**Turning SignPath on for a repository**, once SignPath Foundation has accepted
the project:

1. Once for the organisation: the predefined *GitHub.com* trusted build system
   in the SignPath organisation, a CI user with submitter rights whose API
   token is the `SIGNPATH_API_TOKEN` organisation secret, and the
   `SIGNPATH_ORGANIZATION_ID` organisation variable. Grant the secret to the
   repository.
2. In SignPath, a project for the repository: its repository URL, the GitHub.com
   build system linked, a `release-signing` policy with the approvers and
   origin verification allowing `main` and the release tags `v*` (release
   runs start from a push to `main`; a recovery dispatch starts from the tag,
   see below), and an artifact
   configuration for a zip holding one PE file, with file metadata
   restrictions: product name equal to the project name, product version set.
   The `.exe` therefore needs a version resource (ProductName,
   ProductVersion).
3. In the repository: every job leading to the signing request on
   GitHub-hosted runners, no build cache in the release build (SignPath forbids
   reusing outputs of earlier, unverified builds), a "Code signing policy"
   section in the README with SignPath's wording and team roles added (the
   commented block in `templates/program/README.md`), and
   `signpath-project-slug` set in the call.

From that release on, a Windows file without Authenticode cannot be published:
a missing token or organisation id fails `preflight`, and a request that is
denied, fails or times out fails the run with the release still a draft.

**Recovery dispatches start from the tag.** SignPath does not take the commit
from the build script: it records the run's own commit (`GITHUB_SHA`) as the
source of the file it signs. The build legs check out the tag, so the two must
be the same commit, or the approver sees, and SignPath vouches for, source the
binary was not built from. With SignPath on, `preflight` refuses a run whose
commit is not the tag's. A push run passes, since release-please tags the
merge commit it runs on. A recovery dispatch must therefore be started from the
tag, not from `main`:

```bash
gh workflow run release.yml -R Project-Colony/<repo> --ref vX.Y.Z -f tag=vX.Y.Z
```

**The manual approval.** Every release-signing request waits for an approver
to approve it in SignPath, which notifies by email. The workflow waits up to 5
hours (GitHub-hosted jobs stop at 6), so merging a release pull request means
someone has to be there to approve. If nobody does, the job fails with a link to
the request and nothing is published; re-running the failed jobs submits a new
request, which needs a new approval.

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

N **must** be signed with the outgoing key: its whole job is to widen the
trusted set on machines that only trust `old`. Do not skip to N+2: anyone still
on N-1 when `old` is dropped can no longer self-update and must reinstall by
hand.

The release workflow template also trusts the key: its `Validate colony.json`
step verifies the Colony it downloads against a copy embedded as a PEM, and
runs nothing that fails. Update that PEM in the template and in every program's
copy of it when Colony's releases start being signed with `new` (N+1), or their
releases stop at the manifest check. The public key in the
`## Code signing policy` section of `templates/program/README.md` and of every
program's README changes at the same time.

### Validating before you ship

```bash
colony validate-manifest colony.json
```

Pass the asset names the release publishes to also check that every platform
actually **resolves**. That is the failure that matters, because a manifest can
be structurally perfect and still leave the program listed with no Download
button:

```bash
gh release view v1.2.3 --json assets --jq '.assets[].name' > names.txt
colony validate-manifest colony.json $(tr '\n' ' ' < names.txt)
```

It exits non-zero on any problem. The release workflow template runs it on
every release, in the Linux build leg, with the latest Colony release, and only
once that binary's signature and signed `.meta` check out.

### Distribution artefacts

Arch Colony ships what pacman and a bootloader consume rather than files the
launcher installs, so its artefacts are signed the way those tools verify them:

- **Package repositories.** Every package and every repository database of a
  Colony pacman repository is signed with the distribution's OpenPGP key, the
  one its keyring package installs. The repository's entry in `pacman.conf`
  says `SigLevel = Required DatabaseRequired`, so pacman refuses an unsigned or
  wrongly signed package or database instead of installing it.
- **ISO images.** An image carries GPL software in binary form, so its sources
  travel with it: the complete corresponding source next to the image, from the
  same place and at no extra charge, or a written offer, valid for at least
  three years and to any third party, to provide it. Each image also has a
  detached OpenPGP signature and a sha256 file.

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
oldest toolchain you actually support, and mean it: it is a promise, and CI
should be the thing that keeps you honest about it.

## 7. Checklist for a new program

1. `colony.json` at the repo root: name and category, plus an icon if you have
   one. Validate it: `colony validate-manifest colony.json`.
2. `release-please-config.json` and `.release-please-manifest.json` from
   `templates/`.
3. `.github/workflows/release.yml` from `templates/sign-and-publish-caller.yml`,
   with `{{APP_NAME}}` replaced by the binary name. Keep every pin, the shared
   workflow's included.
4. Signing: nothing to copy. The shared workflow signs every release with
   `COLONY_SIGNING_KEY_PEM`, which the repository must be allowed to read (see
   §8, step 3). The README gets `## Code signing policy` and `## Privacy` from
   `templates/program/README.md`. Set `"signed": true` in `colony.json` **only
   after** the first release that carries signatures.
5. `CHANGELOG.md`: release-please creates it on the first release; you do not
   write it.
6. GPL-3.0-or-later `LICENSE`, matching the rest of the organisation.
7. Conventional commits from the first commit onward, each change merged as a
   squashed pull request.

## 8. Checklist for a program that already ships

For the repositories that predate the current template. Safe in any order, one
repository at a time: nothing here requires coordinating a flag day.

1. Replace `.github/workflows/release.yml` with
   `templates/sign-and-publish-caller.yml`, rather than patching the existing
   workflow, and add the `## Code signing policy` section of
   `templates/program/README.md` to the README, since every release's notes
   link to it. Workflows from before the template sign inside the build legs,
   next to the compiler, some only on Linux and macOS, and some strip
   companions with an `rm` line that does not know about `.meta` (see §5).
   Replacing avoids all of it.
2. Delete `scripts/sign-release.sh`, and any other signing step outside the
   shared workflow: nothing runs them any more, and a stale copy invites
   someone to wire it back into a build job.
3. Confirm the repository can read the `COLONY_SIGNING_KEY_PEM` organisation
   secret. It is restricted to Project-Colony repositories, so a new or renamed
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
| Release workflow template | `templates/sign-and-publish-caller.yml` (this repo) |
| Signing: ed25519 for every asset, Authenticode for Windows | `.github/workflows/sign-and-publish.yml` (this repo), called at the organisation pin |
| Manifest schema | `generated/colony.schema.json` (this repo) |
| Manifest examples | `manifests/examples/` (this repo) |
| Manifest validator | `colony validate-manifest`, shipped in the launcher |
| Embedded trust keys | `src/signing.rs` in Project-Colony/Colony |
| Private signing key | off-machine; `COLONY_SIGNING_KEY_PEM` in CI, never in a repository |

There is deliberately **one** copy of the signing code. It used to be
duplicated in the Colony repository, the copies drifted, and the result was a
template naming a secret that does not exist and skipping Windows signing
entirely, neither visible from either side. The in-build signing template that
came next kept the key in the same job as the compiler, and was retired for
that reason.
