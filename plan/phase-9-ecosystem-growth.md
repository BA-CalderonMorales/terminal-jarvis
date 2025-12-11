# Phase 9: Ecosystem Growth

**Status**: PENDING  
**Priority**: LOW (but strategic)  
**Estimated Sessions**: 2-3

## The Long Game

Terminal Jarvis's value grows with the number of supported tools. But you can't support every tool yourself. The solution: make it trivial for others to contribute.

## Current Barrier to Contribution

To add a new tool today, a contributor needs to:
1. Understand the config/tools/*.toml format
2. Know where detection logic lives
3. Figure out install command structure
4. Test across platforms
5. Update documentation

That's a lot of context. We can lower this barrier dramatically.

## Tasks

### 1. Tool Template Generator
- [ ] `terminal-jarvis add-tool --wizard`
- [ ] Prompts for: name, command, install method, auth requirements
- [ ] Generates config/tools/newtool.toml automatically
- [ ] Validates the config works

### 2. Tool Validation Command
- [ ] `terminal-jarvis validate-tool newtool`
- [ ] Checks: config syntax, command exists, auth vars defined
- [ ] Runs detection to verify it works
- [ ] Outputs clear pass/fail with fix suggestions

### 3. Contribution Guide
- [ ] CONTRIBUTING.md with step-by-step tool addition
- [ ] Example PR template for new tools
- [ ] List of "wanted" tools for contributors to tackle

### 4. Tool Discovery Feed
- [ ] GitHub Action that checks for new AI CLI tools weekly
- [ ] Creates issues: "New tool detected: [tool-name] - contribute support?"
- [ ] Links to the contribution guide

### 5. Plugin Architecture (Future)
- [ ] Allow tools to be loaded from external repos
- [ ] `terminal-jarvis install-plugin github:user/tj-tool-name`
- [ ] For tools that need custom logic beyond config

## Agent Instructions

Start with the contribution guide (Task 3). It's documentation, zero code risk, and immediately lowers the barrier.

Then build the validator (Task 2) - it's useful for you AND contributors.

The plugin architecture is explicitly "future" - don't build it until there's demand.

## Success Criteria

- [ ] Contributor can add a new tool in < 30 minutes
- [ ] At least 2 community-contributed tools merged
- [ ] No support burden from broken community contributions (validation catches issues)

## Community Tools Wishlist

Good first contributions for community members:
- `cursor` - Cursor editor's CLI
- `continue` - Continue.dev CLI
- `cody` - Sourcegraph Cody CLI  
- `tabby` - Tabby code completion
- `ollama` - Local LLM runner
