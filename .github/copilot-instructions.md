# Terminal Jarvis - AI Coding Tools Wrapper

## CRITICAL NO-EMOJIS RULE

**ABSOLUTE REQUIREMENT**: NO EMOJIS anywhere in the codebase, commits, documentation, or any output.

**FORBIDDEN**: Any use of emojis in:
- Commit messages
- Code comments
- Documentation files 
- README content
- GitHub releases
- Terminal output
- Log messages
- Error messages

**REASON**: Professional appearance and accessibility. Emojis create visual clutter and accessibility issues.

## CRITICAL DEPLOYMENT WARNING

**THE #1 DEPLOYMENT FAILURE**: Homebrew Formula changes committed AFTER GitHub release creation.

**NEVER DO THIS**:

1. Create GitHub release first
2. Then commit Homebrew Formula changes later

**ALWAYS DO THIS**:

1. Update and commit ALL changes (including Homebrew Formula)
2. Push changes to GitHub
3. THEN create GitHub release

**This prevents broken Homebrew installations where Formula URLs don't match release assets.**

## Communication & Reference Guidelines

### Reference Clarity Requirements (CRITICAL)

**When providing numbered lists, steps, or procedures**:

- ALWAYS use descriptive section headers: "## Deployment Steps" not just "Steps:"
- Include context in references: "In the deployment workflow above, step 4 refers to..."
- NEVER leave numbered references ambiguous

**When user asks for clarification about numbered items**:

- Quote the specific text being referenced
- Provide full context of which section/workflow the number belongs to
- Explain where that reference appeared in the conversation

**ANTI-PATTERN to avoid**:

```
Steps:
1. Do this
2. Do that
3. Another thing
4. Final step <- User asks "what's step 4?" - ambiguous!
```

**CORRECT PATTERN**:

```
## Homebrew Release Workflow
1. Update Formula version
2. Commit all changes
3. Create GitHub release
4. Verify archive accessibility <- Clear context when referenced
```

**Response Requirements when user asks "what do you mean by step X"**:

1. Acknowledge the ambiguity apologetically
2. Quote the specific numbered list being referenced
3. Explain which procedure/workflow it belongs to
4. Provide the clear answer to their question

## What This Project Does

Terminal Jarvis is a thin Rust wrapper that provides a unified interface for managing and running AI coding tools like claude-code, gemini-cli, qwen-code, opencode, llxprt, and codex. Think of it as a package manager and runner for AI coding assistants.

Key innovation: **Session Continuation System** prevents users from being kicked out of tools during authentication workflows.

The project follows Orhun Parmaksız's approach for packaging Rust applications via NPM, making it easy to install with `npm install -g terminal-jarvis`.

## Current Version & Key Features

**Version**: 0.0.48
**Major Features**:

- **Multi-Platform Distribution** (v0.0.46+) - NPM, Crates.io, and Homebrew publishing
- Session Continuation System (v0.0.44+) - Intelligent handling of authentication workflows
- Enhanced Deployment Workflow (v0.0.45+) - Programmatic version management
- 6 AI Tools Integration - claude, gemini, qwen, opencode, llxprt, codex
- Infinite Loop Prevention (v0.0.45) - Smart detection of exit vs internal commands

## Distribution Channels

Terminal Jarvis is available through **three official distribution channels**:

1. **NPM** (Node.js ecosystem): `npm install -g terminal-jarvis`
2. **Crates.io** (Rust ecosystem): `cargo install terminal-jarvis`
3. **Homebrew** (macOS/Linux package manager): `brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis`

## How The Code Is Organized

The repository has two main parts:

**Rust Application** (`/src/`):

- `main.rs` - Entry point that starts the CLI
- `cli.rs` - Command definitions using clap (run, update, list, info, templates)
- `cli_logic.rs` - The actual business logic for each command + session continuation system
- `tools.rs` - Tool detection, command mapping + session continuation logic with intelligent restart
- `services.rs` - PackageService and GitHubService for managing tools
- `config.rs` - TOML configuration management
- `api.rs`, `api_client.rs`, `api_base.rs` - Future API framework (currently unused)

**NPM Package** (`/npm/terminal-jarvis/`):

- `src/index.ts` - Simple TypeScript wrapper that calls the Rust binary
- `package.json` - NPM package configuration
- `biome.json` - Biome linting configuration (we use Biome, not ESLint)

## 🚨 DEPLOYMENT COMMANDS TRIGGER - READ IMMEDIATELY

**When users say "Let's run local-cd.sh" or mention deployment:**

## GitHub Copilot Deployment Guide

### Standard Release Workflow

```bash
# 1. Pre-flight checks (MANDATORY)
git status                                    # Must show "working tree clean" 
./scripts/cicd/local-cd.sh --check-versions  # Must show "All versions synchronized"

# 2. Update CHANGELOG.md with new version entry (REQUIRED FIRST)
# Format: ## [X.X.X] - YYYY-MM-DD

# 3. Execute deployment pipeline
./scripts/cicd/local-ci.sh      # Validate (no commits)
./scripts/cicd/local-cd.sh      # Deploy (commits, tags, pushes)
```

### Version Management Workflow

