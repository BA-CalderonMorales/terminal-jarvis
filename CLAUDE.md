# CLAUDE.md - Terminal Jarvis AI Assistant Guide

## 🚨 CRITICAL DEPLOYMENT WARNING 🚨

**THE #1 DEPLOYMENT FAILURE**: Homebrew Formula changes committed AFTER GitHub release creation.

**THIS CAUSES BROKEN HOMEBREW INSTALLATIONS** - Formula URLs don't match release assets.

**NEVER DO THIS SEQUENCE**:

1. ❌ Run local-cd.sh (creates Git tag)
2. ❌ Create GitHub release
3. ❌ Discover Homebrew Formula needs updates
4. ❌ Commit Formula changes as a "fix"

**ALWAYS DO THIS SEQUENCE**:

1. ✅ Update ALL files (including Homebrew Formula)
2. ✅ Commit and push ALL changes to GitHub
3. ✅ Verify: `git status` shows "nothing to commit, working tree clean"
4. ✅ THEN run local-cd.sh
5. ✅ THEN create GitHub release

**VERIFICATION COMMAND**: `git log -1 --name-only` MUST include `homebrew/Formula/terminal-jarvis.rb`

## Communication Guidelines

### Reference Clarity Requirements

**CRITICAL**: Always provide specific context when referring to numbered items, steps, or sections.

**NEVER say**: "what do you mean by step 4" without clarifying which step 4
**ALWAYS say**: "what do you mean by step 4 in the deployment workflow" or "step 4 from the previous instructions"

**When providing numbered lists or procedures**:

- Use descriptive headers: "## Deployment Steps" not just "Steps:"
- Reference context explicitly: "In the above deployment workflow, step 4 means..."
- Avoid ambiguous references like "the previous step" or "step X" without context

**When user asks for clarification**:

- Always quote the specific text being referenced
- Provide the full context of where that reference appeared
- Explain which section/workflow/process the numbered item belongs to

**Example of good practice**:

```
## Homebrew Deployment Steps
1. Update Formula version
2. Commit changes
3. Create GitHub release
4. Verify archives are accessible ← When user asks about "step 4", this context is clear
```

**Example of bad practice**:

```
Steps:
1. Do this
2. Do that
3. Another thing
4. Final step ← Ambiguous when referenced later
```

## Project Overview

Terminal Jarvis is a Rust-based CLI wrapper that provides a unified interface for managing AI coding tools (claude-code, gemini-cli, qwen-code, opencode, llxprt). It's distributed through **three official channels**: NPM (Node.js ecosystem), Crates.io (Rust ecosystem), and Homebrew (macOS/Linux package manager).

**Current Version**: 0.0.48  
**License**: MIT  
**Repository**: https://github.com/BA-CalderonMorales/terminal-jarvis

### Multi-Platform Distribution

**Distribution Channels**:

1. **NPM**: `npm install -g terminal-jarvis` (Node.js ecosystem)
2. **Crates.io**: `cargo install terminal-jarvis` (Rust developers)
3. **Homebrew**: `brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis` (macOS/Linux package managers)

## Key Features & Capabilities

### Session Continuation System (v0.0.44+)

- **Intelligent Backslash Command Handling**: Prevents users from being kicked out of AI tools during authentication
- **Seamless Tool Restart**: Tools that exit after authentication flows are automatically restarted
- **Smart Command Detection**: Distinguishes between internal commands (`/auth`, `/login`, `/config`, `/setup`) and intentional exits (`/exit`, `/quit`, `/bye`)
- **Multi-Tool Support**: Works with all 6 AI coding tools (claude, gemini, qwen, opencode, llxprt, codex)
- **Anti-Pattern Prevention**: Eliminates user frustration from authentication workflows that previously forced tool exits

### Enhanced Deployment Workflow (v0.0.45+)

