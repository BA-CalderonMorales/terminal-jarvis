# Terminal Jarvis

Terminal Jarvis is being simplified into a small Rust CLI that switches between
coding-agent harnesses through data contracts instead of hard-coded tool logic.

The v0.1 line is a breaking minor revision focused on the harness catalog,
local release checks, and compact distribution surfaces.

## Quick Start

```bash
cargo run -- list
cargo run -- show codex
cargo run -- plan codex headless
cargo run -- use opencode
cargo run -- current
```

Run the verification gate with:

```bash
scripts/verify.sh
```

Run the stronger local gate with security and package checks:

```bash
scripts/local-ci.sh
```

Exercise the local release asset shape without tagging or publishing:

```bash
scripts/local-cd.sh --check-auth
```

## Development Environment

Prefer a remote or disposable Linux workspace for Terminal Jarvis development:
Codespaces, a short-lived VM/container, or an SSH development host. Coding-agent
harnesses install binaries, inspect repositories, and can run delegated commands,
so avoid testing new harness plans directly on a daily-driver machine.

Keep provider tokens scoped to the work, mount only the repositories being
tested, and treat `headless`, `install`, `update`, and `yolo` capability plans as
commands to review before execution.

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

## Release Path

Cargo is the active development surface. Minimal npm and Homebrew source-build
surfaces are included for smoke testing, while `scripts/package-release.sh`
builds versioned archives, checksums, npm staging files, and generated Homebrew
formula output. Tagged releases use `.github/workflows/cd-multiplatform.yml` to
publish the crate, GitHub release assets, Homebrew tap update, and npm package.

See [docs/release-plan.md](docs/release-plan.md) for the v0.1 release checklist and
auth boundaries.
