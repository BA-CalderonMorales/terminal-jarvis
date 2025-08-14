# Terminal Jarvis Maintainer's Guide

This guide provides detailed instructions for maintainers to publish releases and manage the Terminal Jarvis project.

## Overview

Terminal Jarvis uses a **hybrid CI/CD approach** with automated Git operations and crates.io publishing, but **manual NPM publishing** to avoid authentication issues with 2FA in terminal environments.

## Release Workflow

### 1. Development & Testing

```bash
# Run validation without deployment
./scripts/cicd/local-ci.sh
```

**What local-ci.sh does:**

- Code formatting (`cargo fmt`)
- Linting (`cargo clippy`)
- Test suite (33 comprehensive tests)
- Version consistency validation
- Release binary build
- NPM package build
- **No commits, tags, or pushes**

### 2. Git Deployment & Crates.io Publishing

```bash
# Deploy with full automation
./scripts/cicd/local-cd.sh
```

**What local-cd.sh does:**

- CHANGELOG.md verification
- Version management (bump or manual)
- Git commit with standardized message
- Git tag creation (`v0.0.X`)
- Push to GitHub with tags
- Publish to crates.io automatically
- **No NPM publishing (manual only)**

### 3. Manual NPM Publishing

**Why Manual?** 2FA authentication in terminal environments is unreliable and causes frequent publishing failures.

#### Step-by-Step NPM Publishing

```bash
# Navigate to NPM package directory
cd npm/terminal-jarvis

# Publish with 2FA code (replace with your actual code)
npm publish --otp=123456

# Add distribution tags
npm dist-tag add terminal-jarvis@0.0.X beta
npm dist-tag add terminal-jarvis@0.0.X stable  # For production-ready releases

# Verify tags were applied
npm dist-tag ls terminal-jarvis
```

#### Expected Output

```
beta: 0.0.X
latest: 0.0.X
stable: 0.0.X
```

### 4. Crates.io Publishing (Automated)

The deployment script automatically publishes to crates.io after pushing to GitHub but before NPM publishing.

#### How It Works

```bash
# This happens automatically in local-cd.sh after git push
cargo publish
```

#### Troubleshooting Crates.io Issues

If crates.io publishing fails:

1. **Authentication**: Ensure you're logged in to crates.io

   ```bash
   cargo login
   # Enter your API token from https://crates.io/me
   ```

2. **Version Conflicts**: Check if version already exists

   ```bash
   cargo search terminal-jarvis
   ```

3. **Manual Retry**: If automated publishing fails
   ```bash
   cargo publish
   ```

#### Crates.io Features

- **Installation**: `cargo install terminal-jarvis`
- **Updates**: `cargo install terminal-jarvis --force`
- **Documentation**: Auto-generated at https://docs.rs/terminal-jarvis
- **Package Size**: ~5.0MB (includes pre-built binary)

## Version Management

### Automated Version Bumping

The deployment scripts can automatically bump versions:

- **Patch (0.0.X)**: Bug fixes, small improvements
- **Minor (0.X.0)**: New features, no breaking changes
- **Major (X.0.0)**: Breaking changes

### Manual Version Control

For precise control, manually update all version files first, then use **option 6** in deployment scripts:

1. **Update version files:**

   - `Cargo.toml` - version field
   - `npm/terminal-jarvis/package.json` - version field
   - `npm/terminal-jarvis/src/index.ts` - console.log display

2. **Deploy with validation:**
   ```bash
   ./scripts/cicd/local-cd.sh
   # Choose option 6: "Deploy current version (manually updated)"
   ```

The script will validate all versions match before proceeding.

## Tools Manifest System

Terminal Jarvis uses a **single source of truth** approach for managing tool information to prevent drift between the README bullets and tools table.

### Manifest File: `tools-manifest.toml`

The root-level `tools-manifest.toml` file contains all tool definitions:

```toml
[[tools]]
name = "claude"
description = "Anthropic's Claude for code assistance"
status = "stable"
install_command = "npm install -g @anthropic-ai/claude-code"

[[tools]]
name = "opencode"
description = "Terminal-based AI coding agent"
status = "testing"
install_command = "npm install -g opencode-ai@latest"
```

### Generation Script

Use `scripts/utils/generate-readme-tools.sh` to generate README sections from the manifest:

```bash
# Generate both bullet list and tools table
./scripts/utils/generate-readme-tools.sh
```

**What it generates:**

- **Supported Tools bullets** - Automatically includes "(Testing)" for testing tools
- **Tools table** - Status column, installation commands, descriptions
- **Testing phase note** - Links testing tools to the Testing Guide
- **Sync to NPM package** - Updates both README files automatically

### Adding New Tools

1. **Add to manifest**: Update `tools-manifest.toml` with new tool entry
2. **Update code mappings**: Add tool to `src/tools.rs` and `src/services.rs`
3. **Regenerate README**: Run `./scripts/utils/generate-readme-tools.sh`
4. **Test locally**: Verify tool works with `terminal-jarvis list`

