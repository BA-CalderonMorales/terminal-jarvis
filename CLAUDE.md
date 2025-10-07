# Claude AI Assistant Instructions for Terminal Jarvis

## CRITICAL: NO EMOJIS POLICY - READ FIRST

**NEVER use emojis in any output, code, comments, documentation, commit messages, or communication.**

Use text-based indicators instead: `[INSTALLED]`, `[SUCCESS]`, `[ERROR]`, `[WARNING]`, etc.

See `.claude/NO_EMOJIS_POLICY.md` for complete details.

---

## SINGLE SOURCE OF TRUTH: AGENTS.md

**CRITICAL**: This file is a lightweight reference pointer. ALL detailed instructions, coding standards, deployment procedures, and development guidelines are in:

### ➤ [AGENTS.md](AGENTS.md) - Read This First

**AGENTS.md contains everything you need**:
- Quick Start guide with common user requests
- Critical deployment warnings and procedures
- Code quality standards and refactoring guidelines
- Version management and release workflows
- Testing requirements (Test-Driven Development mandatory)
- Communication guidelines and reference clarity requirements
- Complete pre-commit checklist

---

## Fast Access for Common Tasks

**Use these links for immediate context**:

### Deployment & Releases
- [Deployment Commands Trigger](AGENTS.md#deployment-commands-trigger---read-immediately) - When user says "let's deploy"
- [Critical Deployment Warning](AGENTS.md#critical-deployment-warning) - #1 failure pattern to avoid
- [Version Numbers](AGENTS.md#version-numbers-are-important) - Synchronize ALL version files
- [Pre-Commit Checklist](AGENTS.md#pre-commit-checklist) - Validation before any commit

### Development Workflows
- [Test-Driven Bugfixes](AGENTS.md#test-driven-bugfixes-mandatory) - ALWAYS write failing test first
- [Refactoring Best Practices](AGENTS.md#refactoring-best-practices-critical) - Domain-based module extraction
- [Tool Configuration](AGENTS.md#tool-configuration-consistency-critical-for-new-features) - Adding new AI tools
- [AI Assistant Guidelines](AGENTS.md#ai-assistant-development-guidelines) - Optimal development patterns

### Code Quality
- [Code Quality Rules](AGENTS.md#code-quality-rules) - Clippy, formatting, error handling
- [Commit Messages](AGENTS.md#how-to-write-commit-messages) - Conventional commit format
- [Terminal Commands](AGENTS.md#terminal-command-guidelines) - Single-line commands only

---

## Claude-Specific Optimization Tips

### Leverage Your Strengths

**You excel at**:
- **Context synthesis** - Use the Quick Start table in AGENTS.md to jump to relevant sections quickly
- **Systematic refactoring** - Follow the proven domain-based architecture patterns
- **Incremental validation** - Run `cargo check` after each change, don't batch changes
- **Clear communication** - Always provide descriptive headers for numbered lists
- **Quality gates** - Verify `cargo clippy` and `cargo fmt` pass before suggesting commits

### Optimal Workflow for New Claude

1. **Start with AGENTS.md Quick Start** - Use the table to find relevant sections
2. **Read critical sections first** - NO EMOJIS, deployment warnings, test-driven development
3. **Plan incrementally** - Break large tasks into validatable steps
4. **Validate continuously** - Run checks after each step, not at the end
5. **Document as you go** - Update CHANGELOG.md and external docs during development, not after

### Communication Best Practices

**When providing procedures**:
- Use descriptive headers: "## Deployment Workflow Steps" not just "Steps:"
- Number items only within clearly labeled sections
- Reference by section name: "In the Deployment Workflow section above, step 3..."
- Never use ambiguous references like "step 4" without context

**When user asks for clarification**:
- Quote the specific text being referenced
- State which section/workflow it belongs to
- Provide full context from the conversation

---

## Critical Reminders

### Before Any Action

1. **Check AGENTS.md first** - Don't guess, read the actual guidelines
2. **NO EMOJIS ever** - Use text-based indicators like "[INSTALLED]" / "[AVAILABLE]"
3. **CHANGELOG.md before deployment** - Always update before running scripts
4. **Test-Driven bugfixes** - Write failing test FIRST, then fix
5. **Version synchronization** - Update ALL files: Cargo.toml, package.json, Homebrew Formula

### Quality Gates

Always verify before suggesting commits:
```bash
cargo check                  # Must compile
cargo clippy --all-targets --all-features -- -D warnings  # Must pass
cargo fmt --all             # Must be formatted
cargo test                  # Must pass (if tests exist)
```

---

## Maintaining This File

**When updating agent guidelines**:

1. **Primary updates go to AGENTS.md** - This is the single source of truth
2. **Update reference links here** - Ensure CLAUDE.md points to correct AGENTS.md sections
3. **Keep this file minimal** - Only quick access links and Claude-specific optimization tips
4. **Consistency check** - `.github/copilot-instructions.md` should also reference AGENTS.md

---

## Remember

**AGENTS.md is your complete instruction manual.** This file exists only to:
- Remind you to check AGENTS.md first
- Provide fast access links to common sections
- Share Claude-specific optimization strategies

**When in doubt, read AGENTS.md. When confident, verify against AGENTS.md.**
- don't create docs unless i literally tell you to make them. ever. i know that takes up valuable tokens. so we need to use this wisely. ensure that our documentation-specialist keeps this in mind as well as our agents.md
- leverage our agents whenever possible. if i tell you to not do it, go ahead and listen to me, but for most/general case, go ahead and leverage the agents at our disposal to help accomplish tasks in a more efficient manner.