```bash
# 1. Determine version increment
# Bug fixes/docs: 0.0.X+1 | New features: 0.X+1.0 | Breaking: X+1.0.0

# 2. Update CHANGELOG.md entry for target version

# 3. Synchronize all version files
./scripts/cicd/local-cd.sh --update-version 0.0.48

# 4. Verify synchronization and deploy
./scripts/cicd/local-cd.sh --check-versions
./scripts/cicd/local-ci.sh && ./scripts/cicd/local-cd.sh
```

### Multi-Platform Distribution

```bash
# 1. Complete standard deployment first

# 2. Generate Homebrew release archives
./scripts/utils/generate-homebrew-release.sh --stage
git add homebrew/release/ && git commit -m "feat: Homebrew archives v0.0.48" && git push

# 3. Create GitHub release with assets
gh release create v0.0.48 \
  homebrew/release/terminal-jarvis-mac.tar.gz \
  homebrew/release/terminal-jarvis-linux.tar.gz \
  --title "Release v0.0.48" --notes "Release notes" --latest

# 4. Verify Homebrew installation works
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.48/terminal-jarvis-mac.tar.gz
```

## Critical Deployment Requirements

### Pre-Deployment Validation (NON-NEGOTIABLE)

- [ ] **Working Tree Clean**: `git status` shows no uncommitted changes
- [ ] **Version Sync**: `./scripts/cicd/local-cd.sh --check-versions` passes
- [ ] **CHANGELOG.md Updated**: New version entry added BEFORE running scripts
- [ ] **Homebrew Formula**: Version matches if manually edited

### Deployment Failure Prevention

**Most Common Failures**:
1. **Uncommitted changes** → Always check `git status` first
2. **Version mismatches** → Always run `--check-versions` first
3. **Missing changelog** → Always update CHANGELOG.md before scripts
4. **Formula after release** → Always commit Formula before GitHub release

### Quick Recovery Commands

```bash
# Fix version synchronization
./scripts/cicd/local-cd.sh --update-version X.X.X

# Fix Homebrew Formula manually
sed -i 's/version ".*"/version "X.X.X"/' homebrew/Formula/terminal-jarvis.rb

# Verify all changes committed
git status && git log -1 --name-only
```

## GitHub Copilot Development Guidelines

### Working with GitHub Copilot

**GitHub Copilot excels at**:
- **Code generation**: Creating boilerplate and implementing well-defined patterns
- **Test creation**: Writing comprehensive test suites with good coverage
- **Documentation consistency**: Ensuring technical docs match implementation
- **Incremental development**: Building features step-by-step with validation

**Optimal Copilot workflow**:
1. **Clear specifications**: Provide detailed requirements for what you want to build
2. **Pattern following**: Leverage existing project patterns for consistency
3. **Test-driven development**: Write tests first, then implement features
4. **Continuous validation**: Verify each step compiles and passes tests

**Copilot's code generation strength**: Use Copilot for implementing well-defined features, creating test suites, and generating documentation that matches actual code behavior.

### Copilot-Optimized Quality Checks

```bash
# Copilot-assisted development verification
cargo check                          # Verify code compiles
cargo test                          # Run full test suite
cargo clippy --all-targets --all-features -- -D warnings  # Code quality
cargo fmt --all                     # Code formatting

# Integration testing
cargo run -- list                   # Verify CLI works
cargo run -- --help                 # Verify help text
```

## Version Numbers Are Important

We use semantic versioning with **NO EMOJIS** and **NO DECORATIONS**. Just clean version numbers:

- `0.0.1` - Bug fixes, docs, small improvements
- `0.1.0` - New features that don't break existing functionality
- `1.0.0` - Breaking changes that require users to update their code

🚨 **CRITICAL VERSION SYNCHRONIZATION REQUIREMENT**:
Always update **ALL THREE** version files simultaneously:

- `Cargo.toml`
- `npm/terminal-jarvis/package.json`
- **`homebrew/Formula/terminal-jarvis.rb`** ⚠️ COMMONLY FORGOTTEN!

**Homebrew Formula version MUST match exactly** - This is frequently overlooked and causes deployment failures.

## CHANGELOG.md Management (CRITICAL)

**ALWAYS update CHANGELOG.md BEFORE running deployment scripts** - This prevents version confusion and ensures proper feature attribution.

### Changelog Structure Rules:

```bash
## [X.X.X] - YYYY-MM-DD
### Added
- New features that users can see
### Enhanced
- Improvements to existing features  
### Fixed
- Bug fixes and corrections
### Technical
- Internal changes (refactoring, tests)
```

**Rules**:
1. **One Release = One Complete Feature Set**: Each version should represent a cohesive set of features completed together
2. **Update CHANGELOG.md FIRST**: Before running `local-cd.sh`, always add the changelog entry for the version you're about to release
3. **Match Actual Work Timeline**: Don't mix features from different development sessions into the same version entry

## Quick Command Reference

```bash
# Version management
./scripts/cicd/local-cd.sh --check-versions
./scripts/cicd/local-cd.sh --update-version X.X.X

# Deployment pipeline  
./scripts/cicd/local-ci.sh && ./scripts/cicd/local-cd.sh

# Homebrew release
./scripts/utils/generate-homebrew-release.sh --stage
gh release create vX.X.X homebrew/release/*.tar.gz --title "vX.X.X" --notes "..." --latest
```

