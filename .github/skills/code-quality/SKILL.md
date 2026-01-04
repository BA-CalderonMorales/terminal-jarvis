# Skill: Code Quality Standards

**Name**: code-quality
**Description**: Code quality gates, formatting, and best practices
**Trigger**: All code changes, before commits, code review

---

## Quality Gates (MANDATORY before commits)

```bash
cargo check                      # Must compile
cargo clippy -- -D warnings      # Must pass (no --all-features: avoids C++ deps)
cargo fmt --all                  # Must be formatted
cargo test                       # Must pass (if tests exist)
```

## Commit Message Format

```
<type>(<scope>): <description>

Types: fix, feat, break, docs, style, refactor, test, chore
```

### Examples

```bash
fix: resolve clippy warnings in api module
feat: add support for qwen-code tool
break: change cli argument structure for templates command
docs: update installation instructions
refactor(cli): extract tool execution to separate module
test: add E2E tests for main menu
```

## Rust Code Standards

- Use `anyhow::Result` for error handling
- Add doc comments for public functions
- Keep files under 200 lines (extract domains if larger)
- Follow domain-based module organization
- No `unwrap()` or `expect()` in production code (use proper error handling)

## TypeScript Code Standards

- Use Biome for linting/formatting (NOT ESLint)
- Run `npm run lint` and `npm run format` before committing
- Follow existing patterns in `npm/terminal-jarvis/`

## Pre-Commit Checklist

### Version Consistency
- [ ] Cargo.toml version matches target release
- [ ] npm/terminal-jarvis/package.json version matches
- [ ] homebrew/Formula/terminal-jarvis.rb version matches

### Documentation
- [ ] CHANGELOG.md updated with new version entry
- [ ] README.md reflects new features (if user-facing)
- [ ] Inline documentation updated for changed APIs

### Quality Checks
- [ ] `cargo check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --all` applied
- [ ] `cargo test` passes
- [ ] E2E tests pass (if modified): `cd e2e && npm test`

## Critical Rules

- **NO EMOJIS** - Zero emojis in code, commits, docs, output
- Use text-based indicators: "[INSTALLED]", "►", "•"
