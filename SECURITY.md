# Security Policy

Every Project Colony program releases through this repository's signing
workflow, copies its release templates and builds on `colony-ui`, so a flaw
here reaches every program at once.

## Reporting a vulnerability

Please do not report a vulnerability through a public issue, pull request or
discussion. Report it privately instead: open this repository's **Security**
tab and click **Report a vulnerability**, or go straight to
[the form](https://github.com/Project-Colony/Project-Colony-Resources/security/advisories/new).
Only the maintainers can see the report. Please include:

- the affected file, crate version or commit,
- what an attacker can do, and the steps to reproduce it,
- a proof of concept, if you have one.

We will acknowledge the report, keep you informed while we work on it, and
credit you in the advisory once a fix is released, unless you prefer to stay
anonymous.

## Scope

Reports of particular interest:

- **The shared signing workflow**,
  [`.github/workflows/sign-and-publish.yml`](.github/workflows/sign-and-publish.yml).
  It runs inside each program's release, holds the organisation's ed25519
  release key for one job that builds nothing, and signs every asset and its
  `.meta` before the release is published. Anything that exposes the key, gets
  a file signed that the workflow did not verify, publishes a release whose
  signatures do not verify, or skips Authenticode signing through SignPath
  while it is switched on is critical.
- **The release templates** in [`templates/`](templates/), above all
  `sign-and-publish-caller.yml`. Every program copies them, so a weakness in
  them (an injectable input, a credential left in a checkout, a step that runs
  a binary it has not verified) is copied too.
- **`colony_ui::paths`**, which joins a program name into directories that
  programs then write to and delete. A name that gets out of its
  `Colony/<Program>/` directory, or onto the shared `Colony/apps/` install
  root, is a vulnerability.
- **The `colony.json` schema**,
  [`generated/colony.schema.json`](generated/colony.schema.json). Manifests
  come from third-party repositories and are untrusted input; a manifest the
  schema accepts but no program should trust belongs here. A flaw in how the
  launcher handles a manifest belongs on
  [Colony](https://github.com/Project-Colony/Colony).

Out of scope: vulnerabilities in a program built on these resources (report
them on that program's repository), and in the third-party actions and crates
used here (report them upstream, and tell us if we should move off a version).

## Supported versions

- **`colony-ui`**: the latest version on crates.io. Older versions are not
  patched; update to the latest.
- **The signing workflow**: `main`, and the commit every program pins (the
  organisation pin in
  [`templates/sign-and-publish-caller.yml`](templates/sign-and-publish-caller.yml)).
  A fix lands on `main`, then the organisation pin moves to it for every
  program at once.
