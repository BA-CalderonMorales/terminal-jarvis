# AGENTS.md - Terminal Jarvis

## Current Shape

- `src/` is the slim std-only Rust CLI for the harness catalog model, bucketed
  per domain as `src/<domain>/{index.rs, structs/, logic/, constants/, tests/}`
  with an `index.rs` public face per domain.
- `harnesses/` is the data plane: `harnesses/<agent>/<capability>/index.toml`.
- `gates/` holds optional local security gate data (Trivy by default).
- `build/build.rs` is the build script; no root-level Rust scripts.
- `docs/` holds architecture, ADRs, testing, migration, and release notes;
  `docs/anti-patterns/{type}/index.md` is the ledger of recurring agent
  mistakes and their fixes.
- `scripts/` holds local automation bucketed as
  `scripts/{language}/{domain}/{role}/`, with a public `index.{ext}` per
  domain; callers never reach into `logic/` directly.
- The pre-rewrite implementation is pruned; use Git history for legacy reference.

## Key Sections

| To understand... | Read |
|---|---|
| Code intelligence (GitNexus) | AGENTS.md GitNexus block / `.agents/skills/gitnexus/*/SKILL.md` |
| Architecture decisions & merge intent | `docs/architecture-decision-records/README.md` |
| The harness capability contract | `docs/harness-capability-contract.md` |
| Verification, artifacts, release flow | `docs/development.md` |
| Optional Trivy gate behavior | `docs/security-gates.md` |
| Catalog support truth | `docs/supported-agents.md`, `docs/support-matrix.md` |
| Recurring agent mistakes to avoid | `docs/anti-patterns/index.md` (naming/ structure/ tooling/) |
| Patterns & trade-off reference | <https://rust-unofficial.github.io/patterns/> |
| Everything else | `README.md`, then this file again |

Lost in the woods? Start with `docs/architecture-decision-records/README.md`
for *why* changes exist, then `docs/development.md` for *how* the tool is
built and verified.

## Branch Strategy

- **`develop`**: default base for PRs. Faster experimentation; merges here
  require an ADR (below).
- **`main`**: tagged releases only. `develop` fast-forwards into `main` at
  release time.
- **Feature branches**: branch from `develop`, PR against `develop`.
- **`release/X.Y.Z`**: release-prep branches. Work here accumulates, then merges
  into `develop` with an ADR and into `main` only at tag time.

### Merge-to-develop discipline

- Every merge into `develop` must be accompanied by a record in
  `docs/architecture-decision-records/README.md` capturing the decision and why
  the merge mattered -- intent for humans and agents landing on `main`. It is
  not a changelog.
- **Never merge** `GOAL.md`, `plan/`, `scratch/`, or local working ledgers into
  `develop` or `main`. They are developer-local planning spaces.
- **No root-level Rust scripts** (e.g. a root `build.rs`). Build scripts live
  under `build/`; Cargo.toml points at them with the `build` key.

## CI

- Runs on every PR against `develop` or `main`.
- `plan/`, `scratch/`, and `GOAL.md` are developer-local: gitignored,
  refused by the pre-commit gate on `develop`/`main`, and never merge into
  `develop` or `main`. Validate a local `plan/` ledger with
  `ruby scripts/ruby/plan/index.rb plan`.
- **Docs-only PRs** (changes limited to `docs/`, `README.md`, `AGENTS.md`,
  `CLAUDE.md`) skip CI automatically via `paths-ignore`. Trigger manually with
  `workflow_dispatch` when needed.
- The harness capability contract lives in
  [docs/harness-capability-contract.md](docs/harness-capability-contract.md).
  Keep it in sync when adding capabilities or commands.

## Rules

- Keep Rust source files at 100 lines or fewer.
- Bucket Rust domains as `src/<domain>/` and integration tests as
  `tests/<domain>/`, each with `index.rs` as the domain's public face, plus
  `structs/` (data shapes), `logic/` (behavior), `constants/` (fixed values),
  and `tests/` (in-domain unit/property/mutation trees). Callers import through
  the domain path (`crate::context::platform::*`); no loose modules at the
  `src/` root. Each `tests/<domain>/` registers its own `[[test]]` binary.
- Keep module contracts in `src/contracts/`; quickcheck `Arbitrary` generators
  live once in `src/contracts/tests/` and are imported by consumers, never
  duplicated per module.
