---
name: Pull Request
about: Create a pull request to contribute to Terminal Jarvis
title: "[TYPE] Brief description of changes"
labels: ""
assignees: ""
---

## Pull Request Checklist

**BEFORE submitting this PR, please ensure:**

- [ ] I have joined the [Terminal Jarvis Discord](https://discord.gg/WteQm6MTZW) and discussed this contribution
- [ ] I have read and understand the [CONTRIBUTIONS.md](docs/CONTRIBUTIONS.md) guidelines
- [ ] All tests pass locally (`cargo test`)
- [ ] Code follows project formatting standards (`cargo fmt --all` and `cargo clippy`)
- [ ] **NO EMOJIS** used anywhere in commits, code, or documentation (per AGENTS.md guidelines)

## PR Type

**Select the PRIMARY type of this PR:**

- [ ] **Documentation** - README, docs/, comments, or other documentation changes
- [ ] **Feature** - New functionality or tool integration
- [ ] **Bugfix** - Fixes a specific bug or issue
- [ ] **Security** - Addresses security vulnerabilities or improves security
- [ ] **UI/UX** - Changes to interactive interface, ASCII art, or user experience
- [ ] **Logic** - Core business logic, algorithms, or architectural changes
- [ ] **Maintenance** - Dependency updates, refactoring, or code cleanup
- [ ] **Testing** - Test additions, improvements, or testing infrastructure

## Description

**What does this PR do?**

<!-- Provide a clear, concise description of your changes -->

**Why is this change needed?**

<!-- Explain the motivation or problem this PR solves -->

## Changes Made

**Files Modified:**

<!-- List the key files you changed and why -->

- `src/example.rs` - Added new feature X
- `docs/EXAMPLE.md` - Updated documentation for feature X
- `tests/example_tests.rs` - Added comprehensive test coverage

**Key Implementation Details:**

<!-- Highlight important technical decisions or approaches -->

## Testing Strategy

**For Bugfixes (MANDATORY):**

- [ ] **Failing test written FIRST** - Test reproduces the exact bug behavior
- [ ] **Test fails initially** - Verified the bug exists with failing test
- [ ] **Fix implemented** - Minimal code changes to make test pass
- [ ] **All tests pass** - No regressions introduced

**For Features:**

- [ ] Unit tests added for new functionality
- [ ] Integration tests cover end-to-end scenarios
- [ ] Edge cases and error conditions tested

**For Documentation:**

- [ ] Links verified and functional
- [ ] Examples tested and working
- [ ] Formatting and style consistent

## Tool Configuration Updates

**If adding new AI tools (check all that apply):**

- [ ] Updated `src/tools.rs` with command mapping
- [ ] Updated `src/services.rs` with display name mapping
- [ ] Updated `terminal-jarvis.toml.example` with tool configuration
- [ ] Updated tests in `src/services.rs`
- [ ] Updated README.md and documentation

## Breaking Changes

- [ ] **No breaking changes**
- [ ] **Contains breaking changes** - WARNING: Requires version bump to X.0.0

**If breaking changes, describe:**

<!-- What will break and how users should migrate -->

## Additional Context

**Related Issues:**

<!-- Link any related GitHub issues or Discord discussions -->

**Screenshots/Logs:**

<!-- For UI changes or bug fixes, include relevant screenshots or logs -->

**Distribution Impact:**

<!-- Note: Contributors cannot publish to npm/crates.io/homebrew - maintainer will handle -->

---

## For Maintainers Only

**Post-Merge Tasks:**

- [ ] Update CHANGELOG.md with version entry
- [ ] Check if CI/CD pipeline should run based on PR type
- [ ] Consider distribution channel impacts (NPM/Crates.io/Homebrew)
- [ ] Update version numbers if needed
- [ ] Create GitHub release if significant feature/fix

**Distribution Checklist:**

- [ ] NPM package needs republishing
- [ ] Crates.io package needs republishing
- [ ] Homebrew formula needs updating
- [ ] Documentation sites need updating
