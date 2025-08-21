# CLAUDE.md - Terminal Jarvis AI Assistant Guide

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
- CLI interface elements
- Menu options
- Status indicators
- ANY user-facing text

**REASON**: Professional appearance and accessibility. Emojis create visual clutter and accessibility issues.

**CLAUDE REMINDER**: When improving CLI design, use professional text-based indicators like:
- "[INSTALLED]" / "[AVAILABLE]" instead of checkmarks
- "►" / "◄" / "•" for navigation instead of fancy symbols
- Simple ASCII art and borders
- Text-based status indicators

## CRITICAL DEPLOYMENT WARNING

**THE #1 DEPLOYMENT FAILURE**: Homebrew Formula changes committed AFTER GitHub release creation.

**THIS CAUSES BROKEN HOMEBREW INSTALLATIONS** - Formula URLs don't match release assets.

**NEVER DO THIS SEQUENCE**:

1. Run local-cd.sh (creates Git tag)
2. Create GitHub release
3. Discover Homebrew Formula needs updates
4. Commit Formula changes as a "fix"

**ALWAYS DO THIS SEQUENCE**:

1. Update ALL files (including Homebrew Formula)
2. Commit and push ALL changes to GitHub
3. Verify: `git status` shows "nothing to commit, working tree clean"
4. THEN run local-cd.sh
5. THEN create GitHub release

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

**Cross-Platform Build System** (v0.0.58+):

Terminal Jarvis now includes comprehensive multi-platform build support:

- **Native macOS binaries**: Universal binaries supporting both Intel and ARM64
- **Linux cross-compilation**: True Linux binaries for proper compatibility
- **Automated build system**: `scripts/utils/build-multiplatform.sh` and enhanced Homebrew release generation
- **CI/CD integration**: Seamless integration with existing deployment pipeline
- **Fallback handling**: Graceful degradation when cross-compilation tools unavailable

**Build Scripts**:
- `./scripts/utils/build-multiplatform.sh` - Multi-platform build system
- `./scripts/utils/generate-homebrew-release.sh` - Enhanced with true cross-platform archives
- `MULTIPLATFORM_BUILD=true ./scripts/cicd/local-ci.sh` - CI testing with cross-compilation

See `docs/MULTIPLATFORM_BUILD.md` for detailed technical documentation.

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

### Refactoring Best Practices (CRITICAL)

**OBJECTIVE**: Break large files (>200 lines) into focused domain modules while maintaining functionality.

#### **Proven Refactoring Architecture Pattern**

Based on successful cli_logic.rs refactoring (1,358 lines → 10 focused modules):

**Domain-Based Folder Structure**:
```
src/
  large_module.rs (684 lines) →
  large_module/
    mod.rs                    (re-exports + minimal coordination)
    large_module_domain1.rs   (focused functionality)
    large_module_domain2.rs   (focused functionality)
    large_module_domain3.rs   (focused functionality)
```

**Naming Convention**: `{module}_domain_operations.rs` (e.g., `cli_logic_tool_execution.rs`)

#### **Dead Code Elimination Protocol**

**ABSOLUTE REQUIREMENT**: Zero tolerance for dead code warnings.

**Process**:
1. **Identify**: Run `cargo check` to find unused function warnings
2. **Verify**: Search codebase for actual usage with `grep -r "function_name("`
3. **Remove**: Delete completely unused functions (prefer deletion over `#[allow(dead_code)]`)
4. **Clean imports**: Remove unused `use` statements
5. **Validate**: `cargo check` must show zero warnings

**Recent Success**: Eliminated 14 dead code warnings by removing 260+ lines of unused functions across 6 files.

#### **Compilation-Driven Refactoring**

**MANDATORY WORKFLOW**:
1. **Before refactoring**: `cargo check` - baseline compilation
2. **During refactoring**: Fix one compilation error at a time
3. **After refactoring**: `cargo check` + `cargo clippy` + `cargo fmt` must all pass
4. **Verification**: Run specific tests if available

**Critical**: Never proceed with next refactoring until current one compiles cleanly.

#### **Module Coordination Pattern**

**mod.rs responsibilities**:
- Re-export public functions: `pub use module_domain::*;`
- Minimal coordination logic (usually <50 lines)
- Clear documentation of module purpose

**Domain module responsibilities**:
- Single focused area (tool execution, update operations, etc.)
- Self-contained functionality
- Clear function naming
- Average 150-200 lines per domain module

#### **Refactoring Order Priority**

**Next targets** (by line count):
1. `services.rs` (684 lines) - Package/GitHub service management
2. `tools.rs` (624 lines) - Tool detection and execution  
3. `config.rs` (407 lines) - Configuration and caching
4. `auth_manager.rs` (317 lines) - Authentication management
5. `theme.rs` (235 lines) - UI theming system

