# QA Testing Environments

## Current QA Branches (v0.0.72)

```
qa/v0.0.72-glibc-minimal         # GLIBC compatibility (Issue #24)
qa/v0.0.72-fresh-install-minimal # Download speed & UX (Issues #23, #26)
```

**Note**: These are orphan branches containing only `.devcontainer/` and `README.md` for clean testing.

## Environment: GLIBC Compatibility

**Branch:** `qa/v0.0.72-glibc-minimal`

**Purpose:** Verify Issue #24 fix (musl static linking)

**Base Image:** `mcr.microsoft.com/devcontainers/base:debian-12`

**GLIBC Version:** 2.36 (older than previously required 2.39)

**Test Command:**
```bash
test-glibc.sh
```

**What It Tests:**
- npm install succeeds on older GLIBC
- Binary is statically linked
- CLI commands work

## Environment: Fresh Install & UX

**Branch:** `qa/v0.0.72-fresh-install-minimal`

**Purpose:** Verify Issues #23 (speed) and #26 (UX) fixes

**Base Image:** `mcr.microsoft.com/devcontainers/base:ubuntu-22.04`

**Test Command:**
```bash
test-fresh-install.sh
```

**What It Tests:**
- Progress display during install
- Install timing
- Direct invocation works (`terminal-jarvis claude`)
- Clean menu rendering

## How to Test

### Via GitHub Codespace (Recommended)

1. Go to https://github.com/BA-CalderonMorales/terminal-jarvis
2. Switch to a QA branch (e.g., `qa/v0.0.72-glibc-minimal`)
3. Click "Code" → "Codespaces" → "Create codespace on qa/v0.0.72-glibc-minimal"
4. Wait for environment to build
5. Run the test script shown in terminal

### Via Docker (Local)

```bash
git checkout qa/v0.0.72-glibc-minimal
docker build -t qa-glibc .devcontainer/
docker run -it qa-glibc bash
test-glibc.sh
```

## Previous Test Results (2026-01-04)

Before v0.0.72 fixes:

| Issue | Problem | Status |
|-------|---------|--------|
| #24 | Binary required GLIBC 2.39 | ❌ BLOCKED all testing |
| #23 | No progress during download | ❌ Poor UX |
| #26 | 4-5 steps to launch tool | ❌ Too many steps |

## v0.0.72 Fixes Applied

| Issue | Fix |
|-------|-----|
| #24 | Switched to musl static linking (x86_64-unknown-linux-musl) |
| #23 | Added progress display, timing stats in postinstall.js |
| #26 | Direct invocation, quick launch mode, streamlined menus |

## Pending Verification

Test these QA branches in fresh Codespaces to confirm fixes work:

- [ ] `qa/v0.0.72-glibc-minimal` - Confirm binary works on Debian 12
- [ ] `qa/v0.0.72-fresh-install-minimal` - Confirm progress and UX improvements

## Using an Environment

### Via GitHub Codespace

1. Go to repository
2. Switch to QA branch
3. Click "Code" → "Codespaces" → "Create"
4. Wait for environment setup
5. Run tests

### Via Local Devcontainer

```bash
git checkout qa/env-debian-bookworm
code .
# VS Code prompt: "Reopen in Container"
npm run test:qa
```

### Via Docker CLI

```bash
git checkout qa/env-debian-bookworm
docker build -t tj-qa-debian .devcontainer/
docker run -it tj-qa-debian bash
npm run test:qa
```
