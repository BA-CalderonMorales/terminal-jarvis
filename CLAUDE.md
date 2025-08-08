# CLAUDE.md - Terminal Jarvis AI Assistant Guide

## Project Overview

Terminal Jarvis is a Rust-based CLI wrapper that provides a unified interface for managing AI coding tools (claude-code, gemini-cli, qwen-code, opencode, llxprt). It's packaged via NPM for easy distribution while maintaining the performance of a native Rust binary.

**Current Version**: 0.0.40  
**License**: MIT  
**Repository**: https://github.com/BA-CalderonMorales/terminal-jarvis

## Architecture & Code Organization

### Rust Core (`/src/`)
- `main.rs` - Entry point, minimal delegation to CLI
- `cli.rs` - Command definitions using clap (run, update, list, info, templates)
- `cli_logic.rs` - Business logic + interactive T.JARVIS interface with ASCII art
- `tools.rs` - Tool detection using multiple verification methods (`which`, `--version`, `--help`)
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
- `local-ci.sh` - Continuous Integration (validation only)
- `local-cd.sh` - Continuous Deployment (commit/tag/push)
- `workflow-dashboard.sh` - Development workflow status and recommendations
- `smoke-test.sh` - Basic functionality tests
- `manual_auth_test.sh` - Manual authentication behavior testing
- `interactive_auth_test.sh` - Interactive authentication testing scenarios

### Tests (`/tests/`)
- `config_tests.rs` - Configuration system tests
- `integration_auth_tests.rs` - Authentication and browser prevention integration tests
- `auth_behavior_tests.rs` - Authentication behavior testing utilities

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

### Version Management
**CRITICAL**: All version numbers must stay synchronized:
- `Cargo.toml` - version field
- `npm/terminal-jarvis/package.json` - version field  
- `npm/terminal-jarvis/src/index.ts` - console.log version display
- `src/cli_logic.rs` - uses `env!("CARGO_PKG_VERSION")` (auto-updates)
- `CHANGELOG.md` - must have entry for current version
- `README.md` - version references in note sections

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

## File Sync Requirements

**README.md Synchronization**: The root README.md and `npm/terminal-jarvis/README.md` must be identical. Before NPM publishing:
```bash
cd npm/terminal-jarvis
npm run sync-readme
```

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

### Automated (Recommended)
1. **Update CHANGELOG.md first** - Add entry for current version
2. **Run**: `./scripts/local-cicd.sh` - Handles everything automatically

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

### Pre-Commit Checklist
**Version Consistency:**
- [ ] Cargo.toml version updated
- [ ] npm/terminal-jarvis/package.json version updated
- [ ] npm/terminal-jarvis/src/index.ts version display updated
- [ ] CHANGELOG.md has new version entry
- [ ] README.md version references updated

**MANDATORY Documentation Review:**
- [ ] **docs/ directory reviewed** - REQUIRED when CHANGELOG.md is modified
- [ ] docs/ARCHITECTURE.md updated if architectural changes were made
- [ ] docs/INSTALLATION.md updated if installation procedures changed
- [ ] docs/TESTING.md updated if testing procedures changed
- [ ] docs/LIMITATIONS.md updated if new limitations were introduced
- [ ] Version references in docs/ files updated if version was bumped
- [ ] **README.md reviewed and updated** - REQUIRED when CHANGELOG.md or docs/ are modified

**Quality Checks:**
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo fmt --all` applied
- [ ] `cargo test` passes
- [ ] `cd npm/terminal-jarvis && npm run build` succeeds

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
- Check CHANGELOG.md is updated before running `./scripts/local-cicd.sh`
- Verify all version numbers are synchronized
- Test NPM package locally in `/tmp` environment
- Ensure binary has correct permissions

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