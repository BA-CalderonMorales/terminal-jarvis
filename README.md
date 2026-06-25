# Terminal Jarvis

Terminal Jarvis is being simplified into a small Rust CLI that switches between
coding-agent harnesses through data contracts instead of hard-coded tool logic.

This branch is the first breaking minor revision. The old implementation is
pruned from this PR so review can focus on the lean v0.1 root.

## Quick Start

```bash
cargo run -- list
cargo run -- show codex
cargo run -- plan codex headless
cargo run -- use opencode
cargo run -- current
```

Run the branch verification gate with:

```bash
scripts/verify.sh
```

Run the stronger pre-PR gate with local security and package checks:

```bash
scripts/local-ci.sh
```

## New Layout

```text
terminal-jarvis/
├── docs/           # redesign notes and migration guidance
├── harnesses/      # data contracts for coding-agent harness capabilities
├── src/            # slim Rust CLI
└── tests/          # behavior and contract tests
```

The initial catalog promotes 25 coding-agent harnesses into a shared descriptor
shape. Each harness owns the same capability folders:

```text
harnesses/<harness>/{download,update,headless,version,stats,models,security,yolo,ui}/index.toml
```

The CLI loads those files, validates that every harness exposes the full
capability contract, prints setup guidance, stores the active harness, and can
run a selected capability command when the user asks for it.

Auth setup is tracked at the harness level. A harness can require no key, one of
several provider keys, or all listed keys. That keeps setup guidance accurate
without forcing users to configure every provider a tool supports.

## Design Goals

- Keep Rust source files at 100 lines or fewer.
- Keep dependencies at zero until a dependency proves its value.
- Keep user context local and explicit.
- Prefer harness data over Rust conditionals.
- Keep the v0.1 root easy to inspect while this minor revision breaks old
  interfaces deliberately.
- Aim for 90 percent or better line coverage and mutation score as the CLI
  stabilizes.

## Release Status

This branch is not ready for packaged release. Cargo is the active development
surface. Minimal npm and Homebrew source-build surfaces exist for smoke testing
until versioned release artifacts are rebuilt through
[docs/release-plan.md](docs/release-plan.md).
