# Maintainer Guide

## Branch Strategy

- **`develop`**: default base for PRs. Experimentation and quick iteration.
- **`main`**: tagged releases only. PRs merge into `develop` first, then
  `develop` fast-forwards into `main` at release time.
- **Feature branches**: branch from `develop`, PR against `develop`.

## CI

Runs on every PR against `develop` or `main`. Docs-only changes (limited to
`docs/`, `README.md`, `AGENTS.md`, `CLAUDE.md`) skip CI automatically.
Trigger manually via `workflow_dispatch` when needed.

## Verification

```bash
scripts/verify.sh              # fmt, lint, tests, shape, hardening, security
scripts/local-ci.sh            # stronger gate with repository hygiene
scripts/local-cd.sh --check-auth  # release asset shape without publishing
```

`verify.sh` runs formatting, clippy, tests, the 100-line Rust file invariant,
harness catalog shape checks, CLI smoke checks, the integration hardening
matrix, security tooling, npm audit, npm wrapper smoke checks, Homebrew
formula syntax, and coverage/mutation gates.

## Release

Cargo is the primary distribution. npm and Homebrew surfaces are generated
from the same release build.

```bash
scripts/package-release.sh build dist/
scripts/local-cd.sh --check-auth
```

Tagged releases (`v*`) trigger `.github/workflows/cd-multiplatform.yml`:
crate publish, Linux/macOS/Windows GitHub release assets, Homebrew tap update,
and npm package.

See [release-plan.md](release-plan.md) for the checklist and auth boundaries.

## File Discipline

- Keep Rust source files at 100 lines or fewer.
- Prefer `harnesses/*/*/index.toml` data over Rust conditionals.
- Keep dependencies at zero until one proves its value.
- Keep docs concise and tied to migration, architecture, testing, or release.

## Environment

Use a remote or disposable Linux workspace when exercising harness install,
update, headless, or yolo commands. Harnesses install binaries, inspect
repositories, and can run delegated commands. Keep provider tokens scoped
and do not run unreviewed agent commands on a daily-driver machine.