### Status Types

- **`stable`** - Production-ready, fully tested
- **`testing`** - Looking for testers, may have issues
- **`new`** - Recently added, stable but newer

**Benefits:**

- No drift between bullets and table
- Consistent formatting across sections
- Automatic testing phase documentation
- Single place to update tool information

## Distribution Tags Strategy

### Tag Types

```

- **`latest`**: Automatically assigned when publishing (default install)
- **`beta`**: Preview releases with newest features
- **`stable`**: Production-ready, thoroughly tested releases

### Installation Commands

Users can install through multiple channels:

#### NPM Installation

```bash
# Latest version (default)
npm install -g terminal-jarvis

# Beta releases (newest features)
npm install -g terminal-jarvis@beta

# Stable releases (production-ready)
npm install -g terminal-jarvis@stable

# Specific version
npm install -g terminal-jarvis@0.0.X
```

#### Cargo Installation (Rust Users)

```bash
# Install latest version from crates.io
cargo install terminal-jarvis

# Force update to latest version
cargo install terminal-jarvis --force

# Install specific version
cargo install terminal-jarvis --version 0.0.X
```

**Benefits of Cargo Installation:**

- Direct binary installation (no Node.js requirement)
- Automatic compilation for target platform
- Integration with Rust ecosystem
- Faster updates via Cargo registry

## Pre-Release Checklist

### Before Running Scripts

- [ ] All changes committed and pushed to feature branch
- [ ] CI tests passing locally
- [ ] Version numbers planned/updated (if manual)

### CHANGELOG.md Requirements

- [ ] Entry added for current version
- [ ] Format: `## [0.0.X] - YYYY-MM-DD`
- [ ] Clear categorization: Added, Enhanced, Fixed
- [ ] Descriptive change summaries

### Version Consistency Check

All these files must have matching version numbers:

- [ ] `Cargo.toml`
- [ ] `npm/terminal-jarvis/package.json`
- [ ] `npm/terminal-jarvis/src/index.ts`

## Troubleshooting

### NPM Publishing Issues

**2FA Code Expired:**

```bash
# Generate new code and retry
npm publish --otp=<new-code>
```

**Package Already Published:**

```bash
# Check current version
npm view terminal-jarvis version

# Bump version and republish
# (Update version files first)
```

**Permission Denied:**

```bash
# Verify you're logged in
npm whoami

# Login if needed
npm login
```

### Git Issues

**Push Rejected:**

```bash
# Fetch latest changes
git fetch origin develop
git pull origin develop

# Resolve conflicts and retry deployment
```

**Tag Already Exists:**

```bash
# Delete local tag
git tag -d v0.0.X

# Delete remote tag (if needed)
git push origin --delete v0.0.X

# Re-run deployment
```

## Script Safety Features

### Version Validation

Both CI and CD scripts validate:

- All version files synchronized
- CHANGELOG.md entries present
- No missing version references

### Error Prevention

- Scripts exit on version mismatches
- Clear error messages with fix instructions
- Separate CI (validation) from CD (deployment)

## Emergency Procedures

### Rollback NPM Release

```bash
# Unpublish recent version (within 24 hours)
npm unpublish terminal-jarvis@0.0.X

# Or deprecate version
npm deprecate terminal-jarvis@0.0.X "Use newer version"
```

### Rollback Git Release

```bash
# Remove tag
git tag -d v0.0.X
git push origin --delete v0.0.X

# Reset commit (if needed)
git reset --hard HEAD~1
git push origin develop --force  # Use with caution
```

## Package Details

### Current Stats

- **Package Size**: ~1.4MB compressed / ~3.7MB unpacked
- **Node Requirement**: >=16.0.0
- **Dependencies**: Zero runtime dependencies (self-contained binary)
- **Registry**: https://www.npmjs.com/package/terminal-jarvis

### Architecture

- **Rust binary**: Pre-compiled for immediate functionality
- **NPM wrapper**: TypeScript integration layer
- **Zero deps**: No additional runtime requirements

## Maintenance Commands

### Check Package Health

```bash
# NPM package info
npm view terminal-jarvis

# Distribution tags
npm dist-tag ls terminal-jarvis

# Download stats
npm view terminal-jarvis --json
```

### Local Testing

```bash
# Test package locally
cd npm/terminal-jarvis
npm pack
cd /tmp && npm install /path/to/terminal-jarvis-X.X.X.tgz
npx terminal-jarvis --help
```

## Contact & Support

- **Repository**: https://github.com/BA-CalderonMorales/terminal-jarvis
- **NPM Package**: https://www.npmjs.com/package/terminal-jarvis
- **Issues**: https://github.com/BA-CalderonMorales/terminal-jarvis/issues

---

**Note**: This manual NPM publishing approach prevents the 2FA terminal authentication issues that plagued previous automated attempts, ensuring reliable releases.
