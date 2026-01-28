# AGENTS.md - AI Assistant Guidelines for Terminal Jarvis

**Single Source of Truth for All AI Coding Assistants**

---

## CRITICAL RULES - READ FIRST

| Rule | What | Why |
|------|------|-----|
| **NO EMOJIS** | Zero emojis in code, commits, docs, output | Use "[INSTALLED]", "►", "•" instead |
| **CHANGELOG FIRST** | Update CHANGELOG.md BEFORE deployment scripts | Prevents version confusion |
| **Formula BEFORE Release** | Commit Homebrew Formula BEFORE GitHub release | URL matching for brew install |
| **Test-Driven Bugs** | Write failing test FIRST, then fix | Prevents regression |
| **Version Sync** | Update Cargo.toml, package.json, Formula together | Multi-platform consistency |
| **Verify Changes** | Run verify-change.sh before commits | 2-3x quality improvement |
| **Git Full Paths** | Use `/usr/bin/git` not `git` | Avoid alias issues |

---

## DEPLOYMENT WORKFLOW

**CI/CD Pipeline Flow:**

1. **Develop locally** - Make changes, run `./scripts/cicd/local-ci.sh`
2. **Push to develop** - Triggers GitHub Actions CI
3. **CI validates** - Build, tests, clippy, formatting
4. **CD auto-deploys** - Once CI passes, CD handles release automatically

**Do NOT run `local-cd.sh` for full deployment** - GitHub Actions CD pipeline handles:
- Tag creation
- Binary builds for all platforms
- crates.io publishing
- GitHub release creation
- NPM coordination

**Version updates only:**
```bash
./scripts/cicd/local-cd.sh --update-version X.X.X
git add -A && git commit -m "chore(release): prepare vX.X.X"
git push origin develop
# Then wait for CI/CD pipeline
```

---

## QUICK START

| User Says | Skill | Quick Command |
|-----------|-------|---------------|
| "Let's deploy" | [deployment](.github/skills/deployment/) | Push to develop, CI/CD handles it |
| "Harden release" | [release-checklist](.github/skills/release-checklist/) | Pre-release verification |
| "Test in Codespace" | [qa-testing](.github/skills/qa-testing/) | Create minimal QA branch |
| "Fix this bug" | [testing](.github/skills/testing/) | Write failing test first |
| "Add new AI tool" | [tool-config](.github/skills/tool-config/) | Create `config/tools/<name>.toml` |
| "Refactor this file" | [refactoring](.github/skills/refactoring/) | Domain-based extraction |
| "Update version" | [versioning](.github/skills/versioning/) | `./scripts/cicd/local-cd.sh --update-version X.X.X` |
| "Verify my change" | [verification](.github/skills/verification/) | `./scripts/verify/verify-change.sh` |
| "Before I commit" | [code-quality](.github/skills/code-quality/) | Quality gates checklist |
| "Homebrew release" | [homebrew](.github/skills/homebrew/) | Archive -> Formula -> Commit -> Release |
| "NPM publish" | [npm](.github/skills/npm/) | `npm whoami` then publish |

---

## SKILLS DIRECTORY

All detailed instructions are organized as modular, reusable skills in [.github/skills/](.github/skills/):

| Skill | Description |
|-------|-------------|
| [verification](.github/skills/verification/) | Quality verification feedback loop |
| [deployment](.github/skills/deployment/) | Deployment workflows and CI/CD |
| [release-checklist](.github/skills/release-checklist/) | Pre-release automation and hardening |
| [qa-testing](.github/skills/qa-testing/) | Minimal QA branch creation and testing |
| [versioning](.github/skills/versioning/) | Version management across platforms |
| [testing](.github/skills/testing/) | Test-driven development practices |
| [refactoring](.github/skills/refactoring/) | Code refactoring patterns |
| [database](.github/skills/database/) | Database architecture patterns |
| [tool-config](.github/skills/tool-config/) | AI tool configuration |
| [homebrew](.github/skills/homebrew/) | Homebrew distribution |
| [npm](.github/skills/npm/) | NPM distribution |
| [code-quality](.github/skills/code-quality/) | Code quality standards |
| [git-workflow](.github/skills/git-workflow/) | Branching and merge strategy |
| [token-budget](.github/skills/token-budget/) | Token efficiency for AI sessions |

