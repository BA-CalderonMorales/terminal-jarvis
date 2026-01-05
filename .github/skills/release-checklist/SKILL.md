# Skill: Release Checklist

**Name**: release-checklist
**Description**: Pre-release automation and checklist for hardening versions before tagging
**Trigger**: "Before tagging", "Harden release", "Pre-release checklist", "Make this tag official"

---

## Pre-Release Checklist

Execute these steps in order before making any version tag official:

### 1. CI/CD Verification

```bash
# Check all CI jobs are green
gh run list --limit 1 --json conclusion,status

# View specific run details
gh run view <run-id> --json jobs --jq '.jobs[] | "\(.name): \(.conclusion // .status)"'
```

**Required**: All jobs must show `success`

### 2. Version Synchronization

```bash
# Verify versions match across all distribution packages
grep '^version' Cargo.toml | head -1
grep '"version"' npm/terminal-jarvis/package.json | head -1
grep 'version "' homebrew/Formula/terminal-jarvis.rb | head -1
```

**Required**: All three must show identical version numbers

### 3. CHANGELOG Update

Ensure CHANGELOG.md contains:
- Correct version number and date
- All commits since last tag documented
- Sections: Added, Changed, Fixed, Technical (as applicable)

```bash
# List commits since last tag
git log --oneline $(git describe --tags --abbrev=0)..HEAD
```

### 4. README Synchronization

```bash
# Sync main README to npm package
cp README.md npm/terminal-jarvis/README.md
```

### 5. Documentation Accuracy

Verify these README sections are current:
- [ ] Project Structure module counts match actual `src/` directories
- [ ] Feature descriptions reflect current functionality
- [ ] WIP features have appropriate disclaimers
- [ ] Remote dev environment links are valid

### 6. Quality Gates

```bash
# Run full verification
./scripts/verify/verify-change.sh

# Or quick mode if tests already passed in CI
./scripts/verify/verify-change.sh --quick
```

### 7. Final Commit

```bash
git add -A
git commit -m "docs: prepare release vX.X.X"
git push
```

### 8. Wait for CI

```bash
# Monitor CI run
gh run watch
```

---

## Automated Pre-Release Script

For AI agents, execute this sequence:

```bash
# 1. Check CI status
gh run list --limit 1 --json conclusion --jq '.[0].conclusion'

# 2. Version check
echo "Cargo: $(grep '^version' Cargo.toml | head -1)"
echo "NPM: $(grep '"version"' npm/terminal-jarvis/package.json | head -1)"

# 3. Sync README
cp README.md npm/terminal-jarvis/README.md

# 4. Quick verify
./scripts/verify/verify-change.sh --quick

# 5. Commit if all pass
git add -A && git commit -m "docs: prepare release vX.X.X" && git push
```

---

## Post-Release Checklist

After tagging, verify distribution:

| Channel | Verification Command |
|---------|---------------------|
| NPM | `npm view terminal-jarvis version` |
| Cargo | `cargo search terminal-jarvis` |
| Homebrew | `brew info terminal-jarvis` |
| GitHub | `gh release view vX.X.X` |

---

## Common Issues

| Issue | Detection | Resolution |
|-------|-----------|------------|
| CI still running | `gh run list` shows `in_progress` | Wait for completion |
| Version mismatch | grep shows different versions | `./scripts/cicd/local-cd.sh --update-version X.X.X` |
| README out of sync | diff shows changes | `cp README.md npm/terminal-jarvis/README.md` |
| CHANGELOG missing entries | git log shows unlogged commits | Update CHANGELOG.md manually |
| Module counts wrong | ls shows different file counts | Update README Project Structure |

---

## Integration with Other Skills

- **verification**: Run quality gates before release
- **versioning**: Update version numbers across packages
- **deployment**: Execute actual tagging and release
- **homebrew**: Generate and publish Homebrew archives
- **npm**: Publish to NPM registry