### Changelog Structure Rules:

1. **One Release = One Complete Feature Set**: Each version should represent a cohesive set of features completed together
2. **Update CHANGELOG.md FIRST**: Before running `local-cd.sh`, always add the changelog entry for the version you're about to release
3. **Match Actual Work Timeline**: Don't mix features from different development sessions into the same version entry
4. **Use Clear Section Headers**: `### Added`, `### Enhanced`, `### Fixed`, `### Technical`

### Version Bumping Guidelines:

- **Feature completion**: When a major feature (like Homebrew integration) is complete, increment version
- **Bug fixes only**: Use patch version increments (0.0.X)
- **New major features**: Use minor version increments (0.X.0)
- **Breaking changes**: Use major version increments (X.0.0)

### Example Workflow:

```bash
# 1. FIRST: Update CHANGELOG.md with new version entry
## [0.0.47] - 2025-08-09
### Added
- **Homebrew Integration**: Complete multi-platform distribution system
- **Testing Infrastructure**: Comprehensive validation scripts

# 2. THEN: Run deployment
./scripts/cicd/local-cd.sh
```

### Common Pitfalls to Avoid:

- **DON'T**: Put all work into one massive changelog entry
- **DON'T**: Update changelog after running deployment (creates version confusion)
- **DON'T**: Mix unrelated features from different development sessions
- **DO**: Create changelog entries that reflect actual development timeline
- **DO**: Keep changelog entries focused on user-visible improvements

## How To Write Commit Messages

Keep them simple and clear:

```
fix: resolve clippy warnings in api module
feat: add support for qwen-code tool
break: change cli argument structure for templates command
docs: update installation instructions
```

Types to use: `fix`, `feat`, `break`, `docs`, `style`, `refactor`, `test`, `chore`

## Code Quality Rules

**Rust Code:**

- Must pass `cargo clippy --all-targets --all-features -- -D warnings`
- Must be formatted with `cargo fmt --all`
- Use `anyhow::Result` for error handling
- Add doc comments for public functions

**TypeScript Code:**

- Use Biome for linting and formatting, NOT ESLint
- Run `npm run lint` and `npm run format` before committing

## Refactoring Best Practices (CRITICAL)

**OBJECTIVE**: Break large files (>200 lines) into focused domain modules while maintaining functionality.

### **Proven Architecture Pattern**

Based on successful cli_logic.rs refactoring (1,358 lines → 10 focused modules):

**Domain-Based Folder Structure**:
```
src/
  large_module.rs (684 lines) →
  large_module/
    mod.rs                    # Re-exports + minimal coordination
    large_module_domain1.rs   # Focused functionality
    large_module_domain2.rs   # Focused functionality
    large_module_domain3.rs   # Focused functionality
```

**Naming Convention**: `{module}_domain_operations.rs` (e.g., `cli_logic_tool_execution.rs`)

### **Dead Code Elimination Protocol**

**ABSOLUTE REQUIREMENT**: Zero tolerance for dead code warnings.

**Process**:
1. **Identify**: Run `cargo check` to find unused function warnings
2. **Verify**: Search codebase for actual usage with `grep -r "function_name("`
3. **Remove**: Delete completely unused functions (prefer deletion over `#[allow(dead_code)]`)
4. **Clean imports**: Remove unused `use` statements
5. **Validate**: `cargo check` must show zero warnings

**Recent Success**: Eliminated 14 dead code warnings by removing 260+ lines of unused functions.

### **Refactoring Workflow**

**MANDATORY STEPS**:
1. **Before**: `cargo check` - baseline compilation
2. **Plan**: Domain separation strategy
3. **Implement**: Extract related functions into focused modules
4. **Clean**: Remove dead code aggressively
5. **Validate**: `cargo check` + `cargo clippy` + `cargo fmt` must all pass
6. **Document**: Update REFACTOR.md with metrics

**Critical**: Never proceed to next refactoring until current one compiles cleanly.

### **Quality Gates**

