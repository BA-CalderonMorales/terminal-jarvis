# Skill: Multi-Repo Documentation Sync

**Name**: multi-repo  
**Description**: Cross-repository documentation synchronization pattern for projects with separate code and docs repos  
**Trigger**: "Sync docs across repos", "Cross-repo version sync", "Update external docs", "Multi-repo release"

---

## Overview

This skill provides a reusable pattern for projects that maintain:
- **Code repo**: Source code, releases, package management
- **Docs repo**: Documentation site (e.g., GitHub Pages, MkDocs, Zensical)

The pattern ensures version consistency and reduces release friction by automating cross-repo synchronization.

## Use Cases

| Scenario | Primary Repo | Docs Repo | Sync Trigger |
|----------|-------------|-----------|--------------|
| Terminal Jarvis | terminal-jarvis | my-life-as-a-dev | Version release |
| CLI tool + docs site | tool-repo | tool-docs.github.io | Every release |
| Library + API docs | library-repo | api-docs-site | Version tag |

---

## Pattern Structure

### Repository Layout

```
~/projects/
├── primary-repo/           # Code repository
│   ├── scripts/
│   │   ├── verify/
│   │   │   └── verify-docs.sh      # Validation script
│   │   └── sync-docs-site.sh       # Cross-repo sync script
│   └── .github/
│       └── skills/
│           └── multi-repo/         # This skill
│
└── docs-repo/              # Documentation repository
    └── docs/
        └── projects/
            └── primary-repo/
                ├── index.md        # Main docs (has version refs)
                └── releases.md     # Release notes
```

### Required Files in Primary Repo

**1. `scripts/verify/verify-docs.sh`**

Validates version consistency within the primary repo:

```bash
#!/usr/bin/env bash
# Usage: ./scripts/verify/verify-docs.sh [--fix] [--ci]
# Validates version consistency across all package files

set -e

# Get canonical version
get_canonical_version() {
    # Adapt to your package manager
    grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
    # OR: grep '"version"' package.json | head -1 | sed 's/.*"version": "\(.*\)".*/\1/'
}

# Check each file type
check_file_version() {
    local file="$1"
    local pattern="$2"
    local expected="$3"
    # Implementation...
}

# Main validation logic...
```

**2. `scripts/sync-docs-site.sh`**

Syncs version to external docs repo:

```bash
#!/usr/bin/env bash
# Usage: ./scripts/sync-docs-site.sh <version>
# Syncs version to external docs repository

DOCS_REPO="${DOCS_REPO:-$HOME/projects/docs-repo}"
VERSION="${1:-$(get_canonical_version)}"

# Update docs repo files
update_docs_index() {
    sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$VERSION/" "$DOCS_REPO/docs/index.md"
}

update_releases_page() {
    # Add new release section
    # Archive previous release
}

# Main sync logic...
```

---

## Implementation Steps

### Phase 1: Set Up Validation

```bash
# 1. Create verify script
cat > scripts/verify/verify-docs.sh << 'SCRIPT'
# [See template above]
SCRIPT
chmod +x scripts/verify/verify-docs.sh

# 2. Test it
./scripts/verify/verify-docs.sh
```

### Phase 2: Set Up Cross-Repo Sync

```bash
# 1. Ensure docs repo is cloned
if [[ ! -d ~/projects/docs-repo ]]; then
    git clone https://github.com/USERNAME/docs-repo.git ~/projects/docs-repo
fi

# 2. Create sync script
cat > scripts/sync-docs-site.sh << 'SCRIPT'
# [See template above]
SCRIPT
chmod +x scripts/sync-docs-site.sh

# 3. Test dry-run
./scripts/sync-docs-site.sh 0.0.0 --dry-run
```

### Phase 3: Integrate into Release Workflow

Add to your release script:

```bash
# In local-cd.sh or similar

# After version bump
./scripts/verify/verify-docs.sh --fix

# After successful validation
read -p "Sync to docs repo? (y/N): " sync_docs
if [[ "$sync_docs" =~ ^[Yy]$ ]]; then
    ./scripts/sync-docs-site.sh "$NEW_VERSION"
fi
```

