# Skill: Verification Feedback Loop

**Name**: verification
**Description**: Quality verification system for validating changes before commits
**Trigger**: Before commits, after code changes, bug fixes, UX changes

---

## Overview

> "Give Claude a way to verify its work. If Claude has that feedback loop, it will 2-3x the quality of the final result."
> -- Creator of Claude Code

This skill provides a structured approach to verifying code changes through automated checks.

## Quick Commands

```bash
# Full verification (run before any commit)
./scripts/verify/verify-change.sh

# Quick mode (skip tests, faster iteration)
./scripts/verify/verify-change.sh --quick

# Individual checks
./scripts/verify/verify-build.sh      # Compilation only
./scripts/verify/verify-quality.sh    # Clippy + formatting
./scripts/verify/verify-tests.sh      # Unit + E2E tests
./scripts/verify/verify-cli.sh        # CLI smoke tests
```

## When to Verify

| Situation | Command | Why |
|-----------|---------|-----|
| After any code change | `verify-build.sh` | Catch compile errors immediately |
| After fixing a bug | `verify-tests.sh` | Ensure fix works, no regressions |
| After UX/UI changes | `verify-cli.sh` | Verify CLI behaves correctly |
| Before committing | `verify-change.sh` | Full quality gate |
| Quick iteration | `verify-change.sh --quick` | Fast feedback loop |

## Verification-Driven Development Workflow

1. **Make a change**
2. **Run verification** - `./scripts/verify/verify-build.sh`
3. **If failed** - Fix and repeat step 2
4. **If passed** - Continue with next change
5. **Before commit** - Run full `./scripts/verify/verify-change.sh`

## Exit Codes

- `0` - All checks passed, safe to commit
- `1` - Verification failed, review output and fix issues

## Integration

These scripts are also used by:
- `scripts/cicd/local-ci.sh` - Pre-commit validation
- `scripts/cicd/local-cd.sh` - Deployment pipeline
- GitHub Actions workflows
