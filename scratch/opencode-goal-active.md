# OpenCode Goal

Status: active
Objective: Dogfood terminal-jarvis as shipped (npm 0.1.6 surface) on release/0.1.7:
run a real dogfood pass over every CLI command, land at least one genuine defect
per command, and span the full severity spectrum (low/medium/high/critical).
Reproduce each defect with evidence, then apply the minimal core-improvement fix.
Started: 2026-07-07
Updated: 2026-07-07
Repo: /mnt/c/Users/bacm6/world/repositories/working/terminal-jarvis
Branch: release/0.1.7
Latest Commit: 320e0d2 (then fix commits below)

## Constraints
- Stay on release/0.1.7. Rust <=100 lines; no new deps; TOML over Rust branches.
- Before edits: GitNexus impact() on target symbols; before commit: detect_changes().
- Handoff gate: cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test.
- Dogfood via rebuilt binary (cargo build --release) AND the npm launcher wrapper
  (node npm/terminal-jarvis/bin/terminal-jarvis ...) so the source-binary path is real.
- Avoided real harness execution (install/update/headless/ui) on this machine; reproduced
  resolution via safe fake-bin + unit tests.

## Severity scale (USER IMPACT)
- low = papercut/cosmetic; medium = wrong/confusing in common case;
  high = broken/misleading in default scenario; critical = wrong command executed /
  silent data loss / unrecoverable state.

## Findings (distinct root causes; spectrum covered)

### CRITICAL — `run` hijacks a natural-language prompt as a capability (wrong command executed)
- Evidence (before fix, old resolve::run): `run update my database` / `run yolo clean tmp`
  (no harness token, active harness set) resolved to `Update(opencode,[...])` /
  `Yolo(opencode,[...])` and executed the harness's side-effecting/dangerous command
  instead of a headless chat. `run update my database` would run `npm update -g opencode-ai`.
- Fix: src/cli/resolve.rs `run` — a capability word is now only dispatched as a capability
  when it is the sole token (`run version`) or when an explicit harness token precedes it
  (`run <harness> <cap>`); a multi-word prompt whose first word is a capability is sent to
  the harness as a headless prompt. `run headless <prompt>` still works.
- After evidence (fake `opencode` bin, active=opencode):
  `run update my database` -> "RAN opencode with args: run update my database" (headless).
  `run version` -> "RAN opencode with args: --version" (capability preserved).
  `run headless summarize` -> "RAN opencode with args: run summarize" (headless preserved).
- Test: src/cli/resolve_test.rs (5 cases). impact(run)=LOW.

### HIGH — `version` reports 0.1.6 on the release/0.1.7 RC (mislabels the candidate)
- Evidence: `terminal-jarvis version` -> "terminal-jarvis 0.1.6" while on branch release/0.1.7;
  `update`/`auth`/`config reset` notices also read v0.1.6 (CARGO_PKG_VERSION). Publishing the
  RC would ship as the wrong version.
- Fix: bumped Cargo.toml and npm/terminal-jarvis/package.json version to 0.1.7 (single root
  cause; also corrects every `v{VERSION}` notice). This is RC version hygiene, NOT a re-open of
  the prior hardcoded-string fix (9c5c15b). Flagged for operator awareness.
- After evidence: `version` -> 0.1.7; `update` summary now reads "updates are per harness in v0.1.7".
- Tests: existing release_preflight_tests + cli_version_tests still green (no hardcoded version).

### MEDIUM — `-v` inconsistent: top-level `-v` = plain, but `version -v` = verbose
- Evidence: `terminal-jarvis -v` -> plain; `terminal-jarvis version -v` -> verbose. Same flag,
  two meanings by position; a user learning `-v`=verbose from `version -v` gets plain from `-v`.
- Fix: src/cli/args.rs `version()` makes `-v` plain (consistent). Verbose stays on
  `--verbose`/`--info` (and `version --verbose`). Also documented the global flags in help.rs.
- After evidence: `tj -v` plain 0.1.7; `tj version -v` plain 0.1.7; `tj --info` verbose.
- Tests: cli_args_tests::rejects_unknown_version_flag (version -v is_ok) still passes.

### MEDIUM — `auth set <harness>` is a misleading no-op
- Evidence: `auth set opencode` returned the same guidance as `auth help` with no indication
  that nothing was stored; the verb "set" implies a mutating action that never happens.
- Fix: src/cli/compat.rs adds `auth_set_for` (new `set` arm) that explicitly states
  "terminal-jarvis does not persist credentials; nothing was stored."
- After evidence: `auth set opencode` now ends with that explicit line.

### LOW — `cache` prints a bare, unhelpful "unavailable"
- Evidence: `cache status` -> "cache: unavailable\ndistribution: unknown" with no guidance
  when run outside the npm wrapper.
