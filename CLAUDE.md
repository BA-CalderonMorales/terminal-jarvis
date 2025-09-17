# Claude AI Assistant Instructions for Terminal Jarvis

**IMPORTANT**: This file references the unified agent guidelines. All detailed instructions, coding standards, deployment procedures, and development guidelines are maintained in a single source of truth.

## Primary Reference Document

**READ FIRST**: All Claude AI instructions are documented in:

➤ **[AGENTS.md](AGENTS.md)** - Complete AI Assistant Guidelines

This document contains:
- Critical deployment warnings and procedures
- Code quality standards and refactoring guidelines  
- Version management and release workflows
- Testing requirements and development standards
- Communication guidelines and reference clarity requirements

## Quick Reference Links

For immediate access to key sections in AGENTS.md:

- **[Critical Deployment Warning](AGENTS.md#critical-deployment-warning)** - THE #1 deployment failure pattern
- **[Communication Guidelines](AGENTS.md#communication--reference-guidelines)** - Reference clarity requirements
- **[Deployment Commands Trigger](AGENTS.md#-deployment-commands-trigger---read-immediately)** - When users mention deployment
- **[AI Assistant Development Guidelines](AGENTS.md#ai-assistant-development-guidelines)** - Claude-optimized workflows
- **[Refactoring Best Practices](AGENTS.md#refactoring-best-practices-critical)** - Systematic refactoring approach
- **[Test-Driven Bugfixes](AGENTS.md#test-driven-bugfixes-mandatory)** - Mandatory TDD workflow
- **[Pre-Commit Checklist](AGENTS.md#pre-commit-checklist)** - Essential validation steps

## Claude-Specific Development Strengths

### Where Claude Excels

**Claude is particularly strong at**:
- **Systematic refactoring** → Breaking large files into focused domain modules
- **Quality assurance** → Ensuring code passes `cargo clippy`, `cargo fmt`, and tests
- **Documentation accuracy** → Keeping README.md and docs/ synchronized with actual features
- **Error debugging** → Methodically fixing compilation errors one at a time
- **Communication clarity** → Providing specific context when referencing numbered items

### Optimal Claude Workflow

1. **Plan before implementing** → Describe architecture changes clearly
2. **Incremental validation** → Run `cargo check` after each major change
3. **Quality gates** → Always verify `cargo clippy` and `cargo fmt` pass
4. **Documentation sync** → Update docs when adding or changing features
5. **Clear communication** → Always provide context for numbered references

## Claude Communication Requirements

### Reference Clarity (Critical for Claude)

**When providing numbered lists or procedures**:
- Always use descriptive headers: "## Deployment Steps" not just "Steps:"
- Include context in references: "In the deployment workflow above, step 4 refers to..."
- NEVER leave numbered references ambiguous

**When user asks for clarification**:
- Quote the specific text being referenced
- Provide full context of which section/workflow it belongs to
- Explain where that reference appeared in the conversation

## Keep This Reference Updated

When updating agent guidelines:
1. **Primary updates go to AGENTS.md** - Single source of truth
2. **Update reference links** - Ensure this file points to correct sections  
3. **Maintain consistency** - .github/copilot-instructions.md should reference the same AGENTS.md source

---

**Remember**: The complete instructions are in [AGENTS.md](AGENTS.md). This file serves as a Claude-specific entry point to those unified guidelines.