- **Programmatic Version Management**: Enhanced local-cd.sh with `--check-versions` and `--update-version` commands
- **Controlled Deployment**: Separation of CI validation (local-ci.sh) from deployment (local-cd.sh)
- **Version Synchronization**: Automated version updates across all project files
- **Workflow Flexibility**: Choose between programmatic management and one-shot deployments

## Architecture & Code Organization

### Rust Core (`/src/`)

- `main.rs` - Entry point, minimal delegation to CLI
- `cli.rs` - Command definitions using clap (run, update, list, info, templates)
- `cli_logic.rs` - Business logic + interactive T.JARVIS interface with ASCII art + session continuation system
- `tools.rs` - Tool detection, command mapping + session continuation logic with smart restart capability
- `installation_arguments.rs` - Installation commands with NPM validation
- `services.rs` - PackageService and GitHubService for external integrations
- `config.rs` - TOML configuration management
- `api.rs`, `api_client.rs`, `api_base.rs` - Future API framework (unused, has `#[allow(dead_code)]`)

### NPM Package (`/npm/terminal-jarvis/`)

- `src/index.ts` - TypeScript wrapper calling Rust binary
- `package.json` - NPM configuration, version must sync with Cargo.toml
- `biome.json` - Biome linting (NOT ESLint)
- `bin/` - Contains compiled Rust binary
- `config/` - Default TOML configurations

### Documentation (`/docs/`)

- `ARCHITECTURE.md` - Technical architecture details
- `INSTALLATION.md` - Installation procedures
- `LIMITATIONS.md` - Current limitations
- `TESTING.md` - Testing procedures

### Scripts (`/scripts/`)

**Organized Structure:**
- `scripts/cicd/` - CI/CD automation scripts
  - `local-ci.sh` - Continuous Integration (validation only, no commits/pushes)
  - `local-cd.sh` - Continuous Deployment (commit/tag/push/publish) with enhanced version management
    - `--check-versions` - Verify version synchronization across all files
    - `--update-version X.X.X` - Programmatically update all version references
- `scripts/tests/` - Testing and validation scripts
  - `smoke-test.sh` - Basic functionality tests
  - `manual_auth_test.sh` - Manual authentication behavior testing
  - `interactive_auth_test.sh` - Interactive authentication testing scenarios
  - `auth-test.sh` - Authentication testing
  - `test-opencode-fix.sh` - OpenCode integration testing
- `scripts/utils/` - Utility scripts
  - `workflow-dashboard.sh` - Development workflow status and recommendations
  - `generate-readme-tools.sh` - Generates README sections from tools manifest
  - `demo-auth-fix.sh` - Authentication demonstration utilities

### Tests (`/tests/`)

- `config_tests.rs` - Configuration system tests
- `integration_auth_tests.rs` - Authentication and browser prevention integration tests
- `auth_behavior_tests.rs` - Authentication behavior testing utilities
- `opencode_input_focus_tests.rs` - OpenCode input focus bug validation tests
- `codex_functionality_tests.rs` - Comprehensive codex tool functionality tests

**IMPORTANT**: The `tests/` directory is **ONLY** for Rust test files (`.rs`). **NO SHELL SCRIPTS** (`.sh`) are allowed in tests/. All shell scripts must be placed in `scripts/` directory.

## Development Standards

### Rust Code Quality

- **MUST pass**: `cargo clippy --all-targets --all-features -- -D warnings`
- **MUST format**: `cargo fmt --all`
- **Error handling**: Use `anyhow::Result` consistently
- **Documentation**: Add doc comments for public functions
- **No unwrap()**: Use proper error handling, no `.unwrap()` without justification
- **Constants**: No magic numbers, use named constants

### TypeScript/NPM Code Quality

- **Linting**: Use Biome (NOT ESLint) - `npm run lint`
- **Formatting**: `npm run format` before committing
- **Build**: `npm run build` must succeed
- **Sync**: Always run `npm run sync-readme` before NPM publishing

### DEPLOYMENT WORKFLOW - READ THIS FIRST!

**🚨 CRITICAL DEPLOYMENT CHECKLIST**

