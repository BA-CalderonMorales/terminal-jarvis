# Strategic Roadmap: Terminal Jarvis

## The Opportunity

The AI CLI tool ecosystem is fragmenting. Developers are juggling claude-code, gemini-cli, aider, codex, goose, and more. Each tool has its own:
- Installation method
- Authentication flow
- Configuration format
- Update mechanism

**Terminal Jarvis solves this.** One command center. One credential store. One update mechanism.

## Competitive Moat

1. **Session Continuation** - No other tool preserves context through auth workflows
2. **Unified Credential Management** - One place for all API keys
3. **Tool-Agnostic** - Works with any AI CLI, not locked to one provider
4. **Scrappy Distribution** - NPM, Cargo, AND Homebrew coverage

## Strategic Phases

| Phase | Focus | Goal | Status |
|-------|-------|------|--------|
| [Phase 5](phase-5-first-run-magic.md) | First-Run Experience | Zero-friction onboarding | |
| [Phase 6](phase-6-release-automation.md) | Release Automation | One-command releases | |
| [Phase 7](phase-7-navigation-hardening.md) | Navigation Hardening | Power-user efficiency | |
| [Phase 8](phase-8-credential-intelligence.md) | Credential Intelligence | Smart API key management | |
| [Phase 9](phase-9-ecosystem-growth.md) | Ecosystem Growth | Community contributions | |
| [Phase 10](phase-10-modern-architecture.md) | **Modern Architecture** | Turso DB + Cloud Voice | **IN PROGRESS** |

### Phase 10 Progress (Last Updated: 2026-01-03)

**Session 1 Complete:**
- Database foundation with libSQL/Turso
- QueryBuilder pattern (no hardcoded SQL)
- Hybrid tool loading (DB first, TOML fallback)
- Centralized error module

**Next Session:** Voice Simplification (remove whisper-rs C++ dependency)

## Architecture Evolution

**Current State**: ~~TOML files + whisper-rs (C++ dependency)~~ Hybrid DB/TOML + whisper-rs  
**Target State**: libSQL/Turso + Cloud Voice APIs

**Progress**: Database layer complete, voice simplification pending.

This simplifies builds, enables cloud sync, and removes C++ compilation requirements.

## Guiding Principles

### 1. Ship Small, Ship Often
- Each session should produce a deployable improvement
- Prefer 3 small PRs over 1 large refactor
- If it takes more than 2 sessions, break it down

### 2. Leverage Agentic Workflows
- Automate the boring stuff (version bumps, changelog updates)
- Let agents do the mechanical work, focus human time on decisions
- Build tooling that makes future sessions faster

### 3. Avoid Over-Engineering
- No features without clear user demand
- If in doubt, ship a simpler version first
- Voice feature was correctly deferred - apply this thinking everywhere

### 4. Respect the Developer's Time
- Yours (no late nights for marginal commits)
- Users (fast startup, minimal prompts)
- Future agents (clear documentation, obvious patterns)

## Success Metrics

- **Installation → First Tool Launch**: < 60 seconds
- **Switching between tools**: 1 keypress
- **Release cycle**: < 10 minutes end-to-end
- **New contributor → merged PR**: < 1 day
