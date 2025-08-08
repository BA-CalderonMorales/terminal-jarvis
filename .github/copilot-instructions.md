# Terminal Jarvis - AI Coding Tools Wrapper

## What This Project Does

Terminal Jarvis is a thin Rust wrapper that provides a unified interface for managing and running AI coding tools like claude-code, gemini-cli, qwen-code, and opencode. Think of it as a package manager and runner for AI coding assistants.

The project follows Orhun Parmaksız's approach for packaging Rust applications via NPM, making it easy to install with `npm install -g terminal-jarvis`.

## How The Code Is Organized

The repository has two main parts:

**Rust Application** (`/src/`):

- `main.rs` - Entry point that starts the CLI
- `cli.rs` - Command definitions using clap (run, update, list, info, templates)
- `cli_logic.rs` - The actual business logic for each command
- `services.rs` - PackageService and GitHubService for managing tools
- `config.rs` - TOML configuration management
- `api.rs`, `api_client.rs`, `api_base.rs` - Future API framework (currently unused)

**NPM Package** (`/npm/terminal-jarvis/`):

- `src/index.ts` - Simple TypeScript wrapper that calls the Rust binary
- `package.json` - NPM package configuration
- `biome.json` - Biome linting configuration (we use Biome, not ESLint)

## Version Numbers Are Important

We use semantic versioning with **NO EMOJIS** and **NO DECORATIONS**. Just clean version numbers:

- `0.0.1` - Bug fixes, docs, small improvements
- `0.1.0` - New features that don't break existing functionality
- `1.0.0` - Breaking changes that require users to update their code

Always update BOTH `Cargo.toml` and `npm/terminal-jarvis/package.json` at the same time.

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
- **All shell scripts must go in scripts/ directory** - This keeps testing and scripting concerns separate
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

## How To Release

**Recommended: Use Local CI/CD Script (see "Local CD with Agent Mode" section below for details)**

1. **FIRST: Update CHANGELOG.md** - Add entry for current version with clear change descriptions
2. **MANDATORY: Review docs/ Directory** - **EVERY TIME** CHANGELOG.md is modified, you **MUST** review and update docs/
   - Check docs/ARCHITECTURE.md, docs/INSTALLATION.md, docs/TESTING.md, docs/LIMITATIONS.md
   - Update version references in docs/ files if version was bumped
   - Verify new features/fixes are properly documented
   - This is **NON-NEGOTIABLE** - no CHANGELOG.md updates without docs/ review
   - **ALSO MANDATORY: Review and update README.md** - Required when CHANGELOG.md or docs/ are modified
3. **THEN: Run automated script** - `./scripts/local-cicd.sh` (handles all steps below automatically)

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

### Version Consistency Check:

- [ ] `Cargo.toml` version updated
- [ ] `npm/terminal-jarvis/package.json` version updated
- [ ] `npm/terminal-jarvis/src/index.ts` version display updated
- [ ] `npm/terminal-jarvis/package.json` postinstall script version updated
- [ ] `src/cli_logic.rs` uses `env!("CARGO_PKG_VERSION")` (auto-updates)
- [ ] `CHANGELOG.md` has new version entry with clear changes
- [ ] `README.md` version references updated in note section
- [ ] `README.md` version references updated in note section

### Documentation Updates:

- [ ] README.md reflects current functionality and features
- [ ] Package size information updated if binary changed
- [ ] Installation instructions are accurate
- [ ] Examples work with current version

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

### Testing (Critical):

- [ ] Local package testing in `/tmp` environment completed
- [ ] NPX functionality verified (`npx terminal-jarvis` works)
- [ ] Binary permissions and execution tested
- [ ] Postinstall scripts validated

**Never commit without completing the full checklist!**

## Local CD with Agent Mode

When conducting local continuous deployment with agents, **ALWAYS** follow this order:

### CRITICAL: Pre-CI/CD CHANGELOG.md Update

**BEFORE running `./scripts/local-cicd.sh`**, the CHANGELOG.md MUST be updated:

1. **Update CHANGELOG.md FIRST** - Add entry for current version with clear change descriptions
2. **The local-cicd.sh script will check for this** - If missing, it will prompt you to:
   - Edit CHANGELOG.md immediately (opens in editor)
   - Update manually and re-run the script
   - Continue without update (not recommended)
   - Exit to handle later

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

### Agent Workflow Process:

1. **Update All Version References (if bumping version):**

   - `Cargo.toml` - version field
   - `npm/terminal-jarvis/package.json` - version field
   - `npm/terminal-jarvis/src/index.ts` - console.log version display
   - `src/cli_logic.rs` - interactive mode version display
   - `CHANGELOG.md` - add new version entry with changes (**REQUIRED BEFORE SCRIPT**)
   - `README.md` - version reference in the note section

2. **Run the Local CI/CD Script:**

   ```bash
   ./scripts/local-cicd.sh
   ```

   - The script will verify CHANGELOG.md is updated
   - If not updated, it will prompt for immediate action
   - No manual build/test/commit commands needed

3. **Script Handles Everything Else:**
   - Quality checks (clippy, fmt, tests)
   - Core functionality validation
   - Version bumping (if requested)
   - Building and testing
   - Git operations (commit, tag, push)
   - NPM publishing and dist-tags

**Why CHANGELOG.md First Matters:**

- Ensures proper documentation of changes before release
- Prevents rushed or incomplete release notes
- Maintains high quality project documentation
- Script enforces this requirement automatically
- No releases without proper change documentation

**Agent Instructions:**

- **ALWAYS ask the user to update CHANGELOG.md** before suggesting `./scripts/local-cicd.sh`
- **NEVER run the CI/CD script** without confirming CHANGELOG.md is updated
- **If user runs script without CHANGELOG.md update**, the script will catch this and prompt appropriately

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