When you see requests like "Let's now run local-cd.sh" or anything involving deployment, **ALWAYS** follow this checklist:

#### **Step 1: Version Planning**

**Current Live Version**: Check GitHub releases to see the latest published version  
**Next Version**: Increment appropriately based on changes:

- **Patch (X.X.Y)**: Bug fixes, small improvements, documentation
- **Minor (X.Y.0)**: New features, no breaking changes
- **Major (Y.0.0)**: Breaking changes

#### **Step 2: Pre-Commit Verification - MANDATORY**

**🚨 CRITICAL**: Always check for uncommitted changes BEFORE proceeding with deployment:

```bash
# Check for any uncommitted changes
git status

# If you see "Changes not staged for commit" or "Changes to be committed":
# STOP and commit those changes FIRST before proceeding with deployment
git add <files>
git commit -m "descriptive commit message"
git push origin develop

# Only proceed with deployment when git status shows:
# "nothing to commit, working tree clean"
```

**Why This Matters**: The deployment process expects a clean working directory. Uncommitted changes, especially to documentation files (CLAUDE.md, copilot-instructions.md) or configuration files (homebrew/Formula/terminal-jarvis.rb), must be committed and pushed BEFORE deployment to ensure the complete state is published to GitHub.

**🚨 HOMEBREW FORMULA CRITICAL DEPLOYMENT ORDER**

**THE #1 DEPLOYMENT FAILURE**: Homebrew Formula changes committed AFTER GitHub release creation.

**MANDATORY SEQUENCE** (NEVER deviate from this order):

1. **FIRST**: Update and commit ALL changes (including Homebrew Formula)
2. **SECOND**: Push changes to GitHub
3. **THIRD**: Create GitHub release with archives
4. **NEVER**: Create release first, then commit Formula changes

**This prevents the common failure pattern where:**

- GitHub release is created with v0.0.X URLs
- Homebrew Formula still references the old URLs
- Users get broken installations because Formula URLs don't match releases

#### **Step 3: CHANGELOG.md Update - MANDATORY FIRST STEP**

```bash
# Example: If live version is 0.0.47, next version should be 0.0.48
## [0.0.48] - 2025-08-10

### Added
- **Version Caching System**: Intelligent caching of NPM distribution tag information
- **Cache Management CLI**: Commands for cache status, refresh, and clearing
- **Performance Optimization**: Faster startup times through cached version data

### Enhanced
- **User Experience**: Eliminated API call delays on Terminal Jarvis home page
- **Network Efficiency**: Reduced NPM registry calls with 1-hour cache TTL
```

#### **Step 4: Version Synchronization**

```bash
# Update version across all files automatically
./scripts/cicd/local-cd.sh --update-version 0.0.48
```

#### **Step 5: Homebrew Formula Update & Verification**

```bash
# Update Homebrew Formula for new version (if not done by local-cd.sh automatically)
sed -i 's/version ".*"/version "0.0.48"/' homebrew/Formula/terminal-jarvis.rb

# 🚨 CRITICAL: Verify all versions are synchronized
./scripts/cicd/local-cd.sh --check-versions  # MUST show "All versions are synchronized"

# 🚨 CRITICAL: Ensure working tree is clean BEFORE deployment
git status  # MUST show "nothing to commit, working tree clean"
```

#### **Step 6: Validation**

```bash
./scripts/cicd/local-ci.sh  # MUST pass all tests
```

#### **Step 7: Deployment - COMMITS AND PUSHES ALL CHANGES**

```bash
./scripts/cicd/local-cd.sh  # Creates archives, commits ALL changes (including Formula), tags, pushes to GitHub
```

#### **Step 8: Verification - HOMEBREW FORMULA MUST BE COMMITTED**

**🚨 CRITICAL**: Verify Homebrew Formula was committed and pushed:

```bash
git log -1 --name-only  # MUST include homebrew/Formula/terminal-jarvis.rb
```

