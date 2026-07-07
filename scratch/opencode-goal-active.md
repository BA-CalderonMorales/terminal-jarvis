# OpenCode Goal

Status: active
Objective: Continue dogfooding terminal-jarvis (npm 0.1.6) to surface blind spots / UX gaps, apply meaningful core-improvement fixes to this branch, and hold it as the candidate for a 0.1.7 release.
Started: 2026-07-07T21:17:45Z
Updated: 2026-07-07T21:22:49Z
Repo: /mnt/c/Users/bacm6/world/repositories/working/terminal-jarvis
Branch: release/0.1.7
Latest Commit: f2f1ef1 (will change after this ledger update)
Remote: origin/release/0.1.7 (based on origin/develop)

## Constraints
- Branch must stay based on `develop`; open PRs against `develop`, not `main`.
- RELEASE GATE: Do NOT open the PR against develop, tag, publish, or upload
  release assets until the operator EXPLICITLY states they are ready. The
  operator will say so in their own words; absence of a readiness statement is
  NOT approval. Until then, each /goal session continues dogfooding and applying
  fixes only, then stops and waits.
- Do NOT tag, publish, or upload release assets without an explicit operator decision (AGENTS.md rule).
- Keep Rust source files <= 100 lines; prefer harness data in `harnesses/*/*/index.toml` over Rust branches.
- No new external Rust dependencies unless documented first.
- Before any edit: run GitNexus `impact()` on the target symbol and `detect_changes()` before committing.
- Before handoff: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- The npm package is a Node launcher that downloads/verifies the GitHub Release binary; dogfood via a clean isolated npm prefix, not just the cargo build.

## Plan
- [x] Install npm terminal-jarvis@0.1.6 in a clean prefix and exercise core commands (list/show/use/current/plan/run/check/version/config/security/auth/pass-through).
- [x] Fix #1: active-harness home persisted CWD-relative (use/current broke across dirs/terminals). -> global home.
- [x] Fix #2: stale hardcoded v0.1.2 strings in auth/config/update -> dynamic CARGO_PKG_VERSION.
- [ ] Continue dogfooding to find remaining blind spots (e.g., security/check/status output overlap, env-readiness messaging, Windows/WSL edge cases, npm download/checksum failure UX).
- [ ] For each finding: reproduce with evidence, run impact(), implement minimal fix, update tests + CHANGELOG [Unreleased].
- [ ] Keep commits small and scoped; push to origin/release/0.1.7.
- [ ] Continue until the operator EXPLICITLY states they are ready; only then raise the 0.1.7 release (PR against develop, tag, publish). Until then, stop and wait.

## Progress
- Dogfooded npm 0.1.6 from a clean prefix: download + SHA256 verify + cache path works; pass-through to harness binaries works; error exit codes (rc=2) correct; offline `TERMINAL_JARVIS_NO_DOWNLOAD` error is clear.
- Commit 9c5c15b "fix: persist active harness in a global home and stop hardcoding version strings":
  - `src/context/session.rs`: `default_home()` now resolves to `$XDG_CONFIG_HOME/terminal-jarvis` else `~/.config/terminal-jarvis` (still overridable via `TERMINAL_JARVIS_HOME`).
  - `src/cli/compat.rs`: replaced 4 hardcoded `v0.1.2` strings with `env!("CARGO_PKG_VERSION")`.
  - `tests/context_tests.rs`: replaced the test asserting CWD-relative home with one asserting a global absolute home.
  - `CHANGELOG.md`: added `[Unreleased]` documenting both fixes.
- Verified end-to-end against rebuilt `target/release/terminal-jarvis`: `use opencode` in repo root, then `current` from `/tmp` returns `opencode` (was `none`); `auth help claude` reads `v0.1.6`. `cargo fmt`, `clippy -D warnings`, `cargo test` (all suites) pass.

## Current State
- Branch `release/0.1.7` exists on origin, based on `origin/develop`, tracking `origin/release/0.1.7`. (Renamed from `dogfood/0.1.7-candidate` so the version being targeted is obvious and progress is visible on GitHub.)
- Two dogfood fixes are committed and pushed. No dirty files.
- RELEASE GATE is armed: no PR/tag/publish until the operator explicitly says they are ready.
- Low/medium-priority observation not yet fixed: `check`, bare `security`, and `security status` produce identical output (only `security audit` adds the `X/Y ready` summary). Candidate for a follow-up fix if it reads as confusing to users.

## Blockers
- None. The release is gated behind explicit operator approval; every /goal session must stop short of raising it and wait.

## Verification
- `cargo fmt --all -- --check` -> clean.
- `cargo clippy --all-targets -- -D warnings` -> clean.
- `cargo test` -> all suites pass (context, cli_*, runtime, release_preflight, etc.).
- GitNexus `detect_changes()` on the diff: risk medium, changed symbols limited to `default_home`, `update_summary`/`config`/`config_show` (version strings), and the context test; no HIGH/CRITICAL; `catalog_root` flag is a coarse same-file false positive (unchanged).

## Next Agent Prompt
Work in /mnt/c/Users/bacm6/world/repositories/working/terminal-jarvis on branch `release/0.1.7` (based on origin/develop, latest on origin/release/0.1.7).

Objective: Keep dogfooding terminal-jarvis (npm 0.1.6) to surface blind spots and apply meaningful core-improvement fixes to this branch; hold it as the 0.1.7 release.

RELEASE GATE (critical): Do NOT open the PR against develop, tag, publish, or
upload release assets until the operator EXPLICITLY states they are ready. The
operator will say so in their own words; silence or vague progress is NOT
approval. Each /goal session continues dogfooding + applying fixes, then stops
and waits for that explicit go-ahead.

Hard rules:
- Branch off develop; PRs against develop.
- Rust files <= 100 lines; no new external deps; prefer harness TOML data.
- Before edits: GitNexus `impact()` on the target symbol; before commit: `detect_changes()`.
- Before handoff: `cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test`.
- Dogfood via a clean isolated npm prefix (`npm install -g terminal-jarvis@0.1.6 --prefix <tmp>`) so the npm launcher/download/verify path is exercised, not just `cargo run`.

Required reads:
- AGENTS.md, CHANGELOG.md (Unreleased), src/context/session.rs, src/cli/compat.rs, src/cli/dispatch.rs, src/cli/mod.rs, npm/terminal-jarvis/bin/terminal-jarvis, scratch/opencode-goal-active.md (this ledger).

Already done (commit 9c5c15b): global active-harness home + dynamic version strings. Verified working. Branch renamed to release/0.1.7 for visibility.

Open questions / candidates for next slices:
- Differentiate `check` vs bare `security` vs `security status` (currently identical output).
- Harden npm download/checksum-failure UX (retry/redirect/proxy guidance already present; verify on a throttled/redirecting mirror).
- WSL/Windows path behavior (npm wrapper already excludes win32 binary; confirm messaging).
- Per-harness env-readiness messaging clarity in `auth help`.

Validation before handoff: rebuild (`cargo build --release`), dogfood the rebuilt binary across two working directories to confirm `use`/`current` consistency, run the full test gate, and append findings + fixes to CHANGELOG [Unreleased] and this ledger.
