# Phase 4: Distribution & Release

**Status**: PENDING

## Objective

Prepare and execute a new release with all improvements, ensuring smooth installation across NPM, Cargo, and Homebrew.

## Prerequisites

- Phase 1 completed
- Phase 3 completed (tests passing)
- CHANGELOG.md updated with all changes

## Pre-Release Checklist

### 1. Version Synchronization
- [ ] Determine version increment (likely 0.0.71 for this release)
- [ ] Update `Cargo.toml`
- [ ] Update `npm/terminal-jarvis/package.json`
- [ ] Update `homebrew/Formula/terminal-jarvis.rb`
- [ ] Verify with: `./scripts/cicd/local-cd.sh --check-versions`

### 2. CHANGELOG Update
- [ ] Add new version section with date
- [ ] Document all changes under appropriate categories:
  - Added: New post-session options (uninstall, re-enter API key)
  - Enhanced: Streamlined tool launch flow, minimal startup guidance
  - Fixed: API key input now masked for security
  - Technical: Removed /voice (deferred), clippy fixes, dead code cleanup

### 3. Quality Gates
- [ ] `cargo check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --all` applied
- [ ] `cargo test --lib -- --skip voice` passes
- [ ] E2E tests pass: `cd e2e && npm test`

## Release Steps

### 1. Local CI/CD
```bash
./scripts/cicd/local-ci.sh      # Validate (no commits)
./scripts/cicd/local-cd.sh      # Deploy (commits, tags, pushes)
```

### 2. Homebrew Release (if needed)
```bash
# Generate archives
./scripts/utils/generate-homebrew-release.sh --stage

# Commit archives
git add homebrew/release/
git commit -m "feat: Homebrew release archives vX.X.X"
git push

# Create GitHub release
gh release create vX.X.X \
  homebrew/release/terminal-jarvis-mac.tar.gz \
  homebrew/release/terminal-jarvis-linux.tar.gz \
  --title "Release vX.X.X" \
  --notes "See CHANGELOG.md for details" \
  --latest

# Verify
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/vX.X.X/terminal-jarvis-mac.tar.gz
```

### 3. NPM Publish
```bash
npm whoami  # Verify authenticated
cd npm/terminal-jarvis
npm publish --tag latest
```

### 4. Post-Release Verification
- [ ] `npm install -g terminal-jarvis@X.X.X` works
- [ ] `cargo install terminal-jarvis` works
- [ ] `brew install terminal-jarvis` works (if Homebrew updated)
- [ ] `terminal-jarvis --version` shows correct version
- [ ] Run `terminal-jarvis` and verify UX improvements

## Agent Instructions

When starting this phase:

1. First verify all quality gates pass
2. Update CHANGELOG.md BEFORE any deployment scripts
3. Follow the release steps in order
4. Test installation on at least one platform
5. Create GitHub release with proper notes

**Critical**: Commit Homebrew Formula BEFORE creating GitHub release to avoid URL mismatch.
