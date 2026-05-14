# Contributing

Thanks for helping improve Terminal Jarvis. This repository is maintained with small, focused changes against `develop`.

## Before You Start

- Check the open issues for existing discussion or duplicate work.
- For bugs, add or update a failing test first when feasible.
- Keep one logical change per branch and commit.
- Use conventional commit messages such as `fix(cli): handle missing config`.
- Do not include emojis in code, documentation, commit messages, or generated output.

## Branch And Pull Request Flow

1. Branch from `develop`.
2. Make the smallest change that addresses the issue.
3. Update `CHANGELOG.md` before deployment or release-script changes.
4. Run focused tests for the changed area.
5. Run `./scripts/verify/verify-change.sh` before opening a pull request when feasible.
6. Open the pull request against `develop`.

## Expected Response Times

This is a small maintainer-led project. Expected response targets are:

- Security reports: initial response within 48 hours through the process in `SECURITY.md`.
- Reproducible bug reports: first maintainer response within 7 days.
- Feature requests and maintenance issues: triage within 14 days.
- Pull requests: first review or maintainer status update within 14 days.

If an issue needs external access, package-registry permissions, or third-party service state, maintainers may mark it blocked until that evidence is available.

## Local Validation

Use these commands from the repository root:

```bash
cargo test
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=cc RUSTFLAGS= ./scripts/verify/verify-change.sh --quick
```

For NPM wrapper changes, also run:

```bash
cd npm/terminal-jarvis
npm test
npm run build
npm run typecheck
```
