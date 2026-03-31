# Skill: Release with Docs Sync

**Name**: release  
**Description**: Complete agent-driven release process with cross-repo docs synchronization  
**Trigger**: "Release v0.0.X", "Cut new release", "Publish release with docs sync", "Agent release"

---

## Overview

This skill provides a bulletproof, agent-driven release process that keeps all documentation in sync without relying on GitHub Actions. Since CI can fail, this process is designed to be executed entirely by an agent with manual verification at each step.

## Prerequisites

- Terminal Jarvis repository cloned locally
- my-life-as-a-dev repository cloned at `~/projects/my-life-as-a-dev`
- Git configured with push access to both repos
- All changes for the release merged to `develop` branch

## Release Checklist

### Phase 1: Pre-Release Validation

**Purpose**: Ensure codebase is ready for release

```bash
# 1. Navigate to terminal-jarvis repo
cd ~/projects/terminal-jarvis

# 2. Ensure you're on develop branch and up to date
git checkout develop
git pull origin develop

# 3. Run full verification
./scripts/cicd/local-ci.sh
```

**Validation**: All checks must pass. If not, fix issues before proceeding.

---

### Phase 2: Version Bump

**Purpose**: Update version across all files

```bash
# 1. Check current version
./scripts/cicd/local-cd.sh --check-versions

# 2. Run the version bump (choose: patch/minor/major)
# For patch release (0.0.X+1):
./scripts/cicd/local-cd.sh --update-version 0.0.80
```

**What This Updates**:
- `Cargo.toml` and `Cargo.lock`
- `npm/terminal-jarvis/package.json`
- `npm/terminal-jarvis/src/index.ts`
- `adk/internal/ui/theme.go`
- `homebrew/Formula/terminal-jarvis.rb` (version + URLs)

---

### Phase 3: Docs Drift Prevention

**Purpose**: Ensure all documentation is synchronized

```bash
# 1. Run docs validation
./scripts/verify/verify-docs.sh

# 2. If issues found, auto-fix them
./scripts/verify/verify-docs.sh --fix

# 3. Verify fixes
./scripts/verify/verify-docs.sh
```

**Checks Performed**:
- Version consistency across all files
- README.md synced to npm package
- CHANGELOG.md has entry for new version
- Homebrew formula URLs match version
- Cargo.lock is in sync

---

### Phase 4: Update CHANGELOG.md

**Purpose**: Document changes for this release

```bash
# 1. Check if CHANGELOG entry exists
grep "\[0.0.80\]" CHANGELOG.md

# 2. If not, add entry at the top (after the header)
# Edit CHANGELOG.md and add:
#
# ## [0.0.80] - YYYY-MM-DD
#
# ### Added
# - New features
#
# ### Fixed
# - Bug fixes
#
# ### Changed
# - Improvements
```

---

### Phase 5: Sync External Documentation

**Purpose**: Update my-life-as-a-dev docs site

```bash
# 1. Navigate to docs repo
cd ~/projects/my-life-as-a-dev

# 2. Ensure you're on main branch
git checkout main
git pull origin main

# 3. Update the main index.md
# Edit docs/projects/active/terminal-jarvis/index.md
# Update:
#   - Signal box: Status line with new version
#   - Highlights section mentioning new version
```

**Update releases.md**:

```bash
# Edit docs/projects/active/terminal-jarvis/details/releases.md
#
# Move current "Latest Release" to a "Previous Releases" section
# Add new "Latest Release: v0.0.80" section at the top
# Include:
#   - Published date
#   - GitHub Release link
#   - Crates.io link
#   - Highlights (key changes)
#   - Tool wrappers list
```

**Quick Update Template for releases.md**:

```markdown
## Latest Release: v0.0.80

- **Published**: [Current Date]
- **GitHub Release**: [v0.0.80](https://github.com/BA-CalderonMorales/terminal-jarvis/releases/tag/v0.0.80)
- **Crates.io Version**: [0.0.80](https://crates.io/crates/terminal-jarvis)

### Highlights

- [Key change 1]
- [Key change 2]
- [Key change 3]

### Supported Tool Wrappers Updated in v0.0.80

- `amp`
- `claude`
- `codex`
- `crush`
- `gemini`
- `goose`
- `llxprt`
- `opencode`
- `qwen`

### Upgrade Recommendation

If you are on versions older than `v0.0.80`, upgrade:

```bash
npm update -g terminal-jarvis
```
```

**Commit docs changes**:

```bash
cd ~/projects/my-life-as-a-dev

git add docs/projects/active/terminal-jarvis/
git commit -m "docs(terminal-jarvis): update for v0.0.80 release

- Updated main index with v0.0.80 signal
- Added v0.0.80 release notes
- Synced highlights and tool wrappers list"

git push origin main
```

---

### Phase 6: Build and Test

**Purpose**: Create release artifacts

```bash
# 1. Back to terminal-jarvis
cd ~/projects/terminal-jarvis

# 2. Build release binary
cargo build --release

# 3. Verify version
./target/release/terminal-jarvis --version
# Expected: terminal-jarvis 0.0.80

# 4. Run smoke tests
./scripts/verify/verify-cli.sh

# 5. Build npm package
cd npm/terminal-jarvis && npm run build && cd ../..
```

---

### Phase 7: Commit and Tag

**Purpose**: Lock in the release

```bash
# 1. Stage all changes
git add -A

# 2. Commit with conventional commit
git commit -m "release: v0.0.80

- Version bump to 0.0.80
- Updated CHANGELOG.md
- Synced documentation
- Built release artifacts"

# 3. Create tag
git tag v0.0.80

# 4. Push to GitHub
git push origin develop
git push origin v0.0.80
```

---

