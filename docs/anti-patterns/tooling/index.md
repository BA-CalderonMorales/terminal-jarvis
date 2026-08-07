# Tooling Anti-Patterns

Reaching for a slower or hand-rolled tool when a standard, installed one
exists.

## Entries

### Plain grep when ripgrep is available (docs/)

**Pattern:** Content searches were issued with plain `grep -r` even where
`rg` was installed.

**Why it is wrong:** ripgrep honors `.gitignore`, skips binaries, and is much
faster on large trees; `grep -r` drags in vendored and ignored directories and
returns stale noise. The habit degrades search quality on every repo.

**Fix:** Use `rg` (ripgrep) for every content search. Fall back to plain
`grep` only when `rg` is not installed.

### Forgetting fzf for interactive selection (docs/)

**Pattern:** Interactive or terminal flows reach for hand-rolled pickers and
ad-hoc filters when a fuzzy finder is available on PATH.

**Why it is wrong:** A custom filter duplicates an installed, tuned tool and
produces a worse interaction for the reader.

**Fix:** Leverage `fzf` (fuzzy finder) for interactive selection of files,
lines, git objects, and history when it is present -- commonly piped from `rg`
(e.g. `rg <pattern> | fzf`). Write a custom picker only when `fzf` is absent.
