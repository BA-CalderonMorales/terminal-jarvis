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
- [ ] Document all auth flows
- [ ] Test Codex OAuth
- [ ] Test Gemini auth
- [ ] Decision on each tool