#### **Step 9: GitHub Release Creation - ONLY AFTER FORMULA IS COMMITTED**

**🚨 MANDATORY**: Homebrew formulas require GitHub releases with attached archives. The deployment script only creates Git tags, not releases.

```bash
# Verify the tag was created
git tag -l | grep v0.0.48

# Create GitHub release with archives attached (REQUIRED for Homebrew)
gh release create v0.0.48 \
  homebrew/release/terminal-jarvis-mac.tar.gz \
  homebrew/release/terminal-jarvis-linux.tar.gz \
  --title "Release v0.0.48: [Brief Description]" \
  --notes "Release notes content" \
  --latest
```

**Verification**: Ensure release assets are accessible:

```bash
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.48/terminal-jarvis-mac.tar.gz
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.48/terminal-jarvis-linux.tar.gz
```

**Both should return HTTP 302 (redirect) responses.**

#### **Step 9: Homebrew Release Validation**

**Note**: Homebrew release creation is now handled automatically by the CI/CD pipeline.
Archives are created during the deployment process in `local-cd.sh`.

### Version Management

**CRITICAL**: All version numbers must stay synchronized:

- `Cargo.toml` - version field
- `npm/terminal-jarvis/package.json` - version field
- **`homebrew/Formula/terminal-jarvis.rb` - version field** ⚠️ **COMMONLY FORGOTTEN!**
- `npm/terminal-jarvis/src/index.ts` - console.log version display
- `src/cli_logic.rs` - uses `env!("CARGO_PKG_VERSION")` (auto-updates)
- `CHANGELOG.md` - must have entry for current version
- `README.md` - version references in note sections

🚨 **HOMEBREW FORMULA VERSION MUST MATCH EXACTLY** - This is frequently overlooked and causes deployment failures.

### CHANGELOG.md Management (CRITICAL)

**MANDATORY**: Update CHANGELOG.md BEFORE running any deployment scripts to prevent version confusion.

#### Changelog Best Practices:

1. **Feature-Based Versioning**: Each version should represent one cohesive feature set or development session
2. **Timeline Accuracy**: Don't mix features from different development days/sessions into the same version
3. **Update First**: Always add changelog entry BEFORE running `./scripts/cicd/local-cd.sh`
4. **Clear Structure**: Use `### Added`, `### Enhanced`, `### Fixed`, `### Technical` sections consistently

#### Version Increment Guidelines:

- **Patch (0.0.X)**: Bug fixes, documentation updates, small improvements
- **Minor (0.X.0)**: New features, major enhancements (like Homebrew integration)
- **Major (X.0.0)**: Breaking changes that require user action

#### Example Development Session:

```bash
# Day 1: Working on Homebrew integration
# At END of session: Update CHANGELOG.md with v0.0.47 entry
## [0.0.47] - 2025-08-09
### Added
- **Homebrew Integration**: Complete multi-platform distribution
- **Testing Infrastructure**: Local validation protocols

# Day 2: Deploy the completed feature
./scripts/cicd/local-cd.sh  # Will see v0.0.47 entry and proceed
```

#### Common Mistakes to Avoid:

- **Mixing Sessions**: Don't put Day 1 and Day 2 work in same changelog entry
- **Post-Deployment Updates**: Don't update changelog after running deployment
- **Vague Entries**: Be specific about what users gain from each change
- **Version Confusion**: Each changelog entry should match exactly one deployment

### README.md Maintenance (CRITICAL)

**ESSENTIAL**: The README.md is the first impression for users and must be accurate and current.

#### **README.md Accuracy Checklist**:

- **Core Functionality**: Descriptions must match actual capabilities - no outdated or aspirational features
- **Installation Instructions**: Must work correctly for all distribution channels (NPM, Crates.io, Homebrew)
- **Version References**: All version numbers must be consistent throughout
- **Working Examples**: Code examples and commands must work with current version
- **Distribution Badges**: Badge organization must reflect current multi-platform distribution
- **Package Information**: Size information and technical details must be current
- **Feature Claims**: Only reference features that actually exist in the codebase

