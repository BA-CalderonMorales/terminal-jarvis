# Issue #27: Codex and Gemini Authentication Flow Issues

## Problem

Both Codex and Gemini have problematic authentication flows that differ from other tools.

## Current State

Most tools follow pattern:
1. Set `TOOL_API_KEY` environment variable
2. Tool reads from env
3. Done

**Codex issues:**
- Browser-based OAuth flow
- Token storage location unclear
- Session management problems

**Gemini issues:**
- Google Cloud auth complexity
- Application Default Credentials
- Service account vs user account confusion

## Impact

Users who successfully use Claude, Aider, etc. struggle with Codex/Gemini.

## Solutions

### Option 1: Standardize on API Keys

Document API key setup for each tool:

```bash
# Codex
export OPENAI_API_KEY=sk-...

# Gemini  
export GOOGLE_API_KEY=...
```

Update tool configs to prefer API keys over OAuth.

### Option 2: Auth Abstraction Layer

Create unified auth handling:

```rust
pub trait ToolAuth {
    fn auth_type(&self) -> AuthType;  // ApiKey, OAuth, None
    fn is_authenticated(&self) -> bool;
    fn get_auth_instructions(&self) -> String;
}
```

### Option 3: Mark as "Requires Setup"

For tools with complex auth, show clear warning:

```
⚠ Codex requires additional setup.
  Run: terminal-jarvis auth setup codex
  Docs: https://...
```

### Option 4: Drop Problematic Tools

If auth is too complex/unstable, remove from default list.
Keep as "community" or "experimental" tools.

## Implementation Plan

1. Document exact auth flow for each tool
2. Test each flow in qa/env-auth-flows
3. Decide: fix, abstract, or deprecate
4. Update tool configs

## QA Branch

Test environment: `qa/env-auth-flows`

```bash
git checkout qa/env-auth-flows
npm run test:codex-auth
npm run test:gemini-auth
```

## Status

- [x] QA environment created
- [x] Auth patterns documented
- [ ] ⏸️ Test Codex OAuth - **BLOCKED by Issue #24**
- [ ] ⏸️ Test Gemini auth - **BLOCKED by Issue #24**
- [ ] Decision on each tool

## QA Test Results (2026-01-04)

### ⚠️ TESTING BLOCKED

Auth flow testing cannot proceed due to Issue #24 (GLIBC compatibility).
The binary fails to execute on Ubuntu 22.04/Debian 12, preventing auth testing.

**Must fix Issue #24 first.**

### Auth Pattern Matrix (Documented)

| Tool | Primary Key | Complexity | OAuth |
|------|-------------|------------|-------|
| claude | `ANTHROPIC_API_KEY` | Simple | No |
| codex | `OPENAI_API_KEY` | Complex | Sometimes |
| gemini | `GEMINI_API_KEY` / `GOOGLE_API_KEY` | Complex | Fallback |
| aider | `OPENAI_API_KEY` | Simple | No |
| goose | Multiple | Complex | Provider-dependent |
| qwen | `DASHSCOPE_API_KEY` | Simple | No |

### Tools by Auth Complexity

**Simple (API Key only):**
- claude, aider, qwen, llxprt, opencode

**Complex (Multiple options/OAuth):**
- codex, gemini, goose

### Next Steps

1. Fix Issue #24 (GLIBC)
2. Re-run auth tests
3. Make decision on complex auth tools
