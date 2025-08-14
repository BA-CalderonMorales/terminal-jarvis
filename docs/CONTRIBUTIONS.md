# Contributing to Terminal Jarvis

**Thank you for your interest in contributing to Terminal Jarvis!**

This project is a unified command center for AI coding tools, and we welcome contributions that help improve the experience for developers working with AI assistants.

## **FIRST STEP: Join Our Discord**

**BEFORE opening any Pull Request**, please join our community Discord:

**[Join Terminal Jarvis Discord](https://discord.gg/zNuyC5uG)**

### Why Discord First?

- **Feature Discussion**: Propose new features in the `#features` channel
- **Bug Reports**: Report bugs and discuss fixes in the `#bugfix` channel
- **Architecture Guidance**: Get feedback on technical approaches
- **Maintainer Coordination**: Avoid duplicate work and align with project goals
- **Community Support**: Connect with other contributors and users

**Discord channels for contributors:**

- `#features` - Discuss new functionality, tool integrations, and enhancements
- `#bugfix` - Report bugs, discuss reproduction steps, and coordinate fixes
- `#development` - Technical discussions, architecture questions, and code reviews
- `#general` - Community chat and project updates

## Contribution Types

We welcome various types of contributions:

### **Code Contributions**

- New AI tool integrations
- Bug fixes and stability improvements
- Performance enhancements
- Session continuation improvements
- Authentication workflow fixes

### **Documentation**

- README improvements
- API documentation
- Installation guides
- Usage examples and tutorials

### **Testing**

- Unit and integration tests
- Bug reproduction test cases
- CI/CD improvements
- Cross-platform testing

### **User Experience**

- Interactive interface improvements
- ASCII art and visual enhancements
- Error message clarity
- Command-line ergonomics

## **Contribution Process**

### 1. **Discord Discussion** (MANDATORY)

- Join the Discord and introduce yourself
- Discuss your planned contribution in the appropriate channel
- Get feedback from maintainers and community
- Ensure your approach aligns with project goals

### 2. **Fork & Branch**

```bash
# Fork the repository on GitHub
git clone https://github.com/your-username/terminal-jarvis.git
cd terminal-jarvis
git checkout develop
git checkout -b feature/your-feature-name
```

### 3. **Follow Development Standards**

- Read [ARCHITECTURE.md](ARCHITECTURE.md) for technical details
- Follow code quality guidelines (see below)
- Write tests FIRST for bug fixes (TDD approach)
- Keep commits focused and well-documented

### 4. **Test Your Changes**

```bash
# Run all tests
cargo test

# Check code formatting
cargo fmt --all

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Test NPM package (if applicable)
cd npm/terminal-jarvis && npm run build
```

### 5. **Create Pull Request**

- Use the provided PR template
- Select appropriate PR type (docs, feature, bugfix, etc.)
- Link Discord discussion
- Include comprehensive testing information

## **Code Quality Standards**

### **Rust Code Requirements**

- **Zero Clippy warnings**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Proper formatting**: `cargo fmt --all`
- **Error handling**: Use `anyhow::Result` for error propagation
- **Documentation**: Add doc comments for public functions
- **No unwrap()**: Use proper error handling instead of `.unwrap()`

### **Test-Driven Development (CRITICAL FOR BUGFIXES)**

**MANDATORY**: Every bugfix MUST follow this process:

1. **Write failing test FIRST** - Reproduce the exact bug behavior
2. **Verify test fails** - Confirm the bug exists
3. **Implement minimal fix** - Make the test pass
4. **Verify all tests pass** - No regressions introduced
5. **Commit test + fix together** - Single commit with both

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bug_specific_issue_description() {
        // Bug: Detailed description of the bug
        // Expected: What should happen instead
        // Test implementation that reproduces the bug
        assert_eq!(expected_behavior, actual_behavior);
    }
}
```

### **Commit Message Standards**

Use conventional commits:

```bash
# Good examples:
git commit -m "feat: add support for new AI tool integration"
git commit -m "fix: resolve session continuation issue with auth workflows"
git commit -m "docs: update installation instructions for Homebrew"
git commit -m "test: add integration tests for tool detection"