#### **Common README.md Issues**:

- **Outdated Feature Lists**: Mentioning features that were planned but not implemented
- **Broken Installation Commands**: Commands that fail due to package name changes
- **Version Inconsistencies**: Different version numbers in different sections
- **Dead Links**: URLs that no longer work or point to incorrect resources
- **Misleading Performance Claims**: Outdated information about package size or capabilities

#### **README.md Update Triggers**:

- **Every CHANGELOG.md update**: README must be reviewed for accuracy
- **Version bumps**: All version references must be synchronized
- **New features**: Feature descriptions must be added accurately
- **Installation changes**: All distribution channel instructions must be verified
- **Architecture changes**: Technical descriptions must reflect current implementation

### Commit Standards

Use conventional commits with these types:

- `fix:` - Bug fixes, docs, small improvements
- `feat:` - New features that don't break existing functionality
- `break:` - Breaking changes requiring user updates
- `docs:` - Documentation updates
- `style:` - Code style/formatting
- `refactor:` - Code refactoring
- `test:` - Test additions/modifications
- `chore:` - Maintenance tasks

**NO EMOJIS** in commits, code, or documentation.

### Tool Configuration Consistency (CRITICAL)

**When adding new AI coding tools**, you MUST update these files in sync:

1. **`src/tools.rs`** - Add tool to `get_command_mapping()` and `get_tool_commands()`
2. **`src/services.rs`** - Add display name mapping in `get_display_name_to_config_mapping()`
3. **`terminal-jarvis.toml.example`** - Add tool configuration with install/update commands
4. **Tests** - Update both `test_display_name_to_config_mapping()` and `test_config_key_resolution()`
5. **Documentation** - Update README.md, CLI help text, and package descriptions

**Common Failure Pattern**: Adding a tool to `tools.rs` and config file but forgetting the mapping in `services.rs`, causing "Tool not found in configuration" errors during updates.

**Verification Steps**:

```bash
# Test that all tools can be listed
cargo run -- list

# Test that all tools can be updated (dry run)
cargo run -- update --help

# Run services tests to verify mappings
cargo test --lib services
```

### Homebrew Integration (v0.0.46+)

**Key Innovation**: Complete multi-platform distribution with Homebrew support based on Federico Terzi's approach.

**Note**: Homebrew release creation is now integrated into the main CI/CD pipeline.
Platform-specific archives and formula updates are handled automatically during deployment.

#### **Formula Structure** (`homebrew/Formula/terminal-jarvis.rb`):

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

#### **Local Testing Protocol** (MANDATORY before deployment):

```bash
# Local tap testing (manual validation)
mkdir -p /tmp/homebrew-test-tap/Formula
cp homebrew/Formula/terminal-jarvis.rb /tmp/homebrew-test-tap/Formula/
cd /tmp/homebrew-test-tap && git init && git add . && git commit -m "Test"

# 3. Test complete installation workflow
brew tap local/test /tmp/homebrew-test-tap
brew install local/test/terminal-jarvis

# 4. Verify functionality
terminal-jarvis --version
terminal-jarvis --help
brew test local/test/terminal-jarvis
```

#### **Common Homebrew Pitfalls**:

- **Archive Naming**: Must use consistent names: `terminal-jarvis-{macos|linux}.tar.gz`
- **SHA256 Mismatch**: Always regenerate SHA256 after creating new archives
- **Binary Permissions**: Archives must preserve execute permissions
- **Formula Syntax**: Ruby syntax errors prevent Formula loading
- **Cross-Platform**: Use `on_macos` and `on_linux` blocks for platform-specific handling
  cargo test --lib services

````

## File Sync Requirements

**README.md Synchronization**: The root README.md and `npm/terminal-jarvis/README.md` must be identical. Before NPM publishing:

