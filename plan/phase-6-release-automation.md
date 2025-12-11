# Phase 6: Release Automation

**Status**: PENDING  
**Priority**: HIGH  
**Estimated Sessions**: 1-2

## The Problem

Current release process requires:
1. Manually updating 3 version files (Cargo.toml, package.json, Formula)
2. Manually updating CHANGELOG.md
3. Running local-ci.sh
4. Running local-cd.sh
5. Manually generating Homebrew archives
6. Manually creating GitHub release
7. Manually uploading archives
8. Manually publishing to NPM

That's 8+ manual steps with opportunities for mistakes at each one. The Formula-before-release ordering issue has bitten before.

## The Goal

```bash
./scripts/cicd/release.sh 0.0.71
```

One command. Version bump, changelog stub, tests, tag, push, Homebrew, GitHub release, NPM publish. Done.

## Tasks

### 1. Version Bump Automation
- [ ] Create `bump-version.sh` that updates all 3 files atomically
- [ ] Validate semver format
- [ ] Fail if versions are already mismatched

```bash
./scripts/utils/bump-version.sh 0.0.71
# Updates: Cargo.toml, npm/terminal-jarvis/package.json, homebrew/Formula/terminal-jarvis.rb
```

### 2. Changelog Stub Generation
- [ ] Parse git log since last tag
- [ ] Generate changelog entry skeleton with commit messages
- [ ] Open in editor for human review/cleanup
- [ ] Fail release if changelog not updated

### 3. Homebrew SHA256 Automation
- [ ] After `cargo build --release`, calculate SHA256 of binary
- [ ] Auto-update Formula with new SHA
- [ ] Generate archives with correct naming

### 4. Unified Release Script
- [ ] Combine all steps into `release.sh`
- [ ] Add `--dry-run` flag for testing
- [ ] Add `--skip-npm` and `--skip-homebrew` for partial releases
- [ ] Clear progress output showing each step

### 5. Pre-Release Validation
- [ ] Verify clean git status
- [ ] Run full test suite (skip voice tests)
- [ ] Check NPM authentication
- [ ] Check GitHub CLI authentication

## Agent Instructions

Start by mapping the current scripts:
```bash
ls -la scripts/cicd/
cat scripts/cicd/local-cd.sh | head -100
```

Understand what's already automated vs. manual. Build incrementally - don't try to do everything in one session.

Session 1: Version bump + validation
Session 2: Full release script

## Success Criteria

- [ ] Single command releases
- [ ] No manual version file edits
- [ ] No manual GitHub release creation
- [ ] Dry-run works without side effects
- [ ] Clear error messages when prerequisites missing
