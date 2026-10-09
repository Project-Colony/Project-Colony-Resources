# Templates

Starting points for a new Project Colony program. Copy, replace the
placeholders, commit. The reasoning behind them is in
[design/releases.md](../design/releases.md).

| File | Copy to | Then |
|---|---|---|
| `program/README.md` | repo root | fill the placeholders, see [design/documentation.md](../design/documentation.md) |
| `program/docs/README.md` | `docs/README.md` | delete the rows you do not have |
| `sign-and-publish-caller.yml` | `.github/workflows/release.yml` | replace `{{APP_NAME}}` with the binary name, lowercase |
| `release-please-config.json` | repo root | usually nothing |
| `.release-please-manifest.json` | repo root | set the starting version |
| `dependabot.yml` | `.github/dependabot.yml` | nothing, then merge the PRs it opens |

A `colony.json` to copy is in [`manifests/examples/`](../manifests/examples/).
Start from `minimal.json` unless your release assets cannot follow the naming
convention.

The directory tree these files land in (`crates/<prefix>-<role>/`, `assets/`,
`packaging/`, `scripts/`) is in
[design/repository-layout.md](../design/repository-layout.md). Lay the repository
out first, then copy these in.

## Notes

**`release-type`.** The template uses `"rust"`, which lets release-please bump
`Cargo.toml` and the lockfile itself: the right default for a single-crate
program. For a workspace, or when the version has to appear somewhere that is
not Cargo metadata, switch to `"simple"` and add the files to rewrite:

```json
"release-type": "simple",
"extra-files": ["Cargo.toml"]
```

Set it in `release-please-config.json` only. Never also pass `release-type` to
`googleapis/release-please-action`: with that input set, the action ignores
`config-file` and `manifest-file` entirely, so the changelog sections,
`extra-files` and the manifest version are all silently dropped. The workflow
template passes `config-file` and `manifest-file` and nothing else.

**`sign-and-publish-caller.yml`** is the one release workflow template. Its
build legs check out the tag, build, validate `colony.json`, smoke-test each
binary and upload it as an artifact; they never receive a signing key. The
last job calls the shared reusable workflow
[`.github/workflows/sign-and-publish.yml`](../.github/workflows/sign-and-publish.yml),
which Authenticode-signs the Windows files through SignPath (once a repository
turns it on), writes `.sig`, `.meta` and `.meta.sig` over the final bytes,
verifies everything on the release and publishes it. The key lives only in that
workflow's own job, which checks out nothing and builds nothing: a build runs
every dependency's `build.rs` and proc-macros, and none of that code may share
a job with the key. How to call it and how SignPath is switched on:
[design/releases.md](../design/releases.md#shared-signing-workflow).

The call is pinned to the organisation's commit of the shared workflow. Keep
that pin when you copy the template; it moves for every program at once, as
described in design/releases.md.

**`program/README.md`** carries a `## Code signing policy` section. Keep it:
the shared workflow links every release's notes to
`README.md#code-signing-policy`. A repository that removes the section passes
`code-signing-policy: false` in the call instead.

**Pinned actions.** The workflow template pins every action to a commit SHA
with the version in a trailing comment. A release workflow holds
`contents: write` and can publish, so keep the pins when you copy it; the
`github-actions` entry in `dependabot.yml` bumps the SHA and the comment
together. See [design/dependencies.md](../design/dependencies.md).
