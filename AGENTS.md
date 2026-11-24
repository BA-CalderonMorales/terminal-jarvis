# AGENTS.md - Terminal Jarvis

## Current Shape

- `src/` is the slim std-only Rust CLI for the harness catalog model, bucketed
  per domain as `src/<domain>/{index.rs, structs/, logic/, constants/, tests/}`
  with an `index.rs` public face per domain.
- `harnesses/` is the data plane: `harnesses/<agent>/<capability>/index.toml`.
- `gates/` holds optional local security gate data (Trivy by default).
- `build/build.rs` is the build script; no root-level Rust scripts.
- `docs/` holds architecture, ADRs, testing, and legacy notes; release notes
  live in `CHANGELOG.md` at the root. `docs/anti-patterns/{type}/index.md` is
  the ledger of recurring agent mistakes and their fixes.
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
| Recurring agent mistakes to avoid | `docs/anti-patterns/index.md` (data/ execution/ naming/ structure/ tooling/) |
| Patterns & trade-off reference | <https://rust-unofficial.github.io/patterns/> |
| Everything else | `README.md`, then this file again |

Lost in the woods? Start with `docs/architecture-decision-records/README.md`
for *why* changes exist, then `docs/development.md` for *how* the tool is
built and verified.

## Branch Strategy

Trunk-based. `main` is the only long-lived branch and is always releasable.

- **Short-lived branches**: branch from `main`, open a small PR against `main`,
  squash-merge once the required checks pass. Merged branches are deleted.
- **Unfinished work** merges behind a flag instead of waiting on a branch
  (see [docs/flags.md](docs/flags.md)).
  The working tree never holds more than one PR's worth of change.
- **Releases** are a version bump on `main` plus a `vX.Y.Z` tag. There are no
  `develop` or `release/X.Y.Z` branches.

### Merge discipline

- Write a record in `docs/architecture-decision-records/README.md` when a PR
  makes a decision a later reader would otherwise have to re-derive. Routine
  fixes and features do not need one; it is not a changelog.
- **Never merge** `GOAL.md`, `plan/`, `scratch/`, or local working ledgers into
  `main`. They are developer-local planning spaces.
- **No root-level Rust scripts** (e.g. a root `build.rs`). Build scripts live
  under `build/`; Cargo.toml points at them with the `build` key.

## CI

- Runs on every PR against `main`. Verify, Windows Verify, Package Smoke,
  Adversarial Conformance, and Mutation Exclusions are required checks; a merge
  waits on them, nothing else.
- Mutation testing is manual (`workflow_dispatch`) until the suite is green
  again. A check either blocks a merge or is not scheduled.
- `plan/`, `scratch/`, and `GOAL.md` are developer-local: gitignored,
  refused by the pre-commit gate on `main`, and never merge into `main`.
- CI runs on every PR, docs-only included: a required check that is skipped
  never reports, and the PR would wait on it forever.
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
  `main`.
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

## Rule Enforcers

Each rule is paired with what enforces it. `enforced` means a CI or verify
check fails when it breaks; `partial` means a check covers only part of it;
`none` means no repo artifact can fail, and the row says why it stays prose.

