# Issue #23: Download Speed Needs Improvement

## Problem

Initial download of terminal-jarvis takes too long, especially via `npx`.

## Current Metrics

| Method | Time | Size |
|--------|------|------|
| `npm install -g terminal-jarvis` | ~30-60s | 17MB |
| `npx terminal-jarvis` (cold) | ~45-90s | 17MB + npm overhead |
| Binary download | ~10-20s | 6MB compressed |

## Root Causes

1. **Large binary size** (~17MB uncompressed)
2. **NPM package includes binary** - downloads binary inside npm package
3. **Postinstall downloads again** - sometimes double-download
4. **No CDN** - downloads from GitHub releases

## Solutions

### Option 1: Reduce Binary Size

- Enable LTO (Link Time Optimization)
- Strip debug symbols
- Use `opt-level = "z"` for size
- Remove unused features

```toml
[profile.release]
lto = true
strip = true
opt-level = "z"
codegen-units = 1
```

Target: <10MB compressed

### Option 2: Lazy Binary Download

Don't include binary in npm package. Download on first run.

```
npm install terminal-jarvis  # Just wrapper, <100KB
npx terminal-jarvis          # Downloads binary on first use
```

### Option 3: CDN Distribution

Use jsDelivr or unpkg to cache releases closer to users.

### Option 4: Regional Mirrors

Provide multiple download URLs, auto-select fastest.

## Implementation Plan

1. Add size optimization to Cargo.toml
2. Measure new binary size
3. Update postinstall to show progress bar
4. Consider lazy download approach

## QA Branch

Test environment: `qa/env-fresh-install`

```bash
git checkout qa/env-fresh-install
npm run test:download-speed
```

## Status

- [x] QA environment created
- [ ] Binary size optimization
- [ ] Download progress indicator
- [ ] Lazy download implementation
