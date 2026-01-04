# QA Testing Environments

## Branch Structure

```
qa/env-debian-bookworm     # GLIBC testing
qa/env-fresh-install       # UX and speed testing  
qa/env-auth-flows          # Auth flow testing
```

## Environment: Debian Bookworm

**Branch:** `qa/env-debian-bookworm`

**Purpose:** Test GLIBC 2.39 compatibility issue (#24)

**Base Image:** `mcr.microsoft.com/devcontainers/base:debian-12`

**GLIBC Version:** 2.36 (older than required 2.39)

**Tests:**
- `npm run test:glibc` - Full GLIBC compatibility suite
- Binary symbol analysis
- Installation failure capture

**Expected Results:**
- npm install: FAIL (GLIBC mismatch)
- Binary execution: FAIL (GLIBC mismatch)
- Documents exact error for fixing

## Environment: Fresh Install

**Branch:** `qa/env-fresh-install`

**Purpose:** Test download speed (#23) and UX steps (#26)

**Base Image:** `mcr.microsoft.com/devcontainers/base:ubuntu-22.04`

**Clean State:**
- No terminal-jarvis installed
- No cached packages
- No API keys set

**Tests:**
- `npm run test:download-speed` - Measure install times
- `npm run test:first-run` - Count interaction steps
- `npm run test:steps` - Analyze user journey

**Metrics Collected:**
- Download time (npm, binary)
- Steps to launch tool
- Prompts displayed

## Environment: Auth Flows

**Branch:** `qa/env-auth-flows`

**Purpose:** Test authentication issues (#27)

**Base Image:** `mcr.microsoft.com/devcontainers/base:ubuntu-22.04`

**Features:**
- Browser available for OAuth
- Mock API key testing
- Auth state inspection

**Tests:**
- `npm run test:codex-auth` - OpenAI Codex flow
- `npm run test:gemini-auth` - Google Gemini flow
- `npm run test:claude-auth` - Anthropic Claude flow

**Scenarios:**
1. No API key set
2. Invalid API key
3. Expired token
4. OAuth flow completion

## Test Results Summary (2026-01-04)

All three QA environments have been tested. Results:

| Environment | Branch | Issues Tested | Result |
|-------------|--------|---------------|--------|
| Debian Bookworm | `qa/env-debian-bookworm` | #24 | ❌ GLIBC 2.39 blocks execution |
| Fresh Install | `qa/env-fresh-install` | #23, #24, #26 | ⚠️ Multiple issues confirmed |
| Auth Flows | `qa/env-auth-flows` | #27 | ⏸️ Blocked by #24 |

### Key Findings

1. **Issue #24 is the critical blocker** - Binary requires GLIBC 2.39
2. **Issue #23 partially confirmed** - NPM overhead is the bottleneck, not downloads
3. **Issue #26 confirmed** - 3-5 steps instead of target 1-2
4. **Issue #27 blocked** - Cannot test auth without working binary

### Priority Order

1. **P0: Fix Issue #24** (GLIBC) - Nothing else works until this is fixed
2. **P1: Fix Issue #26** (UX steps) - Quick wins available
3. **P1: Fix Issue #23** (Speed) - Focus on npm overhead, not binary size
4. **P2: Fix Issue #27** (Auth) - Retest after #24 is fixed

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