**Skills are loaded on-demand** - AI agents only load relevant skills into context when needed.

---

## PROJECT OVERVIEW

**Terminal Jarvis** = Unified command center for AI coding tools (claude-code, gemini-cli, qwen-code, opencode, llxprt, codex, goose, amp, aider, crush, copilot-cli).

**Core Innovation**: Session Continuation System (prevents auth workflow interruptions).

**Distribution**: NPM, Cargo, Homebrew

**Current Version**: 0.0.75

### Installation

```bash
npm install -g terminal-jarvis        # NPM
cargo install terminal-jarvis         # Cargo  
brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis  # Homebrew
```

### Architecture

```
/src/           - Rust application (domain-based modules)
/config/        - Global + modular tool configs (config/tools/*.toml)
/npm/           - TypeScript wrapper for NPM distribution
/homebrew/      - Formula + release archives
/scripts/       - CI/CD and utility scripts
  /cicd/        - Deployment automation (local-ci.sh, local-cd.sh)
  /verify/      - Verification feedback loop scripts
/.github/skills/ - AI agent skills (modular instructions)
```

---

## VERIFICATION FEEDBACK LOOP

> "Give Claude a way to verify its work. If Claude has that feedback loop, it will 2-3x the quality of the final result."
> -- Creator of Claude Code

```bash
./scripts/verify/verify-change.sh        # Full verification before commits
./scripts/verify/verify-change.sh --quick # Quick mode (skip tests)
./scripts/verify/verify-build.sh         # Compilation only
./scripts/verify/verify-quality.sh       # Clippy + formatting
./scripts/verify/verify-tests.sh         # Unit + E2E tests
./scripts/verify/verify-cli.sh           # CLI smoke tests
```

**See**: [verification skill](.github/skills/verification/) for full details.

---

## QUALITY GATES

```bash
cargo check                      # Must compile
cargo clippy -- -D warnings      # Must pass
cargo fmt --all                  # Must be formatted
cargo test                       # Must pass
```

**See**: [code-quality skill](.github/skills/code-quality/) for full standards.

---

## PREFERRED TOOLING

| Instead of | Use | Why |
|-----------|-----|-----|
| `grep` | `rg` (ripgrep) | Faster, respects .gitignore |
| `pip` | `uv` | 10-100x faster Python packages |
| `git` | `/usr/bin/git` | Avoid alias issues |

---

## SESSION CONTINUATION SYSTEM

**Key Feature**: Prevents users from being kicked out during authentication workflows.

1. User launches AI tool (e.g., `terminal-jarvis run claude`)
2. Tool requires authentication (redirects to browser)
3. Traditional approach: User returns, session gone
4. Terminal Jarvis: Session preserved, resumes automatically

**Implementation**: `src/tools/tools_execution_engine.rs`

---

## ENVIRONMENT VARIABLES

| Variable | Purpose | Default |
|----------|---------|---------|
| `TERMINAL_JARVIS_CONFIG` | Config file path | `~/.terminal-jarvis/config.toml` |
| `TERMINAL_JARVIS_LOG_LEVEL` | Logging verbosity | `info` |
| `TERMINAL_JARVIS_SESSION_DIR` | Session state directory | `~/.terminal-jarvis/sessions/` |

---

## PROACTIVE AGENT USAGE

AI assistants MUST invoke specialized agents immediately without waiting to be asked:

| Scenario | Agent |
|----------|-------|
| Documentation | @documentation-specialist |
| Testing | @qa-automation-engineer |
| Code review | @code-reviewer |
| Security | @security-specialist |
| Infrastructure | @devops-engineer |
| Architecture | @software-architect |

**See**: [token-budget skill](.github/skills/token-budget/) for orchestration patterns.

---

**Navigation**: Skills are in `.github/skills/` | Use Ctrl+F to search | Load skills on-demand