| Rule | Enforced by | Check fails when |
|---|---|---|
| `src/<domain>/` buckets with an `index.rs` face | none: no check reads the layout; `src/converse/` and `src/tui/` use `lib.rs` faces | a domain folder lacks `index.rs` (unchecked) |
| `harnesses/<agent>/<capability>/index.toml` is the data plane | partial: `scripts/bash/verify/logic/verify.sh` step 6 counts 10 index files per harness | the index count is not 10 times the harness count |
| `gates/` holds optional local security gate data | none: descriptive; `docs/security-gates.md` documents it | n/a |
| `build/build.rs` is the build script; no root-level Rust scripts | partial: `Cargo.toml` `build` key (cargo fails if `build/build.rs` is missing) | a root `build.rs` appears (unchecked) |
| `docs/anti-patterns/{type}/index.md` ledger; record new recurring mistakes | none: judgment; no artifact can tell a recurring mistake | n/a |
| `scripts/{language}/{domain}/{role}/` buckets; no loose files at `scripts/` root | partial: `verify.sh` step 5 checks length of `.sh` and `.rb` only; drift: `scripts/demo/` holds loose files | a script lands outside the bucket layout (unchecked) |
| The pre-rewrite implementation is pruned; use Git history | none: historical fact | n/a |
| Key Sections and GitNexus links resolve to real paths | none: `verify` runs no Markdown link check; checked by hand for this change | a link target is renamed or deleted |
| Trunk-based; `main` is always releasable | partial: `.github/workflows/ci.yml` Verify, Windows Verify, and Package Smoke run on every PR to `main` | a required job fails on the PR; releasability beyond CI is unchecked |
| Short-lived branches; squash-merge once required checks pass | enforced: GitHub branch protection on `main` requires the five `ci.yml` jobs (checked with `gh api repos/.../branches/main/protection`) | a PR merges with a required check failing or missing |
| Merged branches are deleted | none: GitHub repo setting, not visible in the repo | n/a |
| Unfinished work merges behind a flag (`docs/flags.md`) | partial: `tests/flags/tests/used.rs`, run by `cargo test` (verify step 3) | a flag is used but undeclared, or declared but unused; flag-less unfinished work is unchecked |
| Releases are a version bump plus a `vX.Y.Z` tag | enforced: `scripts/bash/release/logic/release-preflight.sh`, run by `.github/workflows/cd-multiplatform.yml` on tag push and by verify step 11 | Cargo, npm, or lock version differ, CHANGELOG lacks the version, or the tag is not on `origin/main` |
| No `develop` or `release/X.Y.Z` branches | none: remote branch state, not in the repo; `docs/development.md` states it | such a branch exists (unchecked) |
| Write an ADR when a PR makes a decision a later reader must re-derive | none: judgment | n/a |
| Never merge `GOAL.md`, `plan/`, `scratch/` (also in CI and Rules) | partial: `.gitignore` plus `scripts/bash/verify/logic/pre-commit.sh`; the hook runs only where `install-hooks` ran, and CI never runs it | the file is committed with `--no-verify` or without the hook, and CI does not catch it |
| CI runs on every PR against `main` | partial: `.github/workflows/ci.yml` `pull_request` trigger | the trigger is narrowed (no separate check) |
| Five required checks: Verify, Windows Verify, Package Smoke, Adversarial Conformance, Mutation Exclusions | enforced: the five job names in `.github/workflows/ci.yml`, matching branch protection (exact match verified) | a job is renamed or removed; the merge then waits forever or passes unchecked |
| Mutation testing is manual (`workflow_dispatch`) | none: `.github/workflows/mutation.yml` declares only `workflow_dispatch`; no check fails if a schedule is added | a schedule or push trigger is added (unchecked) |
| Docs-only PRs run CI; no `paths` filter | partial: `.github/workflows/ci.yml` has no `paths` filter today | a `paths` filter is added to `ci.yml` (unchecked) |
| Keep the harness capability contract in sync | none: `docs/harness-capability-contract.md` is read by no check | a capability or command changes without the contract update |
| Rust source files at 100 lines or fewer | enforced: `scripts/bash/verify/logic/verify.sh` step 4 (covers `src/` and `tests/`, not `build/`) | a `.rs` file under `src/` or `tests/` exceeds 100 lines |
| Each `tests/<domain>/` registers its own `[[test]]` binary | partial: the eight `[[test]]` entries in `Cargo.toml` match the eight `tests/<domain>/` dirs | a `tests/<domain>/` dir lacks its entry; cargo then runs nothing from it, silently |
| Contracts in `src/contracts/`; `Arbitrary` generators live once in `src/contracts/tests/` | none: no check. Drift: `src/cli/tests/args_props.rs` defines an `Arbitrary` impl outside it | a generator is duplicated per module (unchecked) |
| Prefer data in `harnesses/*/*/index.toml` over Rust branches | none: design judgment | n/a |
| No second Go ADK or runtime beside the Rust CLI | none: no check. Drift for maintainer: `scripts/demo/render_frame.go` and `go.mod` are Go code (a demo helper, not an ADK) | a Go module ships with the CLI (unchecked) |
| No external Rust dependencies unless the tradeoff is documented | partial: `cargo deny check` in `scripts/bash/verify/logic/security-check.sh` covers advisories, licenses, and bans, not a dependency policy | a new `[dependencies]` entry lands (unchecked) |
| `quickcheck` is `[dev-dependencies]`-only | none: `Cargo.toml` lists it there; no check fails if it moves | quickcheck moves to `[dependencies]` (unchecked) |
| Keep docs concise | none: judgment | n/a |
| Use `rg` for content search | none: maintainer tool preference; no repo enforcer | n/a |
| Use `fzf` for interactive selection | none: maintainer tool preference; no repo enforcer | n/a |
| Name committed files by durable purpose, never phase counters | none: no check. Clean today (no tracked phase-named files); `docs/support-matrix.md` carries `internal:phase-01@...` as a value | a committed file or CI job name uses a phase counter (unchecked) |
| Consult the Rust patterns reference; record trade-offs in ADRs | none: judgment | n/a |
| The 14 design principles, SRP through Self-Documentation | none: design guidance; no check maps to a principle | n/a |
| Do not reintroduce a `current/` snapshot | none: no check; a `test ! -d current` would do it | a `current/` directory appears (unchecked) |
| Do not tag, publish, or upload release assets from local scripts | none: `scripts/bash/release/logic/local-cd.sh` states it neither tags nor publishes; no check enforces it | a local script tags or publishes (unchecked) |
| Prefer remote or disposable environments for harness install, update, headless, or yolo | none: operator guidance | n/a |
| Run GitNexus `impact` before editing a symbol | none: GitNexus workflow; no repo artifact | n/a |
| Run GitNexus `detect_changes` before every commit | none: GitNexus workflow; no repo artifact | n/a |
| Never ignore HIGH/CRITICAL GitNexus risk | none: GitNexus workflow; no repo artifact | n/a |

## Proposed changes (needs maintainer decision)

Not applied. Each needs a maintainer decision before AGENTS.md changes.

- Replacing the 100-line Rust file limit with clippy `cognitive_complexity` and `too_many_lines` (deny) plus a dependency-direction check, and keeping a 300-line guard.
- Dropping the tool-preference rules (`rg`, `fzf`).

<!-- gitnexus:start -->
<!-- gitnexus:keep -->

This project is indexed by GitNexus as **terminal-jarvis**. Use the MCP tools
for code understanding and impact analysis; the full workflow guidance lives in
`.agents/skills/gitnexus/*/SKILL.md` (linked from the Key Sections table).

Before editing any symbol run `impact({target: "...", direction: "upstream"})`
and report the blast radius; run `detect_changes()` before every commit; never
ignore HIGH/CRITICAL risk.

<!-- gitnexus:end -->
