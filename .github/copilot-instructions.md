# GitHub Copilot Instructions for Terminal Jarvis

## SINGLE SOURCE OF TRUTH: AGENTS.md

**CRITICAL**: This file is a lightweight reference pointer. ALL detailed instructions, coding standards, deployment procedures, and development guidelines are in:

### ➤ [AGENTS.md](../AGENTS.md) - Read This First

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
- [Deployment Commands Trigger](../AGENTS.md#deployment-commands-trigger---read-immediately) - When user says "let's deploy"
- [Critical Deployment Warning](../AGENTS.md#critical-deployment-warning) - #1 failure pattern to avoid
- [Version Numbers](../AGENTS.md#version-numbers-are-important) - Synchronize ALL version files
- [Pre-Commit Checklist](../AGENTS.md#pre-commit-checklist) - Validation before any commit

### Development Workflows
- [Test-Driven Bugfixes](../AGENTS.md#test-driven-bugfixes-mandatory) - ALWAYS write failing test first
- [Refactoring Best Practices](../AGENTS.md#refactoring-best-practices-critical) - Domain-based module extraction
- [Tool Configuration](../AGENTS.md#tool-configuration-consistency-critical-for-new-features) - Adding new AI tools
- [AI Assistant Guidelines](../AGENTS.md#ai-assistant-development-guidelines) - Optimal development patterns

### Code Quality
- [Code Quality Rules](../AGENTS.md#code-quality-rules) - Clippy, formatting, error handling
- [Commit Messages](../AGENTS.md#how-to-write-commit-messages) - Conventional commit format
- [Terminal Commands](../AGENTS.md#terminal-command-guidelines) - Single-line commands only

---

## GitHub Copilot Optimization Tips

### Leverage Your Strengths

**Copilot excels at**:
- **Code generation** - Use the Quick Start table in AGENTS.md to understand patterns quickly
- **Boilerplate creation** - Follow the established architecture in domain-based modules
- **Test suite development** - Write comprehensive tests with good coverage
- **Pattern following** - Leverage existing project structure for consistency
- **Incremental development** - Build features step by step with validation

### Optimal Workflow for Copilot

1. **Start with AGENTS.md Quick Start** - Use the table to find relevant sections
2. **Understand patterns first** - Look at existing code structure before generating new code
3. **Test-driven approach** - Write tests first, then implement features
4. **Validate incrementally** - Run `cargo check` after each generation
5. **Follow conventions** - Use established naming patterns and module organization

### Code Generation Best Practices

**When generating code**:
- Follow Rust idioms and patterns already in the codebase
- Use the same error handling patterns (Result types, error propagation)
- Match existing formatting and documentation styles
- Place code in appropriate domain modules (cli_logic/, tools/, config/, etc.)
- Generate tests alongside implementation

**Quality gates before suggesting commits**:
```bash
cargo check                  # Must compile
cargo clippy --all-targets --all-features -- -D warnings  # Must pass
cargo fmt --all             # Must be formatted
cargo test                  # Must pass (if tests exist)
```

---

## Critical Reminders

### Before Any Generation

1. **Check AGENTS.md first** - Understand the context and patterns
2. **NO EMOJIS ever** - Use text-based indicators like "[INSTALLED]" / "[AVAILABLE]"
3. **CHANGELOG.md before deployment** - Always update before running scripts
4. **Test-Driven bugfixes** - Write failing test FIRST, then fix
5. **Version synchronization** - Update ALL files: Cargo.toml, package.json, Homebrew Formula

---

## Maintaining This File

**When updating agent guidelines**:

1. **Primary updates go to AGENTS.md** - This is the single source of truth
2. **Update reference links here** - Ensure copilot-instructions.md points to correct AGENTS.md sections
3. **Keep this file minimal** - Only quick access links and Copilot-specific optimization tips
4. **Consistency check** - `CLAUDE.md` should also reference AGENTS.md

---

## Remember

**AGENTS.md is your complete instruction manual.** This file exists only to:
- Remind you to check AGENTS.md first
- Provide fast access links to common sections
- Share GitHub Copilot-specific optimization strategies

**When in doubt, read AGENTS.md. When confident, verify against AGENTS.md.**
