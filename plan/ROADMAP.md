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

| Metric | Current | Target |
|--------|---------|--------|
| Install time | 30-60s | <15s |
| Steps to launch | 4-5 | 1-2 |
| GLIBC minimum | 2.39 | 2.17 |
| Binary size | ~17MB | <10MB |
