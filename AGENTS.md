# AGENTS.md - Terminal Jarvis

## Quick Reference

- **Main**: `src/main.rs`
- **ADK**: `adk/` (Go home screen)
- **Run**: `./jarvis.sh`
- **Local LLM**: `./scripts/tj-fast.sh` (gemma4:2b) or `./scripts/tj-local.sh` (gemma4:4b)

## Critical Rules

- Zero emojis anywhere (use [INSTALLED], [], [WARNING])
- CHANGELOG before deployment scripts
- Homebrew Formula before GitHub release
- Test-driven bugs: failing test first, then fix
- Version sync across Cargo.toml, package.json, Formula
- Run verify-change.sh before commits
- Use `/usr/bin/git` not `git`

## Deployment

- Local: `./scripts/cicd/local-ci.sh` then push to develop
- CI/CD: Push to develop triggers GitHub Actions
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
3. Run verify-change.sh
4. Push to develop
5. Monitor CI/CD pipeline

## Working Rules

- Stop and explain before major architectural changes
- One change per commit, commit before starting next
- Conventional commits: `type(scope): description`
