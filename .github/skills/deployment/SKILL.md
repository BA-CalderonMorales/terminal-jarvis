# Skill: Deployment

**Name**: deployment
**Description**: Deployment workflows, CI/CD automation, and release procedures
**Trigger**: "Let's deploy", "Run local-cd.sh", releasing new versions

---

## Standard Deployment Workflow

```bash
# 1. Pre-flight (MANDATORY)
git status                                    # Must be clean
./scripts/cicd/local-cd.sh --check-versions  # Must pass

# 2. Update CHANGELOG.md (REQUIRED FIRST)
## [X.X.X] - YYYY-MM-DD
### Added / Enhanced / Fixed / Technical

# 3. Deploy
./scripts/cicd/local-ci.sh      # Validate (no commits)
./scripts/cicd/local-cd.sh      # Deploy (commits, tags, pushes)
```

## Version Update Workflow

```bash
# Determine increment: 0.0.X (fix) | 0.X.0 (feature) | X.0.0 (breaking)
./scripts/cicd/local-cd.sh --update-version X.X.X
./scripts/cicd/local-cd.sh --check-versions
./scripts/cicd/local-ci.sh && ./scripts/cicd/local-cd.sh
```

## Homebrew Release Workflow

```bash
# 1. Complete standard deployment first
# 2. Generate archives
./scripts/utils/generate-homebrew-release.sh --stage
git add homebrew/release/ && git commit -m "feat: Homebrew archives vX.X.X" && git push

# 3. Create GitHub release
gh release create vX.X.X \
  homebrew/release/terminal-jarvis-mac.tar.gz \
  homebrew/release/terminal-jarvis-linux.tar.gz \
  --title "Release vX.X.X" --notes "..." --latest

# 4. Verify
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/vX.X.X/terminal-jarvis-mac.tar.gz
```

## Deployment Failure Prevention

| Failure | Prevention | Fix |
|---------|-----------|-----|
| Uncommitted changes | `git status` first | Commit or stash |
| Version mismatch | `--check-versions` first | `--update-version X.X.X` |
| Missing CHANGELOG | Update before scripts | Add entry manually |
| Formula after release | Commit Formula before `gh release` | Delete release, fix, recreate |
| NPM auth failure | `npm whoami` check | `npm login` |

## Critical Rules

1. **CHANGELOG FIRST** - Update CHANGELOG.md BEFORE deployment scripts
2. **Formula BEFORE Release** - Commit Homebrew Formula BEFORE GitHub release
3. **Version Sync** - Update Cargo.toml, package.json, Formula together