```bash
cd npm/terminal-jarvis
npm run sync-readme
````

**MANDATORY: docs/ Directory Review**: **EVERY TIME** you modify CHANGELOG.md, you **MUST** review and update the docs/ directory:

- Check if any docs/ files need updates based on the changes in CHANGELOG.md
- Update version references in docs/ files if version was bumped
- Verify that new features/fixes are properly documented
- Update docs/ARCHITECTURE.md, docs/INSTALLATION.md, docs/TESTING.md, docs/LIMITATIONS.md as needed
- This is **NON-NEGOTIABLE** - no CHANGELOG.md updates without docs/ review

**MANDATORY: README.md Updates**: **EVERY TIME** you modify CHANGELOG.md or docs/ files, you **MUST** review and update README.md:

- Ensure README.md reflects any changes made to documentation structure
- Update feature descriptions if new functionality was added
- Verify installation instructions are current if docs/INSTALLATION.md was updated
- Update known issues section if docs/LIMITATIONS.md was modified
- This is **NON-NEGOTIABLE** - no docs/ or CHANGELOG.md updates without README.md review

## Release Process

### Optimal Workflow (Enhanced Deployment)

We've developed an optimal workflow that balances automation with control:

**Phase 1: Development & Version Management**

```bash
# Check current version synchronization
./scripts/cicd/local-cd.sh --check-versions

# Update version programmatically (if needed)
./scripts/cicd/local-cd.sh --update-version 0.0.X

# Validate changes with CI
./scripts/cicd/local-ci.sh
```

**Phase 2: Documentation Updates (MANDATORY)**

1. **Update CHANGELOG.md first** - Add entry for current version with detailed change descriptions
2. **Review docs/ directory** - Check docs/ARCHITECTURE.md, docs/INSTALLATION.md, docs/TESTING.md, docs/LIMITATIONS.md
3. **Update README.md** - Ensure consistency with CHANGELOG.md and docs/ updates

**Phase 3: Deployment**

```bash
# Deploy with controlled workflow
./scripts/cicd/local-cd.sh

# Manual NPM publishing (due to 2FA requirements)
cd npm/terminal-jarvis && npm publish
npm dist-tag add terminal-jarvis@X.X.X stable  # optional
```

### Legacy Automated (One-Shot)

1. **Update CHANGELOG.md first** - Add entry for current version
2. **Run CI validation**: `./scripts/cicd/local-ci.sh` - Validates without deployment
3. **Run deployment**: `./scripts/cicd/local-cd.sh` - Handles everything automatically

### Manual Process

1. Update all version numbers (see Version Management above)
2. Update CHANGELOG.md with new version and changes
3. Run quality checks: `cargo clippy`, `cargo fmt`, `npm run build`
4. Commit: `git commit -m "feat: description of changes"`
5. Tag: `git tag v0.0.X`
6. Push: `git push origin develop --tags`
7. Publish: `cd npm/terminal-jarvis && npm publish`
8. Add dist-tags if needed:
   - Stable: `npm dist-tag add terminal-jarvis@X.X.X stable`
   - Beta: `npm dist-tag add terminal-jarvis@X.X.X beta`

## Testing Requirements

### Test-Driven Bugfixes (MANDATORY)

**EVERY** bugfix session MUST follow this exact workflow:

1. **Identify the Bug**: Understand the exact problem and reproduction steps
2. **Write Failing Test FIRST**:
   - Create a test that reproduces the bug behavior
   - Test MUST fail initially (proving the bug exists)
   - Place in appropriate location:
     - `tests/` directory for integration tests
     - `src/` with `#[cfg(test)]` for unit tests
   - Use descriptive test names: `test_bug_opencode_input_focus_on_fresh_install`
3. **Run Test**: Verify it fails for the expected reason
4. **Implement Fix**: Make minimal changes to make the test pass
5. **Verify Fix**: Test passes, no regressions in other tests
6. **Commit**: Include both test and fix in same commit with clear message

**Test File Guidelines:**

