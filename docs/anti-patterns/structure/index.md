# Structure Anti-Patterns

Layout that hides what a file is, what runtime it needs, or how it is meant to
be called.

## Entries

### Flat, snake-named Rust domains and phase-scattered test fixtures (src/, tests/)

**Pattern:** `src/` kept ~150 flat snake-named files where the filename carried
all semantic weight (`command_truth.rs`, `command_truth_props.rs`,
`args_red_green.rs`), tests, props, and logic were indistinguishable siblings,
and loose root modules (`platform.rs`, `distribution.rs`) had no domain home.
`tests/` mirrored it: 31 of 59 integration files carried `phase02_*`/
`phase03_*` prefixes and fixture/support code lived in four phase-bucketed
directories.

**Why it is wrong:** Nothing tells you which domain a file belongs to or how it
is meant to be consumed; `cargo test` is one 60-file blob with no per-domain
signal; the phase vocabulary orphans files and leaking into CI names. A loose
root module hides its owner, so imports cannot communicate provenance.

**Fix:** Bucket every Rust domain the same way scripts are bucketed:
`src/<domain>/{index.rs, structs/, logic/, constants/, tests/}` and
`tests/<domain>/{index.rs, structs/, logic/, constants/, tests/}`. `index.rs`
is the domain's public face (what the domain is, its methods); `structs/` holds
data shapes, `logic/` the behavior, `constants/` fixed values, `tests/` the
in-domain unit/property/mutation trees. Callers import through the domain path
(`crate::context::platform::*`), never loose root modules. Each `tests/<domain>`
registers its own `[[test]]` binary so failures name the domain. Property
generators live once in `src/contracts/tests/` and are imported by consumers,
never duplicated. `evaluation/` is the sanctioned exception: it is the `model`
role for the evaluation domain -- shipped kit payload whose file names are the
kit contract -- parallel to the `harnesses/` and `gates/` data plane; leave its
layout alone and say so before "fixing" it.

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
