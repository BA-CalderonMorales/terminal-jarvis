# AGENTS.md - Terminal Jarvis Minor Revision

## Current Shape

- `src/` contains the slim Rust CLI for the new harness catalog model.
- `harnesses/` is the data plane for coding-agent harness capabilities.
- `docs/` is intentionally present for this breaking minor revision.
- The pre-rewrite implementation is intentionally pruned from this PR; use Git
  history for legacy reference.

## Rules

- Keep Rust source files at 100 lines or fewer.
- Keep module contracts in `src/contracts/`.
- Prefer data in `harnesses/*/*/index.toml` over Rust branches.
- Do not add a second Go ADK or another runtime beside the Rust CLI.
- Use no external Rust dependencies unless the tradeoff is documented first.
- Keep docs concise and tied to migration, architecture, testing, or release notes.
- Do not reintroduce a `current/` snapshot in this PR.
