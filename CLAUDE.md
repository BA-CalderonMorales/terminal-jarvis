# Claude AI Assistant Instructions for Terminal Jarvis

## AGENT SKILLS AVAILABLE
Terminal Jarvis has 5 curated Agent Skills that auto-load based on your task. See `.claude/skills/README.md` for details.

## CRITICAL: NO EMOJIS POLICY
**NEVER use emojis.** Use text indicators: `[INSTALLED]`, `[SUCCESS]`, `[ERROR]`, `[WARNING]`
See `.claude/NO_EMOJIS_POLICY.md` for details.

## SINGLE SOURCE OF TRUTH
**ALL detailed instructions are in [AGENTS.md](AGENTS.md) - Read this first.**

---

## QUICK REFERENCE TABLE

| Task | AGENTS.md Link | Key Action |
|------|---------------|------------|
| **User says "let's deploy"** | [Deployment Trigger](AGENTS.md#deployment-commands-trigger---read-immediately) | Update CHANGELOG.md FIRST, sync versions, use GitHub Actions |
| **Bugfix** | [Test-Driven](AGENTS.md#test-driven-bugfixes-mandatory) | Write failing test FIRST, then fix |
| **Refactoring** | [Best Practices](AGENTS.md#refactoring-best-practices-critical) | Domain-based module extraction |
| **Add AI tool** | [Tool Config](AGENTS.md#tool-configuration-consistency-critical-for-new-features) | Follow 8-step consistency checklist |
| **Commit** | [Pre-Commit](AGENTS.md#pre-commit-checklist) | Run quality gates, update CHANGELOG.md |
| **Version change** | [Version Numbers](AGENTS.md#version-numbers-are-important) | Sync ALL: Cargo.toml, package.json, Homebrew |
| **New feature** | [Agent Delegation](AGENTS.md#lead-orchestrator-pattern-mandatory-approach) | Spawn specialized agents, parallelize work |
| **Code quality** | [Quality Rules](AGENTS.md#code-quality-rules) | Clippy, fmt, error handling standards |
| **Commit messages** | [Conventions](AGENTS.md#how-to-write-commit-messages) | type(scope): description |

---

## TOKEN BUDGET (MANDATORY)
**Lead (Sonnet 4.5)**: Max 1000 tokens - orchestration, planning, validation
**Agents (Haiku)**: Max 750 tokens - focused implementation
**Strategy**: Delegate aggressively, parallelize, validate incrementally. [Full details](AGENTS.md#token-budget-management-for-session-longevity)

---

## CRITICAL CHECKLIST
- [ ] Check AGENTS.md BEFORE acting
- [ ] NO EMOJIS in any output
- [ ] Update CHANGELOG.md BEFORE deployment
- [ ] Write failing test FIRST for bugfixes
- [ ] Sync ALL version files (Cargo.toml, package.json, Homebrew)
- [ ] Use GitHub Actions for releases (not local-cd.sh unless specified)
- [ ] Spawn agents for focused work, don't do everything yourself

---

## QUALITY GATES (Run before commits)
```bash
cargo check                  # Must compile
cargo clippy --all-targets --all-features -- -D warnings  # Must pass
cargo fmt --all             # Must be formatted
cargo test                  # Must pass (if tests exist)
```

---

## SPECIAL RULES
- **Documentation**: DON'T create docs unless explicitly requested (token conservation)
- **Agents**: Leverage specialized agents by default (unless user says otherwise)
- **Deployments**: GitHub Actions workflow is default, local-cd.sh is last resort