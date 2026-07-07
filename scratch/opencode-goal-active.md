# OpenCode Goal

Status: active
Objective: Continue dogfooding terminal-jarvis (npm 0.1.6) to surface blind spots / UX gaps, apply meaningful core-improvement fixes to this branch, and hold it as the candidate for a 0.1.7 release.
Started: 2026-07-07T21:17:45Z
Updated: 2026-07-07T22:10:00Z
Repo: /mnt/c/Users/bacm6/world/repositories/working/terminal-jarvis
Branch: release/0.1.7
Latest Commit: a5a0dcf
Remote: origin/release/0.1.7 (based on origin/develop)
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
- [x] Fix #3: `check`, `security`, and `security status` printed identical output (security readiness was hidden). -> `check` stays terse; `security`/`security status` now append `status: X/Y harnesses ready` summary; `security audit` keeps `audit summary`.
- [ ] Continue dogfooding to find remaining blind spots (env-readiness messaging, Windows/WSL edge cases, npm download/checksum failure UX).
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

## Progress (continued, 2026-07-07T22:10Z)
- Dogfooded the current build: confirmed `check`, `security`, and `security status` all produced identical per-harness tables (blind spot).
- Commit a5a0dcf "fix: separate check from security status readiness summary":
  - `src/cli/output.rs`: added `status()` and a shared `readiness()` helper; `check` keeps `checks()` (no summary), `security`/`security status` use `status()` (appends `status: X/Y harnesses ready`), `security audit` keeps `audit summary`. File stays <= 100 lines.
  - `src/cli/dispatch.rs`: `security([])` and `security(["status"])` now call `output::status` instead of `output::checks`.
  - `src/cli/output_test.rs`: added `status_adds_readiness_summary_absent_from_checks` asserting `checks()` lacks the summary and `status()` includes `status: 1/1 harnesses ready`.
  - `CHANGELOG.md`: added `[Unreleased]` bullet documenting the differentiation.
- Gate: `cargo fmt --all -- --check` clean; `cargo clippy --all-targets -- -D warnings` clean; `cargo test` all suites pass. GitNexus `detect_changes()` risk medium, only `security`/`env_status` touched, no HIGH/CRITICAL.
- Dogfood via npm wrapper (resolves the freshly built source binary): `security status` prints `status: 1/25 harnesses ready`; `use opencode` then `current` from `/tmp` returns `opencode` (global-home fix still intact).

## Current State
- Branch `release/0.1.7` exists on origin, based on `origin/develop`, tracking `origin/release/0.1.7`. (Renamed from `dogfood/0.1.7-candidate` so the version being targeted is obvious and progress is visible on GitHub.)
- Three dogfood fixes committed and pushed (9c5c15b active-harness home + version strings; a5a0dcf check vs security status). No unrelated dirty files.
- RELEASE GATE is armed: no PR/tag/publish until the operator explicitly says they are ready.
- Resolved: `check` vs `security`/`security status` overlap — `security`/`security status` now append a `status: X/Y harnesses ready` summary; `check` remains the terse table.

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

Already done (commit 9c5c15b): global active-harness home + dynamic version strings. Commit a5a0dcf: `security`/`security status` now append a `status: X/Y harnesses ready` summary so they differ from the terse `check` (the previous identical-output blind spot is resolved). Verified through the rebuilt binary and the npm launcher wrapper.

Open questions / candidates for next slices:
- Harden npm download/checksum-failure UX (retry/redirect/proxy guidance already present; verify on a throttled/redirecting mirror).
- WSL/Windows path behavior (npm wrapper already excludes win32 binary; confirm messaging).
- Per-harness env-readiness messaging clarity in `auth help`.

Validation before handoff: rebuild (`cargo build --release`), dogfood the rebuilt binary across two working directories to confirm `use`/`current` consistency, run the full test gate, and append findings + fixes to CHANGELOG [Unreleased] and this ledger.