- Prefer data in `harnesses/*/*/index.toml` over Rust branches.
- Do not add a second Go ADK or another runtime beside the Rust CLI.
- Use no external Rust dependencies unless the tradeoff is documented first.
  **Approved exception: `quickcheck` (property-based testing) is a
  `[dev-dependencies]`-only crate. It never ships in the release binary -- the
  std-only runtime is unchanged. Tradeoff: deterministic, quantifiable
  red/green coverage for the headless CLI's pure logic beats a zero-dependency
  test-only rule. Do not promote quickcheck to a production dependency.**
- No root-level Rust scripts. Build scripts live in `build/`.
- `GOAL.md`, `plan/`, and `scratch/` are developer-local. They never merge into
  `develop` or `main`, and a merge to `develop` always carries an ADR.
- Keep docs concise and tied to migration, architecture, testing, or release
  notes.
- Use `rg` (ripgrep) for content search when available; fall back to plain
  `grep` only when `rg` is not installed.
- Use `fzf` for interactive fuzzy selection (files, lines, history) when
  available; do not hand-roll a picker (`rg <pattern> | fzf`).
- Name committed files by durable purpose, never by ephemeral phase counters
  (`phase03_*`, `phase-01-*`) -- files, fixtures, CI job names, and artifact
  names alike.
- Bucket scripts as `scripts/{language}/{domain}/{role}/` -- role is `logic`,
  `constants`, `model`, or `index`; callers invoke only the domain
  `index.{ext}`, never files inside `logic/` directly. No loose files at the
  `scripts/` root.
- When you observe a new recurring mistake in committed work, record it under
  `docs/anti-patterns/{type}/index.md` before carrying it forward.
- When a design trade-off is needed to reconcile the domain-bucket rules with
  the vast scenarios this repo holds (spanning Rust, scripts, harness data,
  and release flows), consult the Rust patterns reference
  (<https://rust-unofficial.github.io/patterns/>) for the idiomatic scheme to
  lean on; record the chosen trade-off in `docs/architecture-decision-records/`
  so the reasoning survives.

## Design Principles

North star from the Rust patterns book
(<https://rust-unofficial.github.io/patterns/additional_resources/design-principles.html>),
mapped onto this repo's existing shape:

- SRP -- one responsibility per domain; one index face per domain.
- OCP -- extend by adding a domain or capability, not by widening an existing
  face; data lands in `harnesses/*/*/index.toml`.
- LSP/ISP -- a domain's `structs/` contract is what consumers rely on; keep
  interfaces narrow and substitutable (clean-api's swappable client idea).
- DIP -- depend on the domain's index abstraction, never on `logic/` internals.
- Composition over inheritance -- structure by composition: role buckets
  (`structs/`, `logic/`, `constants/`, `tests/`) inside a domain folder.
- DRY -- one authoritative home per piece of knowledge: one generator store
  (`src/contracts/tests/`), one data plane, one index per domain.
- KISS -- std-only runtime, 100-line files, delete before adding; boring beats
  novel.
- Law of Demeter -- domains know the index of other domains, not their
  internals.
- Design by contract -- a domain's face is its contract; preconditions and
  invariants are witnessed by its `tests/`.
- Encapsulation -- methods sit behind the face; `logic/` is implementation.
- CQS -- separate pure queries from state-changing commands; the CLI's
  read-only vs state-changing/dangerous effect split is this principle.
- POLA -- behavior must not astonish: durable names, exact usage errors, no
  surprising side effects from plain commands.
- Single-Choice -- exhaustive alternatives live in one module (e.g. effect
  classification, platform normalization in `context::platform`).
- Self-Documentation -- durable-purpose names and the anti-patterns ledger are
  part of every module's information.
- Do not reintroduce a `current/` snapshot.
- Do not tag, publish, or upload release assets from local scripts without an
  explicit operator decision.
- Prefer remote or disposable development environments when exercising harness
  install, update, headless, or yolo commands. Keep secrets scoped and do not
  run unreviewed agent commands on a daily-driver machine.

<!-- gitnexus:start -->
<!-- gitnexus:keep -->

This project is indexed by GitNexus as **terminal-jarvis**. Use the MCP tools
for code understanding and impact analysis; the full workflow guidance lives in
`.agents/skills/gitnexus/*/SKILL.md` (linked from the Key Sections table).

Before editing any symbol run `impact({target: "...", direction: "upstream"})`
and report the blast radius; run `detect_changes()` before every commit; never
ignore HIGH/CRITICAL risk.

<!-- gitnexus:end -->
