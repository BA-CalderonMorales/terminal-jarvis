# Verification Feedback Loop

**Purpose**: Give AI coding agents (Claude, Copilot, etc.) a way to verify their work.

> "The most important thing to get great results out of Claude Code -- give Claude a way to verify its work. If Claude has that feedback loop, it will 2-3x the quality of the final result."
> -- Creator of Claude Code

## Quick Start

```bash
# Run full verification suite
./scripts/verify/verify-change.sh

# Run specific checks
./scripts/verify/verify-build.sh      # Compilation only
./scripts/verify/verify-quality.sh    # Clippy + formatting
./scripts/verify/verify-tests.sh      # Unit + E2E tests
./scripts/verify/verify-cli.sh        # CLI smoke tests
```

## For AI Agents

When making changes to Terminal Jarvis:

1. **After each code change**: Run `./scripts/verify/verify-build.sh`
2. **Before committing**: Run `./scripts/verify/verify-change.sh`
3. **After fixing bugs**: Run `./scripts/verify/verify-tests.sh`
4. **After UI changes**: Run `./scripts/verify/verify-cli.sh`

## Verification Steps

| Script | What it checks | When to use |
|--------|----------------|-------------|
| `verify-build.sh` | `cargo check` compiles | After any Rust code change |
| `verify-quality.sh` | Clippy warnings, formatting | Before commits |
| `verify-tests.sh` | Unit tests, E2E tests | After bug fixes, new features |
| `verify-cli.sh` | CLI runs, shows menus, help works | After UX changes |
| `verify-change.sh` | All of the above | Before any commit |

## Exit Codes

- `0` - All checks passed
- `1` - Verification failed (check output for details)

## Integration with CI/CD

These scripts are also used by:
- `scripts/cicd/local-ci.sh` - Pre-commit validation
- `scripts/cicd/local-cd.sh` - Deployment pipeline
- GitHub Actions workflows

## Adding New Verifications

To add a new verification step:

1. Create `scripts/verify/verify-<name>.sh`
2. Make it executable: `chmod +x scripts/verify/verify-<name>.sh`
3. Add it to `verify-change.sh`
4. Document in this README
