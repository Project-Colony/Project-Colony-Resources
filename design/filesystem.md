# Filesystem layout

Where a Colony program keeps its files. One answer, on all three platforms.

## The rule

```
<platform root>/Colony/<Program>/
```

The organisation, then the program. `<Program>` is the display name, capitalised
the way the program spells it — `Colony`, `Digger`, `Grape`, `Eidos` — not a
lowercased slug.

There are three roots, and they are not interchangeable:

| | Linux | Windows | macOS |
|---|---|---|---|
| **config** | `~/.config/Colony/<P>/` | `%LOCALAPPDATA%\Colony\<P>\` | `~/Library/Application Support/Colony/<P>/` |
| **data** | `~/.local/share/Colony/<P>/` | `%LOCALAPPDATA%\Colony\<P>\` | `~/Library/Application Support/Colony/<P>/` |
| **cache** | `~/.cache/Colony/<P>/` | `%LOCALAPPDATA%\Colony\<P>\cache\` | `~/Library/Caches/Colony/<P>/` |

**Windows is `AppData\Local`, never `AppData\Roaming`.** `%LOCALAPPDATA%` *is*
`AppData\Local` — the same place under two names, not a third option.

Roaming is copied to and from the domain server at every logon and logoff on
machines with roaming profiles, and counts against a per-user quota. It is meant
for a few kilobytes of settings that make sense on any machine. Installed
binaries, databases and caches are none of those things, and putting them there
makes a user's logon slower every time they sign in.

### Only Linux keeps the three roots apart

| | config vs data | vs cache |
|---|---|---|
| Linux | different | different |
| Windows | **same** | same root, split by `cache\` |
| macOS | **same** | different (`Library/Caches`) |

Two consequences, and both bite on the platform you are least likely to be
developing on:

- **Never separate config from data by relying on the roots differing.** They
  are the same directory on two platforms out of three. Separate them by
  sub-directory — `preferences/`, `apps/` — or a file will collide everywhere
  except where you tested it.
- **Windows has no cache location at all.** `dirs::cache_dir()` returns
  LocalAppData, the same root as config and data, so the layout adds a `cache\`
  component there. Without it, clearing the cache would delete the preferences
  sitting beside it. `paths::cache_dir` does this for you; the invariant is that
  the cache directory never equals nor contains the config or data directory,
  and that is what the crate tests.

## Which root for what

- **config** — what the *user* chose, and what they would want to keep or carry
  to another machine: preferences, credentials, custom scan paths.
- **data** — what the *program* produced and cannot re-derive: installed
  binaries, databases, history.
- **cache** — what the program can rebuild by asking again. Deleting the whole
  cache directory must cost nothing but time.

When in doubt: if losing it would annoy the user, it is not cache.

## The tree

Colony as the worked example:

```
~/.config/Colony/Colony/          config
├── preferences/
│   ├── preferences.json
│   └── favorites.json
└── auth/
    └── github_token.json         only when no OS keychain is available

~/.local/share/Colony/            data
├── Colony/
│   ├── repo-docs/<repo>/
│   ├── repo-icons/<repo>/icon.png
│   └── update-staging/
└── apps/                         ← shared, see below
    └── <repo>/
        ├── <binary>
        ├── .colony_version
        └── .colony_asset

~/.cache/Colony/Colony/           cache
├── repos_cache.json
└── scan_cache.json
```

### `apps/` is deliberately a sibling

Installed programs live in `<data>/Colony/apps/<repo>/`, hanging off `Colony/`
rather than off `Colony/Colony/`. They belong to the ecosystem, not to
Colony-the-launcher — a user removing the launcher should not discover their
programs were kept inside it.

`apps/` is therefore reserved: no program may be named `apps`.

## Use the helper, do not rebuild the path

```rust
use colony_ui::paths;

let prefs = paths::config_dir("Digger")?.join("preferences.json");
let db    = paths::data_dir("Digger")?.join("history.db");
let tmp   = paths::cache_dir("Digger")?.join("metrics.bin");
```

These create the directory. To *show* a path without bringing it into existence
— an About screen, a log line — use `paths::locate::*`, which are pure.

Three reasons this matters more than it looks:

- **`dirs::config_dir()` and `dirs::config_local_dir()` are identical on Linux
  and different on Windows** (Roaming vs Local). A program written and tested on
  Linux cannot tell which one it picked. The helper picks `config_local_dir`,
  once, for everyone.
- **`cache_dir` adds the `cache\` component on Windows and nowhere else.** That
  is a rule nobody will remember to apply by hand, and forgetting it makes
  "clear the cache" destructive.
- The program name is joined into a path that later gets written to and removed.
  The helper rejects `..`, separators and empty names; hand-rolled code
  historically did not.

## Where the ecosystem does not conform

Current state, so a migration is a decision rather than a discovery:

| Program | What | Should be |
|---|---|---|
| **Colony** | uses `dirs::config_dir()` → **Roaming** on Windows | `config_local_dir()` → Local |
| **Colony** | caches in `<config>/Colony/Colony/cache/` | `<cache>/Colony/Colony/` — on Windows this lands in the same place once the root moves to Local, so only Linux and macOS actually move the files |
| **Colony** | `docs/faq.md` and `docs/architecture.md` document `~/.config/colony/preferences.json`; the code writes `~/.config/Colony/Colony/preferences/preferences.json` | fix the docs |
| **Digger** | `history.db` in `<data>/digger/` — lowercase, no `Colony/` level | `<data>/Colony/Digger/` |
| **Grape** | `logs/` and `history.json` inside the config directory | `<data>/Colony/Grape/` |
| **Eidos** | `~/.config/eidos/`, lowercase, no `Colony/` level at all | `<config>/Colony/Eidos/` |

Only Digger and Grape get the Windows root right today. Colony, the reference
implementation, does not.

### Migrating a program

These paths are live on users' machines, so moving one means moving their files:

1. On startup, compute the new path. If it exists, done.
2. If the **old** path exists and the new one does not, move it, then write a
   marker so the check is skipped next time.
3. Leave the old directory alone if the move fails — a failed migration must
   degrade to the old location, never to an empty profile.
4. Never delete the old path in the same release that adds the move. If the
   migration is wrong, the user's data has to still be there.

## Note on `~/.config/colony/` (lowercase)

Colony also reads `~/.config/colony/<file>` — lowercase, single level — in
`config.rs`, for optional user overrides of the config files it ships, like
`categories.json`. That is a different mechanism from user state, and the two
being one letter apart is a trap. A new program that does not ship overridable
config files should not create this directory at all.
