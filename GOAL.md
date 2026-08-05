# GOAL: Terminal-Jarvis 95%+ Line Coverage via Red/Green Property-Based Testing

## Mission
Drive terminal-jarvis from 92.79% → **95%+ line coverage** using **red/green property-based testing** for every headless CLI command. The test suite must be the **single source of truth** for correct CLI behavior and the **foundation for the future TUI (Ui capability)**.

---

## Current Reality (as of 088fb8b)

| Metric | Value |
|--------|-------|
| Branch | `release/0.1.13` |
| HEAD | `088fb8b` (Phase 2 done) |
| Coverage (all-targets) | 92.79% lines (748 missed / 10,381) |
| Gate | `cargo llvm-cov --fail-under-lines 90` ✅ |
| Lib tests | 97 passed |
| Clippy | 0 warnings |
| Fmt | Clean |

**Completed Phases:**
- **Phase 0** (`39641a8`): quickcheck dev-dep, llvm-cov config, AGENTS.md tradeoff
- **Phase 1** (`b2f0108`): Pure-logic quickcheck props for contracts, catalog, cli args, table layout/width, json escape, diagnostics redact, security checks
- **Phase 2** (`088fb8b`): Red/green matrix for all 21 Action variants through full execute path

---

## Remaining Gap Analysis (from `--all-targets` coverage)

### Critical Gaps (>20 missed functions)

| File | Missed | Coverage | What's Untested |
|------|--------|----------|-----------------|
| `src/cli/guard_intent.rs` | 28 fn | 65.85% fn | dry_run, confirm, interactive, dangerous, ReadOnly, prompt branches |
| `src/cli/self_update.rs` | 21 fn | 61.82% fn | preview(), run(), wrapper detection, homebrew/npm/cargo fallback, dry-run |
| `src/cli/self_update_intent.rs` | 19 fn | 56.82% fn | --dry-run, --no-input, --confirm intent validation, confirmation flow |
| `src/cli/experimental.rs` | 18 fn | 59.09% fn | dashboard subcommand, feature flag gating, disabled path |
| `src/cli/guard_policy.rs` | 20 fn | 68.25% fn | stale evidence, platform mismatch, support state (Unknown/Manual/Stub/Unsupported/Disabled) |
| `src/diagnostics/resolve.rs` | 28 fn | 73.91% fn | DiagnosticInput validation, evidence freshness, platform targeting |
| `src/platform.rs` | 26 lines | 74.00% | `id()`/`libc()` compile-time branches untestable; `shell()`/`wsl()` ✅ |

**Total remaining missed functions to 95%:** ~150 fn across ~15 files

---

## Non-Negotiable Constraints (Hard Rules)

1. **File size ≤ 100 lines** — split with `#[path = "..."] mod name;` if needed
2. **Quickcheck = dev-only** — `[dev-dependencies]`, never ships, documented in AGENTS.md
3. **No external deps** without documented tradeoff
4. **No second runtime** — Rust CLI only, no Go ADK
5. **No `current/` snapshot** — no legacy compatibility layer
6. **No tag/publish** without explicit operator approval
7. **Run `gitnexus impact`** before editing any symbol; warn on HIGH/CRITICAL
8. **Run `gitnexus detect_changes()`** before every commit
9. **Coverage target** lives in `scripts/verify.sh` (currently 90 → will be 95)
9. **Release binary = std-only** — quickcheck never ships

---

## Test Architecture (Must Follow)

### Three Layers of Testing

| Layer | Location | Purpose | Tool |
|-------|----------|---------|------|
| **Pure Logic Props** | `src/**/props.rs` | Arbitrary input → invariant | `quickcheck` |
| **CLI Red/Green** | `tests/cli_red_green_*.rs` | Subprocess → exit code + output | `Command::new()` |
| **Unit Tests** | `src/**/tests.rs` | Pure functions, edge cases | `#[test]` |

### Conventions (MUST FOLLOW)

```rust
// Test module wiring (keeps source ≤ 100 lines)
#[cfg(test)]
#[path = "args_matrix.rs"]
mod matrix;

// Subprocess test helper
fn run(args: &[&str]) -> (i32, String, String) {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain"])
        .args(args)
        .env("TERMINAL_JARVIS_HOME", temp_home())
        .output()
        .expect("terminal-jarvis runs")
    // Returns (code, stdout, stderr) — ORDER MATTERS!
}

// temp_home() = std::env::temp_dir().join(format!("tj-{}-{}", std::process::id(), counter))

// parse_cli skips argv[0] — always pass ["tj", "subcommand", ...] in tests
// destructure: (code, _, stderr) or (code, stdout, _) — ORDER MATTERS!
// PathBuf::from(std::env::temp_dir()) is redundant — use std::env::temp_dir() directly
// quickcheck "No Arguments Provided" = zero-arg property or all-discard
```

---

## Remaining Phases (Execute in Order)

### Phase 3 — Close Remaining Gaps to ~94% (Current Focus)

**Targets (in priority order):**

1. **`guard_intent.rs`** — Add `check()` tests covering:
   - `Effect::ReadOnly` + lifecycle options → reject (exit 2)
   - `dry_run=true` → skip prompt (Ok)
   - `Interaction::Interactive` + no terminal → error (exit 5)
   - `Effect::Dangerous` + missing `--allow-dangerous` → error (exit 5)
   - `confirm=` matching token + terminal/no_input → Ok
   - `confirm=` mismatch / missing / no terminal → error (exit 5)
   - Prompt path (mock stdin/terminal) — accept "y/yes", reject "n/no"

