# Skill: Homebrew Distribution

**Name**: homebrew
**Description**: Homebrew formula management and macOS/Linux distribution
**Trigger**: "Homebrew release", brew installation, macOS/Linux distribution

---

## Formula Location

`homebrew/Formula/terminal-jarvis.rb`

## Key Components

- Version must match Cargo.toml, package.json
- URL points to GitHub release archive
- SHA256 matches archive checksum
- Install script copies binary to bin/

## Release Archive Generation

```bash
# Generate platform-specific archives
./scripts/utils/generate-homebrew-release.sh --stage

# Output:
# homebrew/release/terminal-jarvis-mac.tar.gz
# homebrew/release/terminal-jarvis-linux.tar.gz
```

## Complete Homebrew Release Workflow

```bash
# 1. Standard deployment (includes version updates, CHANGELOG, Formula)
./scripts/cicd/local-ci.sh && ./scripts/cicd/local-cd.sh

# 2. Generate release archives
./scripts/utils/generate-homebrew-release.sh --stage

# 3. Commit archives
git add homebrew/release/
git commit -m "feat: Homebrew release archives vX.X.X"
git push

# 4. Create GitHub release with archives
gh release create vX.X.X \
  homebrew/release/terminal-jarvis-mac.tar.gz \
  homebrew/release/terminal-jarvis-linux.tar.gz \
  --title "Release vX.X.X" \
  --notes "Release notes from CHANGELOG.md" \
  --latest

# 5. Verify archive accessibility
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/vX.X.X/terminal-jarvis-mac.tar.gz

# 6. Test installation
brew uninstall terminal-jarvis
brew install terminal-jarvis
terminal-jarvis --version  # Should show vX.X.X
```

## Common Pitfalls

| Issue | Cause | Fix |
|-------|-------|-----|
| SHA256 mismatch | Formula SHA doesn't match archive | Regenerate archive or update Formula SHA |
| Archive naming | Incorrect filename pattern | Use generate-homebrew-release.sh script |
| Formula syntax error | Ruby syntax mistakes | Run `brew install --dry-run` to validate |
| Binary permissions | Binary not executable | Ensure chmod +x in install script |
| Cross-platform issue | macOS/Linux binary differences | Test on both platforms |
| Formula after release | Committed Formula changes too late | Always commit Formula BEFORE `gh release` |

## Critical Rule

**Formula BEFORE Release** - Always commit Homebrew Formula BEFORE creating GitHub release
