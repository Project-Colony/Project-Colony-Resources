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

Release assets can carry a detached ed25519 signature. Colony's own launcher
self-update **requires** it and is fail-closed; an ordinary program opts in with
`"signed": true`, which turns a missing or invalid signature into an aborted
install rather than a fallback to the unsigned path.

Three files accompany each signed asset:

| File | Contents |
|---|---|
| `<asset>.sig` | raw 64-byte ed25519 signature over the asset bytes |
| `<asset>.meta` | `version=<tag>`, `asset=<basename>`, `sha256=<hex>` |
| `<asset>.meta.sig` | signature over the `.meta` bytes |

The sidecar exists because a signature over raw bytes proves only that the bytes
came from the org — not *which* artefact or *which* version they are. Without it,
someone able to control what the release host serves could replay an older,
genuinely signed build. The sidecar binds the bytes to a version and a filename,
and the launcher refuses anything not strictly newer than itself.

`templates/sign-release.sh` produces all three. It needs nothing but `openssl`:

```bash
COLONY_SIGNING_KEY=/path/to/colony-release.pem COLONY_RELEASE_VERSION=v1.2.3 ./sign-release.sh <asset>...
```

In CI, write the private key from a repository secret to a temporary file and
point `COLONY_SIGNING_KEY` at it. The private key never lives in a repository.
Upload every `.sig`, `.meta` and `.meta.sig` alongside its binary.

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
   one. Validate it against `generated/colony.schema.json`.
2. `release-please-config.json` and `.release-please-manifest.json` from
   `templates/`.
3. `.github/workflows/release.yml` from `templates/`, with `{{APP_NAME}}`
   replaced by the binary name.
4. `CHANGELOG.md` — release-please creates it on the first release; you do not
   write it.
5. GPL-3.0-or-later `LICENSE`, matching the rest of the organisation.
6. Conventional commits from the first commit onward.