- Integration tests go in `tests/` directory as `.rs` files
- Unit tests go in `src/` files using `#[cfg(test)]` mod test blocks
- **NO SHELL SCRIPTS** in `tests/` directory - only Rust test files
- Test names should clearly describe the bug being fixed
- Include comments explaining the bug scenario

**Example Test Structure:**

```rust
#[test]
fn test_bug_opencode_input_focus_on_fresh_install() {
    // Reproduces issue where opencode input box lacks focus on fresh installs
    // Bug: User cannot type directly in input box without manual focus
    // Expected: Input box should be automatically focused

    // Test implementation here
}
```

### Pre-Commit Checklist

**Version Consistency:**

- [ ] **CHANGELOG.md updated FIRST** - CRITICAL: Must be done BEFORE running local-cd.sh
- [ ] Changelog entry reflects actual work completed in current development session
- [ ] Version number incremented appropriately (patch/minor/major)
- [ ] Cargo.toml version updated
- [ ] npm/terminal-jarvis/package.json version updated
- [ ] npm/terminal-jarvis/src/index.ts version display updated
- [ ] README.md version references updated

**MANDATORY Documentation Review:**

- [ ] **docs/ directory reviewed** - REQUIRED when CHANGELOG.md is modified
- [ ] docs/ARCHITECTURE.md updated if architectural changes were made
- [ ] docs/INSTALLATION.md updated if installation procedures changed
- [ ] docs/TESTING.md updated if testing procedures changed
- [ ] docs/LIMITATIONS.md updated if new limitations were introduced
- [ ] Version references in docs/ files updated if version was bumped
- [ ] **CRITICAL: README.md accuracy review** - Essential for user experience
  - [ ] Core functionality descriptions are current and accurate
  - [ ] No misleading information about features or capabilities
  - [ ] Installation instructions reflect all distribution channels (NPM, Crates.io, Homebrew)
  - [ ] Version references are consistent throughout
  - [ ] Examples work with current version and feature set
  - [ ] Package size information updated if binary changed
  - [ ] Badge organization matches current distribution channels
  - [ ] All referenced features actually exist in current codebase
- [ ] **README.md reviewed and updated** - REQUIRED when CHANGELOG.md or docs/ are modified

**Quality Checks:**

- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo fmt --all` applied
- [ ] `cargo test` passes
- [ ] **Failing test added for bugfixes** - If this is a bugfix, verify failing test was created first
- [ ] `cd npm/terminal-jarvis && npm run build` succeeds

**Tool Configuration Consistency (if adding new tools):**

- [ ] Tool added to `src/tools.rs` command mapping and tool commands
- [ ] Tool added to `src/services.rs` display name mapping
- [ ] Tool configuration added to `terminal-jarvis.toml.example`
- [ ] Tests updated in `services.rs` for new tool mapping
- [ ] Documentation updated (README.md, CLI descriptions)

**Homebrew Integration (if updating version):**

- [ ] **🚨 CRITICAL: `homebrew/Formula/terminal-jarvis.rb` version updated** - COMMONLY FORGOTTEN!
- [ ] GitHub release created with version tag
- [ ] Homebrew archives uploaded: `terminal-jarvis-macos.tar.gz`, `terminal-jarvis-linux.tar.gz`
- [ ] SHA256 checksums verified in Formula match actual archives
- [ ] Multi-platform support verified (macOS and Linux archives)
- [ ] **End-to-end Homebrew testing completed** using local tap (see Local Testing Protocol above)

**NPM Package Testing:**

- [ ] Local package testing in `/tmp` environment
- [ ] NPX functionality verified (`npx terminal-jarvis` works)
- [ ] Binary permissions and execution tested

### Local NPM Testing

```bash
cd npm/terminal-jarvis
npm run build && npm pack
cd /tmp && mkdir test-terminal-jarvis && cd test-terminal-jarvis
npm install /path/to/terminal-jarvis-X.X.X.tgz
npx terminal-jarvis --help
```

## Configuration System

Terminal Jarvis uses TOML configuration with precedence:

1. `./terminal-jarvis.toml` (project-specific, highest priority)
2. `~/.config/terminal-jarvis/config.toml` (user-specific)
3. Built-in defaults (fallback)

Example configuration:

```toml
[tools]
claude = { enabled = true, auto_update = true }
gemini = { enabled = true, auto_update = false }
qwen = { enabled = true, auto_update = true }
opencode = { enabled = false, auto_update = false }
llxprt = { enabled = true, auto_update = true }

