# Skill: Version Management

**Name**: versioning
**Description**: Semantic versioning and multi-platform version synchronization
**Trigger**: "Update version", version bumps, release preparation

---

## Semantic Versioning

Format: `MAJOR.MINOR.PATCH`

| Increment | When | Example |
|-----------|------|---------|
| `0.0.X` | Bug fixes, docs, small improvements | 0.0.70 -> 0.0.71 |
| `0.X.0` | New features (no breaking changes) | 0.0.71 -> 0.1.0 |
| `X.0.0` | Breaking changes | 0.1.0 -> 1.0.0 |

## Files to Update (ALL THREE - CRITICAL)

1. `Cargo.toml` - Rust package version
2. `npm/terminal-jarvis/package.json` - NPM package version
3. `homebrew/Formula/terminal-jarvis.rb` - Homebrew formula version (COMMONLY FORGOTTEN)

## Update Command

```bash
# Automated version update
./scripts/cicd/local-cd.sh --update-version X.X.X

# Verify all versions match
./scripts/cicd/local-cd.sh --check-versions
```

## CHANGELOG.md Structure

```markdown
## [X.X.X] - YYYY-MM-DD
### Added
- New user-visible features
### Enhanced
- Improvements to existing features
### Fixed
- Bug fixes and corrections
### Technical
- Internal changes (refactoring, tests, infrastructure)
```

## Version Update Rules

1. Update CHANGELOG.md BEFORE running deployment scripts
2. One release = one cohesive feature set
3. Match actual work timeline (don't mix unrelated features)
4. Always verify with `--check-versions` before deploying
