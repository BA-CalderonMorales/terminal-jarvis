# AGENTS.md - Terminal Jarvis

## Current Shape

- `src/` contains the slim Rust CLI for the new harness catalog model.
- `harnesses/` is the data plane for coding-agent harness capabilities.
- `docs/` is intentionally present for architecture, testing, migration, and
  release notes.
- `scripts/local-ci.sh`, `scripts/local-cd.sh`, and
  `scripts/package-release.sh` are the local release-prep path.
- The pre-rewrite implementation is intentionally pruned; use Git history for
  legacy reference.

## Rules

- Keep Rust source files at 100 lines or fewer.
- Keep module contracts in `src/contracts/`.
- Prefer data in `harnesses/*/*/index.toml` over Rust branches.
- Do not add a second Go ADK or another runtime beside the Rust CLI.
- Use no external Rust dependencies unless the tradeoff is documented first.
- Keep docs concise and tied to migration, architecture, testing, or release notes.
- Do not reintroduce a `current/` snapshot.
- Do not tag, publish, or upload release assets from local scripts without an
  explicit operator decision.
- Prefer remote or disposable development environments when exercising harness
  install, update, headless, or yolo commands. Keep secrets scoped and do not
  run unreviewed agent commands on a daily-driver machine.
