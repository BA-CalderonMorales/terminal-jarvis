# Skill: Git Workflow

**Name**: git-workflow
**Description**: Branching strategy, merge rules, and git best practices
**Trigger**: All git operations, branching, merging, commits

---

## Critical Rule

**NEVER commit directly to main or develop** - Always use feature branches.

## Branch Flow

```
feature/*, bugfix/*, security/*, pipeline/*
                    |
                    v
                develop
                    |
                    v
                  main
```

## Branch Naming

| Prefix | Use Case | Example |
|--------|----------|---------|
| `feature/` | New functionality | `feature/add-aider-support` |
| `bugfix/` | Bug fixes | `bugfix/auth-flow-codex` |
| `security/` | Security patches | `security/api-key-exposure` |
| `pipeline/` | CI/CD changes | `pipeline/update-gh-actions` |

## Merge Rules

**Contributors:** `feature/* -> develop` (via PR)

**Admin Release:** `develop -> main` (direct merge after PR reviews complete)

## Git Best Practices

### Use Full Paths
```bash
# GOOD
/usr/bin/git status

# BAD (may invoke aliases)
git status
```

### Commit Messages
```
<type>(<scope>): <description>

Types: fix, feat, break, docs, style, refactor, test, chore
```

### Agent Attribution
When AI agents contribute to commits:
```bash
docs(readme): update installation - @documentation-specialist
feat(auth): implement OAuth - @security-specialist
refactor(cli): extract domains - @software-architect
```

## Typical Workflow

```bash
# 1. Create feature branch
/usr/bin/git checkout -b feature/my-feature develop

# 2. Make changes and commit
/usr/bin/git add -A
/usr/bin/git commit -m "feat: add new feature"

# 3. Push branch
/usr/bin/git push -u origin feature/my-feature

# 4. Create PR to develop
# 5. After review, merge to develop
# 6. Admin merges develop to main for releases
```
