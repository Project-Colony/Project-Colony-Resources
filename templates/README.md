# Templates

Starting points for a new Project Colony program. Copy, replace the
placeholders, commit. The reasoning behind them is in
[design/releases.md](../design/releases.md).

| File | Copy to | Then |
|---|---|---|
| `program/README.md` | repo root | fill the placeholders — see [design/documentation.md](../design/documentation.md) |
| `program/docs/README.md` | `docs/README.md` | delete the rows you do not have |
| `release.yml` | `.github/workflows/release.yml` | replace `{{APP_NAME}}` with the binary name, lowercase |
| `release-please-config.json` | repo root | usually nothing |
| `.release-please-manifest.json` | repo root | set the starting version |
| `dependabot.yml` | `.github/dependabot.yml` | nothing — then merge the PRs it opens |
| `sign-release.sh` | `scripts/sign-release.sh` | keep it executable |

A `colony.json` to copy is in [`manifests/examples/`](../manifests/examples/) —
start from `minimal.json` unless your release assets cannot follow the naming
convention.

The directory tree these files land in — `crates/<prefix>-<role>/`, `assets/`,
`packaging/`, `scripts/` — is in
[design/repository-layout.md](../design/repository-layout.md). Lay the repository
out first, then copy these in.

## Notes

**`release-type`.** The template uses `"rust"`, which lets release-please bump
`Cargo.toml` and the lockfile itself — the right default for a single-crate
program. For a workspace, or when the version has to appear somewhere that is
not Cargo metadata, switch to `"simple"` and add the files to rewrite:

```json
"release-type": "simple",
"extra-files": ["Cargo.toml"]
```

**Signing.** The signing step in `release.yml` is written for the opt-in case and
fails loudly if the key is missing, because a program whose `colony.json` says
`"signed": true` but ships no `.sig` cannot be installed at all — fail-closed is
the point. If you are not signing, delete the step, the three `.sig` / `.meta`
/ `.meta.sig` lines from the upload, and set `SIGNED: "false"` in the `publish`
job.

The step runs on every runner, Windows included, with `shell: bash`:
`sign-release.sh` needs only a POSIX shell and `openssl`, and the Windows runner
has both. Skipping Windows would not produce an unsigned-but-working `.exe`; in a
repository that declares `"signed": true` it produces one Colony refuses to
install.

**`sign-release.sh`** is copied verbatim from Colony, which is where it is
maintained today. It needs only `openssl`. It is reproduced here so a new program
does not have to go read the launcher's source to find it.

**Pinned actions.** `release.yml` pins every action to a commit SHA with the
version in a trailing comment. A release workflow holds `contents: write` and
can sign and publish, so keep the pins when you copy it; the `github-actions`
entry in `dependabot.yml` bumps the SHA and the comment together. See
[design/dependencies.md](../design/dependencies.md).
