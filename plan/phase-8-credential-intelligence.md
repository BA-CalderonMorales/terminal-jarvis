# Phase 8: Credential Intelligence

**Status**: PENDING  
**Priority**: MEDIUM  
**Estimated Sessions**: 2

## The Insight

Most AI tools share API key requirements:
- Claude, Claude-Code → `ANTHROPIC_API_KEY`
- Gemini, Goose (with gemini provider) → `GOOGLE_API_KEY`
- Codex, GPT-based tools → `OPENAI_API_KEY`
- Aider → Any of the above + `OPENROUTER_API_KEY`

**Current state**: Each tool prompts separately. User enters the same key multiple times.

**Target state**: Enter key once, all compatible tools work automatically.

## Tasks

### 1. Credential Relationship Mapping
- [ ] Define which tools share which credentials
- [ ] Store in config/tools/*.toml under `[tool.auth]`
- [ ] Add `shared_with` field: `shared_with = ["claude", "aider"]`

### 2. Smart Credential Propagation
- [ ] When key saved for one tool, offer to apply to related tools
- [ ] "ANTHROPIC_API_KEY saved for claude. Also use for: aider, amp? [Y/n]"
- [ ] Store decision to avoid re-prompting

### 3. Credential Health Check
- [ ] On startup, validate stored credentials still work
- [ ] Optional API ping (configurable, off by default)
- [ ] Show status in `/auth` menu: "ANTHROPIC_API_KEY: [VALID] / [EXPIRED] / [NOT SET]"

### 4. Secure Storage Upgrade
- [ ] Currently using `~/.terminal-jarvis/credentials.json`
- [ ] Consider OS keychain integration (macOS Keychain, Linux secret-service)
- [ ] Fall back to encrypted file if keychain unavailable
- [ ] Migration path from plaintext to secure storage

### 5. Credential Import/Export
- [ ] Export credentials (encrypted) for backup
- [ ] Import from another machine
- [ ] Sync across machines (optional, manual)

## Agent Instructions

Start with the credential mapping (Task 1) - it's data-only, no code changes.

```bash
cat config/tools/claude.toml
cat config/tools/aider.toml
```

Add the relationship data, then build propagation on top.

For secure storage, research:
- `keyring` crate for cross-platform keychain
- `secrecy` crate for in-memory secret handling

Don't over-engineer security. A well-permissioned file in `~/.terminal-jarvis/` is fine for v1.

## Success Criteria

- [ ] Single API key entry works for all compatible tools
- [ ] User sees clear status of which tools are configured
- [ ] No plaintext API keys in logs or error messages
