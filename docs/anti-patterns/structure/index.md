# Structure Anti-Patterns

Layout that hides what a file is, what runtime it needs, or how it is meant to
be called.

## Entries

### Two styling systems in one surface (src/tui/, src/cli/logic/style.rs)

**Pattern:** The viewport chrome painted through `screen::theme` (theme-driven,
always emits SGR) while every body screen -- `status`, `help`, the primer,
verdict cards -- painted through `cli::style` (hardcoded `1;36`/`1;37`, gated on
`is_terminal()`). The frame had one styling system for its edges and another for
its contents.

**Why it is wrong:** The theme was half-real: `/theme ember` repainted the
header and left every body screen cyan, so the six themes were three colours
with names. Worse, `paint::body_block` wrapped each body line in `theme::dim`,
and an inner `\x1b[0m` cancelled the outer dim mid-line, so a styled row read as
half-dimmed. The flatness was not a taste choice; it was two systems disagreeing.

**Fix:** One role vocabulary (`screen::roles::Role`) that every renderer names,
resolved by the active theme (`theme::paint`). Chrome and body both call it, so
a theme swap is total. Keep `cli::style` for the headless surface, which has its
own colour gate; converting a shared renderer (`show`, `plan`) needs the gate
unified first, not a second hardcoded palette. The theme-completeness and
`paint_wraps_once_and_never_nests` tests hold the line.

### Dual module entry: mod.rs and index.rs in every domain (src/<domain>/)

**Pattern:** Every Rust domain folder carries both `mod.rs` and `index.rs`.
The parent declares `pub mod <domain>;`, so `mod.rs` is the real entry; it
lines up `mod index; mod logic; mod structs;` and finishes with
`pub use index::*;`, re-exporting a second file that is itself only re-exports.
Two files with near-identical names hold the domain's public face, and the
chain entry -> index.rs adds a hop for no content.

**Why it is wrong:** One face per domain is the rule; two entry-looking files
make agents and maintainers guess which is real, and the `index.rs` name
collides with the index.rs the repo uses as the canonical face elsewhere.
The `mod.rs` shim exists only to satisfy the parent's plain
`pub mod <domain>;`; it must not outlive that convenience.

**Fix (landed in 0.1.14):** Keep `index.rs` as the single face, delete
`mod.rs`. `src/lib.rs` declares each domain with
`#[path = "<domain>/index.rs"] pub mod <domain>;` (the established tui
pattern), and each `index.rs` declares its own `mod logic; mod structs;`.
Re-export paths and the mutation shard `--file` lists do not change; only
the redundant `mod.rs` files disappear. Do this as its own cleanup commit;
do not fold unrelated changes into it.


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