### Phase 8: Publish to Registries

**Purpose**: Make release available to users

**Crates.io**:

```bash
# Publish Rust crate
cargo publish

# Verify
cargo search terminal-jarvis
```

**NPM** (requires 2FA):

```bash
cd npm/terminal-jarvis

# Publish
npm publish --otp=<YOUR_2FA_CODE>

# Add dist-tags
npm dist-tag add terminal-jarvis@0.0.80 beta
npm dist-tag add terminal-jarvis@0.0.80 stable

# Verify
npm view terminal-jarvis versions --json | tail -5
npm dist-tag ls terminal-jarvis
```

**GitHub Release**:

```bash
# Generate Homebrew archives first
./scripts/utils/generate-homebrew-release.sh --stage

# Create GitHub release with assets
gh release create v0.0.80 \
  homebrew/release/terminal-jarvis-mac.tar.gz \
  homebrew/release/terminal-jarvis-linux.tar.gz \
  target/release/terminal-jarvis \
  --title "Release v0.0.80" \
  --notes-file <(sed -n '/## \[0.0.80\]/,/^## \[/p' CHANGELOG.md | head -n -1) \
  --latest
```

---

### Phase 9: Post-Release Verification

**Purpose**: Confirm release is live and working

```bash
# Test Cargo install
cargo install terminal-jarvis --force
terminal-jarvis --version

# Test NPM install
npm install -g terminal-jarvis@0.0.80
terminal-jarvis --version

# Verify GitHub release
gh release view v0.0.80

# Check my-life-as-a-dev site deployed
# (May take a few minutes if using GitHub Pages)
curl -s https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/ | grep -o 'v0.0.80'
```

---

## Quick Release (Streamlined)

For most releases, use this condensed workflow:

```bash
cd ~/projects/terminal-jarvis

# 1. Verify and build
./scripts/cicd/local-ci.sh

# 2. Full release (includes version bump, docs validation, external sync)
./scripts/cicd/local-cd.sh
# - Select patch/minor/major when prompted
# - Docs validation runs automatically
# - Offers to sync my-life-as-a-dev site

# 3. Publish
cargo publish
cd npm/terminal-jarvis && npm publish --otp=<CODE> && cd ../..
gh release create v0.0.80 homebrew/release/*.tar.gz --generate-notes
```

**Time saved:** ~10-15 minutes per release through automation

---

## Quick Reference: Common Scenarios

### Hotfix Release

```bash
cd ~/projects/terminal-jarvis

# 1. Fix the bug
# ... edit files ...

# 2. Bump version
./scripts/cicd/local-cd.sh --update-version 0.0.81

# 3. Validate docs
./scripts/verify/verify-docs.sh

# 4. Update CHANGELOG.md

# 5. Sync docs site
./scripts/sync-docs-site.sh 0.0.81

# 6. Commit and publish
git add -A
git commit -m "hotfix: [description] - v0.0.81"
git tag v0.0.81 && git push origin develop && git push origin v0.0.81
cargo publish
cd npm/terminal-jarvis && npm publish && cd ../..
```

### Docs-Only Update

If only updating documentation (no code change):

```bash
# Skip version bump, just update docs
cd ~/projects/my-life-as-a-dev
# ... update docs ...
git add -A
git commit -m "docs(terminal-jarvis): [description]"
git push origin main
```

---

## Verification Commands

| Check | Command |
|-------|---------|
| Version in Cargo | `grep '^version' Cargo.toml` |
| Version in NPM | `grep '"version"' npm/terminal-jarvis/package.json` |
| Version in ADK | `grep 'Version' adk/internal/ui/theme.go` |
| Version in Homebrew | `grep 'version' homebrew/Formula/terminal-jarvis.rb` |
| CHANGELOG entry | `grep '\[0.0.80\]' CHANGELOG.md` |
| Docs site version | Check my-life-as-a-dev index.md |
| Built binary | `./target/release/terminal-jarvis --version` |
| Crates.io | `cargo search terminal-jarvis` |
| NPM registry | `npm view terminal-jarvis version` |

---

## Troubleshooting

### Version Mismatch After Bump

If versions are inconsistent after bump:

```bash
# Re-run version update
./scripts/cicd/local-cd.sh --update-version 0.0.80

# Or manually fix specific files:
sed -i 's/"version": ".*"/"version": "0.0.80"/' npm/terminal-jarvis/package.json
sed -i 's/Version = "v.*"/Version = "v0.0.80"/' adk/internal/ui/theme.go
```

### Homebrew URL Mismatch

If Homebrew formula has wrong URLs:

```bash
# Fix URLs in formula
sed -i 's|download/v[0-9.]*/|download/v0.0.80/|g' homebrew/Formula/terminal-jarvis.rb

# Fix version
sed -i 's/version "[0-9.]*"/version "0.0.80"/' homebrew/Formula/terminal-jarvis.rb
```

### CHANGELOG Missing Entry

```bash
# Add entry manually
sed -i '9a\
## [0.0.80] - '$(date +%Y-%m-%d)'\
\
### Added\
- Release features\
\
' CHANGELOG.md
```

---

## Integration with Other Skills

- **release-checklist**: Legacy checklist skill (use this skill instead)
- **verification**: Run quality gates before release
- **versioning**: Used internally by this skill
- **deployment**: GitHub release creation
- **homebrew**: Formula updates
- **npm**: Package publishing

---

## Key Principles

1. **Agent-First**: This process is designed for AI agents, not CI
2. **Verification at Every Step**: Never assume, always verify
3. **Cross-Repo Sync**: Keep both repos in lockstep
4. **Document Immediately**: Update docs as part of release, not after
5. **Idempotent**: Can re-run safely if something fails
