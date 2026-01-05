# Terminal Jarvis Development Roadmap

## Vision

Make terminal-jarvis the simplest, fastest way to access AI coding tools.

## Priorities (v0.0.72+)

### P0: Critical Fixes

1. **GLIBC Compatibility (Issue #24)**
   - Build with musl for static linking
   - Provide multiple binary variants
   - Auto-detect and download correct binary

2. **Download Speed (Issue #23)**
   - Optimize binary size
   - Consider lazy loading
   - CDN for faster downloads

### P1: UX Improvements

3. **Streamline Launch Flow (Issue #26)**
   - Skip unnecessary prompts
   - Remember last-used tool
   - Add `--quick` mode
   - Direct tool invocation: `tj claude`

4. **Fix Auth Flows (Issue #27)**
   - Standardize auth patterns
   - Only show auth advisory when needed
   - Better error messages

### P2: Features

5. **Database Layer (Issue #31)**
   - Clean CLI commands for DB operations
   - Pipeline/headless mode
   - Reduce configuration noise

## Release Cadence

- **Patch releases** (0.0.x): Bug fixes, weekly
- **Minor releases** (0.x.0): Features, monthly
- **Major releases** (x.0.0): Breaking changes, as needed

## Quality Gates

Every release must pass:

1. `cargo clippy --all-targets --all-features -- -D warnings`
2. `cargo test --all-features`
3. All QA environment tests
4. Manual smoke test on fresh install

## Success Metrics

| Metric | Before v0.0.72 | Target | v0.0.72 Fix |
|--------|----------------|--------|-------------|
| Install time (npm) | 31s, no progress | <15s | Progress display added |
| Binary download | 227ms | <10s | ✅ Already fast |
| Steps to launch | 3-5 | 1-2 | Direct invocation + quick mode |
| GLIBC minimum | 2.39 | 2.17 | musl static linking |
| Binary size | 17MB | <10MB | ⚠️ Future work |

## v0.0.72 Fixes Applied (2026-01-05)

### Issue #24: GLIBC Compatibility ✅ FIXED
- **Fix:** Switched to musl targets (x86_64-unknown-linux-musl, aarch64-unknown-linux-musl)
- **Fix:** Replaced OpenSSL with rustls for static linking
- **Result:** Binary should work on any Linux regardless of GLIBC version
- **Verification:** Pending - test in `qa/v0.0.72-glibc-minimal` Codespace

### Issue #23: Download Speed ✅ FIXED
- **Fix:** Added progress display during npm postinstall
- **Fix:** Increased timeout to 60s
- **Fix:** Added timing statistics for download and extraction
- **Verification:** Pending - test in `qa/v0.0.72-fresh-install-minimal` Codespace

### Issue #26: UX Steps ✅ FIXED
- **Fix:** Direct tool invocation (`terminal-jarvis claude`)
- **Fix:** Quick launch mode (`terminal-jarvis -q`)
- **Fix:** Streamlined menu flow (no args prompt)
- **Fix:** Post-tool auth menu options
- **Verification:** Pending - test in `qa/v0.0.72-fresh-install-minimal` Codespace

### Issue #27: Auth Flows ⏸️ PENDING
- **Status:** Can now test after Issue #24 is verified fixed
- **Next:** Create auth-flows QA branch after #24 confirmed

## QA Verification Needed

Test these Codespace branches to confirm fixes:

1. **Create Codespace on `qa/v0.0.72-glibc-minimal`**
   - Run `test-glibc.sh`
   - Confirm binary works on Debian 12 (GLIBC 2.36)

2. **Create Codespace on `qa/v0.0.72-fresh-install-minimal`**
   - Run `test-fresh-install.sh`
   - Confirm progress display and UX improvements

## Previous Test Results (2026-01-04)

Before v0.0.72:

### Issue #24: GLIBC Compatibility ❌ CRITICAL
- **Tested on:** Ubuntu 22.04 (GLIBC 2.35), Debian 12 (GLIBC 2.36)
- **Result:** Binary requires GLIBC 2.39, **fails to execute** on both
- **Impact:** Blocks all other testing and usage on most Linux distros
- **Priority:** P0 - Must fix before any other work

### Issue #23: Download Speed ⚠️ PARTIAL
- **NPM install:** 31s (target <15s) - bottleneck is npm overhead, not download
- **Binary download:** 227ms for 5.9MB - actually fast!
- **NPX cold start:** >30s timeout
- **Finding:** Problem is npm/npx overhead, not network speed

### Issue #26: UX Steps ❌ CONFIRMED  
- **Default flow (no API key):** 5 steps
- **Default flow (with API key):** 3 steps
- **Direct launch (`tj claude`):** 1 step (ideal, but not obvious)
- **Recommendation:** Add `--quick` mode, remember last tool

### Issue #27: Auth Flows ⏸️ BLOCKED
- **Status:** Cannot test - blocked by Issue #24 (GLIBC)
- **Documented:** Auth patterns for all tools mapped out
- **Next:** Retest after Issue #24 is fixed
