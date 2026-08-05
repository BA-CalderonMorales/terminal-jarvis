# Structure Anti-Patterns

Layout that hides what a file is, what runtime it needs, or how it is meant to
be called.

## Entries

### Mixed script runtimes in one scripts/ root (scripts/)

**Pattern:** Shell, Python, and Ruby scripts all sat loose at the top of
`scripts/` (`verify.sh`, `harness-risk.py`, `check-plan.rb`) with a TOML
config beside them (`release.toml`).

**Why it is wrong:** A flat directory hides the runtime of every file at a
glance, invites one formatter/linter per language into a single planner, and
forces tooling (CI paths, manifest file lists) to name each file individually
instead of globbing a folder.

**Fix:** Bucket scripts by runtime, domain, and role, in that order:
`scripts/{language}/{domain}/{role}/file.ext`. Role is `logic` (executable
behavior), `constants` (fixed values and config), `model` (durable data), or
`index` (the domain's public entrypoint). Every domain exposes an
`index.{ext}` that callers invoke -- CI, docs, plan, tests, and other scripts
always go through the index, never straight into `logic/`. Internals sit at
`scripts/{language}/{domain}/logic/`, config at
`scripts/config/{domain}/constants/`. For example
`scripts/bash/release/index.sh package build dist` dispatches to
`scripts/bash/release/logic/package-release.sh`, and
`scripts/config/release/constants/release.toml` holds release defaults.