- Fix: src/cli/cache.rs `status()` None branch now explains it is wrapper-managed and how to
  enable it (set TERMINAL_JARVIS_CACHE / use the npm launcher). Keeps "cache: unavailable"
  substring so cli_version_tests still passes.
- After evidence: `cache status` -> "cache: unavailable (set TERMINAL_JARVIS_CACHE or run via the npm launcher)".

## Per-command dogfood verdict
- help / --help / -h / no-args: defect (undocumented global flags) -> covered by MEDIUM flags-doc fix. Verified output identical for all four forms.
- version / --version / -v / --info: defects (HIGH mislabel + MEDIUM -v) -> fixed.
- list / tools: VERIFIED-CLEAN (identical, correct).
- check / status: VERIFIED-CLEAN (terse per-harness table; security status adds readiness summary — prior hardening).
- current: VERIFIED-CLEAN.
- use <harness>: VERIFIED-CLEAN (rejects unknown harness before writing; global home).
- show / info <harness>: VERIFIED-CLEAN.
- plan [harness] <capability>: VERIFIED-CLEAN (capability parse error lists all caps; panic not reproducible — catalog validates all caps present).
- run [harness] [capability] [args...]: CRITICAL (fixed).
- install <harness>: VERIFIED-CLEAN surface (unknown harness errors; real install not run per safety rules).
- update [harness]: VERIFIED-CLEAN surface (summary + version string now correct).
- auth [help|set] <harness>: MEDIUM (fixed).
- config [show|path|reset]: VERIFIED-CLEAN (version string now correct).
- cache <...>: LOW (fixed).
- security [status|audit|harness]: VERIFIED-CLEAN (readiness summaries present; harness plan works).
- templates / db: VERIFIED-CLEAN (legacy-removed message correct).
- direct harness invocation: VERIFIED-CLEAN (unknown -> clear error; known -> UI).

## Verification
- cargo fmt --all -- --check: OK
- cargo clippy --all-targets -- -D warnings: OK
- cargo test: ALL green (lib + every test binary; 33 tests). Earlier full-run hang was a
  stale cargo holding the build lock after a tool-timeout; cleared with `pkill -x cargo/rustc`.
- npm wrapper path exercised: `version` -> 0.1.7, `cache status` shows guidance, `check` works,
  distribution: source (resolves built binary, no network download).

## Progress
- Rebuilt release binary; dogfooded full command surface via binary + npm wrapper.
- Fixed 5 findings (CRITICAL/HIGH/MEDIUM/MEDIUM/LOW) with minimal edits; added resolve_test.rs;
  updated CHANGELOG [Unreleased]; bumped version to 0.1.7.

## Current State
- Branch release/0.1.7 has the fixes locally, uncommitted. Release is NOT raised/tagged/published.
- Changed (staged scope): CHANGELOG.md, Cargo.toml, Cargo.lock, npm/terminal-jarvis/package.json,
  src/cli/{args,cache,compat,help,resolve}.rs, src/cli/resolve_test.rs, scratch/opencode-goal-active.md.
- Untracked (do NOT commit): scratch/dogfood-home, scratch/dogfood-home2.

## Blockers
- none.

## Next Agent Prompt
Work in /mnt/c/Users/bacm6/world/repositories/working/terminal-jarvis (branch release/0.1.7).

Objective: dogfood terminal-jarvis 0.1.7 RC and land >=1 genuine defect per CLI command,
spanning low/medium/high/critical. Status: COMPLETE dogfood + fixes implemented locally,
UNCOMMITTED, release NOT raised.

Hard rules: stay on release/0.1.7; Rust <=100 lines; no new deps; GitNexus impact() before edits,
detect_changes() before commit; do NOT tag/publish/PR until explicit operator go-ahead.

Findings fixed (see above): CRITICAL run-prompt-hijack (resolve.rs), HIGH version 0.1.6->0.1.7
(Cargo.toml + npm package.json), MEDIUM -v inconsistency (args.rs + help.rs), MEDIUM auth set
no-op (compat.rs), LOW cache unavailable msg (cache.rs). Each has before/after evidence and a test.

Required reads: AGENTS.md, CHANGELOG.md [Unreleased], scratch/opencode-goal-active.md,
src/cli/{args,resolve,compat,cache,help}.rs, src/cli/resolve_test.rs.

Validation already run: cargo fmt --check OK, clippy -D warnings OK, cargo test all green,
npm wrapper path exercised. Pending only explicit operator approval to tag/publish/PR.

Exact next slice: confirm the 5 fixes, then await operator go-ahead to commit/push/tag/publish.
If committing: `git add` the 10 changed files listed under Current State (NOT scratch/dogfood-home*),
`git commit`, `git push origin release/0.1.7`.