# Types: feat, fix, docs, style, refactor, test, chore, break
```

### **What NOT to Do**

- No emojis in commit messages or code
- No vague commits like "fix stuff" or "update things"
- No combining unrelated changes in one commit
- No force pushing to main or develop branches
- No shell scripts in `tests/` directory (use `scripts/` instead)
- No multi-line bash commands in suggestions

## **New AI Tool Integration**

Adding a new AI coding tool requires updating multiple files for consistency:

### **Required Files (ALL MANDATORY):**

1. **`src/tools.rs`**:

   - Add tool to `get_command_mapping()` HashMap
   - Add tool to `get_tool_commands()` Vec with description

2. **`src/services.rs`**:

   - Add display name mapping in `get_display_name_to_config_mapping()`

3. **`terminal-jarvis.toml.example`**:

   - Add tool configuration with install/update commands

4. **Test Updates**:

   - Update `test_display_name_to_config_mapping()` in `src/services.rs`
   - Update `test_config_key_resolution()` in `src/services.rs`

5. **Documentation**:
   - Update README.md tool list
   - Update relevant docs/ files

### **Verification Commands:**

```bash
cargo run -- list                    # Verify tool appears
cargo test --lib services           # Test mappings work
cargo run -- update --help          # Test end-to-end flow
```

## **Project Structure Understanding**

Terminal Jarvis has a specific architecture:

### **Core Components**

- `src/main.rs` - CLI entry point
- `src/cli.rs` - Command definitions (clap)
- `src/cli_logic.rs` - Business logic + session continuation
- `src/tools.rs` - Tool detection + command mapping
- `src/services.rs` - Package management + GitHub services
- `src/config.rs` - TOML configuration system

### **Session Continuation System**

- **Key Innovation**: Prevents users from being kicked out during auth workflows
- **Smart Detection**: Distinguishes internal commands vs intentional exits
- **Location**: `src/tools.rs` → `should_continue_session()` function

### **Multi-Platform Distribution**

Terminal Jarvis distributes via **three channels**:

1. **NPM** - `npm install -g terminal-jarvis`
2. **Crates.io** - `cargo install terminal-jarvis`
3. **Homebrew** - `brew install terminal-jarvis`

**Note**: Contributors cannot publish to these registries - maintainers handle distribution.

## **Important Limitations for Contributors**

### **Distribution Access**

Contributors do NOT have access to:

- NPM registry publishing
- Crates.io publishing
- Homebrew formula updates
- GitHub release creation

**Maintainers handle all publishing and distribution after PR approval.**

### **Version Management**

- Do NOT manually update version numbers
- Maintainers handle version bumps and CHANGELOG.md updates
- Focus on code quality and functionality in your PR

### **Testing Constraints**

- Test locally with development builds
- Cannot test published package distribution
- Focus on unit/integration tests for your changes

## **Effective Contribution Tips**

### **Start Small**

- Begin with documentation improvements or small bug fixes
- Understand the codebase before attempting major features
- Ask questions in Discord - the community is helpful!

### **Focus Areas for New Contributors**

- **Documentation**: README clarity, installation guides, usage examples
- **Testing**: Add test coverage for existing functionality
- **Bug Reports**: Help identify and reproduce issues
- **Tool Integration**: Add support for new AI coding tools
- **User Experience**: Improve interactive interface and error messages

### **Getting Help**

- **Discord**: Real-time help in `#development` channel
- **GitHub Issues**: Browse existing issues for contribution ideas
- **Architecture Guide**: Read [ARCHITECTURE.md](ARCHITECTURE.md) for technical deep dive
- **Testing Guide**: See [TESTING.md](TESTING.md) for testing approaches

## **Recognition**

Contributors are recognized in:

- GitHub contributor graphs
- Release notes for significant contributions
- Discord contributor role
- README acknowledgments (for major contributions)

## **Useful Links**

- **[Discord Community](https://discord.gg/zNuyC5uG)** - Primary communication channel
- **[Architecture Guide](ARCHITECTURE.md)** - Technical deep dive
- **[Testing Guide](TESTING.md)** - Testing strategies and frameworks
- **[Installation Guide](INSTALLATION.md)** - Platform-specific setup
- **[Known Limitations](LIMITATIONS.md)** - Current issues and workarounds

---

**Ready to contribute? Join our Discord community and let's build something amazing together!**