**Files under 200 lines**: No action required (optimal size).

#### **Quality Verification Checklist**

**Pre-refactoring**:
- [ ] `cargo check` shows baseline warnings count
- [ ] Identify target file and line count
- [ ] Plan domain separation strategy

**During refactoring**:
- [ ] Create domain modules with clear responsibilities
- [ ] Move related functions together
- [ ] Update imports systematically
- [ ] Fix compilation errors incrementally

**Post-refactoring**:
- [ ] `cargo check` - zero warnings
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` - passes
- [ ] `cargo fmt --all` - applied
- [ ] Total line reduction calculated and documented
- [ ] REFACTOR.md updated with results

### DEPLOYMENT WORKFLOW - READ THIS FIRST!

**CRITICAL DEPLOYMENT CHECKLIST**

When you see requests like "Let's now run local-cd.sh" or anything involving deployment, **ALWAYS** follow this checklist:

## Claude-Optimized Command Workflow

### Quick Deployment (Most Common)

```bash
# 1. Check current status
git status                           # MUST be clean
./scripts/cicd/local-cd.sh --check-versions  # MUST be synchronized

# 2. Update CHANGELOG.md first (MANDATORY)
# Add version entry: ## [X.X.X] - YYYY-MM-DD

# 3. Deploy everything at once
./scripts/cicd/local-ci.sh && ./scripts/cicd/local-cd.sh
```

### Version Bump Deployment

```bash
# 1. Plan version increment
# Current: 0.0.47 → Next: 0.0.48 (patch), 0.1.0 (minor), 1.0.0 (major)

# 2. Update CHANGELOG.md with new version entry

# 3. Update version everywhere
./scripts/cicd/local-cd.sh --update-version 0.0.48

# 4. Verify and deploy
./scripts/cicd/local-cd.sh --check-versions && ./scripts/cicd/local-ci.sh && ./scripts/cicd/local-cd.sh
```

### Homebrew Release

```bash
# 1. After deployment, create Homebrew archives
./scripts/utils/generate-homebrew-release.sh --stage
git add homebrew/release/ && git commit -m "feat: add Homebrew archives v0.0.48" && git push

