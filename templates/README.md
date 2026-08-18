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
the point. If you are not signing, delete the step and the three `.sig` / `.meta`
/ `.meta.sig` lines from the upload.

The step is skipped on Windows runners: `sign-release.sh` needs a POSIX shell and
`openssl`. Signing Windows assets means either adding a bash step on the Windows
runner or signing centrally in a follow-up job — decide it deliberately rather
than discovering the gap after a release.

**`sign-release.sh`** is copied verbatim from Colony, which is where it is
maintained today. It needs only `openssl`. It is reproduced here so a new program
does not have to go read the launcher's source to find it.

**Pin the actions.** `release.yml` ships with version tags (`@v4`) because a
commit SHA has to be resolved against the real action repository at the moment
you adopt it. A release workflow holds `contents: write` and can sign and
publish, so pin its actions by SHA once you have copied it — Dependabot bumps
the SHA and the trailing version comment together. See
[design/dependencies.md](../design/dependencies.md).
