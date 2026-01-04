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