**Post-refactoring requirements**:
- [ ] `cargo check` - zero warnings
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` - passes
- [ ] `cargo fmt --all` - applied
- [ ] REFACTOR.md updated with results
- [ ] Total line reduction calculated and documented

## File Sync Requirements

The README.md needs to be the same in both the root directory and `npm/terminal-jarvis/`. Before publishing to NPM, always run:

```bash
cd npm/terminal-jarvis
npm run sync-readme
```

## What Not To Do

- No emojis anywhere (commits, code, documentation)
- No vague commit messages like "fix stuff" or "update things"
- No combining unrelated changes in one commit
- No force pushing to main or develop branches
- No `.unwrap()` without good error handling
- No magic numbers - use named constants
- **No shell scripts (.sh files) in tests/ directory** - Only Rust test files (.rs) belong in tests/
- **All shell scripts organized in scripts/ directory structure**:
  - `scripts/cicd/` - CI/CD automation (local-cd.sh, local-ci.sh)
  - `scripts/tests/` - Testing and validation scripts  
  - `scripts/utils/` - Utility scripts (generate-readme-tools.sh, etc.)
- **No multi-line bash commands in terminal suggestions** - Always use single-line commands
- **NO BUGFIXES WITHOUT FAILING TESTS** - Every bug must have a failing test written first

## Terminal Command Guidelines

When suggesting terminal commands or using the `run_in_terminal` tool:

- **ALWAYS use single-line commands** - Multi-line bash commands cause terminal input issues
- Use `&&` to chain commands instead of separate lines
- Use `;` for sequential execution when `&&` isn't appropriate
- Wrap complex logic in parentheses if needed
- Example: `cargo build --release && cd npm/terminal-jarvis && npm run build`
- **NEVER** suggest commands like:
  ```bash
  if [ condition ]; then
    command1
    command2
  fi
  ```
- **INSTEAD** use: `[ condition ] && command1 && command2`

## Tool Configuration Consistency (CRITICAL FOR NEW FEATURES)

**When adding new AI coding tools**, these files MUST be updated together to prevent "Tool not found in configuration" errors:

### Required File Updates (ALL MANDATORY):

1. **`src/tools.rs`**:

   - Add tool to `get_command_mapping()` HashMap
   - Add tool to `get_tool_commands()` Vec with proper description
   - Example: `mapping.insert("newtool", "newtool-cli");`

2. **`src/services.rs`**:

   - Add display name mapping in `get_display_name_to_config_mapping()`
   - Example: `mapping.insert("newtool", "newtool-cli");`
   - **CRITICAL**: This mapping is what connects the CLI display name to the config file key

3. **`terminal-jarvis.toml.example`**:

   - Add tool configuration with install/update commands
   - Example: `newtool-cli = { enabled = true, auto_update = true, install_command = "npm install -g newtool-cli", update_command = "npm update -g newtool-cli" }`

4. **Test Updates**:

   - Update `test_display_name_to_config_mapping()` in `src/services.rs`
   - Update `test_config_key_resolution()` in `src/services.rs`
   - Add assertions for the new tool mapping

5. **Documentation**:
   - Update README.md tool list and descriptions
   - Update CLI help text and package descriptions
   - Update any relevant docs/ files

### Common Failure Pattern:

Adding a tool to `tools.rs` and config file but **forgetting the mapping in `services.rs`**. This causes the update system to fail with "Tool not found in configuration" because it can't translate the display name to the config key.

### Verification Commands:

```bash
# Verify all tools are listed correctly
cargo run -- list

# Test services module mappings
cargo test --lib services

# Test end-to-end functionality
cargo run -- update --help
```

### Why This Matters:

- **Prevents runtime failures**: Missing mappings cause user-facing errors
- **Maintains consistency**: All parts of the system stay synchronized
- **Enables proper testing**: Tests catch mapping issues before release
- **Improves user experience**: Users can update all tools without errors

## Test-Driven Bugfixes (MANDATORY)

**CRITICAL REQUIREMENT**: Every bugfix session MUST follow Test-Driven Development:

### Bugfix Workflow (NON-NEGOTIABLE):

1. **Write Failing Test FIRST**:

   - Create a test that reproduces the exact bug behavior
   - Test MUST fail initially (proving the bug exists)
   - Use descriptive names: `test_bug_opencode_input_focus_on_fresh_install`
   - Include detailed comments explaining the bug scenario

2. **Test Placement**:

   - **Integration tests**: `tests/` directory as `.rs` files
   - **Unit tests**: `src/` files using `#[cfg(test)]` mod test blocks
   - **NEVER** put shell scripts in `tests/` directory

3. **Verify Failure**: Run `cargo test` to confirm the test fails for expected reasons

4. **Implement Fix**: Make minimal code changes to make test pass

5. **Verify Success**: Test passes, no regressions in existing tests

6. **Commit Together**: Include both test and fix in same commit

### Example Test Structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bug_opencode_input_focus_on_fresh_install() {
        // Bug: opencode input box lacks focus on fresh installs
        // User cannot type directly without manual focus intervention
        // Expected: Input box should be automatically focused on startup

        // Test implementation reproducing the bug
        assert_eq!(expected_behavior, actual_behavior);
    }
}
```

### Why This Matters:

- **Prevents regression**: Test ensures bug never returns
- **Documents behavior**: Test serves as living documentation
- **Validates fix**: Proves the fix actually works
- **Quality assurance**: Maintains high code quality standards

**ABSOLUTE RULE**: No bugfix commits without accompanying failing test. This is enforced during code review.

## Homebrew Integration & Multi-Platform Distribution (v0.0.46+)

**Key Innovation**: Complete multi-platform distribution pipeline supporting NPM, Crates.io, and Homebrew.

### Homebrew Publishing Workflow

**Based on Federico Terzi's approach**: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

#### 1. **Release Archive Creation**

```bash
# Create platform-specific archives for Homebrew
# Homebrew release creation integrated into CI/CD pipeline
```

This script:

- Builds release binaries for macOS and Linux
- Creates .tar.gz archives with proper naming (`terminal-jarvis-{platform}.tar.gz`)
- Calculates SHA256 checksums for Formula verification
- Generates Formula template with multi-platform support

#### 2. **Formula Structure** (`homebrew/Formula/terminal-jarvis.rb`)

```ruby
class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  version "X.X.X"
  license "MIT"

  on_macos do
    url "https://github.com/.../terminal-jarvis-macos.tar.gz"
    sha256 "..."
  end

  on_linux do
    url "https://github.com/.../terminal-jarvis-linux.tar.gz"
    sha256 "..."
  end

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
```

#### 3. **End-to-End Testing Protocol**

**CRITICAL**: Always test Homebrew integration locally before deployment:

```bash
# 1. Test Formula validation
# Homebrew formula testing integrated into CI/CD pipeline