---

## Adapting to Other Projects

### For Python Projects

**Canonical version**: `pyproject.toml`

```bash
get_canonical_version() {
    grep '^version = ' pyproject.toml | sed 's/version = "\(.*\)"/\1/'
}
```

**Files to check**:
- `pyproject.toml`
- `package.json` (if JS wrapper)
- `__init__.py` (if version there)
- `setup.py` (if legacy)

### For Node.js Projects

**Canonical version**: `package.json`

```bash
get_canonical_version() {
    grep '"version"' package.json | head -1 | sed 's/.*"version": "\(.*\)".*/\1/'
}
```

**Files to check**:
- `package.json`
- `package-lock.json`
- `src/index.ts` (version display)

### For Go Projects

**Canonical version**: Variable in main package

```bash
get_canonical_version() {
    grep 'Version = ' cmd/app/main.go | sed 's/.*Version = "\(.*\)".*/\1/'
}
```

**Files to check**:
- `go.mod`
- Main package version variable
- Homebrew formula

---

## Common Patterns

### Pattern 1: Version in Multiple Places

**Problem**: Version appears in 5+ files  
**Solution**: Single script updates all, validation catches drift

```bash
./scripts/cicd/update-version.sh 0.0.80  # Updates all files
./scripts/verify/verify-docs.sh          # Validates
```

### Pattern 2: External Docs Site

**Problem**: Docs repo shows outdated version  
**Solution**: Automated sync script

```bash
./scripts/sync-docs-site.sh 0.0.80
```

### Pattern 3: Release Checklist

**Problem**: Forgetting steps  
**Solution**: Skill with ordered phases

```bash
# Phase 1: Validate
./scripts/cicd/local-ci.sh
./scripts/verify/verify-docs.sh

# Phase 2: Version bump
./scripts/cicd/local-cd.sh --update-version 0.0.80

# Phase 3: External sync
./scripts/sync-docs-site.sh 0.0.80

# Phase 4: Commit and tag
git add -A && git commit -m "release: v0.0.80"
git tag v0.0.80 && git push origin develop && git push origin v0.0.80

# Phase 5: Publish
cargo publish
npm publish
```

---

## Troubleshooting

### "Docs repo not found"

```bash
# Set custom path
export DOCS_REPO=/path/to/docs-repo
./scripts/sync-docs-site.sh
```

### "Version pattern not matching"

Different projects use different version formats:

```bash
# Rust: version = "0.0.80"
sed -i 's/^version = ".*"/version = "'$VERSION'"/' Cargo.toml

# Node: "version": "0.0.80"
sed -i 's/"version": ".*"/"version": "'$VERSION'"/' package.json

# Python: version = "0.0.80"
sed -i 's/^version = ".*"/version = "'$VERSION'"/' pyproject.toml
```

### "Merge conflicts in docs repo"

```bash
cd ~/projects/docs-repo
git stash
git pull origin main
git stash pop
# Resolve conflicts, then re-run sync
```

---

## Integration with Other Skills

- **release**: Uses this pattern for terminal-jarvis releases
- **deployment**: Handles the publish phase after sync
- **verification**: Quality gates before version bumps

---

## Example: Complete Terminal Jarvis Setup

See these files for working implementation:

| File | Purpose |
|------|---------|
| `scripts/verify/verify-docs.sh` | Validate version consistency |
| `scripts/sync-docs-site.sh` | Sync to my-life-as-a-dev |
| `.github/skills/release/SKILL.md` | Full release workflow |

---

## Key Principles

1. **Single Source of Truth**: One file defines the canonical version
2. **Validation First**: Check before committing/publishing
3. **Automated Sync**: Scripts handle cross-repo updates
4. **Agent-Driven**: Designed for AI agent execution, not just CI
5. **Idempotent**: Re-run safely if something fails
