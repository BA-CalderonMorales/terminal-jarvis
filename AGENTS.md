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

## Branch Strategy

- **`develop`**: default base for PRs. Experimentation and quick iteration.
- **`main`**: tagged releases only. PRs merge into `develop` first, then
  `develop` fast-forwards into `main` at release time.
- **Feature branches**: branch from `develop`, PR against `develop`.

## CI

- Runs on every PR against `develop` or `main`.
- **Docs-only PRs** (changes limited to `docs/`, `README.md`, `AGENTS.md`,
  `CLAUDE.md`) skip CI automatically via `paths-ignore`. Trigger manually
  with `workflow_dispatch` when needed.
- The harness capability contract lives in
  [docs/harness-capability-contract.md](docs/harness-capability-contract.md).
  Keep it in sync when adding capabilities or commands.

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

<!-- gitnexus:start -->
# GitNexus — Code Intelligence

This project is indexed by GitNexus as **terminal-jarvis** (916 symbols, 1353 relationships, 55 execution flows). Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> Index stale? Run `node .gitnexus/run.cjs analyze` from the project root — it auto-selects an available runner. No `.gitnexus/run.cjs` yet? `npx gitnexus analyze` (npm 11 crash → `npm i -g gitnexus`; #1939).

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows. For regression review, compare against the default branch: `detect_changes({scope: "compare", base_ref: "main"})`.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `query({search_query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `context({name: "symbolName"})`.
- For security review, `explain({target: "fileOrSymbol"})` lists taint findings (source→sink flows; needs `analyze --pdg`).

## Never Do

- NEVER edit a function, class, or method without first running `impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `rename` which understands the call graph.
- NEVER commit changes without running `detect_changes()` to check affected scope.

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/terminal-jarvis/context` | Codebase overview, check index freshness |
| `gitnexus://repo/terminal-jarvis/clusters` | All functional areas |
| `gitnexus://repo/terminal-jarvis/processes` | All execution flows |
| `gitnexus://repo/terminal-jarvis/process/{name}` | Step-by-step execution trace |

## CLI

| Task | Read this skill file |
|------|---------------------|
| Understand architecture / "How does X work?" | `.claude/skills/gitnexus/gitnexus-exploring/SKILL.md` |
| Blast radius / "What breaks if I change X?" | `.claude/skills/gitnexus/gitnexus-impact-analysis/SKILL.md` |
| Trace bugs / "Why is X failing?" | `.claude/skills/gitnexus/gitnexus-debugging/SKILL.md` |
| Rename / extract / split / refactor | `.claude/skills/gitnexus/gitnexus-refactoring/SKILL.md` |
| Tools, resources, schema reference | `.claude/skills/gitnexus/gitnexus-guide/SKILL.md` |
| Index, status, clean, wiki CLI commands | `.claude/skills/gitnexus/gitnexus-cli/SKILL.md` |

<!-- gitnexus:end -->