# 2. Create local tap for testing
mkdir -p /tmp/homebrew-test-tap/Formula
cp homebrew/Formula/terminal-jarvis.rb /tmp/homebrew-test-tap/Formula/
cd /tmp/homebrew-test-tap && git init && git add . && git commit -m "Test"

# 3. Add tap and test installation
brew tap-new local/test && cp Formula/* $(brew --repository)/Library/Taps/local/homebrew-test/Formula/
brew install local/test/terminal-jarvis

# 4. Verify functionality
terminal-jarvis --version
terminal-jarvis --help
brew test local/test/terminal-jarvis
```

#### 4. **GitHub Release Integration**

Archives must be uploaded to GitHub releases:

- `terminal-jarvis-macos.tar.gz`
- `terminal-jarvis-linux.tar.gz`

Formula URLs point to these release assets.

### Multi-Platform Distribution Best Practices

#### **Distribution Channel Separation**

1. **NPM Users** - JavaScript/Node.js ecosystem
2. **Crates.io Users** - Rust developers who prefer `cargo install`
3. **Homebrew Users** - macOS/Linux users preferring system package managers

#### **README Badge Organization**

Group badges by distribution channel for clarity:

```markdown
<!-- NPM Distribution -->

[![npm version](badge-url)](link) [![npm downloads](badge-url)](link)

<!-- Crates.io Distribution -->

[![Crates.io](badge-url)](link) [![Crates.io downloads](badge-url)](link)

<!-- GitHub Stats -->

[![GitHub release](badge-url)](link) [![GitHub stars](badge-url)](link)
```

#### **Version Synchronization Requirements**

ALL distribution channels must maintain version synchronization:

- `Cargo.toml` - Core Rust package version
- `npm/terminal-jarvis/package.json` - NPM package version
- `homebrew/Formula/terminal-jarvis.rb` - Homebrew Formula version
- GitHub release tags - Must match exactly

### Common Homebrew Integration Pitfalls

#### **Archive Naming Issues**

- **Problem**: Inconsistent archive naming breaks Formula URLs
- **Solution**: Use standardized names: `terminal-jarvis-{macos|linux}.tar.gz`

#### **SHA256 Mismatch**

- **Problem**: Formula SHA256 doesn't match actual archive checksum
- **Solution**: Always regenerate SHA256 after creating new archives

#### **Formula Syntax Errors**

- **Problem**: Ruby syntax errors prevent Formula loading
- **Solution**: Use `# Homebrew formula testing integrated into CI/CD pipeline` for validation

#### **Binary Permissions**

- **Problem**: Extracted binary lacks execute permissions
- **Solution**: Archives must preserve file permissions (`tar -czf` with proper flags)

#### **Cross-Platform Issues**

- **Problem**: Formula doesn't handle macOS vs Linux differences
- **Solution**: Use `on_macos` and `on_linux` blocks for platform-specific handling

### Homebrew Testing Without GitHub Repository

**Local Testing Strategy** (when GitHub repo doesn't exist yet):

1. **Create Local Tap Structure**:

   ```bash
   mkdir -p /tmp/homebrew-test-tap/Formula
   cp homebrew/Formula/terminal-jarvis.rb /tmp/homebrew-test-tap/Formula/
   cd /tmp/homebrew-test-tap && git init && git add . && git commit -m "Test"
   ```

2. **Serve Archives Locally**:

   ```bash
   cd homebrew/release && python3 -m http.server 8000
   ```

3. **Modify Formula for Testing**:

   ```ruby
   # Replace GitHub URLs with localhost for testing
   url "http://localhost:8000/terminal-jarvis-linux.tar.gz"
   ```

4. **Install and Test**:
   ```bash
   brew tap local/test /tmp/homebrew-test-tap
   brew install local/test/terminal-jarvis
   ```

This approach validates the complete installation workflow without requiring actual GitHub repository creation.

## Session Continuation System (v0.0.44+)

**Key Innovation**: Prevents users from being kicked out of AI tools during authentication workflows.

### How It Works:

1. **Smart Command Detection**: Distinguishes between internal commands and intentional exits

   - **Internal commands trigger restart**: `/auth`, `/login`, `/config`, `/setup`
   - **Exit commands terminate properly**: `/exit`, `/quit`, `/bye`
   - **Quick completions return to menu**: Prevents false positive restarts

2. **Session Flow**:

   ```
   User runs tool → Tool exits → Check last input → Decision:
   ├── Internal command → Restart tool seamlessly
   ├── Exit command → Return to Terminal Jarvis menu
   └── Quick completion → Return to menu (prevent false positives)
   ```

3. **Infinite Loop Prevention (v0.0.45)**:
   - **Problem**: Exit commands were incorrectly triggering session continuation
   - **Solution**: Explicit exclusion of exit commands from restart logic
   - **Result**: Exit commands now properly terminate and return to interface

### Debugging Session Issues:

```bash
# Debug session continuation
RUST_LOG=debug cargo run -- run claude

# Test session continuation logic
cargo test --lib tools -- session_continuation

# Check command mapping
cargo run -- list
```

**Code Location**: `src/tools.rs` → `should_continue_session()` function

**ABSOLUTE RULE**: No bugfix commits without accompanying failing test. This is enforced during code review.

## Enhanced Deployment Workflow

### Optimal CI/CD Process (v0.0.45+)

We use a controlled deployment approach with programmatic version management:

**Phase 1: Development & Validation**

```bash
# Check version synchronization
./scripts/cicd/local-cd.sh --check-versions

# Update version programmatically (if needed)
./scripts/cicd/local-cd.sh --update-version X.X.X

# Validate with CI (no commits/pushes)
./scripts/cicd/local-ci.sh
```

**Phase 2: Documentation (MANDATORY)**

- Update CHANGELOG.md with version entry and detailed changes
- Review docs/ directory for consistency
- Update README.md to reflect any documentation changes

**Phase 3: Deployment**

```bash
# Deploy changes (commit/tag/push)
./scripts/cicd/local-cd.sh

# Homebrew archives and Formula preparation
# Homebrew release creation integrated into CI/CD pipeline
# Upload archives to GitHub releases manually

# Manual NPM publishing (due to 2FA)
cd npm/terminal-jarvis && npm publish
npm dist-tag add terminal-jarvis@X.X.X stable  # optional
```

**Key Benefits:**

- **Controlled workflow**: Separate validation from deployment
- **Programmatic version management**: No manual file editing
- **Version synchronization**: Automated consistency across all files
- **Multi-platform support**: Includes Homebrew archive creation
- **Flexibility**: Choose between programmatic and one-shot approaches

## How To Release

**Recommended: Use Enhanced Deployment Workflow (see section above for details)**

1. **FIRST: Update CHANGELOG.md** - Add entry for current version with clear change descriptions
2. **MANDATORY: Review docs/ Directory** - **EVERY TIME** CHANGELOG.md is modified, you **MUST** review and update docs/
   - Check docs/ARCHITECTURE.md, docs/INSTALLATION.md, docs/TESTING.md, docs/LIMITATIONS.md
   - Update version references in docs/ files if version was bumped
   - Verify new features/fixes are properly documented
   - This is **NON-NEGOTIABLE** - no CHANGELOG.md updates without docs/ review
   - **ALSO MANDATORY: Review and update README.md** - Required when CHANGELOG.md or docs/ are modified
3. **THEN: Use Enhanced Workflow** - `./scripts/cicd/local-cd.sh --check-versions`, validation with `./scripts/cicd/local-ci.sh`, then deployment with `./scripts/cicd/local-cd.sh`

**Legacy One-Shot: Use Local CI/CD Script**

1. **Update CHANGELOG.md** - Add entry with clear change descriptions
2. **Run automated script** - `./scripts/local-cicd.sh` (handles all steps below automatically)

**Manual Release Process (if needed):**

1. Update version numbers in both `Cargo.toml` and `npm/terminal-jarvis/package.json`
2. Update version display in `npm/terminal-jarvis/src/index.ts`
3. Update version display in `src/cli_logic.rs` (interactive mode version)
4. **Update CHANGELOG.md with new version and changes** (CRITICAL - must be done first)
5. **MANDATORY: Review and update docs/ directory** - Required whenever CHANGELOG.md is modified
6. **MANDATORY: Review and update README.md** - Required when CHANGELOG.md or docs/ are modified
7. Update version references in README.md (root and NPM package will sync automatically)
8. Run `npm run sync-readme` to sync the README
9. Commit with clear message: `feat: add new feature X`
10. Create tag: `git tag v0.0.6`
11. Push to GitHub: `git push origin develop --tags`
12. Publish to NPM: `cd npm/terminal-jarvis && npm publish`
13. **Add Distribution Tags** (optional - choose one or both):
    - For stable releases: `npm dist-tag add terminal-jarvis@X.X.X stable`
    - For beta releases: `npm dist-tag add terminal-jarvis@X.X.X beta`

**Note: The local-cicd.sh script automates steps 1-11 and includes additional quality checks.**

## NPM Distribution Tags

We use npm dist-tags to provide users with different release channels:

- **latest** - Default npm tag for the most recently published version
- **stable** - For production-ready, thoroughly tested versions
- **beta** - For preview versions that may contain experimental features

**Usage Examples:**

```bash
# Install latest version (default)
npm install -g terminal-jarvis

# Install stable version (recommended for production)
npm install -g terminal-jarvis@stable

# Install beta version (for testing new features)
npm install -g terminal-jarvis@beta

# Check current dist-tags
npm dist-tag ls terminal-jarvis
```

**Best Practices:**

- Use `stable` tag for releases that have been tested and are production-ready
- Use `beta` tag for releases with new features that need user testing
- A single version can have both tags if it serves both purposes
- Always update tags after publishing a new version

## Pre-Commit Checklist

**ALWAYS** verify these items before making any commit:

### 🚨 **CRITICAL: Homebrew Formula Deployment Order**

**THE #1 DEPLOYMENT FAILURE**: Homebrew Formula changes not committed before GitHub release creation.

**MANDATORY SEQUENCE** (NEVER deviate from this order):

1. **FIRST**: Update and commit ALL changes (including Homebrew Formula)
2. **SECOND**: Push changes to GitHub
3. **THIRD**: Create GitHub release with archives
4. **NEVER**: Create release first, then try to fix Formula later

**VERIFICATION COMMANDS** (run BEFORE creating any GitHub release):

```bash
git status                          # MUST show "nothing to commit, working tree clean"
git log -1 --name-only             # MUST include homebrew/Formula/terminal-jarvis.rb
./scripts/cicd/local-cd.sh --check-versions  # MUST show "All versions are synchronized"
```

### Version Consistency Check:

- [ ] **CHANGELOG.md updated FIRST** - MANDATORY before running local-cd.sh
- [ ] Changelog entry matches actual development session work
- [ ] Version increment appropriate for changes made
- [ ] `Cargo.toml` version updated
- [ ] `npm/terminal-jarvis/package.json` version updated
- [ ] **🚨 CRITICAL: `homebrew/Formula/terminal-jarvis.rb` version updated** - COMMONLY FORGOTTEN!
- [ ] `npm/terminal-jarvis/src/index.ts` version display updated
- [ ] `npm/terminal-jarvis/package.json` postinstall script version updated
- [ ] `src/cli_logic.rs` uses `env!("CARGO_PKG_VERSION")` (auto-updates)
- [ ] `README.md` version references updated in note section
- [ ] **Version synchronization verified**: `./scripts/cicd/local-cd.sh --check-versions` passes
- [ ] **🚨 CRITICAL: Working tree clean BEFORE release**: `git status` shows "nothing to commit"

### Documentation Updates:

- [ ] **README.md Accuracy Review** - CRITICAL for user experience
  - [ ] Core functionality descriptions are current and accurate
  - [ ] No misleading information about features or capabilities
  - [ ] Installation instructions reflect all distribution channels (NPM, Crates.io, Homebrew)
  - [ ] Version references are consistent throughout
  - [ ] Examples work with current version and feature set
  - [ ] Package size information updated if binary changed
  - [ ] Badge organization matches current distribution channels
- [ ] Documentation files synchronized (npm/terminal-jarvis/README.md matches root)
- [ ] All referenced features actually exist in current codebase

### Quality Checks:

- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo fmt --all` applied
- [ ] `cargo test` passes (if tests exist)
- [ ] **Failing test added for bugfixes** - If this is a bugfix, verify failing test was created first
- [ ] NPM package builds: `cd npm/terminal-jarvis && npm run build`

### Tool Configuration Consistency (if adding new tools):

- [ ] Tool added to `src/tools.rs` command mapping and tool commands
- [ ] Tool added to `src/services.rs` display name mapping
- [ ] Tool configuration added to `terminal-jarvis.toml.example`
- [ ] Tests updated in `services.rs` for new tool mapping
- [ ] Documentation updated (README.md, CLI descriptions)
- [ ] Verification commands run successfully (`cargo run -- list`, `cargo test --lib services`)

### Homebrew Integration (if updating version):

- [ ] `homebrew/Formula/terminal-jarvis.rb` version updated
- [ ] GitHub release created with version tag
- [ ] Homebrew archives uploaded: `terminal-jarvis-macos.tar.gz`, `terminal-jarvis-linux.tar.gz`
- [ ] SHA256 checksums verified in Formula match actual archives
- [ ] **Homebrew Formula tested locally**: `# Homebrew formula testing integrated into CI/CD pipeline` passes
- [ ] Multi-platform support verified (macOS and Linux archives)

### Testing (Critical):

- [ ] Local package testing in `/tmp` environment completed
- [ ] NPX functionality verified (`npx terminal-jarvis` works)
- [ ] Binary permissions and execution tested
- [ ] Postinstall scripts validated
- [ ] **Enhanced workflow tested**: `./scripts/cicd/local-ci.sh` passes validation

**Never commit without completing the full checklist!**

## Optimal Development Environment

This workspace is designed for optimal AI-assisted development with the following principles:

### Controlled Deployment Workflow (Preferred)

**Use the enhanced workflow instead of one-shot deployments:**

1. **Version Management**: `./scripts/cicd/local-cd.sh --check-versions` and `--update-version`
2. **Validation**: `./scripts/cicd/local-ci.sh` for safe testing without commits
3. **Documentation**: Manual CHANGELOG.md and docs/ updates with quality review
4. **Deployment**: `./scripts/cicd/local-cd.sh` for controlled Git operations
5. **Publishing**: Manual NPM publishing with proper 2FA handling

**Benefits:**

- Better process control and visibility
- Separation of concerns (CI vs CD)
- Programmatic version management prevents human error
- Quality gates at each phase
- Flexibility to skip or repeat phases as needed

### Session Continuation Excellence

**Our session continuation system represents best-in-class UX:**

- Users never get kicked out during authentication workflows
- Intelligent command detection prevents false positives
- Clean exit handling respects user intent
- Comprehensive test coverage ensures reliability

This creates **the optimal environment** for working with AI coding tools.

## Local CD with Agent Mode

When conducting local continuous deployment with agents, **ALWAYS** follow this enhanced workflow:

### CRITICAL: Pre-CI/CD CHANGELOG.md Update

**BEFORE running deployment scripts**, the CHANGELOG.md MUST be updated:

1. **Update CHANGELOG.md FIRST** - Add entry for current version with clear change descriptions
2. **Enhanced workflow checks for this** - Scripts will validate CHANGELOG.md is updated
3. **Controlled approach preferred** - Use `./scripts/cicd/local-cd.sh --check-versions` then `./scripts/cicd/local-ci.sh` then `./scripts/cicd/local-cd.sh`

**CHANGELOG.md Entry Format:**

```markdown
## [X.X.X] - YYYY-MM-DD

### Added

- New feature descriptions

### Fixed

- Bug fixes and improvements

### Enhanced

- Improvements to existing features
```

### Enhanced Agent Workflow Process:

**Phase 1: Version Management**

```bash
# Check current version synchronization
./scripts/cicd/local-cd.sh --check-versions

# Update version programmatically if needed
./scripts/cicd/local-cd.sh --update-version 0.0.X
```

**Phase 2: Validation**

```bash
# Validate all changes without deployment
./scripts/cicd/local-ci.sh
```

**Phase 3: Documentation (MANDATORY)**

1. **Update CHANGELOG.md** - Add detailed entry for current version
2. **Review docs/ directory** - Update all relevant documentation
3. **Update README.md** - Ensure consistency with changes

**Phase 4: Deployment**

```bash
# Deploy with controlled workflow
./scripts/cicd/local-cd.sh
```

**Why Enhanced Workflow Matters:**

- **Controlled deployment**: Separate validation from deployment
- **Version consistency**: Programmatic management prevents human error
- **Quality assurance**: CI validation catches issues before deployment
- **Documentation completeness**: Enforced CHANGELOG.md and docs/ updates
- **Flexibility**: Choose validation vs deployment vs combined approach

**Agent Instructions:**

- **ALWAYS use enhanced workflow** - Prefer controlled approach over one-shot deployments
- **NEVER skip CHANGELOG.md update** - Required before any deployment
- **USE programmatic version management** - `--update-version` instead of manual editing
- **VALIDATE before deployment** - Always run `./scripts/cicd/local-ci.sh` first

## Testing NPM Package Before Publishing

**ALWAYS** test the NPM package locally before publishing to catch issues early:

1. **Build and Pack the Package:**

   ```bash
   cd npm/terminal-jarvis
   npm run build
   npm pack
   ```

2. **Test Installation in Temporary Environment:**

   ```bash
   # Create clean test environment
   cd /tmp
   mkdir -p test-terminal-jarvis && cd test-terminal-jarvis
   npm init -y

   # Install from local tarball
   npm install /path/to/terminal-jarvis-X.X.X.tgz

   # Test the binary directly
   npx terminal-jarvis --help
   npx terminal-jarvis list

   # Test multiple runs to verify NPX caching works
   npx terminal-jarvis --help  # Should not re-download
   ```

3. **Verify Package Contents:**

   ```bash
   # Check what gets included in the package
   npm pack --dry-run

   # Verify binary permissions and functionality
   ls -la node_modules/terminal-jarvis/bin/
   ./node_modules/terminal-jarvis/bin/terminal-jarvis --help
   ```

4. **Test Different Installation Methods:**

   ```bash
   # Test global installation
   npm install -g ./terminal-jarvis-X.X.X.tgz
   terminal-jarvis --help

   # Test npx from registry (after publishing)
   npx terminal-jarvis@X.X.X --help
   ```

**Common Issues to Check:**

- Binary has correct permissions (`chmod +x`)
- Package.json bin entry points to correct file
- Postinstall scripts have proper escaping
- All required files included in `files` array
- Version numbers are consistent across all files

**Benefits of This Process:**

- Catches binary execution issues before publishing
- Verifies NPX caching behavior
- Tests installation process end-to-end
- Prevents publishing broken packages
- Saves time debugging after publication

**Package Size Considerations:**

- Current package size is ~1.2MB compressed / ~2.9MB unpacked due to bundled Rust binary
- This ensures immediate functionality without requiring Rust toolchain installation
- Single generic binary works across platforms via NPM's bin configuration
- **Future optimization opportunities:**
  - Platform-specific packages to reduce download size further
  - Binary compression techniques
  - Splitting debug symbols
  - On-demand binary downloading
- Current approach prioritizes user experience over package size (optimized base case)

## Technical Notes

- The API modules (`api.rs`, `api_client.rs`, etc.) are framework code for future use
- They have `#[allow(dead_code)]` attributes since they're not used yet
- Configuration system uses TOML files for per-tool settings
- NPM package is just a thin wrapper around the Rust binary
