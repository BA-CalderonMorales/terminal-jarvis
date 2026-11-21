# Feature flags

Unfinished work merges behind a flag instead of waiting on a branch. Flags are
data: one file per flag, embedded in the binary at build time.

## Add a flag

Create `flags/<name>/index.toml` (the name is kebab-case and equals the
directory):

```toml
name = "scanner-choice"
default = false
stage = "labs"
description = "Pick which security scanner the gate runs."
since = "0.1.26"
```

| Field | Rule |
|---|---|
| `stage` | `stable` (shipped, can be turned off), `preview` (shipping soon), or `labs` (experimental) |
| `default` | `labs` flags must default to `false` |
| `description` | one plain sentence, 1-100 characters |
| `since` | the version that introduced it, `X.Y.Z` |

Then gate the code with `crate::flags::enabled("scanner-choice")`. Use a
string literal: the structural test in `tests/flags/` scans `src/` for
`flags::enabled("<literal>")` and fails when a literal is undeclared or a
declared flag is never used.

## Toggle a flag

Highest wins: environment, then the settings file, then the default.

- `TJ_FLAG_<NAME>=1|0|on|off|true|false` (name upper-cased, `-` becomes `_`);
  anything else is ignored.
- `terminal-jarvis settings` lists flags (stable and preview first, then Labs)
  and where each value came from; `settings set <flag> on|off` and
  `settings reset <flag>` write `settings.toml` in the Terminal Jarvis home.
  `--dry-run` previews without writing. The same list is the `settings` verb
  in the tui.

An unknown flag is a usage error (exit 2) that names the valid flags. A
malformed `settings.toml` warns once on stderr and falls back to defaults;
unknown names in it are ignored. `TERMINAL_JARVIS_FLAGS` points at a flags
directory instead of the embedded registry (used by tests).

## Promote a flag

Delete the flag in the same change that makes the behavior permanent: remove
`flags/<name>/`, remove the `flags::enabled` call, keep the winning branch. The
declared/used test keeps dead flags from lingering.