2. **`self_update.rs`** — Test all paths:
   - `preview()` returns correct shell command for homebrew/npm/cargo
   - `run(dry_run=false)` with wrapper detection (homebrew/npm/cargo)
   - `run(dry_run=true)` returns preview without executing
   - Fallback when no wrapper found → error
   - Platform-specific wrapper paths (homebrew vs linux)

3. **`self_update_intent.rs`** — Test intent validation:
   - `--dry-run` returns preview without mutation
   - `--no-input --confirm=...` bypasses prompt
   - Missing `--confirm` in non-interactive → error with token
   - `--confirm` mismatch → error
   - `--allow-dangerous` without explicit harness → error

4. **`experimental.rs`** — Dashboard subcommand:
   - `TERMINAL_JARVIS_EXPERIMENTAL_UI=1 experimental dashboard` → Ok
   - Without env var → error (exit 4, feature_disabled)
   - Unknown subcommand → error

5. **`guard_policy.rs`** — Staleness/platform/support guards:
   - `freshness_status != "fresh"` → error (exit 4, evidence_stale)
   - Platform not in `plan.platforms` → error (exit 4, platform_incompatible)
   - Support states: Unknown/Manual/Stub/Unsupported/Disabled → error (exit 4)
   - Support Verified/Expected + fresh + platform match → Ok

6. **`diagnostics/resolve.rs`** — Input validation:
   - `DiagnosticInput::local()` with various platform/tty combos
   - Evidence freshness calculation
   - Platform targeting logic

### Phase 4 — TUI Groundwork (Guard Paths)

**Objective:** Make `guard_intent::check()` and `guard_policy::check()` **fully property-testable** so the future TUI can reuse the same validation logic.

- Extract `HarnessBuilder` / `CapabilityPlanBuilder` test fixtures
- Property-test all `Effect` × `Interaction` × `SupportState` × `options` combinations
- Ensure `--dry-run`, `--no-input`, `--confirm`, `--allow-dangerous` compose correctly

### Phase 5 — Release Gate (95% Gate)

1. Bump `coverage_target=90` → `95` in `scripts/verify.sh`
2. Run full `scripts/verify.sh` (build, test, clippy, fmt, coverage, mutants)
3. `cargo-mutants` pass
4. Update `CHANGELOG.md`
5. **Do not tag** without explicit approval

---

## What "Done" Looks Like

- `cargo llvm-cov --all-targets --fail-under-lines 95` ✅
- All three test layers cover every Action variant red + green
- `guard_intent` + `guard_policy` fully property-tested
- TUI can import validation logic without duplication
- `scripts/verify.sh` passes end-to-end
- `cargo-mutants` survives

---

## Commands Reference

```bash
# Full local gate (what verify.sh runs)
cargo build --all-targets
cargo test --lib
cargo test --test cli_red_green_green --test cli_red_green_red --test cli_red_green_plan
cargo clippy --all-targets
cargo fmt --all -- --check
cargo llvm-cov --all-targets --fail-under-lines 95

# Coverage with missing lines
cargo llvm-cov --all-targets --show-missing-lines 2>&1 | grep "platform.rs\|guard_intent\|self_update"

# Quickcheck debugging
cargo test --lib <module>::props -- --nocapture
RUST_BACKTRACE=1 cargo test --lib <failing_test> -- --nocapture
```

---

## Guardrails (Do Not Cross)

- ❌ No push to remote without explicit approval
- ❌ No new production dependencies
- ❌ No file > 100 lines
- ❌ No quickcheck in release binary
- ❌ No tag/publish from local
- ❌ No `current/` snapshot reintroduction
- ❌ No second runtime/ADK
- ❌ Edit symbol without `gitnexus impact` first
- ❌ Commit without `gitnexus detect_changes()`

---

## Handoff Notes for Next Agent

**Start with:** `src/cli/guard_intent.rs` — add the 6 missing `check()` branches. The test infrastructure (`tests/cli_red_green_*.rs`, `src/cli/args_red_green.rs`, `src/cli/args_rules.rs`) is in place. Use the existing `HarnessBuilder`/`CapabilityPlanBuilder` patterns from `src/cli/args_rules.rs`.

**Key insight:** The `check()` function in `guard_intent.rs` is the **core intent validation** for all lifecycle operations. Covering its 6 branches (ReadOnly, dry_run, interactive, dangerous, confirm, prompt) unlocks ~28 functions of coverage and establishes the pattern for `self_update_intent` and `guard_policy`.

**Watch out for:** `std::io::stdin().is_terminal()` and `std::io::stdin().read_line()` — these need mocking or conditional compilation for unit tests. The integration tests in `tests/cli_red_green_*.rs` already exercise the real binary with `--plain` and temp home dirs — prefer those for prompt/interactive paths.

**Coverage measurement:** Use `cargo llvm-cov --all-targets` for true picture. `--lib` only measures lib-test coverage (misses integration tests).

---

**This goal is the single source of truth. Execute phases in order. Report gaps honestly. No shortcuts.**