[ui]
show_ascii_art = true
center_output = true
```

## Common Tasks

### Adding New Tools

1. Define tool configuration in `cli_logic.rs`
2. Add command interface in `cli.rs`
3. Implement services in `services.rs` if needed
4. Update tool registry and detection logic

### Fixing Rust Code Issues

- **MANDATORY: Write Failing Test First** - For EVERY bugfix session:
  - Create a failing test that reproduces the exact bug behavior
  - Place test in appropriate location: `tests/` for integration tests, `src/` for unit tests
  - Test must fail initially and pass after the fix is implemented
  - This is **NON-NEGOTIABLE** - no bug fixes without failing tests
- Check `cargo clippy` warnings first
- Ensure proper error handling with `anyhow::Result`
- Add doc comments for public functions
- Use `cargo fmt` for consistent formatting

### Updating NPM Package

- Always sync README: `npm run sync-readme`
- Test locally before publishing
- Verify binary permissions: `chmod +x bin/terminal-jarvis`
- Check package contents: `npm pack --dry-run`

### Debugging CI/CD Issues

- Check CHANGELOG.md is updated before running deployment scripts
- Verify all version numbers are synchronized with `./scripts/cicd/local-cd.sh --check-versions`
- Test NPM package locally in `/tmp` environment
- Ensure binary has correct permissions
- Use `./scripts/cicd/local-ci.sh` for validation without deployment

### Debugging Session Continuation Issues

**Common Issues:**

- Tool exits unexpectedly during authentication → Check `should_continue_session()` logic in `tools.rs`
- Infinite restart loops → Verify exit commands (`/exit`, `/quit`, `/bye`) are properly excluded
- Tool doesn't restart after authentication → Check command matching in session continuation logic

**Debug Commands:**

```bash
# Test session continuation with specific tool
RUST_LOG=debug cargo run -- run claude

# Check tool command mapping
cargo run -- list

# Validate session continuation logic
cargo test --lib tools -- session_continuation
```

**Session Continuation Logic Flow:**

1. User runs tool through Terminal Jarvis
2. Tool exits with status code
3. `should_continue_session()` checks last user input
4. If internal command (`/auth`, `/login`, `/config`, `/setup`) → restart tool
5. If exit command (`/exit`, `/quit`, `/bye`) → return to main menu
6. If quick completion (< 3 seconds) → return to main menu (prevent false positives)

## Don'ts

- **No emojis** anywhere in code, commits, or documentation
- **No vague commits** like "fix stuff" or "update things"
- **No combining unrelated changes** in one commit
- **No force pushing** to main or develop branches
- **No `.unwrap()`** without proper error handling
- **No magic numbers** - use named constants
- **No multi-line bash commands** in terminal suggestions - use single-line with `&&`
- **Never commit** without running the pre-commit checklist
- **Never publish NPM** without local testing first
- **NO BUGFIXES WITHOUT FAILING TESTS** - Every bug must have a failing test written first

## Current Package Details

- **Size**: ~1.2MB compressed / ~2.9MB unpacked
- **Node requirement**: >=16.0.0
- **Dependencies**: Zero runtime dependencies (self-contained binary)
- **Distribution tags**: `latest`, `stable`, `beta` available

## Future Architecture

- Plugin system for custom tools
- Web dashboard for tool management
- Enhanced REST API
- Docker support
- Native shell completion

---

**Package Size Philosophy**: Current approach prioritizes user experience over package size by bundling the Rust binary for immediate functionality without requiring Rust toolchain installation.
