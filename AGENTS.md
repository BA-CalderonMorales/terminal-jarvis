# AGENTS.md - Terminal Jarvis

## Quick Reference

- **Main**: `src/main.rs`
- **ADK**: `adk/` (Go home screen)
- **Run**: `./jarvis.sh`
- **Local LLM**: `./scripts/tj-fast.sh` (gemma4:2b) or `./scripts/tj-local.sh` (gemma4:4b)

## Core Agent Loop

All harnesses share identical control flow (`adk/internal/chat/llm.go:Send()`):

```
while not done:
    1. Call LLM with current message context
    2. If text-only response → done
    3. If tool calls → execute, add results, continue
    4. If max turns exceeded → error
```

Max turns: 5 (`maxToolLoops`). Sequential tool execution.

## Critical Rules

- Zero emojis anywhere (use [INSTALLED], [], [WARNING])
- CHANGELOG before deployment scripts
- Homebrew Formula before GitHub release
- Test-driven bugs: failing test first, then fix
- Version sync across Cargo.toml, package.json, Formula
- Run verify-change.sh before commits
- Use `/usr/bin/git` not `git`
- Do not add or update a `docs/` folder in this repo; Terminal Jarvis project docs live in `../my-life-as-a-dev/docs/projects/active/terminal-jarvis`
- Keep issue status in GitHub issues or local `handoffs/`; do not add `.github/issue-updates/*.md` unless explicitly requested

## Deployment

- Local: `./scripts/cicd/local-ci.sh` then push to develop
- CI: Push to develop triggers GitHub Actions (`.github/workflows/ci.yml`)
- **CD/Release: Push a `v*` tag triggers `.github/workflows/cd-multiplatform.yml`**
  - Builds binaries for macOS (x86_64, aarch64) and Linux (x86_64, aarch64)
  - Publishes to crates.io (requires `CARGO_REGISTRY_TOKEN` secret)
  - Creates GitHub release with all binary assets
  - Updates `homebrew-terminal-jarvis` Formula automatically
- Version only: `./scripts/cicd/local-cd.sh --update-version X.X.X`

## Cross-Repo

- Related: agent-harness (Go), lumina-bot (Go gateway), claude-termux (JS CLI)
- Commands: `harness-status`, `sync-philosophy`

## Environment Variables

- `OPENROUTER_API_KEY`: Cloud provider API key
- `JARVIS_MODEL`: Explicit model string
- `OLLAMA_HOST`: Local Ollama endpoint (default: http://localhost:11434)

## Testing

- `cargo test`
- `./scripts/cicd/local-ci.sh`

## Release Checklist

1. Update CHANGELOG.md
2. Update version in Cargo.toml, package.json, homebrew/Formula
3. Run verify-change.sh (or rely on CI green status)
4. Push to develop via PR
5. Ensure CI on develop is green
6. **Push version tag** (`git tag vX.X.X && git push origin vX.X.X`)
   - Triggers `.github/workflows/cd-multiplatform.yml`
   - CI builds multi-platform binaries, publishes to crates.io,
     creates GitHub release, and updates `homebrew-terminal-jarvis` Formula
7. Monitor CD pipeline
8. Merge `develop` into `main`
9. Verify `homebrew-terminal-jarvis` repo has the updated Formula

## Working Rules

- Stop and explain before major architectural changes
- One change per commit, commit before starting next
- Conventional commits: `type(scope): description`