# 2. Create GitHub release
gh release create v0.0.48 homebrew/release/*.tar.gz --title "v0.0.48" --notes "Release notes" --latest

# 3. Verify archives work
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v0.0.48/terminal-jarvis-mac.tar.gz
```

## Critical Safety Rules

### Pre-Deployment Verification

```bash
# These MUST all pass before deployment:
git status                                    # "nothing to commit, working tree clean"
./scripts/cicd/local-cd.sh --check-versions  # "All versions are synchronized"
```

### Common Failures to Avoid

1. **Uncommitted Changes**: Always `git status` first
2. **Version Mismatch**: Always `--check-versions` first  
3. **Missing CHANGELOG.md**: Always update changelog before scripts
4. **Homebrew Formula**: Always commit Formula changes before GitHub release

### Emergency Fixes

```bash
# If versions are out of sync:
./scripts/cicd/local-cd.sh --update-version X.X.X

# If Homebrew Formula is wrong:
sed -i 's/version ".*"/version "X.X.X"/' homebrew/Formula/terminal-jarvis.rb
```
- Builds release binary
- Creates platform-specific archives (terminal-jarvis-mac.tar.gz, terminal-jarvis-linux.tar.gz)
- Calculates SHA256 checksums for Formula verification
- Automatically stages files for commit (with --stage flag)
- Removes temporary binary to avoid repository bloat
- Provides next steps for GitHub release creation

#### **Step 9: Verification - HOMEBREW FORMULA MUST BE COMMITTED**

**CRITICAL**: Verify Homebrew Formula was committed and pushed:

```bash
git log -1 --name-only  # MUST include homebrew/Formula/terminal-jarvis.rb
```

#### **Step 10: GitHub Release Creation - ONLY AFTER ARCHIVES ARE COMMITTED**

**MANDATORY**: Homebrew formulas require GitHub releases with attached archives. The deployment script only creates Git tags, not releases.

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

## Claude-Specific Development Tips

### Working with Claude

**Claude excels at**:
- **Systematic refactoring**: Breaking large files into focused domain modules
- **Quality assurance**: Ensuring code passes `cargo clippy`, `cargo fmt`, and tests
- **Documentation accuracy**: Keeping README.md and docs/ synchronized with actual features
- **Error debugging**: Methodically fixing compilation errors one at a time

**Optimal Claude workflow**:
1. **Plan before implementing**: Describe the architecture changes you want to make
2. **Incremental validation**: Run `cargo check` after each major change
3. **Quality gates**: Always verify `cargo clippy` and `cargo fmt` pass
4. **Documentation sync**: Update docs when adding or changing features

**Claude's refactoring strength**: Use Claude for domain-based module extraction, where large files (>200 lines) are broken into focused modules with clear responsibilities.

## Methodical Deployment Validation Process

### @aviv1 Fix Validation Workflow (v0.0.61)

This process ensures we properly validate @aviv1's Homebrew binary packaging fix before deploying to production:

#### **Phase 1: Version Bump and Local Validation**
1. **Version Management**: Use `./scripts/cicd/local-cd.sh --update-version 0.0.61` for programmatic version updates
2. **Local CI Validation**: Run `./scripts/cicd/local-ci.sh` to validate:
   - All 56 tests pass (core functionality, NPM packages, tool configurations)
   - Code quality checks (clippy, formatting) pass
   - Release binary builds successfully (validates @aviv1's fix prevents debug directory inclusion)
   - NPM package builds and validates correctly
   - Version consistency across all files
3. **CHANGELOG.md Documentation**: Update with proper attribution to @aviv1's contribution and detailed fix description

#### **Phase 2: Controlled Remote CI Testing**
4. **Commit and Push WITHOUT Tags**: 
   - Commit all changes including version bump and CHANGELOG.md updates
   - Push to GitHub develop branch WITHOUT creating version tags
   - This allows GitHub Actions CI to validate the fix in the remote environment
5. **GitHub Actions Validation**: Monitor remote CI to ensure:
   - Multi-platform build system works correctly with @aviv1's fix
   - Binary archives contain actual `terminal-jarvis` executable (not debug directories)
   - Archive creation logic properly filters with `-type f -executable`
   - All existing functionality remains intact across platforms

#### **Phase 3: Production Deployment (Only if Remote CI Passes)**
6. **Tag Creation and CD Trigger**: If GitHub Actions CI passes completely:
   - Create version tag: `git tag v0.0.61`
   - Push tags: `git push origin develop --tags`
   - This triggers the full CD pipeline with GitHub release creation
7. **Homebrew Distribution Validation**: Verify the fix works in production:
   - GitHub release contains properly formatted archives
   - Homebrew Formula points to correct release assets
   - End-to-end installation testing via Homebrew

#### **Phase 4: Rollback Strategy (If Remote CI Fails)**
8. **Pipeline Debugging**: If GitHub Actions fails:
   - Analyze specific failure points in the multi-platform build
   - Fix the pipeline issues while preserving @aviv1's core fix
   - Iterate on the build system without affecting the binary filtering logic
   - Re-test locally and repeat Phase 2

**Why This Approach Matters**:
- **Validates @aviv1's fix** in actual GitHub Actions environment before production
- **Prevents broken releases** by testing the complete pipeline without triggering CD
- **Maintains rollback capability** if the pipeline has integration issues
- **Ensures continuity** of the multi-platform distribution system
- **Preserves contributor trust** by properly validating external contributions

**Key Success Metrics**:
- Remote CI passes completely with @aviv1's changes
- Archive creation produces only executable binaries (no debug directories)
- Existing functionality remains intact across all platforms
- Homebrew installation workflow works end-to-end

This methodical approach ensures we leverage our remote pipeline properly while validating critical fixes from contributors like @aviv1.

## Deployment Verification Checklist

When deploying with Claude assistance:

```bash
# 1. Pre-deployment verification
git status                           # Must be clean
./scripts/cicd/local-cd.sh --check-versions  # Must be synchronized

# 2. Quality verification  
cargo check                          # Must compile cleanly
cargo clippy --all-targets --all-features -- -D warnings  # Must pass
cargo test                          # Must pass all tests

# 3. Documentation verification
# Check that CHANGELOG.md has entry for current version
# Verify README.md accuracy against actual features
```

## Version & Release Management

### Version Synchronization

**CRITICAL FILES** (must always match):
- `Cargo.toml`
- `npm/terminal-jarvis/package.json` 
- `homebrew/Formula/terminal-jarvis.rb` ⚠️ COMMONLY FORGOTTEN!

**Auto-Update Files** (managed by CLI):
- `src/cli_logic.rs` - uses `env!("CARGO_PKG_VERSION")`
- `npm/terminal-jarvis/src/index.ts` - version display

### CHANGELOG.md Best Practices

```bash
# ALWAYS update CHANGELOG.md BEFORE deployment scripts
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
- One version = One development session
- Update BEFORE running `local-cd.sh`
- Be specific about user-visible changes

### Commit Standards

- `fix:` - Bug fixes, docs, small improvements
- `feat:` - New features, no breaking changes  
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

- [ ] **CRITICAL: `homebrew/Formula/terminal-jarvis.rb` version updated** - COMMONLY FORGOTTEN!
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
